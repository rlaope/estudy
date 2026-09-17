# TensorRT

TensorRT is a model optimization engine that helps improve deep learning services by optimizing trained deep learning models, thereby accelerating inference speed by several to tens of times on NVIDIA GPUs.

Deep learning models developed using frameworks like Caffe, PyTorch, TensorFlow, and PaddlePaddle can be optimized with TensorRT and deployed on NVIDIA GPU platforms such as Tesla T4, Jetson TX2, and Tesla V100.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdna%2Fqf0WL%2FbtqDA4p61z3%2FAAAAAAAAAAAAAAAAAAAAAAZ6IzaPITLCybzaEB58m94mNAKbd5uBQi5S3fwQ1FAE%2Fimg.png%3Fcredential%3DyqXZFxpELC7KVnFOS48ylbz2pIh3yKj8%26expires%3D1772290799%26allow_ip%3D%26allow_referer%3D%26signature%3DesvP8kIt3KzmZNr6fFJ8zwemYf4%253D)

TensorRT also includes an optimizer that utilizes optimization techniques suitable for NVIDIA GPU computation to optimize models, and a runtime engine that performs model operations on various GPUs.

TensorRT supports models trained in most deep learning frameworks and provides optimal deep learning model acceleration.

It supports C++ and Python at the API level, making it easy for deep learning developers to use even without extensive knowledge of GPU programming or CUDA.

Furthermore, it builds a Runtime binary that automatically utilizes the optimal computational resources available on the GPU, which can easily improve latency and throughput, enabling efficient execution of deep learning programs and services.

It provides methodologies for customization. While it can be challenging, just know that it allows developers to use it flexibly.

It automatically applies techniques such as network compression, optimization, and GPU optimization to achieve optimal inference performance on NVIDIA platforms. Let's explore these techniques.

### Quantization & Precision Calibration

Lowering precision in deep learning training and inference has become a common practice.

Neural networks with lower precision allow for faster and more efficient computation due to smaller data sizes and fewer bits for weights.

Among the quantization techniques for this purpose, TensorRT uses symmetric linear quantization, which allows reducing the precision of typical FP32 data from deep learning frameworks to FP16 or INT8 data types.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdna%2FdqbtaT%2FbtqDzkHbdoL%2FAAAAAAAAAAAAAAAAAAAAAB_Kz_Vjn_i8xgWKFmTaqT_rwVHRM1JPjTCnYoNmoQ9B%2Fimg.png%3Fcredential%3DyqXZFxpELC7KVnFOS48ylbz2pIh3yKj8%26expires%3D1772290799%26allow_ip%3D%26allow_referer%3D%26signature%3D4hFiw5tdYfQcKHkN57AIgkn4jrU%253D)

Generally, reducing the precision to FP16 data type does not significantly affect accuracy.

**However, reducing precision to the INT8 data type does affect accuracy. Therefore, additional calibration is required.**

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdna%2FceTH0T%2FbtqDzEZICqF%2FAAAAAAAAAAAAAAAAAAAAAND9Cx7BMHDYdc0k_qOXpc3lmf4LnG4_Aoclw7ZhczsY%2Fimg.png%3Fcredential%3DyqXZFxpELC7KVnFOS48ylbz2pIh3yKj8%26expires%3D1772290799%26allow_ip%3D%26allow_referer%3D%26signature%3DJITLU%252BtfGpwJvkLmfl6LqEozB78%253D)

TensorRT supports EntropyCalibrator(2) and MinMaxCalibrator.

These can be used to minimize information loss in weights and intermediate tensors during quantization.

### Graph Optimization

Generally, graph optimization is used to configure graph nodes, whether primitive or compound operations, used in deep learning neural networks with platform-optimized code.

Based on this, TensorRT simultaneously applies layer fusion and tensor fusion methods.

Layer fusion, including vertical and horizontal layer fusion, along with tensor fusion, is applied to simplify the model graph, significantly reducing the number of model layers.

Optimizing backbone neural networks like ResNet and MobileNet has shown the effect of reducing the number of original nodes by tens of times.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdna%2FchgvZQ%2FbtqDzEFuM6C%2FAAAAAAAAAAAAAAAAAAAAAFNitBqDjZvOC4X3GqBmkD7zL-AX8HPZ_99NohmoIQnD%2Fimg.png%3Fcredential%3DyqXZFxpELC7KVnFOS48ylbz2pIh3yKj8%26expires%3D1772290799%26allow_ip%3D%26allow_referer%3D%26signature%3DwTx1IXFT2ek4xqSX%252BY6ewVYdh%252Bo%253D)

### Automatic Kernel Tuning

TensorRT helps generate Runtimes tailored for various NVIDIA platforms and architectures.

Since the optimal kernel differs for each product depending on the number of CUDA engines, memory, and whether a serialized engine is included, TensorRT selectively performs this during Runtime engine build to help generate an optimal engine binary.

### Dynamic Tensor Memory & Multi-Stream Execution

