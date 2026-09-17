# OSI 7 Layers and TCP/IP 4 Layers

Network protocol layers can be divided into OSI 7 layers and TCP/IP 4 layers as follows:

![](https://velog.velcdn.com/images/mmmdo21/post/b22a8105-4e26-4d76-b454-0519fdcc6393/image.png)

Since the TCP protocol exists at a higher layer than the IP protocol, it can compensate for the limitations of the IP protocol discussed earlier.

> The TCP/IP 4-layer model was developed before the OSI 7-layer model, and the layers of the TCP/IP protocol do not exactly match those of the OSI model. Actual network standards are closer to the industry-standard TCP/IP 4-layer model.

### Example) What happens when sending a message in a chat program

![](https://velog.velcdn.com/images/mmmdo21/post/73fd398a-24a2-46a8-b7b7-89b64a20bb47/image.png)

1. Message creation in the chat window
2. Once an HTTP message is created, it is transmitted via a Socket.

> A socket is a connection point created to allow a program to connect to a network environment for sending and receiving data over the network. (Socket)

3. TCP segment creation, including message data
4. IP packet creation, including TCP data
5. The created TCP/IP packet is encapsulated within an Ethernet framework and transmitted to the server to pass through a physical layer, such as a LAN card.
