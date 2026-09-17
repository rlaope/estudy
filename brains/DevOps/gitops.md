# GitOps

GitOps의 기본 아이디어는 시스템 외부에 있는 모델을 기반으로 전체 시스템애 대한 작업들을 자동화하는 것에서 비롯되었다.

Git은 해당 모델을 배치하기로 선택한 것이다.

즉 시스템 외부의 있는 모델을 통해서 프로덕션의 리소스들을 동기화하고 자동화한다. 그리고 그 저장소가 Git으로 선택되어 운영하는 것이 바로 GitOps다.

### GitOps란

Git을 기반으로 CD(Continuous Deployment)를 구현하는 방법중 하나다..

git, IaC, CI등의 익숙한 도구들을 사용하여 개발자 중심의 운영 인프라로 확장한다.

![](https://oopy.lazyrockets.com/api/v2/notion/image?src=https%3A%2F%2Fs3-us-west-2.amazonaws.com%2Fsecure.notion-static.com%2Fe23e305e-9d15-4c2b-972d-84e6bac9bcbf%2FUntitled.png&blockId=fe59f9eb-a3a2-4119-a750-40e0623c0742)

> Continuous Deployment는 Continuous Delivery에서 production으로의 배포를 자동화한 것에 차이가 있다

CI뿐 아니라 CD도 Git에 적용을 하며, 개발자가 사용한 workflows를 통해서 운영팀에도 적용을 한다.

git은 SSOT(Single Source of Truth)면서(모든 데이터에 대해 하나의 출처를 사용), environment에 대한 인터페이스 역할도 담당한다. (e.g: staging, production)


<br>

### GitOps 원칙

깃옵스는 다음과 같은 원칙들을 중요시한다.

1. **선언적(Declarative)**: GitOps에서 관리되는 시스템은 선언적으로 표현되어야 한다.
2. **버전 지정, 불변성(Versioned and Immutable)**: 원하는 상태는 불변적이고 버전을 지정되며 완전한 버전 이력을 강제하여 저장된다.
3. **자동으로 가져오기(Pulled Automatically)**: 소프트웨어 에이전트 소스에서 원하는 상태에대한 선언을 자동으로 가져와야한다.
4. **지속적인 화해(Continuously Reconciled)**: 소프트웨어 에이전트는 지속적으로 실제 시스템 상태를 관찰하고 원하는 상태를 적용하려고 시도하여야한다.

<br>

### Workflow

![](https://oopy.lazyrockets.com/api/v2/notion/image?src=https%3A%2F%2Fs3-us-west-2.amazonaws.com%2Fsecure.notion-static.com%2F402a5c20-a430-4fe6-b2cc-fac93f15b2a3%2FUntitled.png&blockId=15c6307b-6a57-4d4f-b549-e99cfb6c702f)

깃옵스의 흐름은 위와 같다. CI 과정인 기존과 동일하다. 깃으로 코드를 반영하고 Container Registry에 이미지가 반영된다.

그리고 이미지에 대한 변경사항에 대해서 메니페스트 코드 저장소에 업데이트 요청(Pull Request. 상단에서 하단으로 연결 과정 - Update image in staging config)

하단은 CD 과정으로, 제출된 PR에 대해 검토 및 git에 merge. Deploy Operator는 이를 확인하여 DEPLOY 수행