# AWS ECR(Elastic Container Registry)

AWS ECR은 안전하고 확장 가능하고 신뢰할 수 있는 AWS 관리형 컨테이너 이미지 레지스트리 서비스다.

Docker Hub와 동일하다고 볼 수 있지만 장점으로는 S3로 Docker Image를 관리하므로 고가용성을 보장하고, AWS IAM 인증을 통해 이미지 push/pull에 대한 권한 관리가 가능하다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FcAOvUx%2FbtrX6ohUvlB%2FJo0e3EnLU7cVRoe4Gpo5O0%2Fimg.png)

### 구성요소
- Registry: Amazon ECR 프라이빗 레지스트리는 각 AWS 계정마다 제공되며 레지스트리에 하나 이상의 리포지토리를 생성하고 이 리포지토리에 이미지를 저장한다.
- Repository: Amazon ECR 리포지토리에는 Docker 이미지, Open Container Initialtive(OCI) 이미지 및 OCI 호환 아티팩트가 포함된다.
- Repository Policy: Repository또는 Repository내 Docker 이미지에 대한 엑세스 제어권 관리
- Image: Repository에 컨테이너 이미지를 푸시하고 가져올 수 있습니다. 개발 시스템에서 로컬로 이러한 이미지를 사용하거나, ECS 태스크 정의 및 Amazon EKS 포드 사양에서 이를 사용 가능합니다.
- 사용자 권한 토큰: Client는 Amazon ECR Registry에 AWS 사용자로서 인증을 해야 Image를 push/pull 가능합니다.

## ECR이 제공하는 기능

### 수명주기 정책

수명주기 정책은 리포지토리에 있는 이미지의 수명 주기를 관리하는 데 도움이 됩니다. 사용되지 않는 이미지를 정리하는 규칙을 정의합니다. 규칙을 리포지터로에 적용하기 전ㅇ ㅔ테스트할 수 있습니다.

### 이미지 스캔
이미지 스캔은 컨테이너 이미지의 소프트웨어 취약성을 식별하는 데 도움이 됩니다. 각 리포지토리는 푸시 시 스캔하도록 구성할 수 있습니다. 이렇게 하면 리포지토리로 푸시된 각각의 새 이미지가 스캔됩니다. 그런 다음 이미지 스캔 결과를 검색할 수 있습니다.

### 교차 리전 및 교차 계정 복제
교차 리전 및 교차 계정 복제를 통해 이미지를 필요한 곳에 쉽게 배치할 수 있습니다. 이는 레지스트리 설정으로 구성되며 리전별 단위로 구성됩니다.

### 플스루 캐시 규칙
플스루 캐시 규칙은 프라이빗 ECR 레지스트리의 원격 퍼블릭 레지스트리에서 리포지토리를 캐시하는 방법을 제공합니다. ECR은 풀스루 캐시 규칙을 사용하여 정기적으로 원격 레지스트리에 연락하여 Amazon ECR 프라이빗 레지스트리의 캐시된 이미지가 최신 상태인지 확인합니다.