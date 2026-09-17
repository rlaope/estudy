# HTTP

## What is HTTP?
- HTTP is the Hypertext Transfer Protocol.
- HTTP was invented between 1989 and 1991.
- HTTP is the protocol inherent to the World Wide Web (WWW).
- HTTP is a protocol that allows data to be exchanged over the internet.
- Every time you visit a web page, your computer uses HTTP to download that page from another computer somewhere on the internet.

  > http:// ......

<br>

## What is a Protocol?
- A protocol means a rule (or agreement).
- In computer networks, when exchanging data, developing according to these rules enables information exchange.
- Protocols were created based on the nature of sending and receiving data in computer networks.
- When exchanging web documents, HTTP must be used; for files, FTP; for mail, SMTP, POP, etc. Various protocols are created depending on the transport layer and type.

<br>

## Why HTTP is Important in Web Development
- Both clients and servers communicate using HTTP, so web developers must have a good understanding of HTTP.
- HTTP knowledge is also crucial for resolving errors.

<br>

## The Invention of WWW (World Wide Web)
- WWW refers to a structure for using web pages over a network.
- WWW was invented between 1989 and 1991 for exchanging files in a laboratory.
- In WWW, data exchange is carried out based on the communication method (protocol) called HTTP.
- WWW consists of four components.
  1. HTML, a text-based language and file format for expressing hypertext documents
  2. HTTP, a protocol for exchanging documents and other data over the internet
  3. Early versions of HTTPD that allowed access to documents, i.e., servers that support HTTP (building a web server is equivalent to running HTTPD).

<br>

## Characteristics of HTTP
1. Client-Server Architecture
2. Stateless Protocol
3. Connectionless
4. HTTP Messages
5. Simplicity, Extensibility

<br>

## HTTP Connection-Oriented vs. Connectionless

### Connection-Oriented - A Model That Maintains Connections
- In the case of TCP/IP, connections are fundamentally maintained.
- In a connection-oriented model, the connection must be maintained even if the client does not send a request.
- In such cases, server resources used to maintain the connection are continuously consumed.

### Connectionless - A Model That Does Not Maintain Connections
- In connectionless HTTP, a connection is maintained only when requests are actually exchanged, and the TCP/IP connection is terminated after the response is sent.
- This allows the server to be maintained with minimal resources.

### Limitations and Overcoming Connectionlessness
- Connectionlessness has the following limitations:
  - A new TCP/IP connection must be established.
  - When a website is requested via a web browser, numerous resources such as JavaScript, CSS, and additional images are downloaded along with the HTML.

  
- Repeatedly disconnecting and reconnecting for each of these resources is inefficient, so this problem is now solved with HTTP persistent connections.
- Further optimizations in HTTP/2, HTTP/3

### Overcoming Connectionlessness - HTTP Persistent Connections
- Early HTTP - Wasted connections and disconnections
  - In early HTTP, connections and disconnections had to be repeated to download each resource.

- HTTP Persistent Connections
  - In HTTP persistent connections, after a connection is established, resources are requested, and the connection is terminated only after responses for all resources have been received.
