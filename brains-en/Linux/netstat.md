# netstat

![](https://miro.medium.com/v2/resize:fit:1400/format:webp/1*hsoFihOUE1X6wGDDjPhN3g.png)

You can check network connection information on the server.

Since the states of the TCP 3-way handshake and TCP 4-way handshake processes are displayed, it's crucial to understand them well.

You will most frequently encounter the LISTEN, TIME_WAIT, and ESTABLISHED states, and each state can be interpreted as follows.

- LISTEN: The state where a process is listening for requests through a socket.
- ESTABLISHED: The state where a connection has been established.
- TIME_WAIT: The state where a connection has been terminated, and the active closer (the side that initiated the disconnection) waits briefly before cleaning up the socket.
    - This is to allow the active closer to resend the last ACK packet gracefully to terminate the connection if it gets lost during the FIN process of the 4-way handshake. (If the last ACK is lost, the passive closer, not having received the ACK, resends the previously sent FIN. Since the active closer is in the TIME_WAIT state, it can receive the FIN again before cleaning up the socket. The active closer can then send an ACK to gracefully terminate the connection.)

keepalive-timeout: One of the settings in Nginx that maintains connections to prevent network performance degradation that would occur if a TCP 3-way handshake were performed every time.

In HTTP/1.0, this was implemented by explicitly adding a `Connection: keep-alive` header, but from 1.1 onwards, the concept of persistent connections was introduced into the specification itself, so no separate header needs to be added.

**CLOSE_WAIT**: This is the state after receiving a FIN packet and before creating an ACK request. If this state persists, it indicates a situation where the application cannot generate a response, requiring root cause analysis and corrective action.

In summary, you can check network connection status using the `netstat` command.

You can check endpoint IP and port information for connections, as well as socket states like LISTEN, ESTABLISHED, and TIME_WAIT.

The CLOSE_WAIT state should be interpreted as a situation where the application cannot clean up the socket and fails to generate a response, thus requiring identification of the cause and corrective action.
