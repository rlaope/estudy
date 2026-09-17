# Parity bit, Pararell Paritiy, Hamming Code

### Paritiy bit

paritiy bit는 **정보의 전달 과정에서 오류가 생겼는지를 검사**하기 위해서 추가되는 비트다.
1 bit의 오류를 찾아낼 수 있는데 패리티 비트를 포함한 데이터에서 1의 개수가 짝수인지 홀수인지에 따라 짝 홀 패리티로 나뉜다.

![](https://velog.velcdn.com/images/letskuku/post/17091b62-c821-41e2-bc6c-5d73065b4fbe/image.png)

위 사진에서 8 bit 데이터를 전송할 때 맨 끝에 패리티 비트를 추가해서 전송하게 되면 짝수 패리티의 경우 100101010 홀수는 100101011이다.  
이렇게 패리티를 정해서 데이터를 보내면 데이터를 받는 쪽에서 수신된 데이터의 전체 비트를 계산해 패리티 비트로 다시 계산하는 것으로 오류를 확인가능하다.  
그러나, 패리티 비트로는 오류가 발생했다는 사실만 확인할 뿐 어느 비트에서 오류가 발생했는지 알 수 없다. 그래서 재전송 요청 해야됨..  
하나가 아닌 2개의 비트에서 오류가 발생하면 또 검출해내지 못한다.

### pararell paritiy

위와 같은 한계점을 보완하기 위해서 패리티를 가로 세로 구성되는 데이터 블록에 적용해서 에러 위치를 찾아 정정할 수 있도록 하는 병렬 패리티가 또 있다.  
병렬 패리티는 아래 표에서 같이 각각의 가로 1byte에 대해서 패리티를 만들고 각각의 세로 1바이트에 대해 패리티를 구성해 블록 단위로 전송하면 가로와 세로에 대해 각각 패리티를 검사함으로써 에리러를 찾아내고 정정할 수 있다.

전송된 데이터 블록 중에서 한 비트의 에러가 발생하면 가로와 세로 패리티 특정 부분에 패리티가 맞지 않아 아래 표에서 처럼 마주치는 곳에서 에러가 발생했음을 알 수 있다.

![](https://mblogthumb-phinf.pstatic.net/MjAxOTA3MTlfMjU4/MDAxNTYzNDk5NTM0MTc4.GSSbKf1ZlkAd9mC9W6_f9QnbN9Cwv9JW6Es4un7ZZnQg.Ct80Bg3NheAYrSDdsSFxf_woXlbeCoN9O2sz4BggORcg.PNG.cni1577/%EC%BA%A1%EC%B2%98.PNG?type=w800)


<br>

### Hamming Code

위 패리티 문제에서 1bit 이상의 오류를 검출할 수 없으며 1bit에러를 정정할 수 없다.  
Hamming Code는 이런 1bit 이상의 오류를 정정할 수 있는 오류 정정 부호 ECC(Error Correction Code)의 일종이다.

**패리티 비트를 데이터의 비트수에 따라 필요한만큼 사용하여 데이터에 추가하고, 패리티 비트를 조합해 에러 검출 및 교정을 수행한다.**
데이터의 비트 수에 따라서 필요한 패리티 비트의 수를 구하는 공식은 **2^p >= d + p + 1 (p : 패리티 비트 수, d : 데이터 비트 수)** 이다.

예를 들어서 4bit 데이터 전송에서는 패리티 비트가 적어도 3개가 필요하다. 그래서 해밍코드는 7bit가 된다.  
해밍 코드에서 패리티 비트는 2의 거듭제곱에 해당하는 순서에 삽입된다 즉 1, 2, 4, 8 bit의 위치에 삽입된다.

즉 정리하면 해밍코드는 2의 n승 번째 자리인 1, 2, 4 ..번째 자릿수가 패리티 비트인 것이다. (데이터 전송 비트 2^n 승 자리마다 패리티를 넣고 오류를 찾는것이다.)
이 숫자로 부터 시작하는 세개의 패리티 비트가 짝수인지 홀수인지 기준으로 판별한다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fbzx20l%2Fbtra345ZkEc%2FPRtCfFcqDtx6LvTCzRtng0%2Fimg.png)
