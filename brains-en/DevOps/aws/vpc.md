# VPC

## What is VPC

Amazon Virtual Private Cloud (VPC) allows you to provision a logically isolated space in the cloud to launch AWS resources in a virtual network that you define.

You have complete control over your virtual networking environment, including selecting your own IP address range, creating subnets, and configuring route tables and network gateways.

You can securely and easily access resources and applications using both IPv4 and IPv6 in your VPC.

## Types of VPC

### Default VPC
- Automatically set up when an account is created (in all regions)
- All subnets can access the internet
- EC2 instances have both public and private IPs
- Cannot be recovered if deleted

### Custom VPC
- Must be created anew.
- Does not have the characteristics of a Default VPC.

### What you can do with VPC
- Can run EC2 instances
- Can configure subnets
- Can configure security settings (e.g., IP blocking, configuring EC2 instances not exposed to the internet)

### VPC Peering: Connecting VPCs
- Transitive Peering is not possible: being connected indirectly does not mean peering is established.

### VPC Flow log
- VPC logs can be stored in CloudWatch

### IP range can be specified
- One per region: cannot be extended to other regions

<br>

## Components of a VPC
![](https://user-images.githubusercontent.com/28394879/141058705-4ac55134-69e5-441a-b1ba-3b4f71c90e28.png)

1. Availability Zone
2. Subnet
3. Internet Gateway
4. Network Access Control List/Security Group
5. Route Table
6. Network Address Translation Instance/NAT Gateway
7. Bastion Host
8. VPC Endpoint

### Availability Zone
A data center where physically separated infrastructure is gathered.

Always located a certain distance apart for high availability.

A single region consists of two or more AZs.
- AZ-A in Account 1 is in a different location from AZ-A in Account 2.

![](https://user-images.githubusercontent.com/28394879/141059214-0bf68399-1fb8-4a6e-9b75-83d29d2cb893.png)

### Subnet
A subdivision of a VPC

Can only be created in a single AZ -> cannot be extended to other AZs
- Multiple subnets can be created in a single AZ

**Private Subnet**: A subnet that cannot access the internet

**Public Subnet**: A subnet that can access the internet

CIDR block range can be configured

> CIDR: Classless Inter-Domain Routing

### Internet Gateway (IGW)
A path to the internet

High availability is ensured

Subnets not connected to an IGW = Private Subnet

Must be connected in the Route Table

### NACL/Security Group
Checkpoint

NACL -> Stateless, SG -> Stateful

Automatically created when a VPC is created

Deny is only possible with NACLs

### Route Table
![](https://user-images.githubusercontent.com/28394879/141064479-4e31b75a-e564-40a1-8574-306f150a2def.png)
A signpost that tells traffic where to go

Automatically created when a VPC is created.

### NAT Instance/NAT Gateway

![](https://user-images.githubusercontent.com/28394879/141087041-97f1c809-eb26-4950-88ea-5b900c7637e6.png)

A channel for Private Instances to communicate with the external internet

A NAT Instance is a single instance / a NAT Gateway is a service provided by AWS

When using a NAT Instance, Source/Destination Check must be disabled.

A NAT Instance must be in a Public Subnet

### Bastion Host
![](https://user-images.githubusercontent.com/28394879/141087702-b6cdb535-04ea-4c4a-8624-3cf642539183.png)

An instance used to access Private Instances

Must be located in a Public Subnet

### VPC Endpoint
VPC endpoints allow you to privately connect to AWS services powered by AWS PrivateLink and VPC endpoint services without requiring an internet gateway, NAT device, VPN connection, or AWS Direct Connect connection.

Instances in your VPC do not require public IP addresses to communicate with resources in the service.

Traffic between your VPC and other services does not leave the Amazon network.

### VPC Endpoint Types

![](https://user-images.githubusercontent.com/28394879/141247694-0059b7ae-aa55-4f35-96e1-03e15351161e.png)

- Interface Endpoint: Based on ENI (Elastic Network Interface)
  - Creates a private IP to connect to the service
  - Supports many services such as SQS, SNS, Kinesis, Sagemaker, etc.

![](https://user-images.githubusercontent.com/28394879/141247612-e2e9ef57-147f-4889-81c2-c4b2d6ab6293.png)

- Gateway Endpoint: Used by specifying it as a target for routes in the route table
  - Supports S3, DynamoDB
