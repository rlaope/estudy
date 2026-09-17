# Let's Create an HTTP API

Requirement: Create an API for managing user information.

The most important thing is `resource identification`

### Meaning of a Resource
- Registering, modifying, or retrieving a user is not a resource.
  - Example: Mine minerals -> minerals are the resource
  - The concept of a user itself is the resource.
- How should we identify resources?
  - Exclude all actions like registering, modifying, or retrieving users.
  - We only need to identify the user resource -> Map the user resource to a URI

### API URI Design
URI (Uniform Resource Identifier)
Resource identification, utilizing URI hierarchical structure

- Retrieve `user` list / members
- Retrieve `user` /members/{id} -> How to distinguish?
- Register `user` /members/{id} -> How to distinguish?
- Modify `user` /members/{id} -> How to distinguish?
- Delete `user` /members/{id} -> How to distinguish?
- Note: It is recommended to use plural nouns for collections at the top of the hierarchy (member >> members)

### Separating Resources and Actions
The most important thing is identifying the resource.
- URI identifies only the resource!
- Separate the resource from the `action` targeting that resource.
  - Resources are nouns, actions are verbs.
  - How do we distinguish actions (methods)?

> This will be covered in the HTTP Methods section.
