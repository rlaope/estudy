# SSL Handshake

SSL (also known as TLS) is an encryption-based communication protocol, and HTTPS is a concept combining HTTP + SSL.

Its main features are as follows.
- It guarantees communication between the client and server through an SSL certificate.
- It exchanges encrypted data. Mainly, symmetric-key (public key) and asymmetric-key (private key) encryption methods are used.
- The SSL communication process goes through a procedure called handshake.

### handshake

> This can be thought of as the process that occurs after connecting to https://... in a browser.

1. Client -> Server

The client generates random data and sends it to the server.

Additionally, the client also sends the encryption methods it supports to the server.

2. Server -> Client

The server also generates random data and sends it to the client.

It selects (negotiates) an encryption method from those sent by the client and sends it back to the client.

Additionally, it also sends the SSL certificate.

**The SSL certificate contains information such as the public key, certificate issuer (CA), and domain.**

3. Client -> CA

The client verifies the certificate received from the server through the CA.

This process confirms whether the server is trustworthy.

If it's a certificate issued by a recognized CA (a CA list stored in the browser), it can be decrypted with the certificate's public key.

The reason is that a proper certificate is encrypted with the trusted CA's private key. (Asymmetric key encryption)

In other words, if it can be decrypted, it **guarantees that this certificate is trustworthy.**

+Then, the browser also displays a secure connection indicator.

However, what if the certificate was issued by a private CA, not a recognized CA? A private CA will not be in the CA list stored in the browser, and the certificate will be deemed untrustworthy. This is why an HTTPS warning is displayed.

```
공인CA 인증서
- 유로
- 많은 사람들에게 오픈되는 사이트일 때 사용 
- 브라우저에 안전함 표시가 뜬다.

사설CA 인증서
- 무료 keytool, openssl등을 사용해 발급
- 브라우저에 안전하지 않음 표시가 뜬다.
- 공인 인증서에 금지된 정보를 포함할 수 있다.

자체서명 인증서
- CA없이 발급된 인증서
- 인증서 자체가 CA역할을 함
- 보안은 약하지만 사용이 쉽다.
```

4. Client
It combines the random data generated in steps 1 and 2 to create a temporary key (pre-master key).

Then, it encrypts the temporary key with the certificate's public key. (master key)

5. Client -> Server

It sends the encrypted temporary key to the server. Then, both the client and the server have this encrypted key.

6. Server
The temporary key can be decrypted with the private key stored on the server. This is because it was encrypted with the certificate's public key, making decryption possible through asymmetric key encryption.

The server and client now share these temporary keys, and after a series of steps on both the server and client, they create a final key (session key).

Afterward, encrypted data is exchanged using the session key (symmetric-key encryption method).

### Why are public-key and symmetric-key methods used together?

This is because while public-key methods are secure, they consume a lot of computing resources.

Therefore, they are used in combination with the relatively lightweight symmetric-key method.

Thus, public-key methods are typically used when establishing the initial connection, and then symmetric-key methods are used for data exchange afterward.
