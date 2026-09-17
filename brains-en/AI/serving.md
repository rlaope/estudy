# AI Serving Architecture (Triton Inference Server & Dynamic Batching)

Let's say we've reduced the computational speed of the model itself by applying ONNX and TensorRT.

However, if 100 users simultaneously request image searches during an event, the existing web server architecture would either process one request at a time sequentially, or create 100 threads to access the GPU simultaneously, leading to an out-of-memory (OOM) error and server crash.

How can we design an infrastructure that processes 100 requests most safely and quickly?

To solve this problem, the technologies introduced are a microservice architecture that physically/logically separates general web servers and AI inference servers, and dedicated serving engines.

- **Triton Inference Server**: An open-source AI model inference-specific server developed by NVIDIA. It runs multiple models simultaneously and efficiently schedules client requests to maximize GPU utilization.
- **Dynamic Batching**: An optimization technique that collects multiple individual user requests arriving within a second or a configured brief period, groups them into a large batch, and sends them to the GPU for parallel processing.
- **gRPC**: A high-performance remote procedure call framework developed by Google that compresses heavy image or vector data into a binary format instead of JSON and transmits it at ultra-high speed over HTTP/2.

<br>

## The Problem

Let's re-examine the problem we aimed to solve.

**Web Server and AI Model Resource Conflicts**: Web servers are specialized in lightweight I/O operations and network communication. If a deep learning model, several gigabytes in size, is loaded into the server's memory and heavy matrix operations are performed, threads become blocked, causing a bottleneck where even general product lookup APIs experience delayed responses.

**GPU Waste Due to Sequential Processing**: GPUs have cores capable of parallel computing hundreds of data points at once. However, if we repeatedly feed one image from one customer to the GPU for computation, as in a typical API server structure, less than 5% of the GPU's specifications are utilized, leaving the remaining cores idle.

**In other words, there was an architectural flaw where a web server directly holding a heavy AI model would delay all general business logic processing, and because individual user requests were handled one by one, the GPU was not used efficiently, leading to server crashes when traffic surged.**

For example, when 10 customers simultaneously pressed the image search button, the old method meant that while the GPU processed customer #1's image for 10ms, the other 9 customers were stuck in a queue, waiting indefinitely. The last customer, #10, would have to wait at least 100ms.

### Solution

Let's look at the solution.

- **Separation of Inference-Specific Server (Separation of Concerns)**: The AI model is completely removed from the web server. When the web server receives a user request, it acts as a proxy, passing that data (image) to the Triton server and only receiving the result.
- **Utilizing Dynamic Batching**: If requests from customer #1 to #10 arrive consecutively within 0.0005 seconds, the Triton server bundles these 10 images into a single tensor (batch size = 10). It then performs a single computation on the GPU to simultaneously generate results for all 10 users, which are then split and returned to the web server threads.

Let's consider an intuitive example, a clothing store.

- **Old Structure (without dynamic batching)**: A counter clerk (web server) receives one item of clothing for return from one customer, walks directly to the warehouse to hang the clothes (GPU computation), and returns. If 10 customers arrive consecutively, the clerk has to go back and forth to the warehouse with one item per customer. This is inefficient.
- **Triton's Dynamic Batching Structure**: Now, the counter clerk (web server) and the warehouse staff (Triton) are separated. When the counter clerk receives clothes, they throw them into a basket behind them. The warehouse staff, **when 10 items accumulate in the basket or 3 seconds have passed since an item arrived, takes the entire basket to the warehouse and returns just once.** This single movement processes the work for 10 people simultaneously, increasing throughput tenfold.

### How it Works

This is the data flow of an AI pipeline separated into microservices in a real-world service.

