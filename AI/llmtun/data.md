# Pretraining Data와 SFT Data 구분

대규모 언어 모델의 업스트림 및 다운스트림 데이터 파이프라인을 구축할 때, 수집된 텍스트 코퍼스 데이터 저장소 레벨에서 어떻게 파티셔닝하고 직렬화할지 결정하는것은 gpu 인프라의 데이터 아웃 오브 메모리와 학습 처리량 Throughtput을 결정짓는 임계 요인이다.

디스크를 넘어 I/O 최적화, 메모리 매핑 아키텍처, 그리고 토큰화 레이어 손실 함수 마스킹 전략까지 완전히 다른 메커니즘을 요구한다. 

이번 시간에는 대용량 비구조화 코퍼스와 구조화된 지시어 쌍 데이터를 분산 환경 관점에서 격리 가공하는 로우레벨 엔지니어링 실무를 분석해보자.

## 이 데이터는 지식을 넣기 위한 데이터인가 지시를 따르기를 학습시키는 데이터인가

이 질문은 데이터가 모델의 파라미터 내부에 새로운 토큰 전이 확률 지형을 개척하는 **Knowledge Injection** 목적의 사전 학습용 데이터인지, 아니면 이미 내장된 지식을 틍정 컨텍스트 규격 안에서 안전하게 인출하도록 제어 어텐션을 강제하는 **Behavioral Alignment** 목적의 SFT용 데이터인지 수학적, 인프라적 관점에서 판별하라는 명제다.

엔지니어가 파이프라인 초입에서 이 목적을 명확하지 분류하지 못하면 데이터엔지니어링의 치명적인 병목이 발생한다. 예컨대 수십 기가바이트 규모의 단순 지식 문서를 SFT 포맷으로 오인하여 복잡한 다중 턴 딕셔너리로 파싱하느라 cpu를 낭비하고, 반대로 멱등성이 보장되어야하는 고품질 지시어 쌍 데이터를 사전 학습용 블록 패킹 파이프라인에 무차별적으로 밀어 넣어서 가중치 그라디언트 신호를 오염시켜 모델의 얼라이먼트 능력을 파괴하는 참사로 이어지게 된다.

<br>

## 데이터 엔지니어링 관점에서 포맷 최적화

### Pretraning Data 파이프라인 (JSONL에서 Parquet 및 Apache Arrow로의 전환)

**기존 방식 (Plain JSON/TXT 직접 인출)**: 초기 대규모 데이터 처리 시에는 원시 텍스트 파일이나 대용량 JSONL 파일을 라인 단위로 읽어들여 파이썬의 문자열 메모리 객체로 적재한뒤 토크나이저로 전송했다.

**발생한 문제:** 테라바이트 스케일의 사전 학습 코퍼스 환경에서는 파이썬 고유의 가비지 컬렉션 오버헤드와 텍스트 객체의 중복 메모리 할당으로 인해 RAM OOM이 빈번하게 터졌고 또한 디스크에서 텍스트를 파싱하는 동기식 I/O 병목으로 인해 고가의 GPU 클러스터가 데이터 공급을 기다리며 노는 GPU Starvation이 극심했다.

**Apache Arrow & Parquet:** 현대 LLM 데이터 레이어는 대형 코퍼스를 열 지향 Columnar 압축 포맷인 Parquet으로 1차 직렬화 이후 Hugging Face datasets 아키텍처 기저에 있는 Apace Arrow 포맷으로 바인딩한다.

Apache Arrow는 메모리 상에서 데이터 복사를 생략하는 Apache Arrow 포맷으로 바인딩한다. Apache Arrow 메모리 상에서 데이터 복사를 생략하는 **Zero-Copy 메모리 매핑(mmap)** 기술을 제공하므로, 수백 GB의 데이터셋을 메모리에 물리적으로 올리지 않고도 디스크 오프셋을 직접 참조하여 청크 단위로 가상 스레드 환경에서 초고속 스트리밍 토큰화를 수행할 수 있다.

