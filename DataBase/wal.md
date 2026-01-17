# MySQL WAL

대규모 시스템에서 db 부하를 줄이기위해 cdc등을 사용하는것을 검토할때 근간이되는 mysql wal 즉 innodb redo log의 내부 구조를 깊게 이해하는것이 중요하다 생각함 알아보겟다

mysql에서 데이터 변경은 메모리 buffer pool에 먼저 일어나고 이 변경 이력이 디스크 로그 파일에 순차적으로 기록된다 이것이 WAL이다 Write Ahead Logging

### Redo Log

Redo Log는 크게 메모리 영역과 디스크 영역으로 나뉘는데

**redo log buffer(memory)** 라고 사용자가 update, insert를 수행하면 innoDB는 데이터 페이지를 메모리 buffer pool로 올리고 변경한다.

이와 동시에 변경사항을 redo log buffer라는 메모리 공간에 기록한다.

무작위 io가 아닌 순차적인 기록이기 때문에 매우 빠르다.

**redo log files(disk)** 메모리 버포 내용이 실제 디스크 파일 (`ib)logfile0`, `ib_logfile1`등)으로 기록되는 단계다.

기본적으로 circular 구조를 가지고 즉 파일의 끝에 도달하면 다시 처음으로 돌아와서 덮어쓰는거임

덮어쓰이니까 문제가 아닌가 싶을수있는데 redo log는 데이터를 영구히 보관하는 곳이 아니라 메모리에는 반영되었지만 아직 데이터파일에는 기록되지 않은 변경사항을 잠시 보관하는 작업 대기 목록이라서 그렇다.

<br>

### LSN (Log Sequence Number)

LSN은 8바이트의 단조 증가하는 정수로 redo log에 기록된 데이터의 양을 바이트 단위로 나타낸다.

모든 일관성 체크의 기준점이 된다.

- **Log Start LS**N: 로그 파일에서 쓰기가 시작되는 지점
- **Flush LSN**: 디스크의 redo log 파일까지 물리적으로 기록된 지점
- **Checkpoint LSN**: 실제 데이터 파일 idb에 변경사항이 반영된 마지막 지점.

장애 복구시 checkpoint LSN 부터 Flush LSN 사이의 구간만 다시읽어 replay 데이터 파일을 복구한다.

<br>

### Redo Log 기록 메커니즘 innodb_flush_log_at_trx_commit

트랜잭션 커밋시 redo log가 디스크로 넘어가는 방식은 설정에 따라 달라지며, 이는 성능 데이터와 안정성 사이의 트레이드 오프를 결정한다.

- 0: 1초에 한번씩 버퍼를 로그 파일에 쓰고 flush, 성능은 최고지만 mysql crash시 최대 1초 데이터 유실
- 1: 커밋마다 로그 파일에 쓰고 물리적 디스크로 flush(sync) 하는데 acid준수를 하지만 성능이 낮다.
- 2: 커밋마다 로그파일 os cahce에는 쓰지만 물리적 flush는 1초에 한번씩 수행한다 os가 살아있다면 mysql 이 죽어도 안전. os가 죽으면 1초 데이터 유실

<br>

### Checkingpoint

redo log 파일은 크기가 제한되어 있으므로 꽉 차기 전에 메모리의 dirty page를 실제 데이터 파일에 반영하고 로그 공간을 비워야한다 이를 checkingpointing이라고 한다.

- sharp checkpoint: db종료시 모든 dirty page를 반영한다.
- fuzzy checkpoint: 성능 저하를 막기 위해 나누어서 반영. innoDB는 주로 이방식을 사용함.
- async/sync flush checkpoint: redo log 공간이 부족할때 강제로 발생한다.

checkpoint를 통해서 지점을 넘어서는 데이터는 덮어쓰지 않는다. 그래서 circular 구조에서 문제가없고 만약 로그 파일에서 데이터가 꽉찼는데 head가 tail을 따라잡으려하는 그런 순간엔 아직 디스크로 옮겨지지 않은 데이터가 있다면 mysql은 모든 작업을 일시중단하고 메모리의 dirtyupage를 강제로 디스크에 flush한다.

그 이후 checkpoint를 밀어서 빈 공간을 확보한후 다시 로그 쓰기를 시작한다.

<br>

### Doublewrite Buffer: 데이터 오염 방지

WAL이 있어도 해결 못 하는 문제가 Partial Page Write이다. os의 페이지 단위 4kb와 mysql의 페이지단위 16kb가 달라 쓰기 도중 전원이나가면페이지가 깨질수있다.

1. 데이터를 디스크에 쓰기 전, 먼저 doublewrite buffer 시스템 테이블스페이스 공간에 통째로 쓴다.
2. 그 다음 실제 데이터 파일을 쓴다..
3. 쓰다가 페이지가 깨지면, redo log로 복구하기 전 doublewrite buffer에서 원본 페이지를 가져와 복구한다.

<br>

### CDC와의 연결고리

redo log는 innoDB 내부의 복구용 로그고 mysql은 이와 별개로 replication을 위한 Binlog를 남긴다.

debezium같은 도구는 binary log를 읽어서 kafka로 전송하고 트랜잭션 완료시 redo log, binlog는 two phase commit으로 묶여 일관성을 유지한다.

WAL 기반 아키텍처의 장점은 대규모 시스템에서 outbox 테이블을 따로 관리하며 발생하는 추가 쓰기 부하를 제거하고 순서보장도 된다 db에 반영된 순서 그대로가 로그에 박혀 메시지 순서 보장에도 용이하다.