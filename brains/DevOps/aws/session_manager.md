# Systems Manager Session Manager

## Session Manager 탄생 배경

![](https://user-images.githubusercontent.com/28394879/141436264-dceb30d4-7304-420c-859d-45064cef4fee.png)

EC2의 개수가 늘어남에 따라 각각 다른 pem 파일들을 관리하기가 힘들어 진다.  
  
Bastion Host로 관리할지라도 매번 접속할 때 Bastion Host를 거쳐야 한다는 번거로움이 있다. 
  
이것을 해결하기 위해서 나온 것이 Session Manager이다.

## Session Manager
**Systems Manager Session Manager**
- 완전 관리형 AWS 서비스로 EC2 및 온프레미스 인스턴스, 가상머신을 브라우저 기반의 쉘 혹은 AWS CLI로 관리할 수 있는 서비스

### System Manager

![](https://user-images.githubusercontent.com/28394879/141436794-93d1c12c-4a05-4a48-a739-d023204e2ecd.png)

원래는 위의 사진과 같이 굉장히 복잡하게 관리 했던 것들을 아래 사진으로 관리할 수 있는 것이 AWS System Manager 이다.

![](https://user-images.githubusercontent.com/28394879/141436884-929f4ceb-2697-4958-ba84-9d273fe14fea.png)

### Session Manager의 장점.
1. 인스턴스에 대해 원클릭 엑세스를 제공하는 관리형 서비스
2. 인스턴스에 SSH연결 없이, 포트를 열 필요 없이, 배스천 호스트를 유지할 필요 없이 인스턴스에 로그인 가능
3. IAM 유저 단위로 제어 가능 (Key 파일로 제어할 필요 없음)
   - 예) 수백개의 인스턴스에 대해 일일이 로그인을 위한 키 파일을 관리해야 할 때
   - 개발자 별로 지정된 팀의 인스턴스만 로그인 할 수 있도록 하고 싶을 때
4. 웹브라우저 기반으로 OS와 무관하게 사용 가능
5. 로깅과 감사
   - 언제 어디서 누가 접속했는지 확인 가능(CloudTrail)
   - 접속 기록과 사용한 모든 커맨드 및 출력 내역을 S3 혹은 CloudWatch로 전송 가능
   - AWS의 서비스와 연동되어 있어 다양한 시나리오 구현 가능
     - 예: EventBridge 등과 연동하여 실시간으로 접그넹 대한 알림을 받기

