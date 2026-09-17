# Functional Components
When React renders a screen, it is composed of various `components` that users can see. UI elements displayed to the user can be implemented by dividing them into `component` units.

The code below is what is initially written in app.js when a React project is first created with `create-react-app`. A component written like the one below is called a `functional component`.
```js
import React from "react";
import logo from "./logo.svg";
import "./App.css";

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.js</code> and save to reload.
        </p>
        <a className="App-link" href="https://reactjs.org" target="_blank" rel="noopener noreferrer">
          Learn React
        </a>
      </header>
    </div>
  );
}

export default App;
```

<br>

## Writing a Functional Component
```js
import React from 'react';

const MyComponent = () => {
  return <div>테스트 페이지</div>
}

export default MyComponent
```

- `export` defines how the `MyComponent` file can be imported as `MyComponent` from other files.
- You can use this to import it into other code. Try writing `<MyComponent>` in app.js as shown below, then run the project with `yarn start` (or `npm start`). A test page should be displayed correctly in your browser. However, don't forget that you must import the component with `import MyComponent from './MyComponennt';`.

```js
import React from "react";
import "./App.css";
import MyComponent from "./MyComponent";

function App() {
  return <MyComponent />;
}

export default App;
```
