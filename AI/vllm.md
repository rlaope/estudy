# vLLM

수십억개의 파라미터를 가진 LLM Large Language Model을 실 서비스에 배포할 때 gpu oom 현상은 단순히 모델의 크기 때문일까?

실제로는 모델이 텍스트를 생성할 때마다 이전 맥락을 기억하기 위해 임시로 저장하는 데이터인 KV 캐시가 메모리 절반 이상을 낭비하며 차지하게 된다.

이 거대하고 파편화된 메모리 누수를 잡지 못하면 수천만원짜리 GPU로도 처리량이 잘 나오지 않는데 이를 시스템적으로 어떻게 처리해야할까

이 문제를 해결하기 위해 도입된것이 UC 버클리 연구진이 개발한 오픈소스 LLM 추론 엔진 **vLLM** 이다.

OS의 가상 메모리 페이징 기법을 차용해 메모리 낭비를 극한으로 줄이고 처리량을 극대화한다.

- **vLLM**: 대규모 언어 모델의 빠르고 효율적인 추론(Inference)과 서빙 목적으로 설계된 고성능 프레임워크
- **KV Cache (Key-Value Cache)**: Tranformer 기반의 모델이 문장을 생성할 때, 앞서 연산이 끝난 토큰들의 어텐션 결과 (key, value 텐서)를 메모리에 캐싱해두고 다음 토큰 생성시 재사용하여 중복 연산을 막는 기법이다.
- **PagedAttenetion**: os의 가상메모리 페이징 알고리즘을 KV 캐시 관리에 적용한 vLLM의 핵심 기술. 캐시를 고정된 크기의 블록 단위로 쪼개어 비연속적인 물리 메모리 공간에 유연하게 할당한다.
- **Continuous Batching (또는 In-flight Batching)**: 정적인 배치 처리 대신, 매 토큰이 생성되는 단위마다 완료된 요청은 내보내고 새로운 요청을 배치 큐에 즉각 편입시켜 GPU의 유휴시간을 없애는 스케줄링 기법이다.

<br>

## The Problem 해결하고자 한 문제

- **KV 캐시의 내부 파편화 Internal Fragmentation**: 기존 프레임워크 (Hugging Face Transformers등)는 메모리 연속성 때문에 새로운 요청이 들어오면 모델이 생성할 수 있는 최대 길이(ex. 2048 token)를 미리 가정하여 거대한 메모리 블록을 사전에 할당한다. 실제 생성된 문장이 짧은 경우 남은 메모리 공간은 다른 요청이 쓰지 못하고 버려진다.
- **Static Batching의 GPU 대기 오버헤드**: 한 번 배치가 묶여 GPU 연산이 시작되면, 그 배치 안에 있는 가장 긴 문장의 생성이 끝날 때까지 배치 전체가 끝날 수 없다. 짧은 문장을 요구한 요청들은 이미 연산이 끝났음에도 GPU 메모리와 스레드를 점유한 채로 대기해야한다.

**즉 기존의 프레임워크는 최대 길이를 가정하여 연속된 메모리를 할당하다보니 실제 사용되지 않는 공간이 버려지는 내부 파편화 문제가 존재했고, 정적 배치 구조 탓에 가장 긴 텍스트 생성을 기다리느라 gpu 코어가 유휴 상태로 노는 한계가 존재했다.**

예를들면 이런 문제다 4명의 사용자가 동시에 LLM에 질문을 던졌을 때, 기존 시스템은 4명 모두에게 2000자 분량의 연속된 VRAM을 강제 할당하여 모메리 초과 OOM을 유발한다.

또한 1명은 1000자를 요구하고 3명은 10자를 요구한다면 10자 생성이 먼저 끝난 3명의 요청이 즉시 반환되지 못하고 나머지 1명의 1000자 생성이 끝날 때까지 새 요청을 받지 못한 채 GPU가 멈춰있게 된다.

### The Solution

- **PagedAttenetion(파편화 제로화)**: vLLM은 KV 캐시를 위해 연속된 메모리를 요구하지 않는다. 전체 vram을 고정된 크기의 페이지 block으로 분할하고 토큰이 생성되어 메모리가 실제 필요해질때마다 논리적 블록과 물리적 블록을 매핑 (block table) 하여 동적으로 할당한다. 이로인해 메모리 낭비율을 기존 60% 이상에서 4% 미만으로 줄인다.
- **Continuous Batching (동적 병렬화)**: LLM이 토큰 하나씩 뱉어낼 때마다 Step by Step 스케줄러가 개입한다. 10자를 요구한 요청이 끝나면 즉시 결과를 클라이언트에게 반환하고, 빈자리에 대기열에 있던 새로운 5번 사용자의 요청을 끼워 넣어 곧바로 다음 토큰 연산을 이어나간다.

