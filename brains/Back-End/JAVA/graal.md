# GraalVM


GraalVM에서는 HotspotVM의 C2 컴파일러를 대체할 수 있는 새로운 컴파일러를 만드는 것을 목표로 시작되었으며 2012년에 Open JDK 서브 프로젝트로 등록된지 7년 후인 2019년 5월에 1.0이 공개되었다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2F7dqDh%2FbtsmBZEKuE4%2FDnXBBIpyKl8RuT3OACCiZk%2Fimg.png)

GraalVM은 3가지 특징을 가지고 있으며, 아키텍처를 통해 그 이유를 알 수 있다.

- 고성능 자바
- 다양한 언어의 통합
- Native 지원을 통한 빠른 start-up(Native Image)

GraalVM은 자바로 작성된 Advanced JIT 컴파일러인 GraalVM를 Hotspot JVM에 추가하였다. 또한 Language Implemntation 프레임워크인 Truffle도 추가해 JVM위에서 파이썬, 자바스크립트와 같이 여러 언어를 실행할 수 있게 되었다. 이를 통해 같은 메모리 공간에서도 데이터를 주고받을 수 있다.

- 핫스팟 JVM
- Graal 컴파일러
- Truffle
- GraalVM 업데이터

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FF6dCN%2Fbtsmz1cn4R0%2FaKKFKyUIAwkduRyjO8Dow1%2Fimg.png)

### JVM Runtime Mode

Hotspot JVM에서 프로그램을 실행할 때, GraalVM은 기본적으로 최상휘 JIT 컴파일러로 GraalVM 컴파일러를 사용한다.

런타임 애플리케이션은 JVM에 로드되고 실행된다. JVM은 자바나 다른 네이티브 언어의 바이트코드를 컴파일러에게 전달하고 컴파일러는 이를 머신코드로 컴파일해 JVM에게 돌려준다.

지원되는 언어에대한 인터프리터는 프러플 프레임워크 위해 작성되며, 그 자체로 JVM에서 실행되는 자바 프로그램이다.

새롭게 만들어진 Graal JIT 컴파일러는 기존의 C2 컴파일러보다 성능이 더 뛰어나다.


### Native Image

네이티브 이미지는 자바 코드를 독립적으로 네이티브 실행 파일 또는 네이티브 공유 라이브러리로 컴파일하는 혁신적인 기술이다.

네이티브 실행 파일을 빌드하는 동안 처리되는 자바 바이트코드에는 모든 애플리케이션 클래스, 의존성, 외부 라이브러리 및 필요한 모든 jdk 클래스들이 포함된다.

생성된 self-contained 네이티브 실행파일은 각 개별 운영 체제 머신의 아키텍처에 따라 다르며 JVM이 필요하지 않고 이 이미지는 AoT 컴파일러에 통해 처리된다.

> AoT 컴파일러는 다음 글에서 언급하겠다.


### Java on Truffle

트러플 자바는 Truffle 언어 구현 프레임워크로 구축된 JVM 스펙의 구현체이다.

모든 핵심 구성 요소를 포함하고, JRE 라이브러리와 동일한 API를 구현한다.

GraalVM의 모든 JAR와 네이티브 라이브러리를 재사용하는 완전한 Java VM이다.

트러플 자바는 Polyglot API가 제공하는데, 이를 통해 런타임 시에 서로 다른 프로그래밍 언어를 결합시킬 수 있다.

**즉 JVM위에서 서로 다른 언어로 작성된 프로그램을 실행시킬 수 있다!**




> **OpenJDK와 GraalVM**
  가장 먼저 Open JDK9에 [JEP 243](https://openjdk.org/jeps/243)에 따라 JVMCI(자바 컴파일러를 위한 인터페이스)가 추가되었다. 그리고 [JEP 295](https://openjdk.org/jeps/295)에 따라 Open JDK 9에는 GraalVM의 AoT 컴파일러가 추가되었고, 이후에 Open JDK 10에서는 [JEP 317](https://openjdk.org/jeps/317)에 따라 Graal JIT 컴파일러가 추가되었다. 하지만 Open JDK 17부터는 Graal JIT 컴파일러와 AoT가 빠지게 되었고, JVMCI만이 그대로 남았다. 이는 Graal이 더 이상 불필요해졌기 때문이 아니다. **대부분의 GraalVM 사용자들이 GraalVM을 직접 설치해서 사용하지, OpenJDK에 내장된 기능을 활용하지 않기 때문이다.** GraalVM은 앞으로도 계속해서 발전할 예정이다.




