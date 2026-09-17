# SSL (Security Socket Layer), TLS (Transfer Layer Security)

## SSL

Security Socket Layer

An internet communication protocol developed by Netscape to enable secure transactions for information security transmitted over the internet.
  
A protocol deprecated in the mid-90s.

The SSL layer is located between the application layer and the transport layer.

1. Application layer data is passed to the SSL layer.
2. The SSL layer performs encryption on the data received from the application layer.
3. A self-encryption information header called an SSL Header (SH) is added to the encrypted data.
4. SSL layer data becomes input for the transport layer (TCP/UDP).

### Decrypting an SSL certificate is also included as an encryption element.

What is an SSL certificate?  
Simply put, it's like a 'genuine authentication' that proves whether the site a user visits is trustworthy.
  
The CA bundles the public key provided by the server operating company with information such as the certificate issuer and the CA's name, encrypts it with the CA's private key, and issues it as an SSL certificate.

## TLS
![](https://velog.velcdn.com/images/sweet_sumin/post/2f0b2f3a-df2d-4098-9313-88f1c999c337/image.png)

Transfer Layer Security

It is a protocol created by the IETF based on SSL 3.0.

It is a subsequent internet standard with enhanced security and privacy features.

TLS is not a single protocol but a protocol spanning two layers.

I checked my current browser version and confirmed it is TLS 1.3.

![](https://velog.velcdn.com/images/sweet_sumin/post/14ea84f8-4ed0-4256-a450-42c5b1bf47a9/image.png)

## SSL, TLS Encryption Methods

In conclusion, **SSL and TLS encryption use both shared key and public key methods.**

An electronic certificate contains information about the CA, the server, a public key, and a digital signature.

1. The client requests a connection to the server.
2. The server sends an electronic certificate to the client, which can prove its identity and encrypt data between them.
   1. Although there are several methods, the private key is assumed to be provided by the CA.
   2. That is, by the server sending the electronic certificate to the client, both the client and the server simultaneously possess the public key.
3. The client generates a random number and encrypts it using the public key. This encrypted content is the shared key.
4. The shared key is sent to the server. From the next communication onwards, only data encrypted with the shared key is exchanged.

This method is used because, except for the very first time the shared key is sent to the server (when there is no shared key yet) and it might be intercepted, subsequent communications only exchange data, thus maintaining a certain level of security while also leveraging the advantage of fast communication inherent to shared keys.

Calculating the public key and random number to create the shared key is resource-intensive, so it is only performed during the initial communication.

Here, the shared key is a symmetric key, and the public key is an asymmetric key.
