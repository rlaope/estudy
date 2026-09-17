# 스프링 부트 배치 Spring boot Batch

## 스프링 부트 배치
스프링 배치는 백엔드의 **배치 처리 기능**을 구현하는데 사용하는 프레임워크입니다.  
  
스프링 부트 배치는 스프링 배치 설정 요소들을 간편화시켜 스프링 배치를 빠르게 설정하는 데 도움을 줍니다.

> 배치 작업: 데이터를 실시간으로 처리하는 것이 아닌, 일괄적으로 모아 한번에 처리하는 작업을 의미함.

## 스프링 부트 배치의 장점
- 대용량 데이터 처리에 최적화되어 고성능을 발휘
- 효과적인 로깅, 통계 처리, 트랜잭션 관리 등 재사용 가능한 필수 기능을 지원
- 수동으로 처리하지 않도록 자동화되어 있다.
- 예외사항과 비정상 동작에 대한 방어 기능이 있습니다.
- 스프링 부트 배치는 반복적인 작업 프로세스를 이해하면 비즈니스로직에 집중할 수 있습니다.

## 주의 사항
스프링 부트 배치는 스프링 배치를 간편하게 사용할 수 있게 래핑한 프로젝트입니다.  
따라서 스프링 부트와 스프링 배치에 모두에서 다음과 같은 주의사항을 염두해야합니다.

- 가능하면 단순화해 복잡한 구조와 로직을 피해야합니다.
- 데이터를 직접 사용하는 편이 빈번하게 일어나므로 데이터 무결성을 유지하는데 유효성 검사 등의 방어책이 있어야합니다.
- 배치 처리 시스템 I/O 사용을 최소화해야합니다. 잦은 I/O로 데이터베이스 커넥션과 네트워크 비용이 커지면 성능에 영향을 줄 수 있기 때문입니다. 따라서 가능하면 한번에 데이터를 조회하여 메모리에 저장해두고 처리를 한 다음. 그 결과를 한번에 데이터베이스에 저장하는 것이 좋습니다.
- 일반적으로 같은 서비스에 사용되는 웹 API, 배치, 기타 프로젝트들은 서로 영향을 줍니다. 따라서 배치 처리가 진행되는 동안 다른 프로젝트의 요소에 영향을 주는 경우가 없는지 주의를 기울여야합니다.
- 스프링 부트는 배치 스케쥴러를 제공하지 않습니다. 따라서 배치 처리 기능만 제공하여 스케줄링 기능은 스프링에서 제공하는 쿼치 프레임워크 등을 이용해야합니다. 리눅스 crontab 명령은 가장 간단히 사용할 수 있지만 추천하지는 않으며, crontab의 경우 각 서버마다 따로 스케줄러를 관리해야 하며 무엇보다 클러스터링 기능이 제공되지 않습니다. 반면 쿼츠 같은 스케쥴링 프레임워크를 사용한다면 클러스터링 뿐만 아니라 다양한 스케쥴링 기능, 실행 이력 관리 등 여러 이점을 얻을 수 있습니다.

## 스프링 부트 배치 이해하기
배치의 일반적인 시나리오는 다음과 같은 3단계로 이루어집니다.
1. 읽기 read: 데이터 저장소(일방적으로 데이터베이스)에서 특정 데이터 레코드를 읽습니다.
2. 처리(processing): 원하는 방식으로 데이터 가공/처리 합니다.
3. 쓰기(write): 수정된 데이터를 다시 저장소(데이터베이스)에 저장합니다.

배치 처리는 읽기 -> 처리 -> 쓰기의 흐름을 갖습니다 다음 그림은 스프링에서 이러한 배치 처리르 어떻게 구현 했는지 배치 처리와 관련된 객체의 관계를 보여줍니다.