<br>

## 상세 동작 원리 및 구조화

vLLM이 메모리를 관리하고 요청을 처리하는 내부 아키텍처다.

1. **Memroy Allocation (블록 단위 할당)**: os의 페이징과 동일하게 작동하고 연속된 토큰들의 KV 텐서를 일정 개수 (ex. 16 token)로 묶어 논리적 블록으로 정의하고 물리적 GPU 메모리 역시 동일한 크기의 물리적 블록으로 쪼개서 관리한다.
2. **Block Table Mapping**: vLLM은 각 사용자 요청마다 Block Table을 유지한다 논리적 블록 0번이 물리적 블록 105번에 논리적 블록 1번이 물리적 블록 12번에 할당되는 식이다. 배열이 물리적으로 흩어져 있어도 테이블을 통해 순서를 정확하게 찾아간다.
3. **Copy on Write (메모리 공유)**: 여러 사용자가 동일한 시스템 프롬프트를 사용할 경우 vLLM은 물리적 블록을 하나에만 해당 프롬프트의 KV 캐시를 저장하고 여러 사용자의 Block Table이 이를 가리키게 한다. 특정 사용자가 고유의 답변을 생성하기 시작할 때만 메모리를 복제하여 극단적인 메모리 절약을 달성한다.
4. **Token Generation (스케줄링)**: 매 반복 iteration 마다 Scheduler는 현재 가용 물리 블록 수를 평가하고 공간이 부족하면 중요도가 낮은 요청을 cpu 메모리로 밀어내고 swapping, 여유가 생기면 다시 vram으로 가져온다. 이를 preemption 이라고함.

### Example

내부 서버의 백그라운드 작업이나 네트워크 api 서버를 띄우지 않고도 py 코드 내에 직접 PagedAttention 엔진을 가동하여 일괄 처리를 수행하는 기본로직이다. 

```py
from vllm import LLM, SamplingParams

def run_vllm_offline():
    """vLLM 엔진을 메모리에 로드하고 다수의 프롬프트를 일괄 병렬 처리합니다."""
    
    prompts = [
        "What is the capital of France?",
        "Explain the theory of relativity in simple terms.",
        "Write a short python code to reverse a string."
    ]

    # 1. 샘플링 파라미터 정의 (온도, 최대 생성 길이 등)
    # 기존 프레임워크와 달리 max_tokens를 1000으로 잡아도 실제 메모리는 즉시 점유되지 않습니다.
    sampling_params = SamplingParams(temperature=0.8, top_p=0.95, max_tokens=100)

    # 2. vLLM 엔진 초기화 및 모델 로드
    # 내부적으로 GPU 메모리를 스캔하고 PagedAttention을 위한 블록 할당을 준비합니다.
    print("Loading LLM Engine...")
    llm = LLM(model="facebook/opt-125m") 

    # 3. 추론 실행
    # 내부 스케줄러가 Continuous Batching을 적용하여 prompts 리스트를 초고속으로 처리합니다.
    outputs = llm.generate(prompts, sampling_params)

    # 4. 결과 출력
    for output in outputs:
        prompt = output.prompt
        # 최종 생성된 텍스트 추출
        generated_text = output.outputs[0].text
        print(f"Prompt: {prompt!r}\nGenerated: {generated_text!r}\n")

# run_vllm_offline()
```

실제 프로덕션 환경에서 vLLM을 단독 api 서버로 듸우고 OpenAI API 스펙 완벽 호환하는 클라이언트가 웹 요청을 통해 LLM과 통신하는 표준 아키텍처다.

#### 서버 실행 - 터미널

vLLM은 자체적으로 FastAPI 기반의 고성능 서버 모듈을 내장하고 있다.

```bash
# 터미널에서 vLLM 서버 구동 명령 실행
# --model: 사용할 HF 모델
# --gpu-memory-utilization: VRAM의 몇 %를 KV 캐시와 모델 가중치에 할당할지 (기본 0.9)
# --max-model-len: 컨텍스트 윈도우 길이
python -m vllm.entrypoints.openai.api_server \
    --model meta-llama/Llama-2-7b-chat-hf \
    --host 0.0.0.0 \
    --port 8000 \
    --gpu-memory-utilization 0.90 \
    --max-model-len 4096
```

