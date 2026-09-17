# CloudWatch

CloudWatch is a service that allows you to collect, store, visualize, alarm, and automatically respond to the status, performance, and abnormal signs of AWS resources, all in one place.

- Infrastructure metrics like CPU, memory, disk, and network
- Lambda concurrency, error rates, execution time
- API Gateway 4xx/5xx, latency
- RDS connection count, slow queries
- Application business metrics (e.g., number of payment failures, login error rate)

All of this can be integrated and managed within a single observability system.

In practice, without it, the following problems arise:

- Failure signs are not detected early
- Resource bottlenecks go unnoticed
- Root cause analysis (troubleshooting) takes a long time
- Infrastructure adjustments are made based on guesswork -> increased cost + increased risk of failure

Therefore, CloudWatch can be seen as the foundation of observability in AWS operations.

## What Problems Does CloudWatch Solve in Practice?

Let's explore what problems it solves in a real production environment, broken down by area.

### Early Detection of Application Failures

- API Gateway 5xx errors suddenly spike
- Lambda error rates increase
- DynamoDB throttling spikes
- Latency skyrockets for only one instance in an ALB Target group

If CloudWatch alarms are set, they are immediately propagated to the team via Slack/Email/SNS,

or even trigger automatic recovery (Lambda trigger).

ex:
- OOMKill occurs in a specific container -> CloudWatch Events -> Lambda -> ECS Task restart
- DynamoDB throttling detected -> Lambda -> Immediate increase in Auto Scaling policy

### Performance Bottleneck Identification and Trend Analysis

- Lambda execution time increases at the same time every week
- RDS CPU stuck at 80%
- API latency gradually increases
- ECS CPU Limit is hit
- NAT costs surge due to increased network egress

Root cause analysis is possible with CloudWatch Metrics + Logs + Insight.

> The three components above – Metrics + Logs + Insight – are three distinct pillars that constitute Observability.
> - Metrics: Numerical time-series data in CloudWatch, represented by values like CPUUtilization = 34%, Lambda Duration = 120ms, API Gateway 5xx = 3, RDS Connection = 120, NetworkIn = 10MB/s
> - Log: Stores raw logs, such as Lambda execution logs, API Gateway access logs, container logs, VPC flow logs, etc.
> - Insight (Log Insight): A feature that allows high-speed analysis of logs stored in CloudWatch Logs using a SQL-like query language. In other words, it's a powerful feature that goes beyond simple log searching, enabling real-time aggregation, statistics, and pattern analysis.
>
> ```
> fields @timestamp, @message
> | filter @message like /ERROR/
> | stats count(*) by bin(1m)
> ```
> Error log aggregation

### Cost Optimization

Using CloudWatch effectively can cut costs by half.

Example:
- Lambda duration spikes only in certain periods -> insufficient memory configuration.
- NAT Gateway data processing costs surge -> caused by specific API traffic
- Excessive RDS IOPS consumption -> due to a batch job running at a specific time..
- Over-provisioned Auto Scaling leading to EC2 cost leakage

Such patterns cannot be detected without CloudWatch.

### Justification for Auto Scaling

HPA and AWS Auto Scaling Groups can be configured to operate based on CloudWatch metrics, and this is a common practice.

To view CloudWatch in EKS, you can install and use `aws-cloudwatch-metrics-adapter`.

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

In other words, CloudWatch provides the basis for:
- Under what conditions to scale out?
- What is the 60% CPU threshold?
- What are the traffic patterns?

It provides the basis for these decisions.

### SLA/SLO Monitoring

Important metrics in practice:
- API error ratio
- p95 latency
- Request success rate
- Lambda timeout rate
- RDS deadlock frequency

These are visualized using CloudWatch dashboards/Zabbix/Grafana.

### Alarms

- Metric-based judgment -> notification -> execution of Slack, SMS, Lambda, etc.
  - Lambda ErrorRate > 2% -> Alarm
  - RDS FreeStorage < 10GB -> Alarm
  - API 5xx increase -> Alarm
  - EC2 CPU > 80% for 5 mins -> scale out trigger

### Events / EventBridge (Automated Response)

In CloudWatch, an Event is not just a simple notification.

**It captures system state changes and automatically triggers other AWS resources.**

- EC2 abnormal termination -> Event -> Lambda -> Slack notification
- DynamoDB throttling -> Event -> Auto Scaling policy change
- Daily 09:00 -> EventBridge Scheduler -> Lambda execution

Let's briefly write down cases 1, 2, and 3 using Terraform.

```h
// # EC2 Abnormal Termination → Event → Lambda → Slack Notification
// EventBridge Rule: Detect EC2 state changes
// Lambda: Invoke Slack Webhook
Lambda: Invoke Slack Webhook
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
// # DynamoDB Throttling → Event → Auto Scaling Policy Increase
// CloudWatch Alarm → EventBridge → Application Auto Scaling Policy Increase
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

// The reason for using EOF is to allow multi-line values like JSON.
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
// Daily 09:00 → EventBridge Scheduler → Lambda Execution
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

### Evaluation Period

This refers to how many periods the alarm conditions will be observed before making a judgment.

CloudWatch repeatedly evaluates whether a metric has exceeded a threshold for a specified period before an alarm is triggered.

This is because if resources are scaled out due to a single spike in traffic, it leads to unnecessary waste and processing. Therefore, such thresholds are set.

CloudWatch has key parameters:
- Period: Metric sampling interval
- Evaluation Period: How many Periods to observe the state
- Threshold: The threshold value
- Comparison Operator: How the threshold is compared

Alarms are triggered by a combination of these four.

To further understand the evaluation period, let's first assume the following settings:
1. Metric: CPUUtilization
2. Threshold > 80%
3. Period: 60s
4. Evaluation Period: 5

This means that the CPU is measured every 60 seconds, and an alarm will only be triggered if the time exceeding 80% lasts for 5 minutes (5 x 60s).

This constraint is necessary to prevent alarms from being triggered by spikes (temporary surges).

For example, if the CPU briefly hits 90% due to a heavy task and then immediately drops, there's no need for a continuous alarm in such a situation, and it would lead to unnecessary incident response and resource waste.
