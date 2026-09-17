# Component

## Component 란 ?
- 리액트로 만들어진 앱을 이루는 최소한의 단위
- 기존의 웹 프레임워크는 MVC방식으로 분리하여 관리해 각 요소의 의존성이 높아 재활용이 어렵다는 단점이 있었다. 반면 컴포넌트는 MVC의 뷰를 독립적으로 구성하여 재사용을 할 수 있고 이를 통해 새로운 컴포넌트를 쉽게 만들 수 있다.
- 컴포넌트는 데이터(props)를 입력받아 View(state) 상태에 따라 DOM Node를 출력하는 함수이다.
- 컴포넌트 이름은 항상 대문자로 시작하도록 한다 (React는 소문자로 시작하는 컴포넌트를 DOM 태그로 취급하기 때문이다.)
- UI를 재사용 가능한 개별적인 여러 조각으로 나누고, 각 조각을 개별적으로 나누어 코딩한다
- 'props'라고 하는 임의의 입력을 받은 후, 화면에 어떻게 표시되는지를 기술하는 React 엘리먼트를 반환한다

<br>

## Component 종류
- 리액트에서 정의하는 컴포넌트 종류는 크게 함수형 컴포넌트 , 클래스형 컴포넌트 2가지가 있다.

### 1. 함수형 컴포넌트
- 가장 간단하게 컴포넌트를 정의하는 방법은 자바스크립트 함수를 작성하는 것이다.
- 하기 예씨의 export 구문은 작성한 MyComponent 파일을 다른 파일에서 import 할 때 MyComponent로 불러올 수 있도록 정의해 주는 부분이다.
- 여기서 inport시 js,jsx 등 파일 확장자를 생략해도 자동으로 찾을 수 있다. 이는 "웹팩 코드 검색 확장자(Webpack module resolution)" 기능 때문이다. 확장자가 파일 이름에 없는 경우 웹팩의 확장자 옵션에 정의된 확장자 목록을 통해 확장자 이름을 포함한 파일이 있는지 확인하여 자동으로 임포트한다.

ex) import 'Header';의 경우 Header.js > Header.jsx 순서로 확인한다  
  
**함수형 컴포넌트 예시**
```js
import React from 'react';
function MyComponent(props){
  return <div>Hello, {props.name}</div>;
}

export default MyComponent; // 다른 JS 파일에서 불러올 수 있도록 내보내주기
```

<br>

### 2. 클래스형 컴포넌트
- 컴포넌트 구성 요소, 리액트 생명주기를 모두 포함하고 있다.
- 프로퍼티, state, 생명주기 함수가 필요한 구조의 컴포넌트를 만들 때 사용한다.

```js
import React from 'react';
class MyComponent extends React.Component{
  constructor(props){ // 생성함수
    super(props);
  }
  componentDidMount(){ // 상속받은 생명주기 함수
  }
  render(){ // 상속받은 화면 출력 함수, 클래스형 컴포넌트는 render() 필수
  return <div>Hello, {this.props.name}</div>;
  }
}

export default MyComponent
```

주로 함수형 컴포넌트를 사용하지만 , 함수형 컴포넌트로 하지 못하는 작업을 처리할 때가 있는데 이때 클래스형 컴포넌트를 활용하니 둘다 알아보자.