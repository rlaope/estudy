# NAT, NAPT, and AWS NAT Gateway

## NAT

In most networks, only some of the hosts perform internet communication.

Therefore, since most hosts communicate using private IPs, using a public IP only when performing internet communication can significantly reduce the number of public IPs exposed externally.

When a request is sent from a private IP to the internet and passes through a router providing NAT,

**The NAT router performs IP translation, converting the private IP it has in its address translation table to a public IP, sends the request, and records the translation details in the NAT translation table.**

Subsequently, when a response sent to the internet arrives, it refers to the recorded NAT translation table and returns the response to the host with the private IP that sent the request.

## NAPT

In a network, multiple hosts are assigned unique private IPs. When a request is sent to the internet and passes through NAT for conversion to a public IP, the NAT translation table records the private IP (request address) and the public IP (translated address) converted by NAT.

While private IPs are unique to each host, the translated public IP can be the same as it is the representative public IP of the network.

Therefore, to distinguish the private IP that sent the request when a response returns from the internet to NAT, different ports are assigned to each public IP and private IP in the translation table.

This is called `Network Address Port Translation` (NAPT).

## AWS NAT Gateway

A NAT Gateway is a NAT (Network Address Translation) service provided by AWS.

Similar to the concept of NAT, you can use a NAT Gateway to enable instances in a private subnet to connect to services outside the VPC (the internet) while preventing external services from initiating connections to instances within the private subnet.

In other words, it is used to allow EC2 instances within a private subnet to access the internet and AWS Services, while blocking external access to those EC2 instances.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FCua9B%2FbtrM6LovD21%2FZuVRzklBuYOVtupjqNjZ21%2Fimg.png)
