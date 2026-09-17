# HTTP API Design Examples

- HTTP API - Collection
  - POST-based Registration
  - Example: Providing a Member Management API
- HTTP API - Store
  - PUT-based Registration
  - Example: Managing Static Content, Remote Files
- HTML: Using FORM
  - Web Page Member Management
  - Supports only GET, POST

### Member Management System
**API Design - POST-based Registration**
- Member List /members -> GET
- Register Member /members -> POST
- Retrieve Member /members/{id} -> GET
- Update Member /members/{id} -> PATCH, PUT, POST
- Delete Member /members/{id} -> DELETE

<br>

### Member Management System
**POST - Characteristics of New Resource Registration**
- The client does not know the URI of the resource to be registered.
  - Register Member /members -> POST
  - POST /members
- The server creates the URI for the newly registered resource.
  - HTTP/1.1 201 Created
  - Location : /members/100
- Collection
  - A resource directory managed by the server
  - The server creates and manages the resource's URI
  - Here, the collection is /members

<br>

### File Management System
**API Design - PUT-based Registration**
- File List /files -> GET
- Retrieve File /files/{filename} -> GET
- Register File /files/{filename} -> PUT
- Delete File /files/{filename} -> DELETE
- Bulk File Registration /files -> POST

<br>

### File Management System
**PUT - Characteristics of New Resource Registration**
- The client must know the resource URI.
  - Register File /files/{filename} -> PUT
  - PUT /files/star.jpg
- The client directly specifies the resource's URI.
- Store
  - A resource repository managed by the client
  - The client knows and manages the resource's URI
  - Here, the store is /files

<br>

### Using HTML Form
- HTML Forms only support GET, POST
- Can be resolved using technologies like AJAX -> Refer to Member API
- Here, we're talking about pure HTML, HTML Forms
- There are limitations as only GET, POST are supported
- Member List /members -> GET
- Member Registration Form /members/new -> GET
- Register Member /members/new , /members -> POST
- Retrieve Member /members/{id} -> GET
- Member Update Form /members/{id}/edit -> GET
- Update Member /members/{id}/edit . members/{id} -> POST
- Delete Member /members/{id}/delete -> POST
- HTML Forms only support GET, POST
- Control URI
  - There are limitations as only GET, POST are supported
  - To overcome these limitations, resource paths with verbs are used
  - /new, /edit, /delete in POST are Control URIs
  - Used when HTTP methods are ambiguous (including HTTP API)

<br>

### Summary

#### Useful URI Design Concepts to Refer To
- Document
  - Single concept (a single file, object instance, database row)
  - Example: /members/100, /files/star.jpg
- Collection
  - A resource directory managed by the server
  - The server creates and manages the resource's URI
  - Example: /members
- Store
  - A resource repository managed by the client
  - The client knows and manages the resource's URI
  - Example: /files
- Controller, Control URI
  - Executes additional processes difficult to resolve with documents, collections, or stores
  - Directly uses verbs
  - Example: /members/{id}/delete
