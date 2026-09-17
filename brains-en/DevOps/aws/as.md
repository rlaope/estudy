# Auto Scaling

## Auto Scaling
Monitors applications and automatically adjusts capacity to maintain stable, predictable performance at the lowest possible cost.

With AWS Auto Scaling, you can easily set up application scaling across multiple services and resources in minutes.

![](https://user-images.githubusercontent.com/28394879/137873154-df8c7c15-d8a4-4c0c-9d71-2387331edfd4.png)

## Uses of Auto Scaling
Use a minimum number of instances

Maintain the desired number of instances as a target

Keep instances below the maximum instance count

Distribute instances evenly across Availability Zones

Ensure instances are always available to maintain service

### EC2 Auto Scaling Configuration
- Launch Configuration: What and how to launch?
  - EC2 type, size
  - AMI
  - Security Group, Key, IAM
  - User Data
- Monitoring: When to launch? + Status check
  - Example: Launch additional instances when CPU utilization exceeds a certain percentage, or when one EC2 instance dies in a stack requiring two or more.
  - Integrate with CloudWatch (AND/OR) ELB
- Desired Capacity: How many to launch?
  - Example: Minimum 1 ~ Maximum 3
- Lifecycle Hook: Callback on instance launch/termination
  - Can perform pre/post-processing in conjunction with other services -> CloudWatch Event / SNS / SQS
  - Transition to Terminating:wait/Terminating:Proceed state
  - Waits for 3600 seconds by default (allowing tasks like image backup or log backup during this time)

## EC2 Auto Scaling Flowchart

![](https://user-images.githubusercontent.com/28394879/137876824-8fb023db-f32b-4959-93c4-a1c930bf792f.png)
