# AWS 보안 그룹 Security Group

## Security Group

![](https://user-images.githubusercontent.com/28394879/136934211-085a6871-2ff6-423d-b208-88e55945c45f.png)

보안 그룹은 인스턴스에 대한 인바운드 및 아웃바운드 트래픽을 제어하는 가상 방화벽 역할을 한다.  
  
VPC에서 인스턴스를 시작할 때 최대 5개의 보안 그룹에 인스턴스를 할당할 수 있다.  
  
보안 그룹은 **서브넷 수준이 아니라 인스턴스 수준에서 작동**하므로 VPC에 있는 서브넷의 각 인스턴스를 서로 다른 보안 그룹 세트에 할당할 수 있다.
  
시작할 때 특정 그룹을 지정하지 않으면 인스턴스가 자동으로 VPC의 기본 보안 그룹에 할당된다.

### 특징

- 보안 장치
  - Network Access List(NACL)와 함께 방화벽의 역할을 하는 서비스
- Port 허용
  - 트래픽이 지나갈 수 있는 Port, Source를 지정 가능
  - Deny는 불가능 -> NACL로 가능
- 인스턴스 단위
  - 하나의 인스턴스에서 하나 이상의 SG 설정 가능
  - NACL의 경우 서브넷 단위
  - 설정된 인스턴스는 설정한 모든 SG의 룰을 적용 받음

![](https://user-images.githubusercontent.com/28394879/136935520-e5b45cb7-28e1-48e6-863f-02572c399284.png)

- 설정된 모든 룰을 사용해서 필터링
  - NACL의 경우 적용된 룰의 순서대로 필터링
- Stateful
  - Inbound로 들어온 트래픽이 별 다른 Outbound 설정 없이 나갈 수 있음
  - NACL은 Stateless

![](https://user-images.githubusercontent.com/28394879/136936510-05742607-16dc-4031-b1b4-972aa13cad5e.png)