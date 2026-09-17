# AWS Security Group

## Security Group

![](https://user-images.githubusercontent.com/28394879/136934211-085a6871-2ff6-423d-b208-88e55945c45f.png)

A security group acts as a virtual firewall that controls inbound and outbound traffic for your instance.
  
When you launch an instance in a VPC, you can assign the instance to a maximum of five security groups.
  
Security groups operate at the **instance level, not the subnet level**, so each instance in a subnet in your VPC can be assigned to a different set of security groups.
  
If you don't specify a particular group when you launch an instance, the instance is automatically assigned to your VPC's default security group.

### Features

- Security Device
  - A service that acts as a firewall alongside Network Access Lists (NACLs).
- Port Allowance
  - You can specify the Port and Source through which traffic can pass.
  - Denial is not possible -> Possible with NACLs
- Instance-level
  - One or more SGs can be configured for a single instance.
  - NACLs operate at the subnet level.
  - Configured instances are subject to all rules of the configured SGs.

![](https://user-images.githubusercontent.com/28394879/136935520-e5b45cb7-28e1-48e6-863f-02572c399284.png)

- Filters using all configured rules.
  - In the case of NACLs, filtering occurs in the order of applied rules.
- Stateful
  - Traffic that enters via Inbound can exit without any additional Outbound configuration.
  - NACLs are Stateless.

![](https://user-images.githubusercontent.com/28394879/136936510-05742607-16dc-4031-b1b4-972aa13cad5e.png)
