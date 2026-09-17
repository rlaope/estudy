# Triton Inference Server

Dynamic batching increased the throughput of a single model. However, as the actual service grew,

a situation arose where multiple AI models, such as CLIP models for image search, harmful image filtering models, and product recommendation models, had to be operated simultaneously.

Running each model as a separate server process leads to GPU memory fragmentation and an explosion in infrastructure costs.

How should the architecture be designed to allow multiple different models to share a single GPU resource without conflict, and to completely eliminate inter-process communication overhead?

To address the operational bottlenecks in such multi-model environments, this document covers the core architectural features provided by Triton Inference Server: Concurrent Execution and Shared Memory communication technologies.

- **Concurrent Model Execution (다중 모델 동시 실행)**: This feature allows multiple models written in different frameworks such as ONNX, TensorRT, and PyTorch to be loaded into memory simultaneously within a single Triton process, and to execute inference in parallel by splitting available GPU stream resources.
- **Model Ensemble (모델 앙상블)**: This feature bundles multiple model inference pipelines (e.g., pre-processing model -> inference model -> post-processing model) into a single logical virtual model within the server, allowing clients to execute the entire pipeline with just one API call.
- **Shared Memory (공유 메모리)**: When the client web server and the Triton server are on the same physical host, this is a zero-copy communication technique that reads data by exchanging only OS-level system memory (RAM) addresses, without converting data into network packets for transmission.

<br>

## The Problem

Let's delve deeper into the problem we aimed to identify and solve.

- **Decreased GPU Utilization and Memory Fragmentation**: When operating multiple AI models, if each model is allocated an independent container or server process, each framework runtime (e.g., PyTorch process) individually preempts GPU VRAM. This leads to insufficient available memory when computation is actually needed, or resource waste where one model cannot utilize the resources of another model that is idle.
- **Network Serialization Overhead**: When transmitting a 10MB image tensor from a web server to an AI server via HTTP or gRPC, the process of serializing the data into a byte array and then deserializing it on the receiving end consumes enormous CPU cycles, latency, and network I/O.

**In other words, building separate server processes for each model leads to wasted GPU resources and memory fragmentation, and there was a limitation where significant system resources and time were consumed by serialization/deserialization when exchanging large tensor data between containers.**

For example, consider this problem: on a GPU server with 24GB VRAM, models A, B, and C, each 4GB in size, are launched as separate API servers. If traffic is concentrated only on Model A, the 8GB of VRAM occupied by Models B and C remains idle, yet Model A crashes due to insufficient memory. Furthermore, tens of milliseconds of communication delay occur due to copying data over the network to transfer Model A's results to Model B.

### The Solution

The problem-solving approach is as follows.

- **GPU Context Switching via Integrated Backend**: A single Triton instance centrally controls multiple framework runtimes through a C++-based Backend API. An internal scheduler simultaneously allocates computation commands from various models to multiple GPU streams (CUDA Streams), ensuring parallel execution without wasted cores.
- **System/CUDA Shared Memory Pointer Passing**: After the client writes the image tensor to the OS's shared memory space, only a few bytes of string containing the shared memory ID and offset information are transmitted to the Triton server, not the data itself. Triton directly accesses that memory address to read the data without copying and performs inference.

To provide a more intuitive example:

- **Traditional pipeline with independent servers and serial communication** involves [Client -> Network Copy -> Preprocessing Server -> Network Copy -> AI Inference Server -> Network Copy -> Client]. This means repeatedly packing and unpacking packets every time data moves.
- **Triton ensemble and shared memory architecture**: The client places data in a shared warehouse and only provides the warehouse key number to Triton. The ensemble pipeline (preprocessing -> AI inference) configured within Triton directly retrieves data from the warehouse, completes the tasks sequentially within a single process, and then places the results back into the warehouse.

<br>

## Structuring the Operating Principles

This is the internal structure of how Triton handles traffic in a multi-model environment.

1.  **Model Repository(모델 저장소)**: Triton scans the specified folder repository structure at boot time. If multiple models of different types, such as ONNX, TensorRT, and Python backends, are placed in separate directories within a single folder, they are immediately loaded into memory simultaneously.
2.  **Ensemble Scheduler (앙상블 스케줄러)**: It defines dependencies between models in a DAG (Directed Acyclic Graph) format. When a client calls an ensemble model, the scheduler automatically controls the data flow (e.g., Model A Output -> Model B Input) and passes tensors only within internal memory.
3.  **Shared Memroy Manager**: It registers the `shm` region managed by the OS kernel. When a client writes data to this region and sends a query, the Backend C API references that pointer and directly pushes the data to the computational hardware GPU.

### Example

For understanding the principles, I will demonstrate the Model Repository structure and ensemble configuration.

Triton builds pipelines (ensembles) between models using a strict directory structure and `config.pbtxt`, rather than code.

This allows the web server to execute a pipeline involving multiple models with just a single call.

