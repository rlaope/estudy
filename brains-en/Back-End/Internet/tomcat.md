# Apache Tomcat (Apache Tomcat)

### What is Apache Tomcat ?

#### Server, Web Server
The web pages we use are built with Apache Tomcat. When creating a Linux server or a web server, you're often told to install Apache Tomcat. It's also important to understand the functions and differences between Apache and Tomcat.

### Apache (Apache)
- An `open-source project` of the software foundation, also known as a web server.
- Used for static web pages that respond only when a client request comes in.
  - Responds only when a client request (POST, GET, DELETE) comes to web server port 80.
  - Processes only static data (e.g., html, css, images).

### Tomcat (Tomcat)
- Called a web container or servlet container for creating dynamic web pages.
- Data that needs dynamic processing, such as JSP, ASP, and PHP, is passed to the web container (Tomcat), excluding data that should be processed statically.

<br>

## WAS(Web Application Server)
- Also called a container, web container, or servlet container.
- Processes JSP and servlets, receives and responds to HTTP requests. If only Apache is used, it processes only static web pages, so the processing speed is very fast and stable.
- However, using Tomcat (WAS) allows for dynamic data processing.
- It can connect to databases, manipulate data, and interact with other applications.
- Tomcat processes requests on port 8080.

<br>

### Apache Tomcat (Apache + Tomcat)
Since Tomcat provides some of Apache's functionalities, they are often referred to together (WAS Web Application Server).

Tomcat is generally called a WAS, and when combined with Apache, it's called Apache Tomcat.

If only Apache is used, only static web pages can be processed. If only Tomcat is used, dynamic web pages can be processed, but it cannot leverage necessary functionalities from Apache. Furthermore, if multiple users make requests, Tomcat can become overloaded. By using Apache Tomcat together, Apache handles only static data, and JSP processing is sent to the Web Container (part of Tomcat) for distributed processing.

Apache: Port 80
Tomcat: Port 8080

> However, in reality, everything is handled through port 80, so Apache automatically forwards to port 8080. It's rare to deal with or see port 8080 unless you're handling it at the Linux level or manually configuring ports.
