# Frequently Used DOM API Summary

## DOM Tree Components

- Document Node - The top-level tree and the starting point for access.
- Element Node - Represents an HTML element.
- Attribute Node - Represents an attribute of an HTML element.
- Text Node - Represents the text of an HTML element. Cannot have children.

## DOM API

To manipulate web pages using the `DOM`, use the API in the following order.

<br>

  
## 1. Element Selection and Traversal

<br>

### 1-1 Selecting a Single Element Node
- `document.getElementById("id")`
  - Selects a single element node by its id value.
  - If multiple elements are selected, only the first one is returned.

- `document.querySelector("cssSelector")`
  - Selects a single element node by a CSS selector.
  - If multiple elements are selected, only the first one is returned.

<br>

### 1-2 Selecting Multiple Element Nodes

- `document.getElementByClassName(class)`
  - Selects all element nodes by their class value.
  - Multiple classes can be specified using spaces.
  - Returns an HTMLCollection.

- `document.getElementByTagName(tagName)`
  - Selects all element nodes by their tag name.
  - Returns an HTMLCollection.
  
- `document.queySelectorAll(selector)`
  - Selects all element nodes by a CSS selector.
  - Returns a NodeList.

<br>

### 1-3 Traversal

- `parentNode`
  - Traverses to the parent node.

- `firstChild, lastChild`
  - Traverses to child nodes.
  - Be careful as spaces and line breaks are also treated as text nodes.

- `childNodes`
  - Returns a collection of child nodes.
  - Returns a NodeList.

- `children`
  - Returns a collection of child nodes.
  - Returns an HTMLCollection.

<br>

Comparing HTMLCollection and NodeList

- HTMLCollection (live)
  - Array-like
  - Reflects real-time changes in node status.

- NodeList (non-live)
  - Array-like
  - Can sometimes become a live collection depending on the case.


## 2. Manipulation

### 2-1 Accessing and Modifying Text Nodes

Text nodes must be traversed via their parent node.

- `nodeValue`
  - Returns a string for text nodes and null for element nodes.
  - It is the only property of text nodes.


<br>

### 2-2. Accessing and Modifying Attribute Nodes

- `className`
  - Gets or changes the class value.
  - If there are multiple class values, a space-separated string is returned.
  - If the class attribute does not exist when assigning a value, it creates the class attribute and then assigns the value.

- `id`
  - Gets or changes the id value.
  - If the id attribute does not exist when assigning a value, it creates the id attribute and then assigns the value.

### 2-3 HTML Content Manipulation

- `textContet`
  - Gets or changes the text content of an element. Markup is ignored.
  - If markup is included when changing the value, it is recognized as a string and output as is.

- `innerText`
  - Returns a string excluding markup.
  - When changing the value, markup must be added as is.
  - Adding values including markup is vulnerable to XSS.
  - It is better not to use it for the following reasons:
    - It is non-standard.
    - It is CSS-aware.
    - It is slower than textContent because it has to consider CSS.


- `innerHTML`
  - Gets child elements as a single string.
  - Returns including markup.
  - Adding values including markup is vulnerable to XSS.

<br>

### 2-4. DOM Manipulation Methods

Methods for adding new content without using innerHTML.

- `createElement(tagName)`
  - Creates an element by passing the tag name as an argument.

- `createTextNode(text)`
  - Creates a text node by passing text as an argument.

- `appendChild(Node)`
  - Appends the node passed as an argument as the last child element to the DOM tree.

- `removeChild(Node)`
  - Removes the node passed as an argument from the DOM tree.


<br>

## Comparing innerHTML and DOM Manipulation Methods

- innerHTML
  - Faster and simpler compared to DOM manipulation methods.
  - Vulnerable to XSS attacks, so caution is needed when adding values.
  - It's inefficient because it overwrites content.

- DOM Manipulation Methods
  - Suitable for adding a single specific node.
  - Slower and requires more code than innerHTML.


- Conclusion
  > Due to innerHTML's vulnerability to XSS, it is recommended to use `textContent` for `adding or changing text` and **DOM manipulation methods** for **adding or deleting new elements**.
