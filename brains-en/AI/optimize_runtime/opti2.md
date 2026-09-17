# Tuning Triton Inference Server config.pbtxt

Let's start with a question,

Suppose hundreds of users simultaneously press the **image analysis** button in a mobile app.

Hundreds of individual requests will come into the API server, and if these individual requests are pushed to the GPU one by one in sequence?

An expensive GPU with massive computational cores would end up running idle, unable to utilize its full power.

The server should collect individual user requests, form them into one large chunk, and then send them to the GPU to ensure efficient and thorough utilization...

To prevent this waste and maximize GPU throughput, the technique used is **NVIDIA Triton Inference Server's Dynamic Batching**.

- **Triton Inference Server**: An open-source AI model serving platform developed by NVIDIA that loads models in various formats such as TensorRT, ONNX, and PyTorch on a single server, communicates with clients via HTTP/gRPC interfaces, and optimizes resource allocation.
- **GPU's SIMT (Single Instruction, Multiple Threads) Architecture**: GPUs are composed of thousands of small cores, specializing in parallel processing of matrix multiplications. Whether there is 1 piece of data (B = 1) or 16 pieces (B = 16), the physical time required for computation is almost the same. In other words, processing 16 items at once is overwhelmingly faster than processing 1 item 16 times.
- **Dynamic Batching**: Clients asynchronously send individual requests, each containing a single piece of data. However, Triton's internal scheduler on the serving server collects these requests in a queue for a very short period (e.g., 5ms), combines them into one large tensor, and then sends it to the GPU.

## Problem Definition

There was a mismatch between clients' asynchronous and individual request patterns and the GPU's large-scale parallel processing architecture. When requests were processed sequentially one by one, GPU utilization dropped below 10%, leading to bottlenecks under heavy traffic.

ex.) "If 100 clients send requests simultaneously, and it takes 10ms to infer one model request, the 100th user would experience a critical delay, having to wait 990ms in the queue until the preceding 99 requests are processed before receiving a response."

### Solution

- **Scheduler Intervention and Latency Tolerance**: By enabling the `dynamic_batching` option in Triton's config.pbtxt file, the server does not immediately send the first request to the GPU but waits for a maximum delay time specified by `max_queue_delay_microseconds`.
- **Batch Optimization (Trade-off Adjustment)**: While waiting, if the target optimal batch size `preferred_batch_size` is reached, the requests are immediately bundled and sent to the GPU, even if the maximum delay time has not elapsed. This is an optimization strategy that **sacrifices a slight increase in single-request latency to improve the overall system's throughput by tens of times.**

## Detailed Operating Principle and Structure

Let's look at the logical flow where individual requests arriving at different times are combined in the Triton queue and applied as a single batch to the GPU kernel.

```mermaid
graph TD
    Client1[Client A: req 1] -->|t=0ms| TritonQueue[Triton Dynamic Batcher Queue\n(max_delay: 5ms, pref_batch: 4)]
    Client2[Client B: req 1] -->|t=1ms| TritonQueue
    Client3[Client C: req 1] -->|t=2ms| TritonQueue
    Client4[Client D: req 1] -->|t=4ms| TritonQueue
    
    TritonQueue -->|Batch Size = 4 달성 시 즉시 방출| BatchTensor[Batch Tensor Array\nShape: 4 x Dims]
    
    subgraph "Target GPU (e.g., TensorRT Engine)"
        BatchTensor --> GPU[GPU Tensor Cores\n단 1회의 행렬곱 연산 수행]
    end
    
    GPU -->|결과 분리 (Scatter)| TritonQueue
    TritonQueue -.->|응답 1| Client1
    TritonQueue -.->|응답 2| Client2
    TritonQueue -.->|응답 3| Client3
    TritonQueue -.->|응답 4| Client4
```

Let's look at the most basic `config.pbtxt` structure.

Looking at the skeleton of the model configuration file that Triton must read to load a model (dynamic file not yet present):

```proto
# Model name (must match folder name)
name: "resnet50_trt"
# Specify backend engine (TensorRT)
platform: "tensorrt_plan"
# Maximum batch size this model can process
max_batch_size: 32

# Shape of the model's input tensor (Batch dimension is managed by Triton, so it can be omitted or set to -1)
input [
  {
    name: "input_tensor"
    data_type: TYPE_FP32
    dims: [ 3, 224, 224 ]
  }
]

# Shape of the model's output tensor
output [
  {
    name: "output_tensor"
    data_type: TYPE_FP32
    dims: [ 1000 ]
  }
]
```

Also, let's look at the dynamic batching and GPU concurrency maximization settings.

This configuration finely tunes the `dynamic_batching` and `instance_group` parameters to maximize GPU computational efficiency in a real production environment.

```proto
name: "resnet50_trt"
platform: "tensorrt_plan"
max_batch_size: 64

input [ { name: "input", data_type: TYPE_FP32, dims: [ 3, 224, 224 ] } ]
output [ { name: "output", data_type: TYPE_FP32, dims: [ 1000 ] } ]

# [Key 1] Dynamic Batching Settings
dynamic_batching {
  # Maximum time to wait in the queue (unit: microseconds).
  # E.g., 5000 = 5ms. After 5ms, the batch is sent to the GPU regardless of whether it's full.
  max_queue_delay_microseconds: 5000
  
  # Recommended batch sizes for the TensorRT engine to compute most efficiently.
  # If 4, 8, or 16 requests accumulate in the queue, they are executed immediately, even if the delay time (5ms) has not elapsed.
  preferred_batch_size: [ 4, 8, 16, 32, 64 ]
}

# [Key 2] Instance Group Settings
# By launching multiple replicas (instances) of the same model on a single GPU,
# data copying (Host -> Device) and inference operations are pipelined to eliminate GPU idle states.
instance_group [
  {
    count: 2             # Launch 2 replicas if memory allows
    kind: KIND_GPU       # Load into GPU memory
    gpus: [ 0 ]          # Use GPU 0
  }
]

# [Optional] Model Warmup Settings
# Send dummy data once when the server starts to prevent GPU kernel initialization (Cold Start) delay
model_warmup [
  {
    name: "warmup_request"
    batch_size: 1
    inputs: {
      key: "input"
      value: { data_type: TYPE_FP32, dims: [ 3, 224, 224 ], zero_data: true }
    }
  }
]
```

Above, `preferred_batch_size` indicates a preference to execute immediately once that batch size is reached. This might raise a question: if it executes at 4, and also at 8, doesn't it necessarily have to pass through 4 to reach 8?

In a real high-traffic environment, inference requests don't arrive neatly one by one. It's very common for requests to skip 4 and jump directly to 8 or 16.

This is because of traffic bursts occurring in an instant, where millions of users might press a button simultaneously with a 0.0001-second difference, or a front-end API server might throw 10 pieces of data at once.

Let's say the GPU takes 10ms to diligently compute a bundle of 4 requests that were sent earlier. While the GPU is working, Triton continues to accumulate subsequent requests in the queue. When the GPU finishes its computation and signals for the next task, it might find 20 requests already queued up within that 10ms. In that case, it immediately creates a bundle of 16 and sends it to the GPU.

In conclusion, that array of numbers doesn't mean waiting sequentially by counting one by one. Instead, it acts as a reference point: at the exact moment the GPU is ready to work, it checks the queue and takes requests in chunks corresponding to those numbers, based on the volume accumulated.
