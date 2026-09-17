# How the Internet Works

> The Internet refers to a `computer network` that allows individual computers containing information to exchange data with each other using a communication protocol called `TCP/IP`.

## The Internet
- If you break down the word "Internet" into "Inter-net", it means an `internal network`. The dictionary definition of the Internet is 'a communication network connecting computers'. For one or more computers to communicate, a connection between them is necessary, and the vast network formed through these connections is the Internet. The TCP/IP communication protocol is used for communication.

<br>

## What is TCP/IP?
> TCP (Transmission Control Protocol)/IP(Internet Protocol)

TCP/IP, a communication protocol for inter-computer communication, is a set of communication rules designed to enable smooth communication in local area networks (LANs) and wide area networks (WANs).  
  
It is characterized by its openness, meaning it can operate independently of hardware, operating systems, and connection media, and this openness is why it's used for Internet communication.

<br>

## Network

- **Direct or Wireless Connection Between Computers via Cable**  
It would be possible to communicate through direct connections between computers. However, if the number of computers increases, too many cable connections would be required, quickly reaching a physically impossible number.  
  
- **Connection via Router**  
In this case, if computers are indirectly connected using a `router`, the number of cables used is significantly reduced, and countless computers can be connected through router-to-router connections. However, even in this scenario, communication with computers at distances too far for cable connections remains difficult.  
  
- **Connection via Modem**  
Since telephone infrastructure is connected everywhere in the world, a network was already established, and a special device called a `modem` was created to connect this network to computers. The Internet usually comes through telephone lines, and a modem is a device that allows a computer to communicate with an Internet service provider via that telephone line.  
  
- **Using Internet Service via ISP**  
The network connected to the telephone infrastructure connects to an `Internet Service Provider` via a modem. ISPs like KT, SKT, and LG U+ are companies that manage specialized routers and can access routers of other ISPs. Through the ISP network, you connect to the network you wish to reach.  
  
The entire network infrastructure is built through this communication, where ISPs relay data in between.

<br>

![ISP](./image/ISP.png);

<br>

## IP Address
Through the expansion of computers - routers - modems - ISPs, an environment enabling inter-computer communication has been established. At this point, computers need an address to recognize each other. This is called an IP address, and it consists of numbers separated by three dots, like 173.194.121.32. However, since these IP addresses are difficult to remember every time, domain addresses like 'naver.com' are used.

<br>

## The Internet and the Web
The Internet is the technical infrastructure that connects billions of computers. The Web is a service that provides services consisting of web browsers and web servers atop this established technical infrastructure. Besides the Web, there are other services built on the Internet.
