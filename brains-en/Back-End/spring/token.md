# Spring Security AccessToken & RefreshToken

### What is an Access Token?
An Access Token is a token used to perform the authentication process (login) by verifying whether a token is included in requests to REST APIs accessing private resources and whether that token is valid.

An Access Token has a lifespan.
Once its lifespan ends, if you request data from the API server using that token, the API server will no longer provide information.
Therefore, the Access Token must be reissued.

However, it's unreasonable to ask the user to log in again every time. Looking around, there are no sites that log you out even if you maintain a login for a long time and continuously use the site.

However, if the lifespan of an `Access Token` is made long or indefinite, the possibility of a malicious attacker exploiting it increases. (To take an extreme example, if an Access Token's validity period is 5 seconds, it would certainly be difficult for an attacker to do anything with this token.)

**Therefore, the Access Token's lifespan should be maintained for a period that is not too short, but also not so long that frequent logouts occur.**

In such cases, `Refresh Token` is the method that allows new Access Tokens to be easily issued.

![token](./image/Token.png)

> In a typical Spring project, there might be cases where the AuthorizationServer and ResourceServer are implemented within the same API server. In such cases, let's understand the flow by considering the AuthorizationServer as the Security layer and the ResourceServer as the RestController layer.

- When the Client obtains authorization through login (Process A), it receives an Access Token along with an RT (Process B).
- The Client then stores both the AccessToken and RefreshToken. When calling an API (Process C), it submits the AccessToken to retrieve resources (Process D).
- As time passes, when attempting to retrieve resources using the AccessToken again (Process E), an Invalid Token Error appears (Process F), indicating that the `AccessToken` has expired.
- At that point, the stored Refresh Token is immediately sent to the AuthorizationServer (Process G), and a new AccessToken is issued (Process H). (The reason 'Optional Refresh Token' is written in the diagram is that the RefreshToken can also be renewed when a new AccessToken is issued.)
