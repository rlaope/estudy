# Page Cache, Buffer Cache

OS의 페이지 캐시와 버퍼 캐시는 디스크 IO 성능을 높이기 위해 메인 메모리 RAM의 일부를 저장소로 사용하는 핵심 메커니즘이다.

최신 리눅스 커널에서는 이 두개념이 하나로 통합되었지만, 기원과 논리적 구분은 공부할 가치가 있는듯하다.

구분을 먼저 해보면 페이지 캐시는 page 단위(보통 4kb), 버퍼 캐시는 블록 단위(512B~4KB)이다.

페이지 캐시는 파일의 내용 file data가 대상이고 고수준(파일 시스템 계층)에서 추상화 되어있으며, 목적으로는 파일 읽기/쓰기 속도 향상을 위해 사용된다.

버퍼 캐시는 디스크의 물리적 블록이 대상이고. 저수준(block device) 계층에 추상화 되어있으며 디스크 메타데이터 및 블록 정렬 관리 목적이다.

- **페이지 캐시**: 사용자가 `read()`, `wrtie()` 시스템 콜을 통해 파일에 접근할 때 사용된다. 파일의 100번째 바이트부터 읽어줘 라는 요청을 처리한다.
- **버퍼 캐시**: 파일 시스템의 메타데이터(inode, Superblock등)나 디스크 장치에 직접 접근할 때 사용된다. 디스크의 500번째 섹터를 읽어줘라는 요청을 처리한다.

### 통합 페이지 캐시 (Unified Buffer Cache)

과거 구조의 가장 큰 문제는 **이중 캐싱(Double Caching)**이었다. 파일 데이터를 읽으면 페이지 캐시에 저장되고, 그 아래 블록 계층에서 똑같은 데이터가 버퍼 캐시에 또 저장되어 메모리가 낭비됐기 때문이다.

현대 리눅스 커널 (linux 2.4이후) 이를 해결하기 위해 통합 페이지 캐시 구조를 채택했다.

- 구조: 버퍼 캐시는 이제 독립적인 메모리 영역이 아니라, **페이지 캐시 내부에 포함된 논리적인 개념**이다.
- 작동방식: 페이지 캐시의 한 페이지 4kb가 디스크 블록 1kb 4개를 가리키는 `buffer_head` 구조체를 가짐으로써 두 캐시의 역할들 동시에 수행한다.

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


### 커널 레벨에서 읽기/쓰기 흐름

**Read 흐름 (Page Cache Look-up)**
1. 프로세스가 read()를 호출한다.
2. 커널은 해당 파일의 해당 오프셋이 page cache에 있는지 확인한다 `address_space` 구조체 참조
3. Cache Hit: 메모리에서 즉시 데이터를 복사하여 사용자에게 전달한다. (디스크 접근 없음)
4. Cache Miss: 디스크에서 데이터를 읽어 페이지 캐시에 채운 뒤 전달한다. (이때 주변 블록까지 미리 읽는 Read-ahead를 수행한다.)

**Write 흐름 (Write-back 정책)**
1. 프로세스가 `write()`를 호출한다.
2. 데이터를 페이지 캐시에 쓰고, 해당 페이지를 Dirty Page로 표시한다.
3. 특정 조건(시간 경과, 메모리 부족, 명시적 sync)이 충족되면 커널의 Background Worker(flusher thread)가 디스크에 실제 기록을 수행한다.

### 관측

`free -m`을 통해서 buff/cache 항목을 보면 이 두 캐시가 사용하는 메모리 양을 확인할 수 있다.

`/proc/meminfo`: `Cached`, `Buffers`, `Dirty` 항목을 통해 상세한 수치도 확인 가능하다.

`vmstat 1`: `bi`, `bo` 각각 block in block out이고 이를 통해 캐시와 디스크 간의 데이터 이동을 실시간 모니터링 가능하다.

