# Operator Fusion, Quantization

### Operator Fusion

This is an optimization technique that combines multiple consecutive operations (operators) into a single GPU kernel for execution.

Looking at the principles of hardware optimization, many bottlenecks in GPU operations arise not from computational performance (FLOPS) but from memory bandwidth.

-   **Traditional method**: After executing Conv, results are stored in VRAM -> reloaded from VRAM for ReLU execution -> results stored. This process generates unnecessary VRAM I/O.
-   **Fusion applied**: Immediately after the Conv operation, ReLU is executed while the data is still residing in the GPU's internal registers and L1 Cache.
-   This dramatically reduces VRAM access frequency, solving the memory bandwidth bottleneck issue.

### Quantization

This is a technique that reduces the precision of model weights and activation function outputs.

It converts FP32 (32-bit Floating Point) to FP16, BF16, or INT8 (8-bit integer).

INT8 quantization typically uses the following linear transformation equation:

$$Q = \text{clamp}\left(\text{round}\left(\frac{R}{S} + Z\right), Q_{\min}, Q_{\max}\right)$$

(R: RealValue, S: Scale factor, Z: Zero-point)

**Optimization Benefits**
1.  **Reduced Memory Usage:** When converting from FP32 to INT8, the model size is reduced by 1/4.
2.  **Bandwidth Savings:** More data can be transferred from memory to the computational unit in the same amount of time.
3.  **Improved Throughput:** Modern GPUs (such as NVIDIA Tensor Cores, Apple Neural Engine, etc.) have hardware accelerators specialized for low-precision operations, providing several times faster computation speeds for INT8 operations compared to FP32.

### Kernel Optimization

This is the stage of tuning kernel code to maximally leverage the architectural characteristics of specific hardware.

-   **Loop Unrolling**: A technique that physically unrolls loops to reduce loop control overhead.
-   **Tiling (Blocking)**: Divides data into small tiles, loads them into Shared Memory, and performs operations to minimize L2 Cache Misses.
-   **Vectorization:** Explicitly uses SIMD (Single Instruction Multiple Data) instructions, which process multiple data items with a single instruction, to increase computational density.

### Role of Inference Engines (ONNX Runtime, TensorRT)

Inference engines like ONNX Runtime or TensorRT analyze the user-defined model graph and automatically apply the techniques mentioned above.

1.  **Graph Optimization:** Removes unnecessary nodes or replaces them with constants.
2.  **Layout Transformation:** Automatically changes to the hardware-preferred data layout (e.g., NCHW -> NHWC).
3.  **Static Memory Planning:** Pre-calculates memory allocation before execution to eliminate malloc/free overhead at runtime.