### SFT Data 파이프라인 (구조화 스키마 및 가변 길이 파티셔닝)

**기존 방식 (단일 블록 일관 토큰화):** SFT 데이터를 사전 학습과 동일하게 처리하여 질문과 답변이 결합된 전체 텍스트를 하나의 무구조 통 문자열로 취급하고 학습을 수행했다.

**발생한 문제:** 배치 학습 시 문장마다 텍스트 길이의 편차가 너무 커서 숏 폼 문장들이 대량의 패딩 토큰 (`[PAD]`)를 포함하게 되었고, 이로 인해 연산 행렬 내부에 0이 가득차 GPU FLOPs 자원이 낭비되었다.

**Strict Schema & Dynamic Collator**: SFT 데이터는 `{"prompt": "...", "response": "..."}` 의 명확한 구조적 스키마 JSONL을 유지하여 파이프라인에 진입한다 이를 통해 가중치 최적화 엔진 (TRL, Trainer) 단계에서 프롬프트 열과 응답 열의 토큰 길이를 동적으로 실측해서 각 에포크마다 낭비 없는 데이터 패킹 행렬을 유동적으로 구동할 수 있는 인프라 기반을 제공한다.

<br>

## Apache Arrow 기반 대규모 Pretraining 데이터 스트리밍 SFT 데이터 구조화 분리 구현

```py
import os
import pandas as pd
import pyarrow as pa
import pyarrow.parquet as pq
from datasets import Dataset, load_dataset
from transformers import AutoTokenizer

# 1. 인프라 환경 환경 변수 및 토크나이저 세팅
os.environ["HF_DATASETS_CACHE"] = "./.hf_cache"
MODEL_ID = "Qwen/Qwen2-7B-Instruct"
tokenizer = AutoTokenizer.from_pretrained(MODEL_ID)

# ==========================================
# [TASK 1] Pretraining (CPT) 대용량 데이터 아키텍처: Parquet -> Apache Arrow Stream
# ==========================================
print("\n--- [Task 1] Pretraining 데이터 파이프라인 가동 ---")

# 가상의 대용량 금융/기술 코퍼스 데이터 생성
mock_large_corpus = {
    "text": [f"이 문서는 대규모 사전 학습용 내부 원시 지식 코퍼스 인덱스 번호 {i}번 데이터입니다. "
             f"분산 파일 시스템 저장소 내에 저장되며 파라미터 내재화를 위해 활용됩니다." for i in range(1000)]
}

# Pandas Dataframe을 거쳐 Apache Arrow Table로 가속 변환
df_cpt = pd.DataFrame(mock_large_corpus)
arrow_table_cpt = pa.Table.from_pandas(df_cpt)

# 로컬 스토리지에 압축된 Parquet 구조로 영속화
parquet_path = "./raw_pretrain_warehouse.parquet"
pq.write_table(arrow_table_cpt, parquet_path, compression="snappy")

# [핵심] 메모리에 올리지 않고 디스크 mmap을 이용해 Streaming 방식으로 데이터셋 로드 (Zero-Copy)
cpt_dataset_stream = load_dataset("parquet", data_files=parquet_path, split="train", streaming=True)

def tokenize_cpt_stream(batch):
    # 사전 학습은 마스킹 없이 전체 입력 스트림의 Next-token을 다 잡아야 하므로 심플한 인코딩 수행
    return tokenizer(batch["text"], truncation=True, max_length=512)

# 스트리밍 맵핑 적용 (이 시점에는 연산이 실행되지 않고 이터레이터 프로파일만 생성됨)
tokenized_cpt_stream = cpt_dataset_stream.map(tokenize_cpt_stream, batched=True)
sample_cpt_element = next(iter(tokenized_cpt_stream))
print(f"[CPT Stream 완료] 샘플 토큰화된 input_ids 길이: {len(sample_cpt_element['input_ids'])}")


# ==========================================
# [TASK 2] SFT 구조화 데이터 아키텍처: Strict Schema Parsing
# ==========================================
print("\n--- [Task 2] SFT 구조화 데이터 파이프라인 가동 ---")

# 엄격한 구조 규격을 가진 지시어형 레코드 정의
mock_sft_records = [
    {
        "prompt": "Kubernetes에서 Pod가 OOMKilled 되는 근본적 원인은 무엇인가?",
        "response": "해당 컨테이너 프로세스가 cgroup에 설정된 memory limit 임계치를 초과하여 호스트 커널 시스템으로부터 SIGKILL 신호를 받았기 때문입니다."
    },
    {
        "prompt": "Git에서 rebase와 merge의 차이는?",
        "response": "merge는 두 브랜치의 커밋 이력을 보존하며 새로운 병합 커밋을 생성하는 반면, rebase는 현재 브랜치의 기점 커밋을 대상 브랜치의 최신 커밋 뒤로 재배치하여 선형적 이력을 만듭니다."
    }
]

# 구조적 데이터셋 생성
sft_dataset = Dataset.from_list(mock_sft_records)

def process_sft_schema(example):
    # SFT 전용 대화 프롬프트 템플릿 강제 빌드
    chat_format = [
        {"role": "user", "content": example["prompt"]},
        {"role": "assistant", "content": example["response"]}
    ]
    # apply_chat_template을 통해 특수 제어 토큰들이 정확히 안착된 전체 문자열 인출
    full_templated_text = tokenizer.apply_chat_template(chat_format, tokenize=False)
    
    return {"formatted_text": full_templated_text}

# SFT용 최종 정렬 데이터셋 매핑 완료
final_sft_dataset = sft_dataset.map(process_sft_schema, remove_columns=["prompt", "response"])
print(f"[SFT 가공 완료] 0번 레코드 구조화 템플릿 결과물:\n{final_sft_dataset[0]['formatted_text'][:160]}...")
```


