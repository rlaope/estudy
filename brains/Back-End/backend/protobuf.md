# Protocol Buffers


protocol buffers는 구조화된 데이터를 직렬화하기 위한 언어 중립, 플랫폼 중립적이며 확장 가능한 매커니즘입니다.

구글에서 개발, 운영 중이며 JSON과 같은 데이터들 보다 더 소형화되어있고 빠릅니다.

protobuf는 `.proto`라는 파일을 통해서 생성할 수 있으며 다음과 같이 생성됩니다.

```proto
message Person {
  optional string name = 1;
  optional int32 id = 2;
  optional string email = 3;
}
```


그리고 다음과 같이 사용해 볼 수 있습니다.

```java
Person john = Person.newBuilder()
    .setId(1234)
    .setName("John Doe")
    .setEmail("jdoe@example.com")
    .build();
output = new FileOutputStream(args[0]);
john.writeTo(output);
```

protobuf를 사용하면 다음과 같은 이점을 얻어볼 수 있습니다.

- 소형화 된 데이터 저장공간
- 빠른 파싱 속도
- 여러 프로그래밍 언어에서 사용이 가능함
- 자동 생성된 클래스를 통한 기능 최적화

이렇게 소형화되고 빠른 protobuf는 gRPC, Google Cloud, Envoy Proxy 환경에서 사용됩니다.

<br>

그러나 이러한 protobuf는 항상 좋을까요? 좋지 않은 상황도 알아보겠습니다.

**용량이 큰 메세지에 대한 기능**에서 protobuf는 안좋은 선택이 될 수 있습니다. 그 이유는 protobuf는 모든 메세지를 메모리로부터 가져와 사용하려는 경향이 있습니다. 그렇기 때문에 메가바이트 이상의 데이터라면 다른 솔루션을 찾는 것을 추천합니다.

 **직렬화된 데이터에서의 동등성의 보장이 어렵습니다.** protobuf로 작성된 파일이 직렬화 되었을 때 binary serializations 들로는 두개의 messages들을 파싱하지 않으면 비교할 수 없습니다.

message는 압축되지 않습니다. 메세지는 다른 파일처럼 압축되거나 gzip을 사용해 압축할 수 있지만 jpeg, png에서 사용되는 것과 같은 특수 목적 압축 알고리즘은 적절한 유형의 데이터에 대해서 훨씬 더 작은 파일을 생성합니다.

대규모 작업이 포함된 많은 기능들에 대해 크기와 속도 모두에서 최대 효율성을 발휘하지 못합니다. 
즉 대용량 배치 처리 작업과 같은 곳에서는 유용하지 않다는 것입니다.

그리고 protobuf는 객체지향도 지원하지 않습니다.

protobuf의 버퍼 메세지는 본질적으로 자체적으로 기술되어있지 않지만, 자체 내부 설명을 구현하기 위해서 반사적인 스키마를 갖고 있으며 해당 proto 파일에 접근하지 않고는 완전히 해석이 불가능하빈다. 이는 메세지가 자체적으로 설명되도록 하는 protobuf의 특징입니다.

<br>

### Protobuf 사용

protobuf를 통해 파일이나 스트림으로부터 생성된 코드는 다양한 유형의 유틸리티 메서드를 제공합니다. 

개별 값들을 추출하거나 데이터가 존재하는지 확인하거나 다시 파일, 스트림 유형의 데이터로 직렬화하거나 등등

다음과 같이 proto 파일을 통해서 정의하고

```proto
message Person {
  optional string name = 1;
  optional int32 id = 2;
  optional string email = 3;
}
```

proto 파일을 java code에서 빌더를 활용해서 만들어보고

```java
Person john = Person.newBuilder()
    .setId(1234)
    .setName("John Doe")
    .setEmail("jdoe@example.com")
    .build();
output = new FileOutputStream(args[0]);
john.writeTo(output);
```

C++에서 다음과 같이 활용도 할 수 있습니다. 즉 여러 언어에서 호환되는 것이죠!

```cpp
Person john;
fstream input(argv[1], ios::in | ios::binary);
john.ParseFromIstream(&input);
int id = john.id();
std::string name = john.name();
std::string email = john.email();
```

참고 자료는 [protocol buffers 공식 문서](https://protobuf.dev/overview/#syntax) 입니다.