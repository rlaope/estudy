# buffer_head

리눅스 커널의 `buffer_head` 구조체는 파일 시스템의 블록과 메모리의 페이지 사이를 연결하는 가교 역할을 한다.

실제 리눅스 소스코드에 정의된 주요 필드를 확인해보면 아래다

struct buffer_head

```c
struct buffer_head {
    unsigned long b_state;          /* 버퍼의 상태 플래그 (Dirty, Up-to-date 등) */
    struct buffer_head *b_this_page;/* 같은 페이지 내의 다음 buffer_head를 가리킴 (순환 연결 리스트) */
    struct page *b_page;            /* 이 버퍼가 속해 있는 물리 페이지 */

    sector_t b_blocknr;             /* 디스크상의 논리 블록 번호 (Logical block number) */
    size_t b_size;                  /* 블록의 크기 (예: 1024 bytes) */
    char *b_data;                   /* 페이지 내에서 데이터가 시작되는 위치의 포인터 */

    struct block_device *b_bdev;    /* 이 블록이 속한 블록 장치 (하드디스크 등) */
    bh_end_io_t *b_end_io;          /* I/O 완료 시 호출될 콜백 함수 */
    void *b_private;                /* b_end_io용 개인 데이터 */
    
    atomic_t b_count;               /* 사용 횟수 (Reference count) */
};
```

page와 buffer head관계를 알아보면, 만약 시스템 페이지 크기가 4kb이고 파일 시스템의 블록크기가 1kb이라면, 하나의 페이지는 4개의 블록을 포함하게 된다.

이때 커널은 다음과 같이 데이터를 배치한다.

1. `struct page`: 물리 메모리 4kb를 관리, 이 구조체의 private 필드 첫번째가 buffer_head를 가리킨다.
2. `struct_buffer_head` 리스트: 각 buffer_head는 1kb 영역에 대한 정보를 담는다. 
   1. b_this_page 필드를 통해 4개의 buffer_head가 **Circular Linked List(순환 연결 리스트)** 형태로 묶인다
   2. b_data는 해당 페이지 내의 특정 오프셋(0, 1024, 2048, 3072)을 가리킨다.

`b_state`: 이 버퍼가 현재 어떤 상태인지 나타내는 비트 마스크다.
- `BH_Dirty`: 이 데이터가 수정되어 디스크에 기록되어야한다.
- `BH_Uptodate`: 디스크로부터 데이터를 성공적으로 읽어와 메모리와 일치함.
- `BH_Mapped`: 디스크상의 실제 블록 번호(b_blocknr)가 할당된다.

`b_this_page`: 매우 중요한 필드로 하나의 페이지 안에 여러 블록이 있을대, 이들을 체인처럼 엮어서 커널이 페이지 하나만 찾아도 그 안의 모든 블록 상태를 한 번에 파악할 수 있다.

`b_blocknr`: 파일시스템 추상화 계층에서 관리하는 번호가 아니라 해당 디스크 장치에서 **섹터 단위의 위치**를 나타낸다.

운영체제는 파일 단위를 선호하지만, 하드웨어 하위 계층에서는 block이라는 더 작은 단위를 사용하기 때문에 이런 관리가 필요하다.

페이지 캐시 입장에서는 파일 A의 0~4KB 데이터가 메모리에 있나를 확인한다.

버퍼 캐시 buffer_head 입장에서는 디스크 /dev/sda1의 800번 블록이 메모리에 있나?를 확인한다.

통합 페이지 캐시 구조 덕분에, 우리는 파일의 내용을 읽으면서 동시에 그 파일이 저장된 디스크에 물리적인 블록 상태까지 이 buffer_head를 통해 효율적으로 관리할 수 있게 된것이다.

### 참고

최근 리눅스 커널 5.x 이상에서는 `buffer_head`가 가진 오버헤드를 줄이기 위해 iomap 구조와 Folios라는 개념을 도입해 더 대용량의 메모리를 효율적으로 처리하려는 움직임이 있었다.

하지만 여전히 EXT4와 같은 대중적인 파일 시스템의 핵심 메커니즘에서는 buffer_head가 중추적인 역할을 담당한다.