![](https://github.com/cheese10yun/TIL/raw/master/assets/batch-obejct-relrationship.png)
- Job과 Step은 1:M
- Steprhk ItemReader, ItemProcessor, ItemWriter 1:1
- Job이라는 하나의 큰 일감(Job)에 여러 단계(Step)을 두고, 각 단계를 배치의 기본 흐름대로 구성합니다.

### Job
- Job은 배치 처리 과정을 하나의 단위로 만들어 표현한 객체입니다. 또한 전체 배치 처리에 있어 항상 최상단 계층에 있습니다.
- 위에서 하나의 Job안에는 여러가지 Step이 있다고 설명했던 바와 같이 스프링 배치에서 Job객체는 여러 Step 인스턴스를 포함하는 **컨테이너** 입니다.
- Job 객체를 만드는 빌더는 여러 개 있습니다. 여러 빌더를 통합해 처리하는 공장인 JobBuilderFactory로 원하는 Job을 쉽게 만들 수 있습니다.

```java
public class JobBuilderFactory {
    private JobRepostiroy jobrepository;

    public JobBuilderFactory(JobRepository jobRepository){
        this.jobrepository = jobrepository;
    }

    public JobBuilder get(String name){
        JobBuilder builder = new JobBuilder(name).repository(jobrepository);
        return builder;
    }
}
```
- JobBuilderFactory는 JobBuilder를 생성할 수 있는 get() 메서드를 포함하고 있습니다. get() 메서드는 새로운 JobBuilder를 생성해서 반환하는 것을 확인할 수 있습니다.
- JobBuiilderFactory에서 생성된 모든 JobBuilder가 리퍼지토리를 사용합니다.
- JobBuilderFactory는 JobBuilder를 생성하는 역할만 수행합니다. 이렇게 생성된 JobBuilder를 이용해서 Job을 생성해야 하는데, 그렇다면 JobBuilder의 역할은 무엇인지 JobBuilder의 메서드를 통해 기능을 알아보겠습니다.

```java
public SimpleJobBuilder start(Step step){
    //(1)
    // Step을 추가해서 가장 기본이되는 SimpleJobBuilder를 생성합니다.
    return new SimpleJobBuilder(tihs).start(step);
}

public JobFlowBuilder start(Flow flow){
    //(2)
    // Flow를 실행할 JobFlowBuilder를 생성합니다.
    return new JobFlowBuilder(tihs).start(flow);
}

public JobFlowBuilder flow(Step step){
    //(3)
    // Step을 실행할 FlowJobBuilder를 생성합니다.
    return new JobFlowBuilder(tihs).start(step);
}
```
JobBuilder는 직접적으로 Job을 생성하는 것이 아니라 별도의 구체적 빌더를 생성해 반환하여 경우에 따라 Job 생성 방법이 모두 다를 수 있는 점을 유연하게 처리할 수 있습니다.

### JobInstance
JobInstance는 배치처리에서 Job이 실행될 때 하나의 Job 실행 단위입니다. 만약 하루에 한 번 씩 배치의 Job이 실행된다면 어제와 오늘 실행 각각 Job을 JobInstance라고 부를 수 있습니다.
  
각각의 JobInstance는 하나의 JobExecution을 갖는 것은 아닙니다. 오늘 Job이 실행했는데 실패했다면 다음날 동일한 JobInstance를 가지고 또 실행합니다. 
  
Job 실행이 실패하면 JobInstance가 끝난것으로 간주하지 않기 때문입니다. 그렇다면 JobInstance는 어제 실패한 JobExecution과 오늘의 성공한 JobExecution 두개를 가지게 됩니다 즉 **JobExecution는 여러개 가질 수 있습니다.**

### JobExecution
JobInstance에 대한 한 번의 실행을 나타내는 객체다.  
  
만약 오늘 Job이 실패해 내일 다시 동일한 Job을 실행하면 오늘/내일의 실행 모두 같은 JobInstance를 사용합니다.  
  
실제로 JobExecution 인터페이스를 보면 Job 실행에 대한 정보를 담고 있는 도메인 객체가 있습니다. 
  
JobExecution은 JobInstance, 배치 실행 상태, 시작 식ㄴ, 끝난 시간, 실패했을 때 메시지 등의 정보를 담고 있습니다. JobExecution 객체 안에 어떤 실행 정보를 포함하고 있습니다.

### JobParameters
JobParameters는 Job이 실핼될 때 필요한 파라미터들은 Map 타입으로 지정하는 객체입니다.  
  
JobParameters는 JobInstance를 구분하는 기준이 되기도 합니다.  
  
JobParamters와 JobInstance는 1:1 관계입니다.

### Step
Step은 실질적인 배치 처리를 정의하고 제어 하는데 필요한 모든 정보가 있는 도메인 객체입니다. Job을 처리하는 실질적인 단위로 쓰입니다.  
  
모든 Job에는 1개 이상의 Step이 있어야 합니다.

### StepExectuion
Job에 JobExecution Job실행 정보가 있다면 Step에서는 StepExecution이라는 Step 실행 정보를 담는 객체가 있습니다.

### JobRepository
JobRepository는 배치 처리 정보를 담고 있는 매커니즘입니다, 어떤 Job이 실행되었으면 몇 번 실행되었고 언제 끝났는지 등 배치 처리에 대한 메타데이터를 저장합니다.
  
예를 들어 Job 하나가 실행되면 JobRepository에서는 배치 실행에 관련된 정보를 담고 있는 도메인 JobExecution을 생성합니다.  
  
JobRepository는 Step의 실행 정보를 담고 있는 StepExecution도 저장소에 저장하여 전체 메타데이터를 저장, 관리 하는 역할을 수행합니다.

### JobLauncher
JobLauncher는 Job, JobParameters와 함께 배치를 실행하는 인터페이스입니다.

### ItemReader
ItemReader는 Step의 대상이 되는 배치 데이터를 읽어오는 인터페이스입니다. File, xml, db등 여러가지 타입의 데이터를 읽어올 수 있습니다.

### ItemProcessor
ItemProcessor는 IteamReader로 읽어 온 배치 데이터를 변환하는 역할을 수행합니다. 이것을 분리하는 이유는 다음과 같습니다.  

- 비즈니스 롲기의 분리: ItemWriter는 저장을 수행하고, ItemProcessor는 로직 처림나 수행해 역할을 명확하게 분리합니다.
- 읽어온 배치 데이터와 씌여질 데이터의 타입이 다를 경우에 대응할 수 있습니다.

### ItemWriter
ItemWriter는 배치 데이터를 저장합니다. 일반적으로 DB나 파일에 저장합니다.  
  
ItemWriter도 ItemReader와 비슷한 방식을 구현합니다. 제네릭으로 원하는 타입을 받고 write() 메서드는 List를 사용해서 저장한 타입의 리스트를 매개변수로 받습니다.