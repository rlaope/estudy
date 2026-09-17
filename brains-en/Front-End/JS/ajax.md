# AJAX
- AJAX is a technology that effectively utilizes HTTP.
- AJAX is not a standard; it's a technology for effective communication with the server.

> When using AJAX, the user does not receive a new HTML page from the server.
> In other words, the user does not navigate to a new webpage, but rather **modifies the DOM within the same webpage**.

<br>

**Let's consider a feature for entering a name and self-introduction.**

1. Let's say on the request page, you type 'Hope' in the name field and 'Hello, I am Hope' in the content field.
2. From a user event, JavaScript reads the DOM containing the entered name and content.
3. Then, it sends that name and content to the web server via an XMLHttpRequest object.
4. The web server processes the request and sends XML, Text, or JSON back to the XMLHttpRequest object.
5. Then, JavaScript writes the response information to the DOM.
6. This is how the result page is created.

<br>

When using AJAX, you don't have to receive new HTML from the server.
It creates the possibility of modifying only a portion of the same page. As a result, from the user's perspective, there is no page navigation; only internal changes occur within the page. Instead of having to replace the entire HTML page, you can change just a part of it.

### XMLHttpRequest Object
This is the most crucial component of AJAX.
In AJAX, the XMLHttpRequest object is used when the **web browser exchanges data with the server**.
The reason web browsers can continuously communicate with the server in the background is precisely because they use this object.
```js
let httpRequest = new XMLHttpRequest();
```
