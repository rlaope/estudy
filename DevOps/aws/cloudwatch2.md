# CloudWatch

AWS 리소스의 상태, 성능, 비정상 징후를 수집,저장,시각화,경보,자동대응까지 전부 한곳에서 할 수 있도록 해주는 서비스가 CloudWatch 서비스다.

- cpu, memory, disk, network 같은 인프라 메트릭
- lambda concurrency, 에러 비율, 실행 시간
- api gateway 4xx/5xx, latancy
- RDS connection 수, 슬로우 쿼리
- 애플리케이션 비즈니스 지표(예: 결제 실패 개수, 로그인 에러율)

이 모든 걸 하나의 관측 시스템으로 통합시켜 관리할 수 있음.

실무에서는 이게 없으면 아래 문제들이 발생함.

- 장애 징후를 조기에 감지하지 못함
- 특정 리소스 병목이 발생해도 알 수 없음
- 원인 분석(트러블슈팅)에 시간이 오래 걸림
- 추측으로 인프라를 조정하게 됨 -> 비용 낭비 + 장애 위험 증가

그렇기에 클라우드 워치는 aws 실무에서 관측성의 기초라고 볼수있음.

## CloudWatch가 실무에서 어떤 고민을 해결하나.

각 영역별로 실제 생산 환경에서 어떤 문제를 해결하는가를 알아보자.

### 애플리케이션 장애 조기 탐지

- api gateway의 5xx가 갑자기 올라간다
- Lambda 오류 비율이 증가한다.
- DynamoDB throttling이 튀어오른다.
- ALB Target group의 한 인스턴스만 Latancy가 치솟는다.

CloudWatch 알람을 걸어두면 즉시 Slack/Email/SNS로 팀에 전파되거나

혹은 자동 복구(람다 트리거)까지 붙일 수 있다.

ex:
- 특정 컨테이너에서 OOMKill 발생 -> CloudWatch Events -> Lambda -> ECS Task 재시작
- DynamoDB throttling 감지 -> Lambda -> Auto scaling policy 즉시 증가

### 성능 병목 파악 및 트렌드 분석

- Lambda 실행 시간이 매주 동일 시간에 늘어난다
- RDS cpu가 80% 고착
- API latancy가 점진적으로 증가
- ECS CPU Limit에 걸린다.
- 네트워크 출력 증가로 NAT 비용 급등

CloudWatch Metrics + Logs + Insight로 원인 추적이 가능함.

> 위의 Metrics + Logs + Insight 세가지는 Observability를 구성하는 세가지 서로 다른 축임.
> - Metrics: CloudWatch의 수치 기반 시계열 데이터로 CPUUtilization = 34%, Lambda Duration = 120ms, API Gateway 5xx = 3, RDS Connection = 120, NetworkIn = 10MB/s 이런 숫자로 표현되는 값
> - Log: 원시로그를 저장함 lambda 실행 로그나 api gateway acess log, contianer log, vpc flow log 등
> - Insight(Log Insight): CloudWatch logs에 저장된 로그를 sql 비슷한 쿼리 언어로 고속 분석하는 기능임 즉, Logs를 단순히 검색하는 수준을 넘어서 집계, 통계, 패턴을 실시간으로 분석하게 해주는 파워기능
>
> ```
> fields @timestamp, @message
> | filter @message like /ERROR/
> | stats count(*) by bin(1m)
> ```
> 에러 로그 집계

### 비용 최적화

CloudWatch를 잘 쓰면 비용을 절반 수준으로 줄일 수 있음

예:
- Lambda duration이 일정 구간만 급증 -> 메모리 설정이 부족함.
- NAT Gateway의 데이터 처리 비용 급증 -> 원인이 특정 API 트래픽
- RDS IOPS 과다 소비 -> 특정 시점의 batch job이 돌아서..
- Auto Scaling 과하게 되어 EC2 비용 누수

이런 패턴은 CloudWatch가 없으면 감지할 수 없음.

