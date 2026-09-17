# Elastic IP

## ENI
Elastic Network Interface

- Mac address
- 원본/대상 확인
- 한 개 이상의 보안 그룹
- 한 개의 메인 프라이빗 IPv4
- 한 개 이상의 보조 프라이빗 IPv4
- 한 개 이상의 IPv6 주소
- 하나의 퍼블릭 IPv4 주소

EC2에는 여러대의 ENI를 붙일 수 있다. (EC2의 타입에 따라 붙일 수 있는 개수가 다르다)  
ENI에 부여된 public IP는 계속해서 바뀐다.  
Elastic IP Address는 고정된 public IP로 ENI에 붙일 수 있는 서비스다.

![](https://user-images.githubusercontent.com/28394879/141428277-ea3223bc-86c7-48a0-90c7-b7c498d5dd75.png)

## Elastic IP
사용은 무료
  
사용하지 않거나 ENI에 붙어있지 않을때만 사용료 지불
  
EC2 뿐만 아니라, Network Load Balancer 혹은 Nat Gateway에도 고정 IP를 부여하는데 사용할 수 있다.

![](https://user-images.githubusercontent.com/28394879/141428421-e322e5da-2641-4d79-a130-1a5a480bb122.png)