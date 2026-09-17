# Authentication vs. Authorization

**Authentication and authorization** are concepts that always appear together, yet they are often confusing terms to use.
There are frequent cases where their differences are not distinguished, as they can sometimes be used interchangeably without sounding awkward in context.
Authentication and authorization are widely used in modern computer systems, but they are terms that people often confuse, and both are related to security.

### What is Authentication?
- Authentication is the process of verifying the `identity` of an entity (user or device).
- An entity typically authenticates itself by presenting some form of credential as proof. For example, if you go to a bank to withdraw money, the bank teller might ask you to present an ID to confirm who you are. If you're buying an airplane ticket, you might need to present your passport to prove you are eligible to board.
- Both examples illustrate how the authentication process proceeds to verify identity.
- The same applies online. When you try to access your Facebook profile or a company email client, a similar process occurs. Instead of presenting an ID or passport, you enter your ID/password or a code sent via SMS to your phone.
- There can be one, two, or more authentication factors.

<br>

### What is Authorization?

- Unlike authentication, authorization is the process of verifying what resources an entity can access or what actions it can perform; in other words, it's about gaining access rights.
- For example, consider a situation where you buy a ticket to enter a concert venue. In this case, the concert organizers are not interested in your identity but only in whether you have the right to enter the venue.
- To prove your right of entry, you only need a ticket, not an ID or passport. Even if the ticket does not contain your identity information, the authorization process does not fail.
- Internet-based applications typically handle authorization using artifacts called `tokens`. When a user logs in, the application becomes interested in what the user can do. In the example above, a token with details based on the user's identity would be generated.
- The system uses the authorization token to determine what permissions to grant, i.e., whether to allow or deny resource access requests.

<br>

### Authentication vs. Authorization
Although we've clarified what authentication and authorization mean, these terms are often used interchangeably and cause confusion. For example, in the banking scenario, the ID you hand to the bank teller is also used by the teller for authorization to access your account assets. In a similar scenario, a company using badges to control access to meeting rooms uses the badge to authenticate the person (name and photo) and authorize access. As you can see, authentication and authorization can be interchangeable topics in some scenarios, which leads to confusion.

The important point is that authentication can lead to authorization, but authorization does not necessarily lead to authentication. Even if proof of identity is sufficient to grant access rights—that is, even if you can be authorized to obtain something—authorization cannot always be used to identify an entity.

For example, a boarding pass serves to authorize you to board a flight and also contains identity data. Thus, flight attendants can know your name from your boarding pass. However, a concert ticket does not contain identity details. The ticket merely represents the right to enter the venue and nothing more.

<br>

### Summary

- Authentication is the act of `proving` the identity of a user or device.
- Authorization is the act of granting or denying access rights to a user or device.
- Authentication can be a factor in authorization decisions.
- Using authorization artifacts (tokens) to identify a user or device is not useful.