### Auto Scaling 근거 확보

HPA, AWS Auto Scaling Group은 CloudWatch 메트릭을 기반으로 동작하게 짤수있으며 그렇게 짜는게 일반적

EKS에서 cloudwatch를 보려면 `aws-cloudwatch-metrics-adapter`를 설치해서 쓸수있음.

```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: honest-api-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: honest-api
  minReplicas: 3
  maxReplicas: 10
  metrics:
    - type: Resource
      resource:
        name: cpu
        target:
          type: Utilization
          averageUtilization: 70
```

즉 CloudWatch 는
- 어떤 조건에서 scale out?
- CPU 60% 기준은 무엇?
- 트래픽 패턴은 어떤가

이런 판단의 근거가 됨.

### SLA/SLO 모니터링

실무에서 중요한 지표: 
- API error ratio
- p95 latanch
- 요청 성공률
- Lambda timeout 비율
- RDS deadlock 빈도

이걸 CloudWatch dashboard/Zabbix/Grafana로 그린다.


### Alarms(경보)

- 지표 기반 판단 -> 알림 -> 슬랙 sms lambda등 실행
  - Lambda ErrorRate > 2% -> 알람
  - RDS FreeStorage < 10GB -> 알람
  - API 5xx 증가 -> 알람
  - EC2 CPU > 80% for 5 mins -> scale out trigger

### Events / EventBridge(자동 대응)

CloudWatch에서 Event는 단순 알림이 아니다.

**시스템의 상태 변화를 포착해서 자동으로 다른 aws 리소스를 실행한다.**

- EC2 비정상 종료 -> Event -> Lambda -> Slack 알림
- DynamoDB throttling -> Event -> AutoScaling 정책 변경
- 매일 09:00 -> EventBridge 스케줄러 -> Lambda 실행

1, 2, 3번케이스를 테라폼으로 간단하게 적어보겠다.

```h
// # EC2 비정상 종료 → Event → Lambda → Slack 알림
// EventBridge Rule: EC2 상태 변화 감지
// Lambda: Slack Webhook 호출
Lambda: Slack Webhook 호출
resource "aws_lambda_function" "slack_notify" {
  function_name = "ec2-termination-notify"
  role          = aws_iam_role.lambda_role.arn
  runtime       = "python3.9"
  handler       = "index.handler"

  filename = "lambda.zip"   # Slack Webhook 호출 파이썬 코드 압축파일
}

resource "aws_lambda_permission" "allow_event" {
  statement_id  = "AllowExecutionFromEventBridge"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.slack_notify.function_name
  principal     = "events.amazonaws.com"
  source_arn    = aws_cloudwatch_event_rule.ec2_termination.arn
}

resource "aws_cloudwatch_event_rule" "ec2_termination" {
  name        = "ec2-termination-rule"
  description = "Detect EC2 instance termination"

  event_pattern = <<EOF
{
  "source": ["aws.ec2"],
  "detail-type": ["EC2 Instance State-change Notification"],
  "detail": {
    "state": ["termianted", "stopping", "stopped"]
  }
}
EOF
}

resource "aws_cloudwatch_event_target" "send_to_lambda" {
  rule      = aws_cloudwatch_event_rule.ec2_termination.name
  target_id = "LambdaTarget"
  arn       = aws_lambda_function.slack_notify.arn
}
```