```
# Example Triton Model Repository Directory Structure
model_repository/
├── image_preprocess_model/     # 1. Image Preprocessing (Python Backend)
│   ├── 1/model.py
│   └── config.pbtxt
├── clip_encoder_model/         # 2. Image Embedding (ONNX Backend)
│   ├── 1/model.onnx
│   └── config.pbtxt
└── search_pipeline_ensemble/   # 3. Ensemble Virtual Model (connecting 1 and 2)
    ├── 1/empty.txt             # Ensembles have no logic, so this is an empty file
    └── config.pbtxt
```

```proto
# search_pipeline_ensemble/config.pbtxt
# Ensemble scheduler configuration to bundle the preprocessing model and embedding model into one

name: "search_pipeline_ensemble"
platform: "ensemble"

# Input: Raw binary image sent by the client
input [ { name: "RAW_IMAGE", data_type: TYPE_UINT8, dims: [ -1 ] } ]
# Output: The final 512-dimensional vector obtained
output [ { name: "FINAL_VECTOR", data_type: TYPE_FP32, dims: [ 512 ] } ]

ensemble_scheduling {
  step [
    {
      model_name: "image_preprocess_model" # First model to execute
      model_version: -1
      input_map { key: "image_bytes", value: "RAW_IMAGE" }
      output_map { key: "preprocessed_tensor", value: "PREPROCESSED_DATA" }
    },
    {
      model_name: "clip_encoder_model"     # Second model to execute
      model_version: -1
      # Directly inject the output of the previous step (PREPROCESSED_DATA) as input to this model
      input_map { key: "pixel_values", value: "PREPROCESSED_DATA" }
      output_map { key: "image_embeds", value: "FINAL_VECTOR" }
    }
  ]
}
```

Next, let's look at the high-performance Python client code that exchanges data using OS system shared memory without network serialization, when the web server API and Triton server are on the same physical instance (or a pod with the same shared memory volume mounted).

```py
import numpy as np
import tritonclient.grpc as grpcclient
import tritonclient.utils.shared_memory as shm
from PIL import Image

class SharedMemoryTritonClient:
    """
    High-speed client that exchanges data with the Triton server using OS shared memory without network packet serialization.
    """
    def __init__(self, triton_url="localhost:8001"):
        self.client = grpcclient.InferenceServerClient(url=triton_url)
        self.model_name = "clip_encoder_model"
        
        # Calculate tensor size (Batch=1, C=3, H=224, W=224, float32=4bytes)
        self.byte_size = 1 * 3 * 224 * 224 * 4
        
        # 1. Create and map OS-level shared memory (name: "input_shm")
        self.shm_handle = shm.create_shared_memory_region(
            "input_shm", "/input_shm", self.byte_size
        )
        
        # 2. Register the created shared memory with the Triton server (Registration)
        # Triton can now directly access this memory region via the name "/input_shm".
        self.client.register_system_shared_memory(
            "input_shm", "/input_shm", self.byte_size
        )

    def request_embedding_zerocopy(self, preprocessed_image_array: np.ndarray) -> np.ndarray:
        """Writes data to shared memory and requests inference based on pointers."""
        
        # 1. Directly write data to the shared memory region (Copy-in)
        # This is simply writing to local RAM, not network transmission.
        shm.set_shared_memory_region(self.shm_handle, [preprocessed_image_array])
        
        # 2. Create Triton input object (does not contain the data itself)
        triton_input = grpcclient.InferInput("pixel_values", preprocessed_image_array.shape, "FP32")
        
        # Key: Instead of a data array, set only the registered shared memory name ("input_shm") and offset
        triton_input.set_shared_memory("input_shm", self.byte_size)
        
        triton_output = grpcclient.InferRequestedOutput("image_embeds")
        
        # 3. Inference request
        # Since there is no data in the payload, network I/O latency is close to zero.
        response = self.client.infer(
            model_name=self.model_name,
            inputs=[triton_input],
            outputs=[triton_output]
        )
        
        # 4. Return result
        return response.as_numpy("image_embeds")

    def cleanup(self):
        """Deallocates shared memory upon server shutdown"""
        self.client.unregister_system_shared_memory("input_shm")
        shm.destroy_shared_memory_region(self.shm_handle)

# --- Practical Usage Flow ---
# client = SharedMemoryTritonClient()
# img_tensor = preprocess(Image.open("test.jpg"))  # [1, 3, 224, 224] float32
# 
# # Obtain inference results immediately without serialization/deserialization overhead
# vector = client.request_embedding_zerocopy(img_tensor)
```

The link https://arxiv.org/html/2602.00053v refers to a paper titled 'Scalable and Secure AI Inference in Healthcare: A Comparative Benchmarking of FastAPI and Triton Inference Server on Kubernetes', which includes benchmarking results:

| Framework | Hardware | Batch Mode | Batch Size | p50 Latency (ms) | p95 Latency (ms) | Throughput (req/s) |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **FastAPI** | CPU | None | 1 | 22 | 45 | 450 |
| **Triton** | GPU | No Batching | 1 | 28 | 52 | 420 |
| **Triton** | GPU | Dynamic | 16 | 34 | 60 | 780 |

Latencies like p95 and p99 were slightly slower, and throughput also decreased without dynamic batching. However, after enabling dynamic batching, throughput nearly doubled. While latency might slightly increase, this can be considered a better trade-off in high-traffic environments.
