# B+Tree

B-Tree와 다르게 **B+Tree는 모든 레코드가 leaf 노즈에 저장되고 중간 노드에는 인덱스가 저장되는 특징을 가지고 있다.** 이는 B Tree의 범위 검색에 대한 단점을 보완해주는 데, 이는 leaf 노드들이 모두 링크드 리스트로 연결되어있기 때문이다 

### 데이터 저장

한 번 데이터를 쓰는 데 B+Tree 구조로 인해 여러 디스크 write가 필요할 때가 있다. 이를 write amplication(쓰기 증폭) 이라고 한다. 이러한 특징으로 인해서 쓰기하는 도중에 DB Crash 가 발생할 경우 데이터가 손상될 가능성이 존재한다.

이러한 문제를 해결하기 위해 대부분 디스크 영역에 WAL Log(Redo Log라고도 불림)라는 곳에 어떤 write를 할 지 미리 영구적으로 써놓고 B+Tree 쓰기 작업을 처리한다. (LSM Tree의 Logfile 역할을 생각하면 쉬움) 이로써 DB 내구성을 보장한다.

루트노드에는 인덱스가 존재하고 1, 2, 3, 7 데이터가 리프 노드 3개가 존재하고 있을 때, 데이터 6을 추가로 삽입했는데 B+Tree 노드가 3개가 더 추가된 모습을 볼 수 있다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FRZeZ1%2FbtsvktSIOec%2FseUagR1ArCNVCnIUZx2BFK%2Fimg.png)


모든 리프 노드들이 서로 연결되어 있어서 B-Tree에 비해 쿼리 효율이 좋다. (BTree는 binary search로 계속해서 여러 노드들을 거치면서 범위 탐색을 이어감 그러나 B+Tree는 leaf node에서 scan만 하면 됨)

모든 레코드가 리프 레벨에 상주해 B-Tree보다 공간 활용도도 좋다.

그러나 B+Tree는 리프 수준에 추가된 구조로 인해 구현하기가 더욱 복잡하고 B-Tree와 마찬가지로 쓰기 증푹 발생으로 인한 오버헤드가 있다.

<br>

### LSM Tree vs B+Tree

일반적으로 B+Tree는 Read, LSM Tree는 Write 성능에 좋다.

**데이터 읽기**
- B+Tree는 트리를 따라서 내려가면 최대 O(logN)안에 조회한다.(tree의 최대 depth)
- LSM Tree는 memtable안에 조회할 데이터가 존재하지않으면 SSTable을 확인해야하고 또한 백그라운드 작업(comaction)이 영향을 줄 수도 있다.

**쓰기 증폭(Write Amplification)**
- LSM Tree는 memtable, logfile(+ merge)만 쓰면 된다. (sorting, 순차 쓰기 및 merge는 시간이 걸리지 않음)
- B+Tree는 여러번의 Disk Write가 필요할 수 있따 (+ WAL Log) (Page 분리)
- 따라서, LSM Tree가 상대적으로 B+Tree보다 쓰기 증폭이 낮다.

**Fragmentation**
- LSM Tree는 주기적으로 compaction 작업을 통해 데이터를 최신화해 압축률이 좋다. 그래서 B+Tree보다 디스크에 더 적은 파일을 생성한다.
- B+Tree는 파편화로 인해 사용하지 못하는 공간이 일부 존재한다.

**동시성 제어**
- B+Tree는 데이터가 한 곳에만 존재하여 관리하기가 쉽다
- LSM Tree는 각기 다른 세그먼트에 다중 복제본이 존재할 수 있다. 높은 쓰기 처리량에서 (logging + sstable로 flush) 과정에서 백그라운드 compaction 과정이 공유되어야 하는데, 여기서 유입 속도를 못따라가 중복이 발생할 수 있다.
- 그래서 **트랜잭션 기능을 중시하는 데이터는 B+Tree를 주로 사용한다.**




