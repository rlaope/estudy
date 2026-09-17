# Implementing and Optimizing AI Serving Infrastructure for High-Volume Traffic

The following will be learned:

- Implementing a pipeline to convert model weights and optimize inference latency using ONNXRuntime and TensorRT APIs
- Configuring an environment to maximize GPU computation efficiency based on Dynamic Batching by tuning Triton Inference Server's config.pbtxt
- Testing PagedAttention-based memory optimization by controlling `gpu_memory_utilization` and `max_num_batched_tokens` parameters of the vLLM engine
- Implementing a high-performance asynchronous API layer that communicates with a C++-based core inference engine by combining Python FastAPI and gRPC
- Analyzing the characteristics of the Continuous Batching algorithm and testing scheduling logic to reduce GPU idle states during concurrent requests
- Occurrences during large language model serving using Nsight Systems and PyTorch Profiler
