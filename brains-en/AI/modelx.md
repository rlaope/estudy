# AI Model Optimization and Inference Acceleration with ONNX & TensorRT

As we learned in the previous article, Elasticsearch and HNSW significantly reduce the speed of vector similarity searches.

But what if a user requests a search by image instead of text?

We need to respond to requests like "Find products from this image."

When a customer uploads an image, passing it through a PyTorch-based ResNet-50 or CLIP encoder to convert it into a 512-dimensional vector for inference alone consumes 100-200ms on a CPU.

No matter how fast the database search is, if the deep learning model computation itself causes a bottleneck, how should we optimize this heavy model for a real-world service environment?

The technology introduced to solve the bottleneck problem described above is **model optimization and inference acceleration**.

It's an engineering process that transforms models from research-oriented Python frameworks to fit system hardware CPU/GPU architectures, compressing them to maximize computation speed.

-   **ONNX (Open Neural Network Exchange)**: An open standard format that converts models created with various frameworks like PyTorch and TensorFlow into a single common format.
-   **ONNX Runtime (ORT)**: An inference-only engine that executes (infers) models converted to ONNX format at ultra-high speed in a C++-based environment.
-   **TensorRT**: A deep learning inference optimization engine developed by NVIDIA that merges and optimizes computation graphs to fully leverage the potential of GPU hardware's Tensor Cores.
-   **Quantization**: A technique that compresses model weights (FP32 32-bit floating-point numbers) to FP16 16-bit floating-point or INT8 8-bit integers, minimizing accuracy loss while improving computation speed and memory efficiency.
-   **Kernel Fusion**: A technique that combines multiple independent operation layers (e.g., Convolution + BatchNorm + ReLU) into a single computational block, eliminating memory read/write overhead.

<br>

## Problem to Solve

**Python and Framework Overhead**: PyTorch is a framework optimized for research and training. When inferencing models, the GIL (Global Interpreter Lock) constraint of the Python interpreter and the dynamic computation graph allocation structure cause unnecessary memory allocation and scheduling overhead.

**Lack of Hardware Optimization**: Layers defined in PyTorch each require separate GPU memory access. For example, to pass data through three consecutive layers (Conv -> BN -> ReLU), data must be read from and written to VRAM three times, which causes significant latency.

### Solution

**C++ Runtime Migration with ONNX**: Extract only the model's weights and computational graph into a pure `.onnx` file. In the inference server, inference is executed through a lightweight C++-written ONNX Runtime, eliminating heavy PyTorch dependencies and Python overhead.

**Graph Optimization and Quantization (TensorRT Integration)**: The inference engine analyzes the architecture of the actual hardware (NVIDIA GPU) where the model will be deployed, merging individual operations into one. It also halves the data type size required for computation (FP16), allowing twice the data to be processed in a single clock cycle.

### Understanding with an Intuitive Example (Analogy of a Clothing Store Warehouse)

-   **PyTorch Model (Research Assistant)**: This assistant reads the headquarters' Python manual line by line to inspect clothes. "Take clothes from box 1 (memory read), close box 2 (memory write), then box 3, unfold clothes and check tag (memory read)..." The operation is flexible but too slow.
-   **ONNX Conversion (Standardization)**: Eliminate dependence on the headquarters' manual and directly input an optimized, single-page action guide (C++ runtime) into the assistant's mind. Now, they move mechanically and quickly without needing to ask headquarters.
-   **TensorRT Kernel Fusion and Quantization**: Kernel Fusion: Combines three separate actions—"take out clothes, check tag, pack"—into one continuous action ("take clothes out of the box, check the tag simultaneously, and immediately put them back in the box") (single kernel operation).
    -   Quantization: The standard for measuring dimensions, which was strictly up to 6 decimal places (FP32), is lowered to 2 decimal places (FP16). Accuracy barely drops, but inspection speed doubles.

<br>

### Detailed Operation Principle and Structure

This is the low-level structure where a deep learning model is converted to ONNX format and optimized at runtime.

1.  **Tracing (Computation Graph Tracking)**: When PyTorch's `torch.onnx.export()` is called, a dummy tensor is passed through the model. It tracks the data flow path (operation order, nodes, edges) to draw a static computational graph.
2.  **Graph Optimization (Runtime Graph Optimization)**: When an ONNX Runtime instance is executed, it analyzes the traced graph to remove redundant or unnecessary nodes (constant folding).
3.  **Execution Provider (Backend Assignment)**: ONNX Runtime detects the system environment and selects the fastest hardware accelerator execution provider. In a GPU environment, it uses `TensorrtExecutionProvider` or `CUDAExecutionProvider`; in a CPU environment, it uses `CPUExecutionProvider` to delegate computations.
4.  **Kernel Fusion (in TensorRT Environment)**: Multiple layers are grouped and compiled into a single CUDA kernel. This process significantly reduces the number of accesses to the GPU's global memory, and computations are completed within registers.

