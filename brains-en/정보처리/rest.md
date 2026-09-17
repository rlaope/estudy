# Meaning and Design Rules of RESTful API

### RESTful API
REST, an acronym for REpresentational State Transfer, is a software architectural style that defines constraints for using the web. It means specifying resources via HTTP URLs and applying CRUD operations to those resources using HTTP Methods. In short, it can be described as a `communication protocol that aims to leverage the advantages of HTTP`. Introduced in Roy Fielding's 2000 doctoral dissertation, a RESTful API is designed resource-centrically based on these conventions, using HTTP Methods appropriate for its functionality.
  
- GET : Retrieve a representation of the resource at the specified URL
- POST : Create a new resource at the specified URL
- PUT : Create or update a resource at the specified URL
- PATCH : Partially update a resource
- DELETE : Remove the resource at the specified URL

<br>

### Characteristics of REST

**Six Constraints Applied to REST Architecture**
- Uniform Interface : Should be separated into consistent interfaces.
- Stateless : Does not store client state information, such as context or session, on the server between requests.
- Cacheable : Clients must be able to cache responses. Caching allows for efficient handling of a large number of requests.
- Layered System : Clients cannot tell whether they are connected directly to the target server or via a proxy.
- Code on Demand : Through the provision of Java applets or JavaScript, the server can extend functionality by sending logic that the client can execute.
- Client-Server Architecture : By simplifying the architecture and separating it into smaller units, each part of the client-server is independently distinguished, reducing interdependencies.

<br>

### REST Components
REST consists of the following three components:
1. Resource: HTTP URL
2. Action on the resource: HTTP Method
3. Representation of the resource

<br>

### REST API Design Rules and Examples

> 1. Use lowercase letters.

```
❌ http://cocoon1787.tistory.com/users/Post-Comments
```
```
⭕ http://cocoon1787.tistory.com/users/post-comments
```
**Uppercase letters can sometimes cause issues, so use lowercase.**
  
> 2. Use hyphens instead of underscores.

```
❌ http://cocoon1787.tistory.com/users/post_comments
```
```
⭕ http://cocoon1787.tistory.com/users/post-comments
```
**If precise meaning or word combination is unavoidable, use hyphens (`-`), designing their use to be minimal. Do not use underscores (`_`).**
  
> 3. Do not include a trailing slash.

```
❌ http://cocoon1787.tistory.com/users/
```
```
⭕ http://cocoon1787.tistory.com/users
```
**A slash (`/`) is used to indicate a hierarchical relationship.**

> 4. Do not include actions.

```
❌ POST http://cocoon1787.tistory.com/users/post/1
```
```
⭕ DELETE http://cocoon1787.tistory.com/users/1
```
**Actions on resources are expressed using HTTP Methods (GET, POST, DELETE, PUT).**
  
> 5. Do not include file extensions in the URL.

```
❌ http://cocoon1787.tistory.com/users/photo.jpg
```

```
⭕ GET http://cocoon1787.tistory.com/users/photo
   HTTP/1.1 Host: cocoon1787.tistory.com Accept: image/jpg
```

**Do not include file extensions in the URL to indicate the format of the message body content. Instead, use the Accept header.**
  
> 6. Use nouns for resources, not adjectives or verbs; verbs are an exception when referring to control resources.

```
❌ http://cocoon1787.tistory.com/duplicating
```

```
⭕ http://cocoon1787.tistory.com/duplicate
```
**URLs should focus on representing resources, so nouns should be used rather than verbs or adjectives.**
