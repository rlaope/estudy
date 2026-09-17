# Paging, Fragmentation, TLB

Memory - Address, Contiguous allocation, MMU 에서 배운 것을 복습해보자

contiguous allocation을 하면 일단 mmu가 굉장히 간단해진다는 장점이 있다.

더하기 + 상한선만 체크해주면 되기 때문이다. 하지만 이전 글에서 언급했듯, 현재 시스템에서 사용되지 않는다.

그 이유는 오늘 한 번 알아보도록 하겠다.

**contiguous allocation(연속적 할당)의 단점**

우리가 사용하는 컴퓨터 프로세스는 한번에 여러개가 실행되는게 일반적이다

그리고 그 프로세스들은 각각의 크기와 작업 수행 시간 등이 물론 다 다를 것이다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FL1WmC%2FbtrtqOLUSGk%2FUJG7R7aIcVmDAcZmfLNweK%2Fimg.png)

먼저 process1을 실행시켰다. 앞으로 실행되는 프로세스들은 연속적으로 배치될 것이다.

contiguous allocation이라는 게 logically contiguous 이면 physically contiguous기 때문이다.

즉 process1을 쪼개서 비어있는 메모리에 넣는다던지 쪼개서 순서를 바꾼다던지의 행위는 일어날 수 없다.

연속적으로 놔야하는 것이다.

위 사진은 process1 ~ 3이 비어있는 메모리에 연속적으로 배치된 모습이다. 맨 마지막에 process3이 들어오면서 메모리가 꽉 찼다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FdjvT3a%2FbtrtytlIncM%2FfL2qbcTlApl23DiBsrAGVK%2Fimg.png)

이제 process 3이 종료되고 process 3 메모리 공간이 비게 된다. 이 공간을 HOLE이라고 한다.

HOLE에 process4, 5가 들어왔다. 비어있는 메모리 공간에 딱 맞추어 실행되면 좋겠지만 현실은 그렇지 않다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fu0E1s%2Fbtrtwqje11U%2FhNKzvHM6MSK8FWHqJ50h81%2Fimg.png)

process5까지 들어가면서 약간의 메모리 공간이 비어있지만 process6이 실행하는데 필요한 메모리 공간은 존재하지 않는다. process1이 끝나면 공간이 생기겠지만, 나머지 process들은 끝나도 2,4,5가 한 번에 끝나지 않는 이상 공간이 부족할 것 같다. 이것이 fragmentation이 발생했다고 한다.

fragmentation은 internal fragmentation과 external fragmentation으로 나뉜다.

위에 예제에서는 바로 external fragmentation 외부 단편화다. fragmentation은 단편화라는 뜻 말고도 조각이라는 뜻이 있는데, 메모리가 남아있는데 사용하지 못하고 조각조각으로 나뉜다해서 붙여진 이름이다.

정리해보면, 메모리가 할당되고 해제되는 작업이 반복될 때 동안 중간중간 사용하지 않는 빈 메모리 조각들이 많이 생겨버려 **총 메모리 공간은 충분하지만 실제로 process 실행을 위해 할당해줄 수 있는 메모리 공간이 없는 문제를 external fragmentation이라고 한다.**

이러한 문제를 해결하기 위해 compaction이 존재한다.

<br>

### Compaction

외부 단편화 문제를 해결하기위한 방법은 간단하다. 실행되어야할 process가 필요한 메모리 공간만큼 사용중인 메모리 공간들을 정리해서 자리를 만들어주면 된다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FS7C1Y%2FbtrtyeoLjtc%2FaaNEfTTjgF0o90qNskydyk%2Fimg.png)

위에서는 process2, 5,를 옮기고 process6가 필요한 메모리 공간을 만들어 주었다.

하지만 이 방법은 process 2,4를 옮길 때 임시로 복사했다가 다시 가져오는 방식을 사용한다.

즉, 다른 저장소 하나가 더 필요하다는 것인데, 지금은 ssd지만 sdd가 없을 시절에는 hdd를 사용했을 텐데 속도가 굉장히 느릴 것이다.

정리하면 compaction 이란 비어있는 메모리 공간을 연속적인 공간으로 만드는 과정이라고 할 수 있다.

