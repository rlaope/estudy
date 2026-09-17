# 캐시 메모리 구조와 작동 방식

### 캐시 메모리

캐시 메모리는 속도가 빠른 장치와 느린 장치 사이에서 속도 차에 따른 병목 현상을 줄이기 위한 범용 메모리다.

![](https://velog.velcdn.com/images/letskuku/post/adbf3347-552a-4de3-988a-d72c9a1e848a/image.png)

대표적으로, 비교적 속도가 빠른 cpu 코어와 속도가 느린 메모리 사이에서 속도 차에 따른 병목 현상을 완화하는 역할을 예로 들 수 있다. cpu는 메모리에 저장된 데이터를 읽어오는데, 자주 사용하는 데이터를 캐시 메모리에 저장하면 그 데이터를 다시 사용할 때는 캐시에 있는 데이터를 읽어서 사용할 수 있다.

이 외에도 인터넷 웹 브라우저에 캐시라는 단어를 볼 수 있는데, 이때의 캐시는 웹 페이지 상의 이미지와 같은 용량인 높은 데이터들을 하드디스크에 미리 저장해두고, 웹 페이지를 재방문했을 때, 사이트가 아닌 하드디스크에서 이미지를 불러들여 로딩 속도를 높이는 역할을 한다. 즉, 속도가 빠른 하드디스크와 속도가 느린 웹 페이지 사이에 병목 현상을 완화시킬 수도 있는 것이다.

![](https://velog.velcdn.com/images/letskuku/post/9c38f99c-2294-44ae-b62a-18ed573d2fda/image.png)

<br>

### 캐시 메모리 Level

CPU에서는 캐시메모리가 2~3개 정도 사용되며, 각각 level로 분류한다.

다음은 intel core 2 duo 모델에 대한 케시 매모리를 나타낸 것이다.

![](https://velog.velcdn.com/images/letskuku/post/cac3b81c-ebb1-45fa-8896-e11c4f83ab05/image.png)

각 cpu 코어마다 독립적인 L1 캐시 메모리를 가지고 있는데, 특정 코어만을 위한 캐시를 private cache라고 한다. 즉, 여기서 L1은 private cache 이고 물리적으로 DL1 캐시와 IL1 캐시로 나누어져있다.

DL1 캐시에는 데이터가 저장되고, IL1 캐시에는 명령어가 저장되어서 시간적으로 instruction fetch와 memory access가 겹쳐도(구조 해저드) 진행 가능하다. 이외에 두 코어가 공유하는 L2 캐시 메모리 (shared cache)가 존재한다.

캐시 메모리는 level에 따라서 속도와 크기가 분류되어있다. L1캐시는 일반적으로 cpu 칩 안에 내장되어 있어 가장 빠르게 접근이 가능하다. L1 캐시에서 먼저 데이터를 찾고, 찾지 못하면 L2 캐시로 넘어간다.

다음은 core i7 2세대 모델에 대한 캐시 메모리다.

![](https://velog.velcdn.com/images/letskuku/post/6aa90984-8d16-43be-9e7c-8e964acedaee/image.png)

코어 4개가 각각의 L1 캐시와 L2 캐시를 갖고 private하게 사용하며 shared cache로 L3를 사용하는 것을 볼 수 있다. 이렇게 주로 마지막 레벨에서 해당되는 캐시는 커다랗고 각 코어가 공유할 수 있는 구조로 만든다.

<br>

### 지역성의 원리

캐시 메모리는 메인 메모리에 비해 작은 크기의 메모리다. 메인 메모리에 저장된 많은 데이터중, cpu가 자주 사용할 것이라고 생각되는 일부의 데이터를 캐시 메모리에 저장하는 것이다. 이때, 캐시 메모리는 지역성의 원리(principle of locality)를 이용한다.

- **시간 지역성(Temporal locality, locality in time)**: 한 번 참조된 데이터는 잠시 후에 또 참조될 가능성이 높다. (ex. for, while 등의 반복문)
- **공간 지역성(Spatial locality, locality in space)**: 참조된 데이터 근처에 있는 데이터가 잠시 후에 사용될 가능성은 높다.(ex. A\[0], A\[1]과 같은 데이터 배열에 연속으로 접근)

<br>

### 캐시 메모리 관련 용어

- **Block(or cache line)**: 캐시 메모리와 메인 메모리 사이에 주고받는 데이터의 단위(일반적으로 64byte)
- **Hit**: CPU가 읽어오려고 하는 데이터가 캐시에 있는 경우
	- Hit rate: cpu가 요청한 데이터 중 캐시에 저장된 비율
	- Hit time: 캐시에서 읽어오는데 필요한 시간
- **Miss**: CPU가 읽어오려고 하는 데이터가 캐시에 없는 경우
	- Miss rate: cpu가 요청한 데이터중 캐시에 저장되지 않은 비율 (= 1 - Hit rate)
	- Miss penalty: miss가 발생해 데이터 block만큼 메인 메모리가 캐시 메모리로 가져오는데 필요한 시간

> 당연하게도 cache를 적용하면 cache hit 비율이 높아야한다. 적어도 90퍼 이상


<br>

### Cache Miss 종류

cache miss는 크게 3가지의 경우로 발생한다.

- **cold miss**: 해당 메모리 주소를 처음 불렀기 때문에 miss가 발생하는 경우로, cache가 비워져있을때는 어쩔 수 없이 발생한다.
- **conflict miss**: 캐시 메모리 A 데이터와 B 데이터를 저장해야하는데, A, B가 같은 캐시 메모리 주소에 할당되어 miss가 발생하는 경우다 예를 들어 A 데이터를 저장한 후 B 데이터가 같은 캐시 메모리 주소에 할당어 저장되면 이후 A 데이터를 읽어오려 할때 miss가 발생한다(일관성x)
- **capacity miss**: 캐시 메모리 공간이 부족해 miss가 발생하는 경우다.


<br>

## 구조와 동작 방식

### Direct mapped Cache

![](https://velog.velcdn.com/images/letskuku/post/e35858e4-50e0-4aa4-b6dc-8ea6cfdda796/image.png)

가장 기본적인 캐시 메모리 구조로, **각 memory block이 cache의 정확히 칸 한 block에만 대응**된다. 예를 들어 memory 00001에 위치한 block은 cache-memory의 001에만 저장될 수 있다. 따라서 hit miss 판단을 위해서는 한 개의 cache block만 확인하면 된다.

memory block이 저장될 cache block의 주소는 `(memory block 주소) % (cache block 수)` 로 구할 수 있다. 즉, 00000(0) 부터 11111(31)을 111(8)로 나눈 나머지이므로 memory block 주소의 하위 세 자리이다.

그런데, 메모리는 캐시보다 크므로 여러개의 memory block은 하나의 cache block을 공유한다. 예를 들어 캐시의 회색 부분은 001에 저장된 데이터가 메모리 4부분 (00001, 01001, 10001, 11001) 중 어디에서 왔는지 알 수 있어야 한다. 이를 식별하기 위해 **tag**를 사용하며 아래와 같은 정보들을 제공한다.

![](https://velog.velcdn.com/images/letskuku/post/3d267455-ec15-491a-841b-97c3eae9793b/image.png)

tag는 memory block 주소를 cache block 수로 나눈 몫으로, 이 경우 memory block 주소의 상위 두 자리이다.

캐시 메모리는 valid, tag, data로 구성이 된다.

- valid: 최초에는 0으로 세팅, 실제로 의미있는 데이터가 들어오는 순간부터 1로 기록한다.
- tag: 어던 memory block에 cache에 저장되어있는 부분인지 확인하기 위해 필요한 정보가 저장되어있다.
- data: 실제 데이터가 저장된 부분이다.

<br>

### Fully associative Cache

Fully associative cache는 direct mapped cache와 정반대 개념으로, memory block은 캐시 메모리의 비어있는 모든 block에 저장될 수 있다.

![](https://velog.velcdn.com/images/letskuku/post/567cb412-39b6-4290-b4bf-e4b55b17701e/image.png)

따라서 hit miss 판별을 위해서 모든 cache의 block을 확인해야한다.

<br>

### N-way set associative Cache

N-way set associative Cache는 direct mapped cache와 fully associative cache의 중간쯤 개념이라고 이해할 수 있는데, 캐시 메모리를 n개의 block(way)를 가진 set이라는 단위로 나누고, memory block을 하나의 set에서 비어있는 block에 저장하는 것이다. 다음은 2-way set associative cache이다.

![](https://velog.velcdn.com/images/letskuku/post/081d977e-b027-4e80-a573-f892f296b95c/image.png)

memory block은 하나의 set에서 2개의 block중 하나에 저장될 수 있으며, hit or miss 판별을 위해서는 block 2개를 모두 확인해야한다. (memory block 주소) % (캐시의 set 수)를 계산해 어느 set을 확인해야하는지 구할 수 있다.