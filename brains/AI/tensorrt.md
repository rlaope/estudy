# TensorRT

TensorRT는 학습된 딥러닝 모델을 최적화하여 nvidia gpu상에서 추론속도를 수배 ~ 수십배까지 향상시켜

딥러닝 서비스를 개선하는데 도움을 주는 모델 최저고하 엔진이다.

caffe, pytorch, tensorflow, paddlepaddle등 딥러닝 프레임워크를 통해 짜여진 딥러닝 모델을 TensorRT를 통해 최적화해 tesla t4 jetson tx2, tesla v100등 nvidia gpu 플랫폼에 실을수있었던것

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdna%2Fqf0WL%2FbtqDA4p61z3%2FAAAAAAAAAAAAAAAAAAAAAAZ6IzaPITLCybzaEB58m94mNAKbd5uBQi5S3fwQ1FAE%2Fimg.png%3Fcredential%3DyqXZFxpELC7KVnFOS48ylbz2pIh7yKj8%26expires%3D1772290799%26allow_ip%3D%26allow_referer%3D%26signature%3DesvP8kIt3KzmZNr6fFJ8zwemYf4%253D)

또한 tensorRT는 nvidia gpu연산에 적합한 최적화 기법들을 이용하는 모델을 최적화하는 optimizer와 다양한 gpu에서 모델 연산을 수행하는 runtime engine을 포함한다.

tensorRT는 대부분 딥러닝 프레임워크에서 학습된 모델을 지원하며 최적의 딥러닝 모델 가속화를 지원한다.

cpp, python을 api level에서 지원하고 있어서 gpu programming CUDA 지식이 별로 없어도 딥러닝 분야의 개발자들이 쉽게 이용이 가능하다.

또한 gpu가 지원하는 활용 가능한 최적의 연산 자원을 자동으로 사용할 수 있도록 Runtime binary 를 빌드해주기 때문에 레이턴시 및 스루풋을 쉽게 향상시킬 수 있고 이를 통해 딥러닝 프로그램 및 서비스 효율적인 실행이 가능하다.

커스터마이징 할수있는 방법론을 제공한다. 어렵긴한데 어쨋든 개발자들이 유연하게 활용할 수 있도록 해준다는것만 알아두자.

nvidia platfor에서 최적의 추론 성능을 낼수있도록 Network compression, optimzation, gpu 최적화등 자동 적용해준다. 그 기법들을 좀 알아보자.

### 양자화 및 정밀도 캘리브레이션 quantization & precision calibration

딥러닝의 학습 및 추론에서 정밀도 precision을 낮추는 일은 거의 일반적인 방법이 되었다.

낮은 정밀도를 가지는 신경망일수록 데이터의 크기 및 가중치들의 bit 수가 작기 때문에 더 빠르고 효율적인 연산이 가능하다.

이를 위한 양자화 기법중 tensorRT는 symmetric linear quantization을 사용하고 있으며, 이를 통해 딥러닝 프레임워크의 일반적인 FP32의 데이터를 FP16 or INT8 데이터 타입으로 정밀도를 낮출 수 있다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdna%2FdqbtaT%2FbtqDzkHbdoL%2FAAAAAAAAAAAAAAAAAAAAAB_Kz_Vjn_i8xgWKFmTaqT_rwVHRM1JPjTCnYoNmoQ9B%2Fimg.png%3Fcredential%3DyqXZFxpELC7KVnFOS48ylbz2pIh7yKj8%26expires%3D1772290799%26allow_ip%3D%26allow_referer%3D%26signature%3D4hFiw5tdYfQcKHkN57AIgkn4jrU%253D)

일반적으로 FP16의 데이터타입 정밀도를 낮추는 것은 정확도에 큰 영향을 주지는 않는데

**INT8의 데이터타입으로 정밀도를 낮추는것은 정확도에 영향을 준다. 그래서 추가적으로 캘리브레이션 작업이 필요하다**

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdna%2FceTH0T%2FbtqDzEZICqF%2FAAAAAAAAAAAAAAAAAAAAAND9Cx7BMHDYdc0k_qOXpc3lmf4LnG4_Aoclw7ZhczsY%2Fimg.png%3Fcredential%3DyqXZFxpELC7KVnFOS48ylbz2pIh7yKj8%26expires%3D1772290799%26allow_ip%3D%26allow_referer%3D%26signature%3DJITLU%252BtfGpwJvkLmfl6LqEozB78%253D)

