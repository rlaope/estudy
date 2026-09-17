# JVM GC 

## Garbage Collection
가비지 컬렉션은 **자바의 메모리 관리 방법** 중 하나로 **JVM Heap 영역**에서 동적으로 할당했던 메모리 영역 중 필요 없게 된 메모리 영역을 주기적으로 삭제하는 프로세스를 말합니다. C, C++은 이러한 가비지 컬렉션이 없어 프로그래머가 수동으로 메모리 할당과 해제를 일일이 해줘야하는 반면에 Java는 JVM에 탑재되어있는 가비지 컬렉터가 메모리 관리를 대행해주기 때문에 개발자 입장에서 메모리 관리, 메모루 누수 문제에 대해 완벽하게 관리하지 않아도 되어 오로지 개발에만 집중할 수 있다는 장점이 있습니다.

### 단점
1. 개발자가 메모리가 언제 해체되는지 정확히 알 수 없다.
2. 가비지 컬렉션이 동작하는 동안에는 다른 동작을 멈추기 때문에 오버헤드가 발생한다.



<br>

## 가비지 컬렉션의 대상이 되는 객체들

![](https://blog.kakaocdn.net/dn/bW5c5r/btrvAb4nrdH/lYHuQZya8ECvEndRkQchjk/img.png)

객체들은 실질적으로 Heap 영역에서 생성되고 Method Area나 Stack Area 등 Root Area에서는 Heap Area에 생성된 객체의 주소만 참조하는 형식으로 구성됩니다. 하지만 이렇게 생성된 Heap Area 객체들이 메서드가 끝나는 등의 특정 이벤트들로 인하여 Heap Area 객체의 메모리 주소를 가지고 있는 참조 변수가 삭제되는 현상이 발생하면 위의 그림에서의 빨간색 객체와 같이 **Heap 영역에서 어디서든 참조하고 있지 않은 객체**들을 발생하게 됩니다. 이러한 객체들을 Unreachable하다고 하며 주기적으로 가비지 컬렉터가 제거해줍니다.  
  
> Reachable: 객체가 참조되고 있는 상태
> Unreachable: 객체가 참조되고 있지 않은 상태

### Mark And Sweep 알고리즘

![](https://blog.kakaocdn.net/dn/bGghBW/btrvvDgIHRO/HxoX3w9skgah3xFVhfEgD0/img.png)

Mark And Sweep 알고리즘은 가비지 컬렉션이 동작하는 원리로 루트에서부터 해당 객체에 접근 가능한지에 대한 여부를 메모리 해제의 기준으로 삼습니다. Mark And Sweep은 위의 그림과 같이 총 3가지 과정으로 나뉩니다.  

- Mark 
  - 먼저 루트로부터 그래프 순회를 통해 연결된 객체들을 찾아내 각각 어떤 객체를 참조하고 있는지 찾아서 마킹합니다.
- Sweep
  - 참조하고 있지 않은 객체 즉 Unreachable 객체들을 Heap에서 제거합니다.
- Compact
  - Sweep 후에 분산된 객체들을 Heap의 시작 주소로 모아 메모리가 할당된 부분과 그렇지 않은 부분으로 압축합니다. (가비지 컬렉터 종류에 따라 하지 않는 경우도 있다.)

<br>

## GC의 대상이 되는 Heap Area


![](https://blog.kakaocdn.net/dn/bti1oP/btrvtcdoBC9/upBBOdB4mJF6tfyhL8GPbK/img.png)

Heap Area는 효율적인 GC를 위해 위와 같이 Eden, Survival, Old Generation으로 나뉜다.

<br>

## GC 동작 과정

### 첫 번째 과정
![](https://blog.kakaocdn.net/dn/7pVmj/btrvu28jcRt/Iy5eB9flQ8L4eIkc0a1FX1/img.png)
객체가 처음 생성되고 Heap 영역의 Eden에 age-bit 0으로 할당된다. 이는 age-bit는 Minor GC에서 살아남을 때 마다 1씩 증가하게 된다.

### 두 번째 과정
![](https://blog.kakaocdn.net/dn/cTWRqo/btrvxlfT2KU/gIDFZpUapbTZTKR1Gi16M0/img.png)
시간이 지나 Heap Area의 Eden 영역에 객체가 다 쌓이면 Minor GC가 한번 일어나게 되고 참조 정도에 따라 Survival 영역으로 이동하거나 회수됩니다.

### 세 번째 과정
![](https://blog.kakaocdn.net/dn/b42htO/btrvuPvhcQ2/HwXDNmku8NbhSkaGoJEywK/img.png)
계속해서 Eden 영역에는 신규 객체들이 생성됩니다. 이렇게 또 Eden 영역에 객체가 다 쌓이게 되면 Young Generation(Eden + Survival) 영역에 있는 객체들은 비어있는 Survival인 Survival1 영역에 이동하고 살아남은 모든 객체들은 age가 1씩 증가합니다.

### 네 번째 과정
![](https://blog.kakaocdn.net/dn/dpYphN/btrvocRXzk2/PANFhltyaGtzuDak9nqd61/img.png)
또다시 Eden 영역에 신규 객체들로 가득 차게 되면 다시한번 minor GC가 일어나고 Young Generation 영역에 있는 객체들을 비어있는 Survival인 Survival0으로 이동시킨 뒤 age를 1 증가시킵니다. 이 과정을 계속 반복합니다.

### 다섯 번째 과정
![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FGvbFe%2FbtrvqaMQzF5%2F2pYF0QKjwBZWF7EtYNJ8OK%2Fimg.png)
이 과정을 반복하다 보면 age bit가 특정 숫자 이상으로 되는 경우가 발생하고 이때 JVM에서 설정해 놓은 age bit에 도달하게 되면 오랫동안 쓰일 객체라고 판단해 Old generation 영역으로 이동시킵니다. 이 과정을 프로모션이라고 합니다.

### 마지막 과정
![](https://blog.kakaocdn.net/dn/b015X4/btrvtcRX3Go/DG6GyfMsZv0xgJRujfOeRK/img.png)
시간이 지나 Old 영역에 할당된 메모리가 허용치를 넘게되면 Old 영역에 있는 모든 객체들을 검사해 참조되지 않는 객체들을 한꺼번에 삭제하는 GC가 실행이됩니다. 이렇게 Old generation 영역의 메모리를 회수하는 GC를 Major GC라고 합니다. Major GC는 시간이 오래 걸리는 작업이고 이때 GC를 실행하는 스레드를 제외한 모든 스레드는 작업을 멈추게 됩니다. 이를 Stop-the-World라 합니다. 이 작업이 너무 잦으면 프로그램 성능에 문제가 될 수 있습니다.