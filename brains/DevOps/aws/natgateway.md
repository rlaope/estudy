# NAT, NAPT 그리고 AWS NAT Gateway

## NAT

대부분의 네트워크는 모든 호스트 중 일부만이 인터넷 통신을 수행한다.

따라서 대부분의 호스트는 private ip를 이용해 통신하고있기에 인터넷 통신을 수행할 때만 public ip를 사용하게 된다면 외부에 노출되는 public ip수를 크게 줄일 수 있다.

private ip에서 인터넷으로 요청을 보낼 떄 요청이 nat을 제공하는 라우터를 통과하게되면

**Nat 라우터는 주소 변환 테이블에 가지고 있던 private ip를 public ip로 ip 변환을 하여 요청을 보내고 변환 내용을 nat변환 테이블에 기록한다.**

이후 인터넷으로 보낸 응답이 도착하면 기록해두었던 nat 변환 테이블을 참조하여 요청을 보낸 private ip를 가진 호스트에게 응답을 반환한다.

## NAPT

네트워크에서는 여러 호스트가 고유한 private ip를 할당받고, 인터넷으로 요청을 보낼 때 public ip로 변환을 위해 nat을 통과할때 nat 변환 테이블은 private ip(요청 주소)와 nat으로 변환된 public ip(변환된 주소)를 기록합니다.

private ip는 host마다 고유하지만 변환된 public ip는 네트워크의 대표 공인 ip이기에 동일할 수 있습니다.

따라서 인터넷에서 응답이 nat으로 돌아올 때 요청을 보낸 private ip를 구분하기 위해서 변환 테이블의 public ip와 private ip마다 각자 다른 port를 할당합니다.

이를 `Network Address Port Translation` NAPT라고 합니다.

## AWS NAT Gateway

NAT 게이트웨이는 AWS에서 제공하는 NAT(Network Access Translation, 네트워크 주소 변환) 서비스입니다.

NAT의 개념과 동일하게 private 서브넷의 인스턴스가 VPC 외부의 서비스(인터넷)에 연결할 수 있도록하고 외부 서비스에서 프라이빗 서브넷 내의 인스턴스와의 연결을 할 수 없도록 하기 위해 NAT 게이트웨이를 사용할 수 있습니다.

즉 프라이빗 서브넷 내의 EC2를 인터넷, AWS Service에 접근 가능하게 하고 외부에서는 해당 EC2에 대한 접근을 막기 위해 사용됩니다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FCua9B%2FbtrM6LovD21%2FZuVRzklBuYOVtupjqNjZ21%2Fimg.png)