# vLLM

When deploying LLMs (Large Language Models) with billions of parameters to production, is the GPU OOM (Out Of Memory) error simply due to the model's size?

In reality, the KV cache, which is temporary data stored to remember previous contexts each time the model generates text, wastes and occupies more than half of the memory.

If this massive and fragmented memory leak isn't addressed, even a GPU costing tens of thousands of dollars won't yield good throughput. How should this be handled systemically?

The solution introduced to solve this problem is **vLLM**, an open-source LLM inference engine developed by UC Berkeley researchers.

It borrows the virtual memory paging technique from operating systems to drastically reduce memory waste and maximize throughput.

- **vLLM**: A high-performance framework designed for fast and efficient inference and serving of large language models.
- **KV Cache (Key-Value Cache)**: A technique where Transformer-based models, when generating sentences, cache the attention results (key, value tensors) of previously computed tokens in memory and reuse them for subsequent token generation to prevent redundant computations.
- **PagedAttention**: vLLM's core technology that applies OS virtual memory paging algorithms to KV cache management. It divides the cache into fixed-size blocks and flexibly allocates them to non-contiguous physical memory spaces.
- **Continuous Batching (or In-flight Batching)**: A scheduling technique that, instead of static batch processing, eliminates GPU idle time by immediately sending out completed requests and incorporating new requests into the batch queue as each token is generated.

<br>

## The Problem to be Solved

- **Internal Fragmentation of KV Cache**: Existing frameworks (e.g., Hugging Face Transformers) pre-allocate a large memory block by assuming the maximum possible generation length (e.g., 2048 tokens) for new requests due to memory contiguity. If the actual generated sentence is shorter, the remaining memory space cannot be used by other requests and is wasted.
- **GPU Waiting Overhead in Static Batching**: Once a batch is formed and GPU computation begins, the entire batch cannot complete until the generation of the longest sentence within that batch finishes. Requests that required short sentences, even if their computation is already done, must wait while occupying GPU memory and threads.

In other words, existing frameworks suffered from internal fragmentation where unused space was wasted due to allocating contiguous memory based on a maximum length assumption, and static batching led to limitations where GPU cores remained idle while waiting for the longest text generation to complete.

For example, consider this problem: when four users simultaneously query an LLM, the existing system forcibly allocates 2000 characters worth of contiguous VRAM to all four, causing an Out Of Memory (OOM) error.

Furthermore, if one user requests 1000 characters and three users request 10 characters, the three requests that finished generating 10 characters cannot be returned immediately. The GPU remains idle, unable to accept new requests, until the remaining user's 1000-character generation is complete.

### The Solution

- **PagedAttention (Zero Fragmentation)**: vLLM does not require contiguous memory for the KV cache. It divides the entire VRAM into fixed-size page blocks and dynamically allocates them by mapping logical blocks to physical blocks (block table) only when memory is actually needed as tokens are generated. This reduces memory waste from over 60% to less than 4%.
- **Continuous Batching (Dynamic Parallelization)**: A step-by-step scheduler intervenes each time the LLM outputs a token. When a request for 10 characters is completed, the result is immediately returned to the client, and a new request from user #5, which was in the queue, is inserted into the empty slot, allowing the next token computation to proceed without delay.

<br>

## Detailed Operating Principles and Structure

This is vLLM's internal architecture for managing memory and processing requests.

1.  **Memory Allocation (Block-Unit Allocation)**: It operates identically to OS paging, defining logical blocks by grouping KV tensors of contiguous tokens into a certain number (e.g., 16 tokens), and managing physical GPU memory by splitting it into physical blocks of the same size.
2.  **Block Table Mapping**: vLLM maintains a Block Table for each user request. For example, logical block 0 might be allocated to physical block 105, and logical block 1 to physical block 12. Even if the array is physically scattered, the table accurately tracks the order.
3.  **Copy on Write (Memory Sharing)**: If multiple users use the same system prompt, vLLM stores the KV cache for that prompt in only one physical block, and multiple users' Block Tables point to it. Memory is duplicated only when a specific user begins generating a unique response, achieving extreme memory savings.
4.  **Token Generation (Scheduling)**: In each iteration, the Scheduler evaluates the number of currently available physical blocks. If space is insufficient, it pushes lower-priority requests to CPU memory (swapping); if space becomes available, it brings them back to VRAM. This is called preemption.

### Example

This is the basic logic for performing batch processing by directly running the PagedAttention engine within Python code, without needing to launch a background server or a network API server.

