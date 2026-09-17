# Component

## What is a Component?
- The smallest unit that makes up an app built with React.
- Traditional web frameworks managed separation using the MVC pattern, which led to high dependencies between elements, making reuse difficult. In contrast, components allow for independent configuration and reuse of the MVC view, making it easy to create new components.
- A component is a function that takes data (props) as input and outputs a DOM Node based on the View (state).
- Component names should always start with an uppercase letter (because React treats components starting with a lowercase letter as DOM tags).
- Divide the UI into reusable, individual pieces and code each piece separately.
- After receiving arbitrary inputs called 'props', it returns a React element that describes what should appear on the screen.

<br>

## Types of Components
- There are two main types of components defined in React: functional components and class components.

### 1. Functional Components
- The simplest way to define a component is by writing a JavaScript function.
- The `export` statement in the example below defines how the `MyComponent` file you wrote can be imported as `MyComponent` in other files.
- Here, when importing, file extensions like `.js` or `.jsx` can be omitted and will be found automatically. This is due to the "Webpack module resolution" feature. If an extension is not present in the file name, Webpack checks for files with extensions defined in its `resolve.extensions` option and automatically imports them.

e.g., for `import 'Header';`, it checks in the order of `Header.js` > `Header.jsx`.

**Functional Component Example**
```js
import React from 'react';
function MyComponent(props){
  return <div>Hello, {props.name}</div>;
}

export default MyComponent; // Export to be able to import from other JS files
```

<br>

### 2. Class Components
- They include all component elements and React lifecycle methods.
- They are used to create components that require properties, state, and lifecycle functions.

```js
import React from 'react';
class MyComponent extends React.Component{
  constructor(props){ // Constructor function
    super(props);
  }
  componentDidMount(){ // Inherited lifecycle method
  }
  render(){ // Inherited screen output function, render() is mandatory for class components
  return <div>Hello, {this.props.name}</div>;
  }
}

export default MyComponent
```

While functional components are primarily used, there are times when tasks cannot be handled by functional components alone. In such cases, class components come in handy, so let's explore both.
