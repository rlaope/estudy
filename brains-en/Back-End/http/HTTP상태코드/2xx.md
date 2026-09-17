# 2xx - Success

### 2xx (Successful)
Successfully processed the client's request

- 200 OK : Request successful
  - e.g.) GET
- 201 Created : Request successful, new resource created
  - e.g.) POST, the created resource is identified by the Location header field in the response
  - Location : /members/100
- 202 Accepted
  - Used in places like batch processing
  - e.g.) A batch process handles the request 1 hour after it's received
  - The request has been accepted, but processing is `not yet complete`
- 204 No Content
  - e.g.) Save button in a web document editor
  - No content is needed as a result of the save button.
  - The same screen should be maintained even after pressing the save button.
  - Even without result content, success can be recognized solely by the 204 message (2xx).
