# URI

### URI(Uniform Resource Identifier)
- A unified way to identify resources
- URIs can be further classified as locators, names, or both.
- URL and URN are types of URI.

![uri](../image/uri.png)

The top is URL, the bottom is URN, but URL is commonly used.

<br>

### URI Terminology

- Uniform: A unified way to identify resources
- Resource: Anything that can be identified by a URI (no limitations)
- Identifier: Information needed to distinguish from other items

### URL, URN Terminology
- URL - Locator: Specifies the location of a resource
- URN - Name: Assigns a name to a resource
- Location can change, but the name does not.
- urn:isbn:8960777331 (ISBN URN of a book)
- A method to find actual resources using only URN names is not yet widespread.
- From now on, I will use URI and URL interchangeably.

<br>

### URL Analysis
https://www.google.com/search?q=hello&hl=ko

Full syntax

scheme://[userinfo@]host[:port][/path][?query][#fragment]
https://www.google.com:443/search?q=hello&hl=ko

- Protocol (https)
- Hostname (www.google.com)
- Port number (443)
- Path (/search)
- Query parameters (q=hello&hl=ko)

#### scheme
scheme://[userinfo@]host[:port][/path][?query][#fragment]
https://www.google.com:443/search?q=hello&hl=ko
- Uses major protocols
- Protocol: A set of rules that dictates how to access a resource. E.g., http, https, ftp, etc.
- http primarily uses port 80, https primarily uses port 443; ports can be omitted.
- https adds security to http

#### userinfo
scheme://[userinfo@]host[:port][/path][?query][#fragment]
htps://www.google.com:443/search?q=hello&hl=ko

- Includes user information in the URL for authentication
- Rarely used

#### host
scheme://[userinfo@]host[:port][/path][?query][#fragment]
htps://www.google.com:443/search?q=hello&hl=ko

- Hostname
- Can use a domain name or an IP address directly

#### port

scheme://[userinfo@]host[:port][/path][?query][#fragment]
htps://www.google.com:443/search?q=hello&hl=ko

- Port (PORT)
- Connection port
- Generally omitted; if omitted, http is 80,
- https is 443

#### query
scheme://[userinfo@]host[:port][/path][?query][#fragment]
htps://www.google.com:443/search?q=hello&hl=ko
- Resource path, hierarchical structure
- E.g., /home/file1.jpg, /members, /members/100, /items/iphone12

#### fragment
scheme://[userinfo@]host[:port][/path][?query][#fragment]
https://docs.spring.io/spring-boot/docs/current/reference/html/getting-started.html#getting-started-introducing-spring-boot

- Fragment
- Used for internal HTML bookmarks, etc.
- Not information sent to the server
