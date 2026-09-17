# ☁️ Understanding OAuth 2.0 Concepts and Grant Types

### Overview

I decided to adopt OAuth for a new project. While implementing OAuth, I didn't fully understand how it works, so I decided to organize the concepts this time.

### OAuth 2.0?

**OAuth 2.0 (Open Authorization 2.0)**

While surfing the web, it's easy to find ways to conveniently sign up and log in using external social accounts like Google or Facebook. These services offer the advantage of **easily using features provided by Facebook, Twitter, etc., within the integrated external web application.**

It is an *open standard protocol* for authentication that allows a third-party client to obtain a user's access rights to data from various platforms such as Google, Facebook, Twitter, and Naver.

#### Roles in OAuth

**Resource Owner:** The owner of the resource, who is the entity that approves access to their information. For example, a user logging in with Google. The Resource Owner performs authentication, and once authenticated, grants the client an Authorization Grant through consent.

**Client:** An **application** that requests access to use the Resource Owner's resources.

**Resource Server:** The **server where the Resource Owner's information is stored**.

**Authorization Server:** The authorization server. This server performs authentication and authorization, verifying the client's access qualifications and issuing Access Tokens to grant permissions.

**AccessToken:** A **credential** indicating that the Resource Owner has authorized access to a resource.

**RefreshToken:** Access Tokens have a short expiration period for security reasons, so if they expire quickly, users would have to log in again. However, using a Refresh Token allows a new Access Token to be reissued, eliminating the need for the user to log in again.

### Grant Types

The OAuth 2.0 protocol provides four different protocols based on various grant types to suit diverse client environments.

***1. Authorization Code Grant***

***2. Implicit Grant***

***3. Resource Owner Password Credentials Grant***

***4. Client Credentials Grant***

#### Authorization Code Grant

![](image/oauth2_0.png)

This is the **most commonly used and fundamental method, sending an Authorization Code** for authorization approval.

It's the method used in simple login features and when a client requests access to a specific resource on behalf of a user. It's used for authenticating to provide protected resources to third-party clients. This method allows the use of Refresh Tokens.

When requesting authorization approval, the `response_type` is set to `code`. The client then displays the login page provided by the authorization server in the browser. After the user logs in through this page, the authorization server sends the Authorization Code to the `redirect_url` that was provided during the authorization code request. The Authorization Code is exchanged for an Access Token via an API provided by the authorization server.

#### Implict Grant

![](image/oauth2_1.png)

This method is **optimized for clients that have difficulty storing credentials, such as browsers using scripting languages like JavaScript**.

In the Implicit Grant flow, an Access Token is issued directly without an Authorization Code. Because it's delivered directly, the expiration period is set short. This method does not allow the use of Refresh Tokens, and in this flow, the authorization server does not authenticate the client using a `client_secret`. While the process for obtaining an Access Token is simplified, increasing responsiveness and efficiency, it has the disadvantage of the token being transmitted via the URL.

When requesting authorization approval, the `response_type` is set to `token`. The client then displays the login page provided by the authorization server in the browser, and upon successful login, the server directly sends the Access Token to the `redirect_url` instead of an Authorization Code.

#### Resource Owner Password Credentials Grant

![](image/oauth2_2.png)

This method simply obtains an Access Token using a username and password.

**This method is applied when the client is an external program.** It is an authentication method used only when the application is provided by one's own service, and Refresh Tokens can also be used.

It involves sending the username and password via the provided API to receive an Access Token. The important point is that this method should only be used when the authorization server, resource server, and client all belong to the same system.

#### Client Credentials Grant

This method obtains an Access Token using only the client's credentials.

It is the simplest OAuth 2.0 grant type and is used when the client manages its own resources or when the authorization server has limited resource access rights configured for that client. This method is only used by clients that can securely store credentials, and Refresh Tokens cannot be used.
