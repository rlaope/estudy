# IP/IP Packet, Packet Delivery, Limitations

![](https://velog.velcdn.com/images/mmmdo21/post/86e48148-c89b-4c89-a23b-63b94ebbcfb5/image.png)

How can clients and servers communicate through numerous nodes in a complex internet network? (Here, a node refers to a single computer)
  
Rules are necessary for data to be successfully delivered from the source to the destination.
  
Therefore, computers are assigned what is commonly known as an IP (Internet Protocol) address, which is then used for communication.
  
IP delivers data to a specified IP (IP Address) using a communication unit called a `packet`.

## IP Packet Information

![](https://velog.velcdn.com/images/mmmdo21/post/6bca6ee8-07a2-4b44-af75-880d22880145/image.png)

An IP packet can be compared to a parcel, as the word "packet" combines "pack" and "bucket."
  
Like a postal waybill, an IP packet contains information such as the source IP and destination IP to ensure the successful transmission of data.

## Client Packet Delivery - Server Packet Delivery

When data is transmitted in packet units, nodes forward data to each other to reach the destination IP.

![](https://velog.velcdn.com/images/mmmdo21/post/7463ee3b-4eb1-447c-9d69-36337ebb4c49/image.png)

If the server successfully receives the data, it must also send a response.
  
The server also delivers its response to the client using IP packets.

![](https://velog.velcdn.com/images/mmmdo21/post/7e4dde6d-c554-4f33-b2a9-8369a2885bd3/image.png)

## Limitations of the IP Protocol

### Connectionless
Packet transmission even if the recipient is unavailable or the service is down.
  
Since the client has no way to ascertain the server's status, it transmits packets regardless.

### Unreliable
Packets can be lost in transit.
  
Even if an intermediate server experiences a failure during data transmission and packets are lost, the client has no way of knowing.
  
Packet order cannot be guaranteed.

![](https://velog.velcdn.com/images/mmmdo21/post/4351e802-7d18-4a15-adf0-659c94602c1a/image.png)

If the data to be transmitted is large, it is divided into packets for delivery. These packets can then be transmitted through different intermediate nodes. This can result in packets arriving at the server in an order unintended by the client.
