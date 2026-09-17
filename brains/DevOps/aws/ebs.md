# EBS, Instance Storage, AMI

## EBS
`Elastic Block Store`  
  
Amazon Elastice Block Store(EBS)는 AWS 클라우드의 Amazon EC2 인스턴스에서 사용할 영구 블록 스토리지 볼륨을 제공한다.  
  
각 Amazon EBS 볼륨은 가용 영역 내에 자동으로 복제되어 구성요소 장애로부터 보호해주고, 고가용성 및 내구성을 제공한다.  
  
Amazon EBS 볼륨은 워크로드 실행에 필요한 지연 시간이 짧고 일관된 성능을 제공한다.  
  
Amazon EBS를 사용하면 단 몇 분 내에 사용량을 많게 또는 적게 확장할 수 있으며, 프로비저닝한 부분에 대해서만 저렴한 비용을 지불한다.

![](https://user-images.githubusercontent.com/28394879/136925202-f5785c89-9377-43ee-8fc2-45bbb47e424d.png)

- `EBS Based`: 반 영구적인 파일의 저장 가능
  - Snapshot 가능
  - 인스턴스 업그레이드 가능
  - STOP이 가능함
- `Instance Store`: 휘발성이 빠른 방식
  - 빠르지만 저장이 필요 없는 경우
  - STOP이 불가능하다.


## AMI

Amazon Machine Image
  
Amazon 머신 이미지 AMI는 인스턴스를 시작하는데 필요한 정보를 제공한다.  
  
인스턴스를 시작할 때 AMI를 지정해야 한다.
  
동일한 구성의 인스턴스가 여러 개 필요할 때는 한 AMI에서 여러 인스턴스를 시작할 수 있다.  
  
서로 다른 구성의 인스턴스가 필요할 때는 다양한 AMI를 사용하여 인스턴스를 시작하면 된다.  

### 특징
![](https://user-images.githubusercontent.com/28394879/136926299-e8917a9f-404e-4a96-b485-c6722d608950.png)

AMI는 다음을 포함한다.
1. 1개 이상의 EBS 스냅샷 또는 인스턴스 저장 지원 AMI의 경우 인스턴스 루트 볼륨에 대한 템플릿 (예: 운영체제, 애플리케이션 서버, 애플리케이션)
2. AMI를 사용하여 인스턴스를 시작할 수 있는 AWS 계정을 제어하는 시작 권한
3. 시작될 때 인스턴스를 연결할 볼륨을 지정하는 블록 디바이스 매핑