# fetch API

## XML Http Request Method
This is the traditional, early asynchronous server request method using the XMLHttpRequest object.
While there were no performance issues, it had the disadvantage of complex code and poor readability.
```js
let httpRequest = new XMLHttpRequest();
httpRequest.onreadystatechange = function (){
  if(httpRequest.readyState === XMLHttpRequest.DONE && httpRequest.status === 200){
    document.getElementById('text').innerHTML = httpRequest.responseText;
  }
}

httpRequest.open("GET", "ajax_intro_data.txt",true);
httpRequest.send();
```

<br>

## fetch API Approach
Unlike the event-based XMLHttpRequest, the fetch API is built on Promises, making it well-suited for **asynchronous programming patterns**.
This offers the advantage of being able to write code using chaining with `then` or `catch`.

```js
fetch('ajax_intro.data.txt').then(response  => response.text()).then(text => {document.getElementById("#t').innerHTML = text;})

// `fetch('server_address')` means 'please send a request to this server address' to the web browser,
// and if `.then` is appended, it means 'after the request is finished, do this task!'
```
Furthermore, since the fetch API is a native JavaScript feature, it can be used immediately without needing additional setup like CDNs, unlike jQuery.

<br>

## fetch Syntax & Usage

```js
fetch("https://jsonplaceholder.typicode.com/posts",option).then(res => res.text()).then(text => console.log(text));
```
1. The first argument to `fetch` is typically the `url` to request.
2. By default, it operates as an HTTP GET method.
3. When calling AJAX via `fetch`, it sends a request to the specified address and then receives a response object.
4. The first `then` receives that response and returns the text value parsed by the `res.text()` method.
5. The subsequent `then` then receives the returned text value, allowing you to perform the desired processing.

<br>

## response Properties and Methods
When a request is made via `fetch` and a value is received from the server, it's passed as an argument to the function within `.then`. This value is a `Response object` containing various pieces of information.
You can extract the necessary variable values or methods to obtain the data.

### Basic Syntax
- response.status >> HTTP status code
- response.ok >> true if the HTTP status code is between 200 and 299
- response.body >> content
- response.text() >> Reads the response and returns text.
- response.json() >> Parses the response into JSON format.
- response.formData() >> Returns the response as a FormData object.
- response.blob() >> Returns the response as a Blob (typed binary data).
- response.arrayBuffer() >> Returns the response as an ArrayBuffer (a low-level representation of binary data).

<br>
