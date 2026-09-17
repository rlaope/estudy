# DOM API

## DOM Introduction

What is `DOM (Document Object Model)`?
- The Document Object Model is a programming interface for HTML and XML documents. The DOM provides a structured representation of the document and offers a way for programming languages to access the DOM structure, helping them to change the document's structure, style, and content.
- The `DOM` represents documents as nodes and objects. These serve to connect web pages, making them usable by scripts or programming languages.

### 1. Through DOM

The interactions we can perform through the DOM involve `accessing the DOM structure` and `modifying the document's content`.
Simply put, changing the document's content means we can create `dynamic` documents.

Fundamentally, a web page can be said to consist of HTML, CSS, and JS. Here, changing HTML is the role of JS, and you can think of this as being related to the DOM.

Of course, JS isn't limited to just this role, but since it mostly operates in browsers, I'll move on.

### 2. DOM Structure, DOM Tree
- The `DOM` represents HTML documents as a tree structure.

![tree](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FssgJG%2FbtqDO1V1e2g%2FbpBfkWNPkIluSiQfhsaxG0%2Fimg.png)

<br>

As it's the Document `Object` Model, each and every tag is an **object**. Keep this in mind as well.

### 2.1 Accessing the DOM Structure
> When writing scripts, you can immediately use APIs for `document` or `window` elements to manipulate the document itself or get its children, either by using inline elements or script loading commands within the web page.

### 2.2 Modifying the DOM
Once you've accessed an object, you can modify the `DOM` using `DOM API`s.

![API](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbnAfcx%2FbtqDRNhmftL%2FK2n3pKhOfEE1NikYKzm0BK%2Fimg.png)

- The process involves creating an element with `document.createElement` and inserting it as a child element with `appendChild`.

Below is a simple DOM API example.
```js
const p = document.getElementByTagName("p");

p.innerText = "Hello";
```

### DOM Nodes
Although it's an object-oriented concept, objects in the DOM inherit from `classes (prototypes)`.

![Inheritance](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FpnQ2a%2FbtqDRLDP1Ue%2FFVBUb3q0PX1KKj9ntb0Z61%2Fimg.png)

> The way the DOM is implemented can vary. Because browsers create the DOM.
