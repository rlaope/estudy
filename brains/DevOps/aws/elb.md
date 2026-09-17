# AWS ELB (Elastic Load Balancer), Sticky Session

`Elastic Load Balancing`은 들어오는 애플리케이션의 트래픽을 Amazon EC2 인스턴스, 컨테이너, IP 주소, Lambda 함수와 같은 여러 대상에 자동으로 분산시킨다.  
   
단일 가용 영역 또는 여러 가용 영역에서 다양한 애플리케이션 부하를 처리할 수 있다.
  
Elastic Load Balancing이 제공하는 세 가지 로드 밸런서는 모두 애플리케이션의 내결함성에 필요한 **고가용성**, **자동 확장/축소**, **강력한 보안**을 갖추고 있다.

![](https://user-images.githubusercontent.com/28394879/137287365-896396b6-3eca-4894-afd1-6c1750340e39.png)

## Vertical Scale

![](https://user-images.githubusercontent.com/28394879/137288267-d5a5a8b8-4907-44d1-9bcd-12a13be07bf1.png)

![](https://user-images.githubusercontent.com/28394879/137288362-b1ce0bd9-4d9a-44b2-a8ee-ed6b544915a2.png)

## Horizontal Scale

![](https://user-images.githubusercontent.com/28394879/137288510-7d1db362-be70-4dce-b59e-9c5bfe216d33.png)

![](https://user-images.githubusercontent.com/28394879/137288634-1ed44f9c-a25e-41b1-a928-b698d694d0c2.png)

### ELB 특징

IP가 지속적으로 변경됨
- 지속적으로 IP 주소가 변경된다.
- 따라서 도메인 기반으로 사용해야 한다.
  
Health Check
- 직접 트래픽을 발생시켜 Instance가 살아있는지를 체크한다.
- InService,OutService 두 가지 상태로 나누어진다.

  
3가지 종류
- Application Load Balancer
- Network Load Balancer
- Classic Load Balancer

### Application Load Balancer
Application Level이며 `똑똑한 놈`으로 생각하면 된다.

### Network Load Balancer
`빠른놈`으로 생각하면 된다. Elastic IP 할당 이 가능하다.

### Classic Load Balancer
`옛날놈`으로 생각하면 된다. 요즘은 잘 안쓴다.

## Sticky Session

![](https://user-images.githubusercontent.com/28394879/137290519-58ba8dec-02b3-400a-8973-20412d1fcc0b.png)

2개 이상의 Instance가 있다고 했을 때 A Instance의 웹 서버에 로그인을 하면 Session이 하나 발급될 것이다.  
  
그런데, 한번더 요청을 했을때 B Instance의 웹 서버에 요청하느라 Session 없이 재 로그인을 하라고 요청을 할 것이다.  
  
이 것을 방지하기 위해 나온 것이 Sticky Session이다.  
  
Sticky Session은 **사용자마다 어떤 인스턴스에 접근했는지를 저장해두고 다음번 요청시에 해당하는 인스턴스로 접속할 수 있도록 해주는 것이다.**

