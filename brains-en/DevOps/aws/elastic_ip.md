# Elastic IP

## ENI
Elastic Network Interface

- Mac address
- Source/destination check
- One or more security groups
- One primary private IPv4 address
- One or more secondary private IPv4 addresses
- One or more IPv6 addresses
- One public IPv4 address

Multiple ENIs can be attached to an EC2 instance. (The number that can be attached varies depending on the EC2 instance type)
The public IP assigned to an ENI changes dynamically.
An Elastic IP Address is a service that allows you to attach a static public IP to an ENI.

![](https://user-images.githubusercontent.com/28394879/141428277-ea3223bc-86c7-48a0-90c7-b7c498d5dd75.png)

## Elastic IP
Usage is free

Charges apply only when not in use or not attached to an ENI

It can be used not only for EC2 but also to assign static IPs to Network Load Balancers or NAT Gateways.

![](https://user-images.githubusercontent.com/28394879/141428421-e322e5da-2641-4d79-a130-1a5a480bb122.png)