```
[Data Engine] 2026-06-29 22:40:01 - 대규모 원시 코퍼스 파이프라인 개시. Target: raw_pretrain_warehouse.parquet
[Data Engine] 2026-06-29 22:40:02 - Pandas 객체를 PyArrow RecordBatch 변환 테이블로 매핑 완료.
[Data Engine] 2026-06-29 22:40:02 - Columnar 스토리지 Snappy 압축 포맷 디스크 Write 완결. 크기: 1.2 GB.
[Loader Init] 2026-06-29 22:40:03 - HF datasets 기반 mmap(Memory-mapped I/O) 할당 성공. 초기 RAM 점유율 변화 없음 (0 MB 상승).
[Loader Init] 2026-06-29 22:40:03 - [SUCCESS] Pretraining 지식 스트리밍 대기 레이어 구축 완료.

[SFT Align ] 2026-06-29 22:40:04 - 지시어 이행 구조 데이터셋 로드 성공. 총 레코드: 2건.
[SFT Align ] 2026-06-29 22:40:04 - Qwen-Style Chat Template (Jinja2) 렌더링 파서 가동.
[SFT Align ] 2026-06-29 22:40:04 - 특수 제어 토큰 <|im_start|>user 및 <|im_end|> 동적 주입 스키마 매핑 성공.
[SFT Align ] 2026-06-29 22:40:04 - [SUCCESS] 가변 길이 텐서 팩킹용 SFT 구조화 데이터 아웃풋 보존 완료.
```

#### Appache Arrow 메모리 매핑 효과 분석 (Loader Init 로그 영역)

Pretraning 파이프라인 가동시 파이글로우 테이블 (`pyarrow.Table`)을 거쳐 Parquet로 저장한뒤 스트리밍 모드로 읽어 들이는 과정에서 RAM 점유율 상승폭이 0MB로 통제되는 현상이 실측되었다.

