# 오토 스케일링 Auto Scaling

## Auto Scaling
애플리케이션을 모니터링하고 용량을 자동으로 조정하여, 최대한 저렴한 비용으로 안정적이고 예측 가능한 성능을 유지한다.  
  
AWS Auto Scaling을 사용하면 몇 분 만에 손쉽게 여러 서비스 전체에서 여러 리소스에 대해 애플리케이션 규모 조정을 설정할 수 있다.

![](https://user-images.githubusercontent.com/28394879/137873154-df8c7c15-d8a4-4c0c-9d71-2387331edfd4.png)

## Auto Scaling의 활용
최소한의 인스턴스를 사용
  
원하는 만큼의 인스턴스 개수를 목표로 유지
  
최대 인스턴스 개수 이하로 인스턴스 유지
  
Avaliablity Zone에 골고루 분산될 수 있도록 인스턴스를 분배
  
항상 서비스가 유지될 수 있는 인스턴스 확보

### EC2 Auto Scaling 구성
- Launch Configuration: 무엇을 어떻게 실행시킬 것인가?
  - EC2의 타입, 사이즈
  - AMI
  - Security Group, Key, IAM
  - User Data
- Monitoring: 언제 실행시킬 것인가? + 상태 확인
  - 예: CPU 점유율이 일정 %를 넘어섰을 때 추가로 실행 or 2개 이상이 필요한 스택에서 EC2 하나가 죽었을 때
  - Cloud Watch (AND/OR) ELB와 연계
- Desired Capacity: 얼만큼 실행 시킬 것인가?
  - 예 최소 1개 ~ 최대 3개
- Lifecycle Hook: 인스턴스 시작/종료시 콜백
  - 다른 서비스와 연계하여 전/후 처리 가능 -> Cloud Watch Event / SNS / SQS
  - Terminationg: wait/Terminationg: Proceed 상태로 전환
  - 기본 3600초 동안 기다림 ( 기다리는 동안 이미지 백업이나 로그 백업 등의 작업을 할 수 있게끔 )

## EC2 Auto Scaling의 순서도

![](https://user-images.githubusercontent.com/28394879/137876824-8fb023db-f32b-4959-93c4-a1c930bf792f.png)