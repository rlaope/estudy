# Building a Server

### 1. Create a server folder

### 2. Create server.js file
- server .js

```js
// To use the server, store the http module in the http variable
let http = require('http');

// Create a server using the http module
// Writing it as below creates a server, and when an http request comes from a user, it executes the code inside the function block to respond.
let server = http.createServer(function(request,response){
  response.writeHead(200, {'Content=Type':'text/html'});
  response.end('Hello node.js!');
});

// 3. Run the server with port 8080 using the listen function.
server.listen(8000, function(){
  console.log('Server is running...');
});
```

### Running and Accessing server.js

Terminal
```
$ node server
```
After running, if "Hello node.js!" appears correctly on http://localhost:8080, it's a success.

<br>

### Code Analysis

```js
let http = require('http');
```
To run a web server, we load the http module using `require`. (`require` has a similar function to `import`.) After `require`, Node.js stores the module in an `http` variable and uses it as an independent object.

The server is created using the `createServer` function defined in the `http` module.
A typical function is used like this:
```js
function nameOfFunction(parameter){
  // execution logic
}
```
The `function(request, response){}` passed as a parameter to `createServer()` does not have a name.

A parameter written simply as `function` without a name becomes a callback when an event occurs. That is, when a request comes to the created server, the logic inside the function is executed, passing values that can be used under the names `request` and `response` declared within the function. Therefore, any values passed as `request` and `response` can be used inside the function block `{}`.

```js
let server.createServer(function(request,response){
  response.writeHead(200,{'Content-Type':'text/html'});
  response.end('Hello node.js!');
})
```

In the code below, the `response` object is used to return a value to the client. First, in the `writeHead()` function:

The first value is the number 200,

The second value is in the form of `{ 'key' : 'value' }` inside the `{}` curly braces.

```js
response.writeHead(200,{'Content-Type':'text/html'});
```
The first number, 200, is an HTTP status code used when a web server successfully returns a value for an incoming request. If the server processes the request normally without errors, it sets the response header with a 200 code.

<br>

The second value, `{'Content-Type':'text/html'}`, defines that the content type sent from the server is text and in HTML format.