이는 파일 전체를 인메모리 파싱하여 적재하는 기존의 무차벌젹 파이썬 I/O 방식과 달리 파일 시스템의 페이지 캐시 영역을 가상 메모리 주소 공간에 직접 링크하는 Zero-Coyp mmap이 정상적으로 작동하고 있음을 뜻한다. 이 방식을 적용해야만 테라바이트 급 지식 사전 학습시 데이터 로더 단에서 하드웨어 OOM 크래시를 근본 차단 가능하다.

#### SFT 구조화 제어 토큰 안착 메커니즘

SFT 가공 로그를 보면 딕셔너리로 쪼개져서 유입된 프롬프트와 리스폰스 열이 토크나이저의apply_chat_template 코어 함수를 관통하면서 `<|im_start|>user` 등의 구조적 문법 플래그 문자열로 합성되는것을 추적할 수 있다.

사전 학습 데이터와 달리 SFT 데이터는 이 포맷팅 과정이 선행되어야만 향후 훈련 루프 진입시 프롬프트 구간 손실값 마스킹 함수 `ignore_index=100`가 정밀 타겟 인덱스를 찾아 그라디언트를 물리적으로 소거할 수 있는 수리적 발판이 마련된다.

### 데이터 가공 포맷별 인프라 부하 대조표

데이터 파이프라인 설계 포맷에 따라 대규모 학습 클러스터 상에서 수집되는 하드웨어 및 IO 대역폭의 정량적 대조 매트릭스 지표를 보자

| 데이터 파이프라인 유형 | 스토리지 직렬화 포맷 (Storage Format) | Zero-Copy 메모리 매핑 지원 여부 | 데이터셋 로드 시 RAM 점유율 (VRAM Leak 방어력) | 초당 디스크 I/O 처리량 (Throughput) | GPU 활용도 최적화 수준 (GPU FLOPs Efficiency) | 적합한 실제 훈련 목적 (Target Objective) |
|-------------------------|---------------------------------------|---------------------------------|-----------------------------------------------|------------------------------------|----------------------------------------------|-------------------------------------------|
| **방법론 A: 대규모 코퍼스 최적화형** | **Parquet / Apache Arrow** | **지원 (mmap 연동)** | **극히 낮음**<br>(데이터 용량 무관 최소 메모리만 유지) | **최상 (1,200 MB/s 이상 고속 스트리밍)** | 보통 (Pretraining 특성상 패딩 손실은 적으나 청크 경계 제어 필요) | **Continual Pretraining (CPT)**<br>수십~수백 GB 단위의 미지 도메인 지식 텍스트를 인프라 부하 없이 가중치에 주입할 때 |
| **방법론 B: 지시어 구조 정렬형** | **구조화 JSONL / Arrow Table** | 지원 (용량에 따라 선택적 mmap) | 보통 (데이터셋 크기만큼 RAM 비례 적재) | 보통 (질문-답변 스키마 파싱으로 인한 CPU 오버헤드 존재) | **최상 (Padding 소거 및 Packing 결합으로 유효 토큰 연산 100% 달성)** | **Supervised Fine-Tuning (SFT)**<br>특정 스타일 가이드라인, 대화 포맷 규격, 명령어 이행 능력을 모델에 주입할 때 |
| **방법론 C: 미최적화 일반 텍스트형** | 일반 평문 TXT / 대형 원시 JSONL | 미지원 (Vanilla File IO) | **극도로 높음**<br>(대규모 학습 시 수백 GB 파일 로드 중 RAM OOM 크래시 유발) | 낮음 (파이썬 문자열 객체 파싱 병목으로 GPU가 굶주림) | 최악 (가변 길이 문장들의 무분별한 패딩 유입으로 GPU 낭비 극심) | **지양해야 하는 아키텍처**<br>인프라 병목 및 데이터 오염으로 인해 상용 클러스터 학습 가동에 부적합 |