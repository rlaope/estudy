# Cache Eviction, Expiration, Passivation

cache는 속도를 위해 대부분 메모리를 사용한다.

메모리는 디스크에 비해 사용 공간이 훨씬 적다.

### Eviction
공간이 필요할 때 어떤 데이터를 지워주는 것이다.

메모리가 가득 차면 사용하지 않는 데이터를 지워저야 새로운 데이터가 들어올 수 있다.

대부분 LRU(Least Recently Used)라는 **가장 오랜 기간 참조되지 않는 데이터를 교채** 알고리즘 방식을 사용한다.

### Expiration

데이터의 유통기한이다.

일반적으로 TTL(TIme To Live)라는 단어를 사용한다.

### Passivation

기능을 사용하면 eviction의 대상이 되는 데이터가 **지워지기 전에 우선 디스크등 다른 스토리지에 저장**한다.

추후 같은 데이터에 대한 요청이 들어오면 파일에서 찾아 돌려준다.