```h
// # DynamoDB Throttling → Event → AutoScaling 정책 증가
// CloudWatch Alarm → EventBridge → Application Autoscaling 정책 증가
resource "aws_cloudwatch_metric_alarm" "dynamodb_throttling" {
  alarm_name          = "dynamodb-throttling-alarm"
  metric_name         = "ReadThrottleEvents"
  namespace           = "AWS/DynamoDB"
  statistic           = "Sum"
  period              = 60
  evaluation_periods  = 1
  threshold           = 1
  comparison_operator = "GreaterThanThreshold"

  dimensions = {
    TableName = "my-table"
  }

  alarm_actions = [aws_cloudwatch_event_rule.throttle_event.arn]
}

// EOF를 넣은 이유는 json같은 값을 멀티라인으로 넣기 위해서 해당 표현식을 사용함.
resource "aws_cloudwatch_event_rule" "throttle_event" {
  name        = "dynamodb-throttle-detected"
  description = "Detect DynamoDB throttling"
  event_pattern = <<EOF
{
  "source": ["aws.cloudwatch"],
  "detail-type": ["CloudWatch Alarm State Change"]
}
EOF
}

resource "aws_cloudwatch_event_target" "increase_scaling" {
  rule      = aws_cloudwatch_event_rule.throttle_event.name
  target_id = "ScalingAction"
  arn       = aws_appautoscaling_policy.dynamodb_scale_up.arn
}

resource "aws_appautoscaling_policy" "dynamodb_scale_up" {
  name               = "scale-up-dynamodb"
  policy_type        = "StepScaling"
  resource_id        = "table/my-table"
  scalable_dimension = "dynamodb:table:ReadCapacityUnits"
  service_namespace  = "dynamodb"

  step_scaling_policy_configuration {
    adjustment_type = "ChangeInCapacity"
    step_adjustment {
      scaling_adjustment = 10
      metric_interval_lower_bound = 0
    }
  }
}
```

```h
// 매일 09:00 → EventBridge Scheduler → Lambda 실행
resource "aws_cloudwatch_event_rule" "daily_schedule" {
  name        = "daily-09"
  description = "Run every day at 09:00"
  schedule_expression = "cron(0 0 9 * * ? *)"
}

resource "aws_lambda_function" "daily_job" {
  function_name = "daily-job-run"
  role          = aws_iam_role.lambda_role.arn
  runtime       = "python3.9"
  handler       = "index.handler"
  filename      = "daily_job.zip"
}

resource "aws_lambda_permission" "daily_permission" {
  statement_id  = "AllowExecutionFromEventBridge"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.daily_job.function_name
  principal     = "events.amazonaws.com"
  source_arn    = aws_cloudwatch_event_rule.daily_schedule.arn
}

resource "aws_cloudwatch_event_target" "daily_target" {
  rule      = aws_cloudwatch_event_rule.daily_schedule.name
  target_id = "DailyLambda"
  arn       = aws_lambda_function.daily_job.arn
}
```

### evaluation period

알림 조건을 몇번 기간동안 관찰해서 판단할 것이간에 대한것이다.

CloudWatch는 알림이 트리거되기 전에 지정한 기간동안 메트릭이 threshold를 넘었는지 반복해서 평가하는 횟수다

왜냐면 spike 트래픽때문에 딱 한번만 값이 튀는것때문에 리소스를 확장시킨다면 이는, 불필요한 낭비와 처리연산을 진행하기 때문에. 이런 임계치를 두는것이다.

CloudWatch에는 주요 파라미터들이 있는데.
- Period: metric sampling 간격
- Evaluation Period: 몇 개의 Period 동안 상태를 확인할지
- Threshold: 임계값
- Comparison Operator: threshold 비교 방식

이 네가지 조합으로 알람이 발생ㅎ나다.

evaluation period에 대해서 더 알아보자면 다음과 같이 설정했다고 가정먼저해보자.
1. Metric: CPUUtilization
2. Threshold > 80%
3. Period: 60s
4. Evaluation Period: 5

이 의미는 60초 단위로 CPU를 측정하는데 80%를 초과하는 시간이 5분 (5 x 60s)이 지나야 알람을 발생시킨다 라는 뜻이다.

이렇게 제약을 걸어둬야 스파이크(일시적 튐) 때문에 알람이 오면 안된다ㅏ.

예를들어 cpu가 잠깐 한 무거운 테스크덕에 90%찍고 바로 내려가면? 이런 상황에서는 알람이 지속적으로 울릴 필요가 없고 불필요한 장애대응과 리소스 낭비가 있게 된다.
