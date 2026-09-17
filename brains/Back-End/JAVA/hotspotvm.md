# Hotspot VM, JIT Compiler 그리고 한계


### C 언어 동작 방식

C, C++과 같은 컴파일 언어는 컴파일 과정에서 바로 기계어로 번역하고 실행파일을 만든다.

그리고 컴파일시에 코드 최적화까지 진행해서 처리성능이 상당히 뛰어나다.

그러나 대신 생성된 기계어가 빌드 환경(CPU 아키텍처)에 종속적인 단점이 있다.

플랫폼이 바뀐다면 다시 빌드를 해야한다는..

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbHFBej%2FbtsmAL1jfGz%2FwgTWSn9U10bsdsA0IeXXdk%2Fimg.png)



### Java 동작 방식

자바는 이러한 플랫폼 종속 문제를 해결하기위해서 JVM을 도입하였다.

동작과정이 C언어같은 언어랑 다른데, 먼저 자바 코드는 바이트 코드로 컴파일이 된다.

바이트코드는 자바 코드보다 간단하다. 그러나 컴퓨터가 읽을 수 있지 않다 0과 1이 아니라서

그래서 이 바이트 코드를 JVM을 위한 중간 언어라고도 볼 수 있으며 JVM은 애플리케이션이 실행되면서 읽어드린 바이트코드를 실시간으로 기계어로 번역하고 CPU가 번역된 기계어를 처리한다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FdW1j9K%2Fbtsmz3HTV3H%2FUFiL9zon70psPwHMAoyNGk%2Fimg.png)

여기서 java는 플랫폼에 종속적이지 않다. 그러나 JVM은 각 플랫폼마다 기계어 번역이 다르기때문에 플랫폼에 종속적이라고 볼 수 있다.

하지만 이런식으로 바이트코드 -> 기계어 순으로 번역하는 것은 비교적 속도가 느리다.

그래서 이러한 문제를 해결하고자 바이트 코드를 기계어로 컴파일하는 JIT 컴파일러를 도입해 사용하고 있다.

JIT 컴파일의 목표는 빠른 컴파일, 특정 환경에 맞춤화된 최적화다. 이를 위해 실행 프로파일 정보를 확인한다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FlNWQk%2FbtsmzIYn2KP%2Flu5xb2awy3ZwqjeW3FDHBK%2Fimg.png)


### JIT Compiler

Java 1.3부터 **Hotspot VM**이 추가되었고, Hotspot VM에는 2개의 JIT 컴파일러가 포함되어있다.

> **JIT?**  
> JIT은 Just In Time의 약자로 .net환경이나 자바 환경에서 도입된 개념이다.
> 매번 컴파일을 통해 java -> bytecode -> 인터프리터로 변환하는 방식에서 속도면에서 매우 비효율적이였다. 그 문제를 해결하기 위해서 JIT가 생겼는데 JIT 컴파일러는 먼저 소스 코드 전체를 확인한 후 중복된 부분을 미리 기계어로 번역시켜 저장해놓는다. 그 이후 인터프리터 방식으로 번역하다가 중복된 부분을 만나게 되면 이미 변환된 기계어를 재사용한다. (초반에 메모리를 잡아두거나 하는 선행 작업들이 있어 초기 작업에는 실행속도가 다소 느릴 수 있음)


![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbRIWSX%2FbtsmBZEKoz7%2FJGTkxTQH2awKzvMKfxn4B0%2Fimg.png)

C1은 클라이언트 컴파일러로 코드 최적화는 덜하지만 시작되는 속도가 빠르다. 즉시 실행되는 데스크톱 애플리케이션에 적합하다.

C2는 서버 컴파일러로 즉시 시작되는 속도는 느리지만 코드 최적화가 많이 되어 warm-up 후에는 빠르다. 장기 실행되는 서버 애플리케이션 등에 적합하다.

> JVM Warm-up은 프로그램이나 애플리케이션을 실행하기 전 미리 초기화되고 최적화 하는 과정입니다.


Hotspot VM은 초기에 C1 컴파일러를 사용합니다. 여기서 Hostpot VM은 각 메서드 호출여부를 게속 주시하고, 메서드 호출 수가 많아지면 해당 메서드를 재컴파일 하는데 여기서는 C2 컴파일러를 사용합니다. **이를 계층형 컴파일이라고도 합니다. (Tiered Compilation)**

C2는 최적화를 극도로 진행하기때문에 컴파일언어를 능가하는 성능을 보여주기도 합니다. 최적화에는 프로파일링 정보(디바이스 수, 클럭 수)들을 사용하며, 자바의 성능 향상은 이러한 프로파일링을 통해 얻어낸 정보를 바탕으로 하는 JIT 컴파일러에 최적화 역할이 큽니다.

반대로 초기에는 warm up 하는 과정에서 실행, 속도가 느려지는 문제가 발생하기도 합니다.

### JIT의 한계

C2 컴파일러도 한계에 직면했는데 그 문제는 바로 C++로 구현되어있기때문에 개발자 구하기도 어렵고 오래된만큼 굉장히 복잡하다. 이러한 이유로 최근 몇 년 동안 개발된 중요한 최적화가 없었을 뿐만 아니라 전문가들 역시 유지보수를 어려워한다. 이제 End of life...

그래서 최적화가 더 이상 어려운 코드들은 HotspotInstrinsicCandidate 어노테이션을 붙이도록 하는 작업들도 진행되었다.

즉, JIT 컴파일러는 느리지 않고, 오히려 뛰어난 최적화 성능을 가지고있다. 그러나 유지보수하기가 어렵다는 문제가 가장 큰 것이다.

이러한 문제로 [[GraalVM]] 이 등장하게 되었다.