```py
from vllm import LLM, SamplingParams

def run_vllm_offline():
    """vLLM 엔진을 메모리에 로드하고 다수의 프롬프트를 일괄 병렬 처리합니다."""
    
    prompts = [
        "What is the capital of France?",
        "Explain the theory of relativity in simple terms.",
        "Write a short python code to reverse a string."
    ]

    # 1. Define sampling parameters (temperature, max generation length, etc.)
    # Unlike existing frameworks, setting max_tokens to 1000 does not immediately occupy actual memory.
    sampling_params = SamplingParams(temperature=0.8, top_p=0.95, max_tokens=100)

    # 2. Initialize vLLM engine and load model
    # Internally, it scans GPU memory and prepares block allocation for PagedAttention.
    print("Loading LLM Engine...")
    llm = LLM(model="facebook/opt-125m") 

    # 3. Execute inference
    # The internal scheduler applies Continuous Batching to process the prompts list at high speed.
    outputs = llm.generate(prompts, sampling_params)

    # 4. Print results
    for output in outputs:
        prompt = output.prompt
        # Extract the final generated text
        generated_text = output.outputs[0].text
        print(f"Prompt: {prompt!r}\nGenerated: {generated_text!r}\n")

# run_vllm_offline()
```

This is a standard architecture where, in a real production environment, vLLM is launched as a standalone API server, and a client fully compatible with the OpenAI API specification communicates with the LLM via web requests.

#### Server Execution - Terminal

vLLM internally includes a high-performance server module based on FastAPI.

```bash
# Run vLLM server startup command in terminal
# --model: Hugging Face model to use
# --gpu-memory-utilization: Percentage of VRAM to allocate for KV cache and model weights (default 0.9)
# --max-model-len: Context window length
python -m vllm.entrypoints.openai.api_server \
    --model meta-llama/Llama-2-7b-chat-hf \
    --host 0.0.0.0 \
    --port 8000 \
    --gpu-memory-utilization 0.90 \
    --max-model-len 4096
```

#### Client Communication

Once the server is launched, clients can treat the vLLM server like an OpenAI server (ChatGPT) and send requests using the `openai` library. During this process, vLLM's Continuous Batching operates.

```py
from openai import OpenAI

def call_vllm_api_server():
    """Use the OpenAI library to request text generation from the local vLLM server."""
    
    # Create an OpenAI client instance, but change the endpoint to the local vLLM server address.
    # The api_key is not validated by the vLLM server, so a dummy value is used.
    client = OpenAI(
        base_url="http://localhost:8000/v1",
        api_key="EMPTY"
    )

    # Call standard ChatCompletion API
    completion = client.chat.completions.create(
        model="meta-llama/Llama-2-7b-chat-hf",
        messages=[
            {"role": "system", "content": "You are a highly skilled software engineer."},
            {"role": "user", "content": "Can you explain how PagedAttention works?"}
        ],
        max_tokens=200,
        temperature=0.7
    )

    # Print the result returned by the vLLM server
    print("vLLM Response:")
    print(completion.choices[0].message.content)

# call_vllm_api_server()
```

<br>

## Sequential Processing vs. Continuous Batching

Let's further explore the key differences between continuous batching and sequential processing.

We've just defined that batching processes a bundle of job requests up to the batch size, meaning individual tasks must wait until others are finished, which leads to inefficiencies.

Continuous batching returns a task once it's completed from the queue and then queues the next request for processing. Let's explore how this differs from sequential processing.

The core difference is **whether replacement occurs while leveraging GPU's parallel computation.**

The phrase 'a task finishes, is immediately returned, and a new request is inserted' can easily be misunderstood as a one-to-one replacement (sequential processing). However, Continuous Batching means that **while tens to hundreds of requests are processed in parallel simultaneously, the completion and entry of individual requests alternate in real-time, like cogs in a machine.**

1.  **Sequential Processing (batch size 1)**: Only one request is sent to the GPU at a time. Until 'a' finishes generating 1000 characters, 'b', 'c', and 'd' don't even start their computations. Over 90% of the massive GPU cores remain idle, leading to significant resource waste.
2.  **Traditional Static Batching**
    - To improve GPU efficiency, four items (a, b, c, d) are grouped together and generate one character at a time simultaneously.
    - If 'a' needs only 10 characters and 'b' needs 1000, even after 'a' finishes generating 10 characters, it cannot return the result and must wait, occupying GPU memory, until 'b' finishes its 1000 characters. A new request 'e' cannot be inserted into the empty slot, and the next batch of requests can only be accepted after all four are complete.
3.  **Continuous Batching / In-flight Batching**
    - Grouping A, B, C, D four items for simultaneous parallel generation is similar to static batching.
    - The key difference is that the LLM checks the state at every step as it outputs a single token.
    - When A completes 10 characters, at the 10th tick, it's extracted from the batch and immediately returned to the client.
    - Then, the prompt for new request E, which was in the waiting queue, is immediately pushed into the GPU memory space vacated by A.
    - In the very next 11th token generation tick, the GPU can immediately continue parallel computation by grouping E, B, C, D without any idle time.

In summary, sequential processing computes one item at a time, whereas continuous batching is a **high-density parallel scheduling technique that always fills the GPU's processing capacity to compute simultaneously, performing real-time, token-unit replacements without wasted turns, even if text generation completion times vary.**
