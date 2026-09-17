# HCL 언어

테라폼의 코드는 HCL이라는 언어로 작성된다.

테라폼은 .tf 라는 확장자를 갖는 파일이며 tf확장자 파일은 HCL언어로 작성된다.

HCL은 프로그래밍 언어와 다르게 **구성(Configuration)을 표현**하는 것을 집중하는 언어다.

### 단위

HCL은 `block`이라는 기본 단위로 구성된다.

아래 그림은 테라폼 공식 문서의 예제이다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fc2RSCS%2Fbtslffvyceq%2FhDch5TZH2datPz24miUXS1%2Fimg.png)

위에서 노란 글시인 BLOCK LABEL은 옵션이며 0개 이상 있다.

BLOCK TYPE은 Block이 어떤 동작을 하는지 결정한다. 

```tf
resource "local_file" "demo" {
  content = "hi"
  filename = "hello.txt"
}
```
위의 예제에서 resource block이 생성하는 리소스 종류는 첫 번재 block label이 담당한다.

local_file 블럭 라벨은 테라폼을 실행하는 로컬 pc에 파일을 생성한다.

resource block은 여러 개 사용할 수 있으므로, 두 번째 label에 이름을 설정함으로써 각 리소스들을 구별할 수 있다. (그렇기에 두번째 라벨이 중복된다면 오류가 발생한다.)

> 정리하면 첫번째 블럭 라벨은 리소스를 생성하는 라벨이고 두번째 라벨은 구별 이름이다.

그리고 block body에는 block의 설정을 나타낸다 바디에 입력된 내용을 인자(argument)라고 부른다.

인자는 테라폼과 함께 변수, 조건문, 반복문과 함께 사용이 가능하다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FAVLPd%2Fbtslb5z2Bve%2FhF7rL1dMbeV4TSkwnTB5c0%2Fimg.png)


<br>

### 테라폼 인식 범위

테라폼 코드를 실행하는 경로를 루트 모듈이라고 한다.

루트 모듈에 있는 모든 tf 파일은 테라폼이 실행한다.

그러나, 하위 모듈에 있는 tf 파일들은 실행하지 않기 때문에 하위 경로에 있는 테라폼 파일을 실행시켜주려면 

```bash
terraform -chdir="./submodule" init
```

위와 같이 작성해줄 수 있다.

