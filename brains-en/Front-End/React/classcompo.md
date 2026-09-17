# Class Components

```js
import React, { Componennt } from 'react';

class App extends Component{
  render() {
    const name = '리액트';
    return <div>{name}</div>;
  }
}
export defalut App;
```

Class components and functional components serve the same purpose. The difference is that class components can use state and lifecycle features, and custom methods can be defined. Also, a `render` function is mandatory, and it must return the JSX to be displayed. Furthermore, class syntax, which was previously implemented using prototypes, *can now be implemented using the `class` keyword starting from ES6.*
```js
class Dog {
  constructor(name) {
    this.name = name;
  }

  say(){
    console.log(this.name + '멍멍');
  }
}

const dog = new Dog('강아지');
dog.say(); // 강아지
```

- On the other hand, functional components have the advantage of being easier to declare than class components and using fewer memory resources.
- In the past, functional components had the disadvantage of not being able to use state and lifecycle APIs, but this drawback was resolved with the introduction of React Hooks, as mentioned earlier.

When declaring functional components, there are two methods: the traditional function declaration and the ES6 arrow function. Arrow functions are useful when passing functions as parameters. While they share many similarities, the following examples clearly demonstrate their differences.
```js
function BlackDog() {
  this.name = '흰둥이';
  return {
    name: '검둥이',
    bark: function() {
      console.log(this.name + ': 멍멍!');
    }
  }
}

const blackDog = new Blackdog();
blackDog.bark(); // 검둥이: 멍멍!

function WhiteDog() {
  this.name = '흰둥이';
  return {
    name: '검둥이',
    bark: () => {
      console.log(this.name + ': 멍멍!');
    }
  }
}

const whiteDog = new Whitedog();
whiteDog.bark(); // 흰둥이: 멍멍!
```
Using `function()` results in '검둥이' (Blackie), while `() => {}` results in '흰둥이' (Whitey). **A regular function points `this` to the object it belongs to, whereas an arrow function points `this` to the instance it belongs to.**

## props
- Props is an abbreviation for properties, used to set component attributes.
- Props values can be set in the parent component that imports and uses the component. Optionally, you can specify the type of props using the `propTypes` component property. If you're using TypeScript, you don't necessarily need `propTypes` for type checking.

```js
import React from 'react';
import MyComponent from './MyComponent';

const App = () => {
  return <MyComponent name = '리액트'></MyComponent>;
}

export default App;

// MyComponent.js
import React from 'react';
import PropTypes from 'prop-types';

const MyComponent = ({name}) => {
  return (
    <div>제 이름은 {name}입니다.</div>
  );
};

MyComponent.propTypes = {
  name: PropTypes.string
};

export default MyComponent;
```
