# 함수형 컴포넌트
리액트는 화면을 구성하게 되면, 사용자가 볼 수 있는 여러 가지 `컴포넌트`로 구성되어 있습니다. 사용자에게 보여지는 UI 요소를 `컴포넌트` 단위로 구분하여 구현할 수 있습니다.  
  
아래 코드는 `create-react-app` 으로 처음 리액트 프로젝트를 생성하였을떄 app.js 에 기본적으로 작성되어있는 코드입니다. 아래처럼 작성한 컴포넌트를 `함수형 컴포넌트` 라고 합니다.
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

## 함수형 컴포넌트 작성하기
```js
import React from 'react';

const MyComponent = () => {
  return <div>테스트 페이지</div>
}

export default MyComponent
```

- export는 다른 파일에서 MyComponent 파일을 import 해올 때 MyComponent로 불러올 수 있도록 정의해 주는 부분입니다.
- 이것을 활용하여 다른 코드에서 import로 불러올 수 있습니다. 아래와 같이 app.js 에서 `<MyComponent>` 라고 작성한 뒤 `yarn start(혹은 npm start)` 로 프로젝트를 실행시켜 보세요. 정상적으로 브라우저에 테스트 페이지라고 표시될 것입니다. 단, `import MyComponent from './MyComponennt';` 로 컴포넌트를 불러와야 한다는 것을 꼭 잊지 마세요

```js
import React from "react";
import "./App.css";
import MyComponent from "./MyComponent";

function App() {
  return <MyComponent />;
}

export default App;
```