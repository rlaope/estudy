# JVM Heap Object Headers on Internals

객체의 메모리 레이아웃을 살펴보면 인스턴스 데이터 이외에 객체 데이터가 존재한다

HotSpot JVM 객체 메모리 구조에서 보이듯이, 모든 자바 객체 인스턴스 데이터(payload) 이외에도 객체의 헤더나 정렬패딩같은 메모리 공간이 존재한다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdna%2FcaT8fy%2FbtsQ1XlvM5O%2FAAAAAAAAAAAAAAAAAAAAABJkufEpLzAlOU9P_3DliY3pMc0dYRg9qMWNVvYJBSub%2Fimg.png%3Fcredential%3DyqXZFxpELC7KVnFOS48ylbz2pIh7yKj8%26expires%3D1761922799%26allow_ip%3D%26allow_referer%3D%26signature%3D6l%252BJukuTTSTlgAODct%252F6iwbPgAU%253D)

위와 같은 정보들이 모여서 하나의 객체가 되는건데, 여기서 객체 헤더를 알아보자. 

<br>

## 마크 워드

마크 워드는 포인터가 아니라 그 자체로 데이터를 담고있는 비트 필드다, 각 객체별로 고유하게 관리되는 메타데이터가 저장된다.

대표적으로 해시 코드는 객체의 메모리 주소를 가리켜 각 객체별로 고유하게 할당되며 마크 워드에 할당된다.

그 외에도 generational gc 를 위한 나이 및 객체가 gc 대상으로 수집되었는지 여부에 대한 마킹 비트들도 존재한다.
- Lock 정보
- gc age-bit
- hashcode
- unused: 미사용구간

마크 워드에는 위의 정보들이 순차적인 비트단위로 인코딩되어 저장되며 hashcode는 객체 생성 초기에는 0으로 존재하다 hashCode가 처음 호출될때 할당된다.

<br>

## 클래스 워드

클래스워드는 포인터로 해당 객체의 클래스 메타데이터가 존재하는 메모리 주소를 가리킨다.

클래스에 종속적이며 모든 객체마다 동일한 메타데이터는 하나로 관리되기 때문에, 이를 **클래스 공유 메타데이터**라고도 부른다.

대포적으로 클래스 정보 kclass가 클래스 워드를 통해 얻어와지는 정보중 하나로 해당 객체가 어떤 클래스의 인스턴스인지 표현한다

- 클래스 정보: 해당 객체가 어떤 클래스인지 나타냄 kclass
- VTable: 가상 메서드의 테이블 주소 (다형성을 지원하기 위한 공간)
- 필드 정보: 필드 오프셋, 타입 정보
- 정적 변수: static field

자바 개발자들을 위해 클래스 정보를 표현하는 class 타입을 제공하여, 클래스에 대한 원하는 정보를 획득하고 객체를 생성할 수 있는 등의 기능을 제공한다.

하지만 jvm 내부적으로 관리하는 klasss는 개발자들에게 친숙한 class와는 다른 개념이다.

jvm 내부적으로 **빌드를하고 클래스를 로딩할 때, 클래스에 대한 메타데이터**가 필요한데, 클래스 파일에서 뽑ㅂ아내는 클래스 정보가 바로 kclass인것이다.

jvm은 얻어낸 kclass정보를 바탕으로 자바를 위한 복제본 미러를 생성하는데 이것이 Class 객체로 자바에 반환되는 것이다.

kclass word는 모든 클래스가 공유하는 메타데이터 정보기 때문에, jvm metaspace 영역에 저장 및 관리된다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdna%2FbnKgRq%2FbtsQ4QdMYHC%2FAAAAAAAAAAAAAAAAAAAAAM3k0VGsHl7cYAkuBC12bBzNYmeofrFN0s7MbbonsOxI%2Fimg.png%3Fcredential%3DyqXZFxpELC7KVnFOS48ylbz2pIh7yKj8%26expires%3D1761922799%26allow_ip%3D%26allow_referer%3D%26signature%3DyApkjuUajdOnfsSMk6XnqZvpnQs%253D)

이러한 객체의 헤더 부분이 메모리 상에서 어떻게 구성되는지 쉽게 표현하면 다음과 같다.

마크워드는 메타데이터로 힙 내부에 존재하지만, 클래스 워드는 클래스 정보를 가리키는 포인터라 최종적으로는 메타스페이스 영역을 향한다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdna%2FcGUnd2%2FbtsQ2SqpEXc%2FAAAAAAAAAAAAAAAAAAAAAJHjNJPE_FEK1KML4-4wKS-QX2eDQiXuZWFZF22cGn2C%2Fimg.png%3Fcredential%3DyqXZFxpELC7KVnFOS48ylbz2pIh7yKj8%26expires%3D1761922799%26allow_ip%3D%26allow_referer%3D%26signature%3D6T%252FIHHzbkjMwmRjaYiuyouP6Xds%253D)

<br>

## 배열의 길이

자바 배열의 경우 배열 길이도 헤더에 저장한다. 헤더에 저장되는 객체 타입은 배열에 담긴 원소 타입에 해당한다.

따라서 배열 길이까지 알아야 배열 객체가 차지하는 메모리 크기를 제대로 계산할 수 있다.

배열의 길이 정보는 해당 객체가 배열이 아니라면 존재하지 않는다. 따라서 객체 헤더의 길이는 배열 유무에 따라 달라질 수 있다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdna%2FzpqcN%2FbtsQ4XjDBRt%2FAAAAAAAAAAAAAAAAAAAAAE2504q3Zrdm264RhXJqA7T50yHn_zrfR3XubwSMUxFv%2Fimg.png%3Fcredential%3DyqXZFxpELC7KVnFOS48ylbz2pIh7yKj8%26expires%3D1761922799%26allow_ip%3D%26allow_referer%3D%26signature%3DaCeaNjC%252BMUQuoIzOg3yg4nLLpYM%253D)

<br>

## 인스턴스 데이터, 정렬 패딩

객체 레이아웃의 두 번째 부분인 인스턴스 데이터는 객체가 실제로 담고있는 정보다.

예컨대 프로그램 코드에서 정의한 다양한 타입의 필드 관련 내요으 부모 클래스 유무, 부모 클래스에서 정의한 모든 필드가 이 부분에 기록된다.

`+XX:ComatFields` 매개 변수를 true로 두면(default) 하위 클래스의 필드 중 길이가 짧은 것들은 상위 클래스 변수 사이에 끼워 넣어 공간이 조금이나마 절약된다고 한다.

정렬 패딩 부분은 존재하지 않을 수 있으며 특별한 의미 없이 자리를 확보하는 역할만 한다.

핫스팟에서 자동 메모리 관리 시스템에서 객체의 시작 주소는 반드시 8바이트 정수여야 하는데, 

달리 말하면 모든 객체의 크기가 8바이트 정수배여야 한다는 뜻이도 따라서 인스턴스 데이터가 조건을 충족하지 못하는경우 **패딩으로 채운다.**