# React
- React is a declarative, efficient, and flexible JavaScript `library` for building user interfaces.
- It helps you compose complex UIs using small, isolated pieces of code called `components`.

```js
class Shopping extends React.Component {
  render(){
    return (
      <div className ="shopping-list">
        <h1>Shopping List for {this.props.name}</h1>
        <ul>
          <li>Instagram<li>
          <li>WhatApp<li>
          <li>Oculus<li>
        </ul>
      </div>
    );
  }
}
```

You will use interesting tags similar to XML. We use components to tell React what we want to display on the screen. When data changes, React efficiently updates and re-renders components.  
  
Here, `ShoppingList` is a `React component` class or a `React component type`. Individual components receive parameters called `props` and return the view hierarchy to be displayed through the `render` function.  
  
- The render function returns what you want to see on the screen.
- React receives the description and displays the result. Specifically, `render` returns a lightweight `React Element` that describes what to render.
- Many React developers use a special syntax called JSX to write React's structure more easily. The `<div>` syntax is converted to `React.createElement('div')` at build time. The example above is converted as follows:

```js
return React.createElement('div', {className : 'shopping-List'},
  React.createElement('h1', /* h1 children */),
  React.createElement('ul', /* ul children*/)
  )
```
  
JSX has the powerful features of JS. You can use any JS expression inside curly braces within JSX. React elements are JavaScript objects and can be stored in variables or passed around your program.  
  
The Shopping-list component renders only intrinsic DOM components like `<div />` and `<li>`, but it's also possible to combine components to render custom React components. For example, you can write `<ShoppingList/>` to refer to all shopping lists. React components are encapsulated and can operate independently. For this reason, complex UIs can be implemented using simple components.

<br>

## Passing Data via Props

- Let's try passing data from the Board component to the Square component.
- Modify the code [here](https://codepen.io/gaearon/p... ).

To pass the `value` prop to Square, modify the `renderSquare` function code in Board.
```js
class Board extends React.Component{
  renderSquare(i)}
  return <Square value={i}/>;
}
```
To display the value, please change `/*TODO*/` to `{this.props.value}` in Square's `render` function.

```js
class Square extends React.Component{
  render(){
    return (
      <button className = "square">
      {this.props.value}
      </button>
    );
  }
}
```

After the change, the number for each square will be displayed in the rendered result.
