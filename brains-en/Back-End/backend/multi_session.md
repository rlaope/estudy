# How are sessions shared and managed in a multi-environment?


![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FdBEVdk%2FbtqESXKZqyy%2F4SA9mg3L9LwKPCbSkGm5vK%2Fimg.png)

Sessions are configured in a multi-server environment as shown below.

One session store is formed per server. As we learned last time, without separate handling for divided session stores, each session will cause consistency issues.

For multiple servers, as shown in the figure above, to operate a single service, the four separate sessions must either function as a single system or use sticky sessions.

### Sticky Session

Sticky Session literally means a fixed session.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FboFlh4%2FbtqESXc9LPa%2Fc6j2klIYPLK1ni9QAmXLUk%2Fimg.png)

For example, if user1 creates a session on server 1 among servers 1 to 3, all subsequent requests from User1 will only be sent to server 1. The load balancer redirects all requests to the server where the user first created a session, ensuring only sticky sessions are used.

To achieve this, the load balancer **first checks if a cookie exists in the request upon receiving it. If a cookie is present, the request is sent to the server specified in the cookie.** If there is no cookie, the load balancer selects a server based on its existing load balancing algorithm.

Server information is continuously inserted into the response via cookies to ensure that the same user can keep sending requests to that specific server.

This method avoids consistency issues because it uses the same server throughout the user's session.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FlaJ6c%2FbtqETl5WCNm%2FngXl4OR3XvH4HDKbThLBrK%2Fimg.png)

However, as shown in the figure below, if even one service fails, all users using that service lose their session information, leading to reduced availability.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fb8YE8A%2FbtqESWZEy7C%2FOzLcVq3gCq1462GKrWz4S0%2Fimg.png)

While sticky sessions solve consistency issues, they prevent full utilization of the benefits of scale-out, such as availability and traffic distribution.

Is there a session management method that can solve consistency issues while also ensuring availability and traffic distribution? Let's now explore session clustering methods that address these considerations.

<br>

### Session Clustering with Tomcat

Connecting multiple computers to operate as a single system is called **clustering**. Servers, being computers, also need clustering for multiple units to provide a single service.

Tomcat proposes an all-to-all replication method using `DeltaManager` for session clustering implementation.

#### all-to-all Session Replication

All-to-all session replication means that when a change occurs in one session store, the change is replicated to all sessions.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbeH7KU%2FbtqESih0Ula%2F6QwZUMToYxOh8j1LLIwYdK%2Fimg.png)

If sessions are replicated as shown in the figure, consistency issues can be resolved because login information is replicated across sessions, regardless of which server the user connects to later. This allows the service to continue operating without interruption even if one server fails. However, Tomcat's all-to-all session replication method has disadvantages that need to be considered.

First, it requires a lot of memory because all servers must hold identical session objects.

Additionally, every time data is stored in a session store, values must be entered into all servers, leading to performance degradation such as increased network traffic proportional to the number of servers.

Therefore, this method shows good efficiency for small clusters. It is not recommended for large clusters with more than 4 servers.

Is there a way to overcome the scale-out limitations that arise when using the all-to-all session replication method?

**Tomcat proposes a primary-secondary** session replication method using `BackupManager`.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FB8Hzk%2FbtqE6xL36i3%2FSUiyun3l639PQMzYvkgTTK%2Fimg.png)

As shown in the figure, the Primary server replicates the entire key-value of the session object to the Secondary (Backup) server. However, other servers only replicate the JSESSION ID corresponding to the key, which reduces memory usage compared to the all-to-all method. Replicating only the key will relatively save time compared to replicating the entire session object.

Therefore, this method is suitable for large clusters with 4 or more servers.

Nevertheless, a problem still exists: while the time taken to replicate sessions can be reduced, if a **proxy server requests session information from a server other than the primary or secondary, it must request the corresponding object for that key from the primary server again.**

Thus, while session clustering solves consistency issues, it has performance limitations. What methods can complement these two approaches to share sessions across multiple servers?

<br>

### Solving Consistency Issues During Scale-Out by Separating Session Storage

What does it mean to separate session storage? It means using a separate session store instead of the local session store that the existing server has.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FcvGZ64%2FbtqESWyzU7U%2FwPLINLrekwoSehSrteRLa0%2Fimg.png)

When session storage is separated like this, no matter how many servers are added, sessions are shared simply by providing each server with information about the session storage.

If this method is used, assuming a well-implemented load balancing algorithm, there's no need to consider abnormal traffic surges like with sticky sessions.

Furthermore, even if a server fails, the separate session store ensures continuous service provision, thus securing availability.

Moreover, it can fundamentally solve the consistency problem. Since multiple servers use a single session, data inconsistencies that previously occurred in individually held local session stores are avoided.

Most importantly, because there is only one session store, separate session replication for data consistency is unnecessary, which also resolves performance issues.

However, session stores also need to replicate session objects. This is not to solve data consistency issues, but rather to address the problem that operating a session store can make that server a Single Point of Failure (SPOF), potentially making all sessions unavailable.
