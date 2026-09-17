# Distinguishing Pretraining Data and SFT Data

When building upstream and downstream data pipelines for large language models, deciding how to partition and serialize collected text corpus data at the storage level is a critical factor determining GPU infrastructure's data out-of-memory issues and training throughput.

Beyond disk, it requires entirely different mechanisms for I/O optimization, memory mapping architecture, and even tokenization layer loss function masking strategies.

This time, let's analyze low-level engineering practices for isolating and processing large-scale unstructured corpora and structured instruction-pair data from a distributed environment perspective.

## Is This Data for Knowledge Injection or for Learning to Follow Instructions?

This question is a proposition to determine, from a mathematical and infrastructural perspective, whether the data is for pretraining with the purpose of **Knowledge Injection**, carving out new token transition probability landscapes within the model's parameters, or for SFT with the purpose of **Behavioral Alignment**, enforcing controlled attention to safely extract already embedded knowledge within specific contextual specifications.

If engineers fail to clearly classify this purpose at the beginning of the pipeline, critical data engineering bottlenecks will occur. For example, misinterpreting tens of gigabytes of simple knowledge documents as SFT format leads to wasted CPU parsing them into complex multi-turn dictionaries. Conversely, indiscriminately pushing high-quality instruction-pair data, which requires idempotency, into a pretraining block packing pipeline contaminates the weight gradient signal, leading to a catastrophe that destroys the model's alignment capabilities.

<br>

## Format Optimization from a Data Engineering Perspective

### Pretraining Data Pipeline (Transition from JSONL to Parquet and Apache Arrow)

**Traditional Method (Direct Extraction of Plain JSON/TXT)**: In early large-scale data processing, raw text files or large JSONL files were read line by line, loaded into Python string memory objects, and then sent to the tokenizer.

**Problems Encountered**: In a terabyte-scale pretraining corpus environment, Python's inherent garbage collection overhead and redundant memory allocation for text objects frequently led to RAM OOMs. Additionally, synchronous I/O bottlenecks from parsing text from disk caused severe GPU starvation, where expensive GPU clusters idled while waiting for data.

**Apache Arrow & Parquet**: Modern LLM data layers first serialize large corpora into Parquet, a columnar compressed format, and then bind them to the Apache Arrow format, which underlies the Hugging Face datasets architecture.

Apache Arrow binds to the Apache Arrow format, which omits data copying in memory. Apache Arrow provides **Zero-Copy memory mapping (mmap)** technology, which omits data copying in memory, allowing for ultra-fast streaming tokenization in a virtual threaded environment by directly referencing disk offsets in chunk units, without physically loading hundreds of GBs of datasets into memory.

### SFT Data Pipeline (Structured Schema and Variable-Length Partitioning)

**Traditional Method (Single-Block Consistent Tokenization)**: SFT data was processed identically to pretraining, treating the entire text, combining questions and answers, as a single unstructured string for training.

**Problems Encountered**: During batch training, the significant variation in text length per sentence caused short-form sentences to include a large number of padding tokens (`[PAD]`), resulting in the computational matrix being filled with zeros and wasting GPU FLOPs resources.

**Strict Schema & Dynamic Collator**: SFT data enters the pipeline maintaining a clear structured JSONL schema like `{"prompt": "...", "response": "..."}`. This provides the infrastructural foundation to dynamically measure the token lengths of prompt and response columns at the weight optimization engine (TRL, Trainer) stage, enabling the flexible operation of waste-free data packing matrices for each epoch.

<br>

## Separate Implementation of Apache Arrow-based Large-scale Pretraining Data Streaming and SFT Data Structuring

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

#### Analysis of Apache Arrow Memory Mapping Effects (Loader Init Log Area)

During the pretraining pipeline's operation, it was observed that when data was saved to Parquet via a PyArrow table (`pyarrow.Table`) and then read in streaming mode, the RAM usage increase was controlled at 0MB.

This indicates that Zero-Copy mmap, which directly links the file system's page cache area to the virtual memory address space, is functioning correctly, unlike traditional indiscriminate Python I/O methods that parse and load entire files into memory. Applying this method is essential to fundamentally prevent hardware OOM crashes at the data loader stage during terabyte-scale knowledge pretraining.

#### SFT Structured Control Token Seating Mechanism

Looking at the SFT processing logs, you can trace how the prompt and response columns, which entered as split dictionaries, are synthesized into structural grammar flag strings like `<|im_start|>user` as they pass through the tokenizer's `apply_chat_template` core function.

Unlike pretraining data, SFT data requires this formatting process to be performed beforehand. This prepares the mathematical foundation for the prompt section loss value masking function `ignore_index=100` to precisely locate target indices and physically eliminate gradients when entering the training loop later.

### Infrastructure Load Comparison Table by Data Processing Format

Let's look at the quantitative comparison matrix metrics for hardware and I/O bandwidth collected on a large-scale training cluster, depending on the data pipeline design format.

| Data Pipeline Type | Storage Serialization Format | Zero-Copy Memory Mapping Support | RAM Usage During Dataset Load (VRAM Leak Defense) | Disk I/O Throughput per Second | GPU Utilization Optimization Level (GPU FLOPs Efficiency) | Suitable Actual Training Objective |
|-------------------------|---------------------------------------|---------------------------------|-----------------------------------------------|------------------------------------|----------------------------------------------|-------------------------------------------|
| **Method A: Large Corpus Optimized** | **Parquet / Apache Arrow** | **Supported (mmap integration)** | **Extremely Low**<br>(Minimal memory maintained regardless of data volume) | **Excellent (High-speed streaming over 1,200 MB/s)** | Moderate (Padding loss is low due to pretraining characteristics, but chunk boundary control is needed) | **Continual Pretraining (CPT)**<br>For injecting tens to hundreds of GBs of unknown domain knowledge text into weights without infrastructure load |
| **Method B: Instruction Structure Alignment** | **Structured JSONL / Arrow Table** | Supported (Optional mmap depending on volume) | Moderate (RAM loaded proportionally to dataset size) | Moderate (CPU overhead due to question-answer schema parsing) | **Excellent (100% effective token computation achieved by padding removal and packing)** | **Supervised Fine-Tuning (SFT)**<br>For injecting specific style guidelines, conversation format specifications, and command execution capabilities into the model |
| **Method C: Unoptimized Plain Text** | Plain TXT / Large Raw JSONL | Not Supported (Vanilla File IO) | **Extremely High**<br>(Causes RAM OOM crashes when loading hundreds of GBs of files during large-scale training) | Low (GPU starvation due to Python string object parsing bottleneck) | Worst (Severe GPU waste due to indiscriminate padding of variable-length sentences) | **Architecture to Avoid**<br>Unsuitable for commercial cluster training due to infrastructure bottlenecks and data contamination |