TensorRT에서는 EntropyCailbrator(2), MinMaxCalibrator를 지원하고 있고

이를 활용해서 양자화 시 가중치 및 intermediate tensor들의 정보 손실을 최소화할수있다.

### Graph Optimization

일반적으로 그래프 최적화는 딥러닝 신경망에서 사용되는 Primitive 연산 형태, Compound 연산 형태의 그래프 노드들을 각 플랫폼에 최적화된 코드들로 구성하기 위해 사용된다.

TensorRT에서는 이를 기반으로 layer fusion, tensor fusion 방식을 동시에 적용중이다.

layer fusion은 vertical layer fusion, horizaontal layer fusion이 적용되고 tensor fusion이 또한 적용되어 모델 그래프를 단순화 시켜 모델 레이어수가 크게 감소한다.

RestNet, MobileNet과 같은 백본 신경망들을 최적화하면 기존 노드 수가 몇십배까지 줄어드는 효과를 보았다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdna%2FchgvZQ%2FbtqDzEFuM6C%2FAAAAAAAAAAAAAAAAAAAAAFNitBqDjZvOC4X3GqBmkD7zL-AX8HPZ_99NohmoIQnD%2Fimg.png%3Fcredential%3DyqXZFxpELC7KVnFOS48ylbz2pIh7yKj8%26expires%3D1772290799%26allow_ip%3D%26allow_referer%3D%26signature%3DwTx1IXFT2ek4xqSX%252BY6ewVYdh%252Bo%253D)

### 커널 자동 튜닝

TensorRT는 NVIDIA의 다양한 플랫폼 및 아키텍처에 맞는 Runtime 생성을 도와준다.

각 제품들은 CUDA engine의 갯수, 메모리 그리고 serialized engine 포함 여부에 따라 최적화된 커널이 다르기 때문에 이를 TensorRT Runtime engine build시 선택적으로 수행해 engine binary가 최적으로 생길수있도록 돕는다.

### dynamic tensor memroy & multi-stream execution

그 외에도 메모리 관리를 통해 풋프린트를 줄여 재사용할 수 있도록 도와주 동적 텐서 메모리 기능이 있고

cuda stream 기술을 사용해 multiple input stream의 스케쥴링을 통해 병렬 효율을 극대화가 가능하다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdna%2FsphHA%2FbtqDyMKKE0i%2FAAAAAAAAAAAAAAAAAAAAABaRHebQN0SIEF_ZDO86X11NUdo-vOdhkAqh7QXIhLAe%2Fimg.png%3Fcredential%3DyqXZFxpELC7KVnFOS48ylbz2pIh7yKj8%26expires%3D1772290799%26allow_ip%3D%26allow_referer%3D%26signature%3Djg9G6zz2ZPoHtnxKaNY9hi0LwEw%253D)

이런 가속화 기술들로 tensorRT는 속도향상이라는 결과를 얻을 수 있고.

기본적으로 ResNet50 기준으로 동일한 gpu에서 tensorRT를 사용하는것만으로 대략 8배 이상의 성능 향상 효과가 있다고 한다.

Pytorch, tensorflow 모델 추론 속도에 비해 tensorRT Engine화 하였을때 적게는 5배 많게는 10배까지 속도 향상 결과가 있었다.

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

위 코드에서 onnx 모델을 tensorRT엔진 .plan으로 변환한다.

그 이후 백엔드 서버 런타임에서 호출되는 코드를 보면 Gpu 메모리 복사 과정이 핵심인데

아래코드는 tensorRT를 쓰는게 아니고 맞춰서 짜는 느낌이라고 생각하면 편함

tensorRT는 모델 연산만 담당하지 데이터 I/O는 개발자 담당이라서, 최적화로직은 개발자가 직접 작성해야함

Gpu 메모리 주소를 직접할당하고 데이터를 쏴주는 코드를 둬서 엔진 연산 자체가 돌아갈 수 있도록 유도하는코드다

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