compaction은 external fragmentation을 줄일 수 있는 방법이 될 수 있기는 하지만 메모리를 복사하고 다시 가져오는 과정인 i/o problem이 생기게 된다. 이 과정에서 오버헤드가 발생하며 따라서 더욱 좋은 방법을 고안해냈다 이 방법 이바로 paging(페이징 기법)이다.

페이징이 나오기 이전에도 위의 방법도 효율적인 방법으로 사용하기 위해 많은 노력들을 했었다. 어떻게 하면 프로세스를 효율적으로 fit하게 넣을까라는 고민을 했고 first-fit best-ft worst-fit이라는 세 가지 개념들이 탄생하게 되었다.

- first-fit: 메모리에 할당해줄 hole을 찾기 위해 순차적으로 앞에서부터 탐색후 hole이 발견되면 최초에 hole에 메모리 할당
- best-fit: 여기저기 다 대입시켜보고 가장 크기가 잘 맞는 hole에 배치
- worst-fit: 남은 hole중 가장 큰 hole에 배치, 이렇게 하면 남은 hole의 크기가 클 테니 다른 프로세스가 또 사용할 수 있음

위에서 설명했던 방법들은 프로세스 크기별로 메모리를 할당한다. 이를 **가변 분할**이라고 한다.

모든 프로세스 크기는 동일하지 않기 때문이다. 이 말은, 어떻게 집어넣든 프로세스 크기가 다르기 때문에 external fragmentation 문제를 막을 수 없다는 것이다.

그래서 나온 솔루션이 paging이며 **paging은 메모리를 고정된 크기로 분할시켜 할당하는 것이다.**

이 방법을 고정 분할이라고 부른다. 위에서 언급했듯, external fragmentation은 가변 분할로 인해 발생하는데 , internal fragmentation은 고정 분할로 인해 발생하게 된다. 더 자세히 알아보겠다.

<br>

### Paging

1. pages: logical memory를 일정한 사이즈로 분할한 블록
2. frames: physical memory를 일정한 사이즈로 분할한 블록

먼저 프로세스의 논리적 주소 공간을 여러 개로 쪼갠 것이 page라고 하고, 물리적 주소 공간을 여러 개의 크기로 쪼갠 것을 frame이라고 한다. 페이지와 프레임은 모두 동일한 사이즈로 쪼개야 한다.

페이징 기법은 쪼개진 페이지와 프레임을 매핑시켜준다. 

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FHS9vo%2FbtrtwKhADHK%2FETbN4QumvE5JIcqc7vq5hk%2Fimg.png)

위 그림에서 왼쪽은 논리적 메모리 주소, 오른쪽은 물리적 메모리 주소다. 그림에서 볼 수 있듯 둘 다 같은 크기로 쪼개져 있다.

여기서 논리적 메모리 특정 주소 값은 페이지1의 첫 번째 offset으로 표현이 가능하고, 물리적 특정 주소값 또한 프레임 1의 첫번째 offset으로 표현이 가능하다.

> offset이란?  
> 객체 안에서 객체의 처음부터 주어진 인덱스라고 생각하면 된다. 메모리에서는 두 번째 주소를 만들기 위해 기준이 되는 첫번째 주소에 더해진 값이라고 한다. 예를 들어 객체 A에 문자열 abcdef가 있다고 할 때 c는 A 시작점에서 2의 오프셋을 지닌다고 볼 수 있다.


<br>

### Internal Fragmentation

자 이제 고정 분할의 문제점 internal fragmentation에 대해서 알아보겠다.

동일한 크기로 잘랐고, 연속적으로 배치하지 않아도 되기 때문에 external fragmentation은 이제 발생하지 않는다.

그렇다면 이제 메모리에 비어있는 hole 없이 할당받을 수 있을까? 

동일한 크기로 자를 때, page 크기에 맞게 딱 나눠 떨어지지 않을 가능성이 존재한다. 한 블록의 크기보다 약간 더 큰 메모리 크기를 가지고 있다면 남는 공간이 생길 것이다. 이러한 경우를 Internal Fragmentation 내부 단편화라고 한다.

