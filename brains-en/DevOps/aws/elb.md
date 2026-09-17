# AWS ELB (Elastic Load Balancer), Sticky Session

`Elastic Load Balancing` automatically distributes incoming application traffic to multiple targets, such as Amazon EC2 instances, containers, IP addresses, and Lambda functions.  
   
It can handle varying application loads in a single availability zone or across multiple availability zones.
  
All three load balancers provided by Elastic Load Balancing offer the **high availability**, **automatic scaling**, and **robust security** required for application fault tolerance.

![](https://user-images.githubusercontent.com/28394879/137287365-896396b6-3eca-4894-afd1-6c1750340e39.png)

## Vertical Scale

![](https://user-images.githubusercontent.com/28394879/137288267-d5a5a8b8-4907-44d1-9bcd-12a13be07bf1.png)

![](https://user-images.githubusercontent.com/28394879/137288362-b1ce0bd9-4d9a-44b2-a8ee-ed6b544915a2.png)

## Horizontal Scale

![](https://user-images.githubusercontent.com/28394879/137288510-7d1db362-be70-4dce-b59e-9c5bfe216d33.png)

![](https://user-images.githubusercontent.com/28394879/137288634-1ed44f9c-a25e-41b1-a928-b698d694d0c2.png)

### ELB Features

IP changes continuously
- IP addresses change continuously.
- Therefore, it must be used based on a domain.
  
Health Check
- It checks if an instance is alive by directly generating traffic.
- It is divided into two states: InService and OutService.

  
Three types
- Application Load Balancer
- Network Load Balancer
- Classic Load Balancer

### Application Load Balancer
It operates at the Application Level; you can think of it as the 'smart one'.

### Network Load Balancer
You can think of it as the 'fast one'. Elastic IP allocation is possible.

### Classic Load Balancer
You can think of it as the 'old one'. It's not commonly used nowadays.

## Sticky Session

![](https://user-images.githubusercontent.com/28394879/137290519-58ba8dec-02b3-400a-8973-20412d1fcc0b.png)

When you have two or more instances, if you log in to the web server of Instance A, a session will be issued.  
  
However, if you make another request, it might be directed to the web server of Instance B, which would then ask you to log in again without a session.  
  
To prevent this, Sticky Session was introduced.  
  
Sticky Session **stores which instance each user accessed and ensures that subsequent requests are directed to that same instance.**
