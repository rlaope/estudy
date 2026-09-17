# EBS, Instance Storage, AMI

## EBS
`Elastic Block Store`

Amazon Elastic Block Store (EBS) provides persistent block storage volumes for use with Amazon EC2 instances in the AWS cloud.

Each Amazon EBS volume is automatically replicated within its Availability Zone to protect against component failure, providing high availability and durability.

Amazon EBS volumes deliver the low-latency, consistent performance needed to run your workloads.

With Amazon EBS, you can scale usage up or down in just minutes, and you only pay for what you provision.

![](https://user-images.github.com/28394879/136925202-f5785c89-9377-43ee-8fc2-45bbb47e424d.png)

- `EBS Based`: Allows storage of semi-persistent files.
  - Snapshots possible.
  - Instance upgrades possible.
  - Can be stopped.
- `Instance Store`: A highly volatile method.
  - Fast, but for cases where storage is not needed.
  - Cannot be stopped.

## AMI

Amazon Machine Image

An Amazon Machine Image (AMI) provides the information required to launch an instance.

You must specify an AMI when you launch an instance.

If you need multiple instances with the same configuration, you can launch multiple instances from a single AMI.

If you need instances with different configurations, you can launch instances using various AMIs.

### 특징
![](https://user-images.githubusercontent.com/28394879/136926299-e8917a9f-404e-4a96-b485-c6722d608950.png)

AMI includes the following:
1. One or more EBS snapshots, or for instance store-backed AMIs, a template for the instance's root volume (e.g., operating system, application server, applications).
2. Launch permissions that control which AWS accounts can use the AMI to launch instances.
3. A block device mapping that specifies the volumes to attach to the instance when it is launched.
