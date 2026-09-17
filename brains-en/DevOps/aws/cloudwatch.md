# CloudWatch vs CloudTrail

## CloudWatch

It is a monitoring and observability service built for DevOps engineers, developers, SREs, and IT managers.

It provides the data and actionable insights needed to monitor applications, respond to performance changes across the system, optimize resource utilization, and gain a unified view of operational status.

### Features
A monitoring service for all AWS services provided by AWS: performance check.

It primarily offers three functions.
- Logs
- Alarms
- Events

It also provides other features such as dashboards for logs.

### CloudWatch Logs

It collects, stores, and delivers logs from within and outside AWS to users.

It provides monitoring (logs, metrics, etc.) for key services.

EC2, Autoscaling Groups, ELB, Route 53, CloudFront, EBS, Storage Gateway, etc.

It records the output results of major services. (e.g., Lambda)

Users can create log groups directly to ingest logs from external sources.
- Primarily used to store and utilize on-premise logs.

It's possible to leverage Insights for analyzing logs in query format.

### CloudWatch Events

Rules that detect events at regular intervals or various AWS events and invoke other AWS services (e.g., SNS, Lambda).
- Same as EventBridge rules.

Events can be generated at regular intervals.
- Example: Analyze logs accumulated over the day every hour on the hour.

Captures and generates various AWS events.

![](https://user-images.githubusercontent.com/28394879/141958237-d8fb0542-f5c1-4e32-85a8-20e24ff78097.png)

### CloudWatch Alarms

Alarms are triggered based on conditions for specific metrics, which are generated from logs.
- Alarms can invoke other services (via SNS).

Examples
- If CPU usage exceeds a certain level~
- If an error occurs in an invoked Lambda~

<br>

## CloudTrail

It is a service that supports governance, compliance, operational auditing, and risk auditing for your AWS account.

CloudTrail allows you to record, continuously monitor, and retain account activity related to actions across your AWS infrastructure.

### CloudTrail Features

A service for AWS security and auditing: surveillance.

It provides API usage logs for various services.
- Records API call time, results, errors, and authentication information used.
- For S3 and Lambda, separate activation is required.
- All events, including AWS CLI, console usage, and API calls, are targets.
  - For some data APIs, manual activation is required (e.g., S3, Lambda, DynamoDB).

<br>

## CloudTrail vs CloudWatch

**CloudTrail is a service for auditing AWS. (Surveillance CCTV)**

Stores usage logs every time any AWS service is used.

When, where, and by whom was AWS used?

Simply stores AWS usage logs.

**CloudWatch is a service for monitoring AWS. (Performance Check)**

Collects not only AWS service logs but also application logs and operational logs.

How did the application operate? What was the bug? How much memory was consumed?

Provides services for monitoring, such as dashboards and alarms.