Additionally, there's a dynamic tensor memory feature that helps reduce footprint through memory management, allowing for reuse.

It's also possible to maximize parallel efficiency by scheduling multiple input streams using CUDA stream technology.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdna%2FsphHA%2FbtqDyMKKE0i%2FAAAAAAAAAAAAAAAAAAAAABaRHebQN0SIEF_ZDO86X11NUdo-vOdhkAqh7QXIhLAe%2Fimg.png%3Fcredential%3DyqXZFxpELC7KVnFOS48ylbz2pIh3yKj8%26expires%3D1772290799%26allow_ip%3D%26allow_referer%3D%26signature%3Djg9G6zz2ZPoHtnxKaNY9hi0LwEw%253D)

With these acceleration technologies, TensorRT can achieve speed improvements.

It is said that simply using TensorRT on the same GPU for a ResNet50 model can result in a performance improvement of approximately 8 times or more.

Compared to PyTorch and TensorFlow model inference speeds, converting models to a TensorRT Engine has shown speed improvements ranging from 5 to 10 times.

```bash
# 1. 필수 시스템 라이브러리 (Ubuntu 기준)
# CUDA Toolkit이 설치되어 있어야 합니다.

# 2. Python 라이브러리 설치
pip install tensorrt tensorrt_dispatch tensorrt_lean
pip install pycuda  # GPU 메모리 할당 및 복사를 위한 필수 라이브러리
pip install onnx    # ONNX 모델 로드용
```

```python
import tensorrt as trt

def build_engine(onnx_file_path, engine_file_path):
    # 1. 로거 및 빌더 초기화
    logger = trt.Logger(trt.Logger.INFO)
    builder = trt.Builder(logger)
    
    # 2. 네트워크 정의 및 ONNX 파서 생성
    network = builder.create_network(1 << int(trt.NetworkDefinitionCreationFlag.EXPLICIT_BATCH))
    parser = trt.OnnxParser(network, logger)

    # 3. ONNX 파일 파싱
    with open(onnx_file_path, 'rb') as model:
        parser.parse(model.read())

    # 4. 빌드 설정 (최적화 프로파일)
    config = builder.create_builder_config()
    config.set_memory_pool_limit(trt.MemoryPoolType.WORKSPACE, 1 << 30) # 1GB 할당
    
    # FP16 양자화 적용 (하드웨어 지원 시 극적인 가속)
    if builder.platform_has_fast_fp16:
        config.set_flag(trt.BuilderFlag.FP16)

    # 5. 엔진 생성 및 저장
    serialized_engine = builder.build_serialized_network(network, config)
    with open(engine_file_path, 'wb') as f:
        f.write(serialized_engine)
    print("Engine build complete!")
```

The code above converts an ONNX model into a TensorRT engine (.plan) file.

Afterwards, looking at the code called by the backend server runtime, the GPU memory copying process is key.

It's easier to think of the code below as being tailored to TensorRT rather than simply using it.

TensorRT handles only model computation; data I/O is the developer's responsibility, so optimization logic must be written by the developer.

This code directly allocates GPU memory addresses and sends data, guiding the engine computation to run.

```python
import pycuda.driver as cuda
import pycuda.autoinit
import numpy as np

class TensorRTInference:
    def __init__(self, engine_path):
        runtime = trt.Runtime(trt.Logger(trt.Logger.WARNING))
        with open(engine_path, 'rb') as f:
            self.engine = runtime.deserialize_cuda_engine(f.read())
        
        self.context = self.engine.create_execution_context()
        self.inputs, self.outputs, self.bindings, self.stream = self.allocate_buffers()

    def allocate_buffers(self):
        inputs, outputs, bindings = [], [], []
        stream = cuda.Stream()
        
        for binding in self.engine:
            size = trt.volume(self.engine.get_binding_shape(binding))
            dtype = trt.nptype(self.engine.get_binding_dtype(binding))
            
            # Host & Device 메모리 할당
            host_mem = cuda.pagelocked_empty(size, dtype)
            device_mem = cuda.mem_alloc(host_mem.nbytes)
            
            bindings.append(int(device_mem))
            if self.engine.binding_is_input(binding):
                inputs.append({'host': host_mem, 'device': device_mem})
            else:
                outputs.append({'host': host_mem, 'device': device_mem})
        return inputs, outputs, bindings, stream

    def infer(self, input_data):
        # 1. 데이터 복사 (Host -> Device)
        np.copyto(self.inputs[0]['host'], input_data.ravel())
        cuda.memcpy_htod_async(self.inputs[0]['device'], self.inputs[0]['host'], self.stream)
        
        # 2. 추론 실행
        self.context.execute_async_v2(bindings=self.bindings, stream_handle=self.stream.handle)
        
        # 3. 결과 복사 (Device -> Host)
        cuda.memcpy_dtoh_async(self.outputs[0]['host'], self.outputs[0]['device'], self.stream)
        self.stream.synchronize()
        
        return self.outputs[0]['host']
```