#### 클라이언트 통신

서버가 띄워지면, 클라이언트는 vLLM 서버를 마치 OpenAI 서버 (chatgpt) 처럼 대하며 `openai` 라이브러리를 사용해 요청을 보낼 수 있다. 이 과정에서 vLLM Continous Batching이 동작한다.

```py
from openai import OpenAI

def call_vllm_api_server():
    """OpenAI 라이브러리를 사용하여 로컬 vLLM 서버에 텍스트 생성을 요청합니다."""
    
    # OpenAI 클라이언트 인스턴스 생성하되, 엔드포인트를 로컬 vLLM 서버 주소로 변경합니다.
    # api_key는 vLLM 서버에서 검증하지 않으므로 더미 값을 넣습니다.
    client = OpenAI(
        base_url="http://localhost:8000/v1",
        api_key="EMPTY"
    )

    # ChatCompletion 표준 API 호출
    completion = client.chat.completions.create(
        model="meta-llama/Llama-2-7b-chat-hf",
        messages=[
            {"role": "system", "content": "You are a highly skilled software engineer."},
            {"role": "user", "content": "Can you explain how PagedAttention works?"}
        ],
        max_tokens=200,
        temperature=0.7
    )

    # vLLM 서버가 처리하여 반환한 결과 출력
    print("vLLM Response:")
    print(completion.choices[0].message.content)

# call_vllm_api_server()
```

<br>

## 순차처리와 연속 배치

연속 배치를 하는것과 순차처리의 핵심적인 차이를 좀 더 알아보겠다.

배치는 작업요청을 한 묶음으로 batch size만큼 처리하는 것이라 개별작업이 끝나도 다른것이 끝나기전까지 대기해야하는 특성이 있고 이로인해서 비효율이 발생했던것을 방금까지 정의해보았다.

연속 배치는 큐에서 한 작업이 완료되면 반환하고 다음 요청을 큐잉해 처리하는데 이게 순차처리랑 어떤 차이가 있을지를 더 알아보자.

핵심적인 차이는 **GPU의 병렬연산을 활용하는 상태에서 교체가 일어나는가?**이다.

"작업이 끝나고 바로 반환하고 새 요청을 넣는다."는 말은 1대1 교대 (순차처리)로 오해하기 쉬우나, 연속 배치(Continuous Batching)는 **수십에서 수백개의 요청을 동시에 병렬로 처리하는 와중에, 개별 요청의 종료와 진입이 톱니바퀴처럼 실시간 교대된다는 의미다.**

1. **순차처리 batch size 1**: 한 번에 1개의 요청만 gpu에 넣고 a가 1000글자를 다 생성할때까지 b c d는 연산을 아예 시작조차 안한다. 거대한 gpu 코어의 90퍼이상이 놀게되어 자원낭비가 심함
2. **기존 정적 배치**
   - gpu 효율을 높이기 위해 a b c d 4개를 한 번에 묶어 동시에 1글자씩 생성한다
   - a는 10글자만 필요하고 b는 1000이라면 a의 10글자 생성이 끝나도 b 1000글자가 끝날때까지 a는 결과를 반환받지 못하고 gpu 메모리를 점유한채 대기한다 빈자리 새요청 e를 넣을 수 없고 4명이 다 끝나야 다음 요청 배치를 받는다
3. **연속 배치 Continuous Batching / In-flight Batching**
   - A, B, C, D 4개를 묶어 동시에 병렬 생성하는 것이 정적 배치와 같다
   - 핵심 차이점은 LLM이 1글자 토큰을 뱉어내는 매 단계마다 상태를 검사한다.
   - A가 10글자를 완성하면 10번째 틱에 배치해서 A만 쏙 빼서 클라이언트에게 즉시 반환해버린다.
   - 그리고 A가 빠져나가서 빈 공간이 된 GPU 메모리에 대기 큐에 있던 새요청 E의 프롬프트를 즉시 밀어넣는다.
   - 바로 다음 11번ㅉ ㅐ토큰 생성 틱에는 GPU가 대기시간없이 곧바로 E B C D를 묶어 병렬연산을 이어갈 수 있다.

요약하면 순차처리를 한 번에 하나만 연산하는 구조고 연속 배치는 **항상 gpu의 처리 한계치를 꽉 채워서 동시에 연산하되 텍스트 생성이 끝나는 타이밍이 제각각이더라도 낭비되는 턴 없이 토큰 단위로 실시간 교체를 수행하는 고밀도 병렬 스케줄링 기술이다.**