**+ 페이지를 프레임에 어떻게 매핑하는 것일까?**

먼저 페이지 테이블에 대해서 알아보자, 페이지가 각각 매핑될 프레임에 대한 정보가 기록된 테이블을 **페이지 테이블**이라고 한다. 즉, 페이지 테이블은 페이지의 수 만큼 엔트리를 가지게 된다.

예를 들어 시스템이 1024kb(1024 * 1024 byte)의 물리적 주소 공간을 갖고, 각 페이지는 1024byte의 공간을 가지고 있다고 가정햇을 때, 페이지와 프레임의 사이즈는 동일해야하므로 현재 물리적 공간에 페이졷 총 1024개가 존재하게 되는 것이다.

그럼 하나의 페이지 내에서 가질 수 있는 offset주소는 0 ~ 1023이 되고 프레임 역시 같은 사이즈이므로 0부터 1023의 offset이 존재하게 된다. offset의 정보는 그대로 넘어가게 된다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FcfUa30%2FbtrtsVLdCh6%2FISkCxokuB9ENSYlGpuZEd0%2Fimg.png)

그럼 페이지 테이블에는 어떤 데이터들이 담겨있을까?

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fxqzv0%2FbtrtvsaGFKc%2FeRG8oux5OUyArn0OeKOt2K%2Fimg.png)

다음과 같이 부가적인 정보들이 포함되어 있다.

- reference bit (access bit): 해당 페이지가 참조된 적이 있는지 판단하는 정보를 담고있다.
- valid bit (present bit): 현재 페이지가 물리 메모리에 있는지, 페이징 파일에 있는지에 대한 여부를 담고있다.
- dirty bit (modified bit): 현재 페이지가 수정이 된 적이 있는지에 대한 정보를 담고 있다.

지금까지 내용으로 보면, 페이지 테이블은 각각의 프로세스마다 존재하게 되고, 각 페이지 테이블은 프로세스가 가지고 있는 페이지 수만큼의 엔트리를 가지게 된다. 각각의 엔트리에는 부가적인 데이터를 포함해서 매핑되는 프레임의 정보들을 가지고 잇다.

그럼 이 페이지 테이블에 문제점에 대해서도 알아보겠다.

**large page table problem(page table 크기 문제)로**

먼저 프로세스마다 페이지 테이블이 존재하게 된다고 했는데, 프로세스 최대 크기를 기준으로 생각해봤을때

예를 들어 2^32bit 의 논리적 메모리가 존재하고 각 페이지 사이즈는 2^12bit, 페이지 테이블에서 하나의 엔트리의 크기는 4byte라고 해보자, 그러면 프로세스의 페이지 테이블은 약 2^20개의 페이지 엔트리를 들고 있어야한다.

페이지 엔트리의 개당 크기가 4bytes면 2^20 * 4byte = 4MB 정도가 된다. 컴퓨터에서 돌아가는 프로세스의 수가 10개라고 가정해보면 벌써 페이지 테이블 크기만 40MB가 된다. 

이런 문제들을 어떻게 해결해야 될까?

<br>

### Multi-level paging

멀티 레벨 페이징 기법은 페이지 테이블 자체가 다시 페이징이 되는 것이다. 예를 들어, 일반적인 페이징 기법은 1000페이지 중 38페이지 1번째 줄에 데잋터가 있어라는 식으로 페이지 테이블을 작성한다.

여기 멀티 레벨 페이징 기법을 적용하면 3번째 챕터 안에 두 번째 페이지에 1번째 줄에 데이터가 있어라고 알려주는 것이다. 이 예시는 챕터라는 레이어가 하나 추가되었으니 두 단계 페이징을 적용한 것이다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fts65f%2Fbtrtwla29e5%2FkBHpx9nGN4L7IYEYt1szw0%2Fimg.png)

멀티 레벨 페이징 기법은 모든 페이지 테이블을 들고 다니지 않아도 되기 때문에 메모리 이슈가 덜해지지만, chapter -> page -> offset 순으로 접근해야 하기 때문에 레이어가 많아진다면 메모리 접근에 대한 오버헤드가 생길 수 있다.

<br>

### Hashed Page Table