> Here, "grouping multiple layers" refers to individual mathematical operation blocks of deep learning models, such as Convolution, BatchNorm, and ReLU.
>
> Originally, in environments like PyTorch, the GPU performs read-compute-write operations on data every time it passes through a layer. For example, if a model has a Conv $\rightarrow$ BatchNorm $\rightarrow$ ReLU structure, the GPU makes three memory round trips, which takes time.
>
> "Compiling these multiple layers into a single CUDA kernel (kernel fusion)" means that data is read from memory only once, and then the three calculations are instantly completed within the GPU, and saved only once.
>
> This completely eliminates unnecessary memory round-trip time, dramatically speeding up inference.

<br>

### Example: Converting a PyTorch Model to ONNX

This logic performs the heaviest task, model conversion (export), in an offline development PC environment to generate a static `.onnx` file in advance.

```py
import torch
from transformers import CLIPModel

def export_clip_to_onnx(model_id="openai/clip-vit-base-patch32", output_path="clip_image_encoder.onnx"):
    """
    Extracts and saves only the Image Encoder part of a PyTorch-loaded CLIP model
    as an independent ONNX file.
    """
    print("1. Loading PyTorch model...")
    model = CLIPModel.from_pretrained(model_id)
    model.eval() # Must be set to evaluation mode
    
    # Dummy input data to pass through the model
    # Batch Size=1, Color=3(RGB), Width=224, Height=224
    dummy_pixel_values = torch.randn(1, 3, 224, 224)
    
    print("2. Starting ONNX Graph Tracing...")
    # Export only the image encoder to ONNX
    torch.onnx.export(
        model.vision_model,                # PyTorch model to convert (extract only vision_model)
        dummy_pixel_values,                # Dummy input for tracing the model structure
        output_path,                       # File name to save
        export_params=True,                # Whether to include learned weights
        opset_version=14,                  # ONNX operator version for compatibility
        do_constant_folding=True,          # Apply constant folding optimization
        input_names=['pixel_values'],      # Define input node names
        output_names=['image_embeds'],     # Define output node names
        dynamic_axes={                     # Settings to dynamically receive batch size without fixing it
            'pixel_values': {0: 'batch_size'},
            'image_embeds': {0: 'batch_size'}
        }
    )
    print(f"✅ ONNX conversion complete: {output_path}")

# export_clip_to_onnx()
```

If you were to completely remove the PyTorch library from a production server and use only the onnxruntime library to perform ultra-fast inference, you could configure the server as follows:

```py
import onnxruntime as ort
import numpy as np
from PIL import Image
from torchvision import transforms

class ONNXImageEncoder:
    """
    A production-level inference engine that extracts image vector embeddings
    using pure C++ based ONNX Runtime without PyTorch.
    """
    def __init__(self, onnx_model_path="clip_image_encoder.onnx"):
        # 1. Execution Provider Configuration (Hardware accelerator selection)
        # Prioritize GPU (TensorRT/CUDA) if available, otherwise use CPU
        providers = [
            'TensorrtExecutionProvider', 
            'CUDAExecutionProvider', 
            'CPUExecutionProvider'
        ]
        
        # 2. Create Inference Session (Model loading and graph optimization)
        sess_options = ort.SessionOptions()
        sess_options.graph_optimization_level = ort.GraphOptimizationLevel.ORT_ENABLE_ALL
        
        self.session = ort.InferenceSession(
            onnx_model_path, 
            sess_options=sess_options, 
            providers=providers
        )
        
        # Cache input and output node names
        self.input_name = self.session.get_inputs()[0].name
        
        # Lightweight preprocessing pipeline for numpy only (PIL Image -> Numpy Array)
        self.preprocess = transforms.Compose([
            transforms.Resize((224, 224)),
            transforms.ToTensor(),
            transforms.Normalize(mean=[0.48145466, 0.4578275, 0.40821073], 
                                 std=[0.26862954, 0.26130258, 0.27577711])
        ])

    def encode_image(self, pil_image: Image.Image) -> np.ndarray:
        """Takes a single image and returns a 512-dimensional vector."""
        # 1. Image preprocessing and dimension addition (Batch size=1)
        # Use a pure numpy array (float32) instead of a PyTorch tensor.
        input_tensor = self.preprocess(pil_image).unsqueeze(0).numpy().astype(np.float32)
        
        # 2. Execute ONNX Runtime inference
        # Pass input data as a dictionary
        inputs = {self.input_name: input_tensor}
        outputs = self.session.run(None, inputs)
        
        # 3. Extract embedding results (Apply L2 normalization)
        embeds = outputs[0]
        embeds = embeds / np.linalg.norm(embeds, axis=1, keepdims=True)
        return embeds

# --- Example of using a real-world API router ---
# onnx_encoder = ONNXImageEncoder("clip_image_encoder.onnx")
# 
# @app.post("/vectorize/image")
# def get_image_vector(file: UploadFile):
#     # Load image file
#     img = Image.open(file.file).convert("RGB")
#     
#     # Inference that took 100ms with PyTorch
#     # is reduced to about 10-20ms with ONNX+TensorRT
#     vector = onnx_encoder.encode_image(img)
#     
#     return {"vector": vector.tolist()[0]}
```
