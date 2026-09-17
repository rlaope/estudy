# Annotations for Spring Security Authentication in Spring Tests

### @WithMockUser
The @WithMockUser annotation creates UserDetails with the specified username, password, and authorities, then loads the security context. If no values are specified, it uses the following defaults:

- username: user
- roles: ROLE_USER
- password: password

### @WithAnonymousUser
Allows testing as an anonymous user.

### @WithUserDetails
It loads the security context by looking up an account with the specified username and then retrieving the UserDetails object.

- value: The specified username. Default User
- userDetailsServiceBeanName: The bean name of the UserDetails lookup service. If there's only one, it doesn't need to be specified.