주소 공간이 32bit보다 크면 해시 값이 가상 페이지 번호가 되는 해시형 페이지 테이블을 많이 사용한다. 해시형 테이블의 각 항목은 연결 리스트를 가지고 있고, 해시 원소는 세 개의 필드(가상  페이지 번호, 매핑 페이지 프레임 번호, 연결 리스트 상의 다음 포인터)를 가지고 있다.

논리적 주소 공간의 레이지 번호인 p를 해시한 결괏값으로 테이블을 검사한 뒤, 해당 물리적 메모리를 찾아가게 된다. 같은 해시값으로 리스트 하나를 이용할 수 있기 때문에 문제를 해결할 수 있다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbsQszU%2FbtrtsECdmGe%2FN21rLLQtTRxevabRq9LXf1%2Fimg.jpg)

<br>

### Inverted Page Table

물리적 메모리 주소 입장에서 만들어진 테이블 구조로 이전까지의 테이블은 모두 논리적 주소 공간을 기준으로 테이블을 만들고 나누어진 페이지의 순서대로 페이지 테이블에 매핑하는 구조였다.

따라서 이전에 페이지 테이블에 몇 번째 프레임인지에 대한 정보를 담았다면, 이제는 프로세스 번호의 pid값도 담고 있다.

inverted page table은 프로세스마다 하나의 페이지 테이블이 만들어지는 것이 아니라 전체 테이블만 있으면 되는 장점이 있지만, 그만큼 크기가 크기 때문에 검색할 때 시간이 더 소요된다. 또한 pid 정보가 없으면 원래 페이지 테이블로 가서 업데이트를 하고 수정을 해야한다는 번거로움이 있다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2F9SPew%2FbtrtwKaTQrj%2Fod9O7zp4KqcBod1AnZHRxk%2Fimg.png)

<br>

### Memory Access Overhead Problem

페이지 테이블을 어디에 저장해야 가장 빠르게 참조하고 주소 변환을 시킬 수 있을까?

바로 MMU다. 하지만 4mb의 메모리를 mmu에 집어넣는 것은 말이 되지 않는다.

결국 cpu가 아닌 램에 저장을 해야한다는 것인데, 이렇게 되면 메모리에 1번 접근하기 위해 메모리를 2번 접근해야 하는 샘이 된다.

페이지 테이블에 접근해서 매핑된 프레임의 정보를 읽고 난 뒤, 실제 메모리에 접근해서 데이터를 읽어와야 하는 것이다.

이 과정에서 오버헤드가 발생하는데.. 어떻게 해결할 수 있을까?

바로 캐시 메모리를 사용하는 것이다. 페이지 테이블의 크기가 커도 실제로 다 쓰이는 것은 아니기 때문에 이 점을 이용하는 것이다.

프로그램은 locality라는 특성을 가지고 있어 자주 쓰이는 녀석은 자주 쓰이고, 안쓰이는 녀석은 거의 안쓰이기 때문이다.

이 특성을 가져와 실제 참조되는 페이지는 이 중에서 몇개 되지 않는다 라는 원리를 이용해 캐싱해두는 것이다.

<br>

### TLB(Translation Look-aside Buffer)

이렇게 나온 방법이 Associative Memory인 TLB이다. 일반 메모리는 주소에 접근해서 데이터를 반환하지만 Associative Memory는 인덱스를 이용해 매핑된 데이터를 저장해두고, 다음에 동일한 접근일 때는 해당 데이터를 빠르게 반환할 수 있는 것이다. 캐시와 동일한 개념이라 볼 수 있다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbqLuOE%2FbtrtzafEYWI%2Fig1nyCAdrXxtujVo3MGt2k%2Fimg.jpg)

그런데 위처럼 TLB를 사용해 프레임에 대한 매핑 정보를 빠르게 알아보려 했지만 TLB에 아직 프레임 번호가 없으면 다시 페이지 테이블로 찾아가서 프레임 번호를 얻고 이 녀석을 TLB에 다시 저장한다.

TLB의 크기는 정해져있기 때문에 모든 프레임 번호에 대한 데이터를 다 저장하고 있을 수 없기 때문이다.