1.  **Client Request**: A mobile app requests an image search from the server.
2.  **Web Server (API Gateway)**: After reading the image binary, it sends an inference request to the Triton server across the network using the gRPC protocol.
3.  **Triton Queueing & Batching**: The Triton server does not compute immediately; instead, it temporarily places requests in an internal queue and waits for a configured delay (e.g., `max_queue_delay_microseconds: 5000` = 5ms), bundling incoming requests into a single multi-dimensional array.
4.  **Backend Execution**: The bundled batch is passed to an optimized engine, ONNX Runtime or TensorRT.
5.  **Result Distribution**: The output batch results are then split back into individual request units and returned via gRPC to each waiting web server thread.

### Example

Let's look at the `config.pbtxt` configuration file that is placed in the model folder when starting the Triton server. These few lines of settings dictate the throughput of the entire system.

```pbtxt
# Configuration file for the clip_image_encoder model (config.pbtxt)

name: "clip_image_encoder"
platform: "onnxruntime_onnx"  # Declares it as an ONNX model
max_batch_size: 32            # Maximum number of requests (images) that can be batched at once

# Input tensor specification definition (Batch dimension omitted as Triton manages it)
input [
  {
    name: "pixel_values"
    data_type: TYPE_FP32
    dims: [ 3, 224, 224 ]
  }
]

# Output tensor specification definition
output [
  {
    name: "image_embeds"
    data_type: TYPE_FP32
    dims: [ 512 ]
  }
]

# Key: Enable Dynamic Batching
dynamic_batching {
  # Waits for other requests for a maximum of 5ms (5000 microseconds) before bundling the batch and starting.
  max_queue_delay_microseconds: 5000
}
```

The communication part for requesting image inference from the FastAPI side to the Triton server is implemented as follows.

The key is that heavy computations are entirely delegated externally.

```py
import numpy as np
import tritonclient.grpc as grpcclient
from PIL import Image
from torchvision import transforms

class TritonInferenceClient:
    """
    This client operates on the web server (API server) and communicates with the Triton server via gRPC.
    """
    def __init__(self, triton_url="triton-server:8001"):
        # Create a high-speed gRPC connection with the Triton server
        self.client = grpcclient.InferenceServerClient(url=triton_url)
        self.model_name = "clip_image_encoder"
        
        # The web server only performs lightweight resizing and normalization (CPU operations).
        self.preprocess = transforms.Compose([
            transforms.Resize((224, 224)),
            transforms.ToTensor(),
            transforms.Normalize(mean=[0.481, 0.457, 0.408], std=[0.268, 0.261, 0.275])
        ])

    def request_embedding(self, pil_image: Image.Image) -> list:
        """Sends a single image to Triton and receives a 512-dimensional vector."""
        
        # 1. Prepare input data (Shape: [1, 3, 224, 224], Type: FP32)
        input_data = self.preprocess(pil_image).unsqueeze(0).numpy().astype(np.float32)
        
        # 2. Create Triton gRPC input/output objects
        # Must match the name defined in config.pbtxt.
        triton_input = grpcclient.InferInput("pixel_values", input_data.shape, "FP32")
        triton_input.set_data_from_numpy(input_data)
        
        triton_output = grpcclient.InferRequestedOutput("image_embeds")
        
        # 3. Request inference to the Triton server across the network (joins the dynamic batching queue)
        response = self.client.infer(
            model_name=self.model_name,
            inputs=[triton_input],
            outputs=[triton_output]
        )
        
        # 4. Parse and return the result (Numpy array) received from the server
        result_vector = response.as_numpy("image_embeds")
        
        return result_vector.tolist()[0]

# --- Web API Router Execution Environment ---
# triton_engine = TritonInferenceClient("192.168.1.100:8001")
#
# @app.post("/api/v1/search/image")
# def search_by_image(file: UploadFile):
#     img = Image.open(file.file).convert("RGB")
#     
#     # Heavy computations are handled by the Triton server, and only the vector value is returned.
#     vector = triton_engine.request_embedding(img)
#     
#     # Then, pass the vector to Elasticsearch 8.x for search (Step 4)
#     results = es_hybrid_search(vector)
#     return {"results": results}
```
