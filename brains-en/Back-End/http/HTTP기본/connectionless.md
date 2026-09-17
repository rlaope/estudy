# Connectionless

Clients and servers maintain a connection only when sending and receiving requests, using minimal resources.

- HTTP is fundamentally a connectionless model.
- Typically responds at high speeds, often less than a second.
- Even if thousands of people use the service for an hour, the number of requests actually processed simultaneously by the server is very small, typically dozens or fewer. E.g., users don't continuously click the search button in a web browser.
- Allows for very efficient use of server resources.

#### Limitations and Overcoming Them
- A new TCP/IP connection must be established > adds 3-way handshake time.
- When requesting a site with a web browser, numerous resources such as HTML, JS, CSS, and images are downloaded together.
- This issue is now resolved with HTTP persistent connections.
- Further optimizations in HTTP/2 and HTTP/3.

![Connectionless](../image/connectionless.png)

#### Remember Statelessness
Tasks that server developers find challenging:

- High-volume traffic that occurs precisely at the same time.
- E.g., first-come, first-served events, holiday KTX reservations, course registration.
- E.g., a chicken discount event for the first 1000 people at 6 PM > tens of thousands of simultaneous requests.
