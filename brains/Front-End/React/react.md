# React
- React는 사용자가 인터페이스를 구축하기 위한 선언적이고 효율적이며 유연한 JavaScript `라이브러리` 입니다.
- `컴포넌트` 라고 불리는 작고 고립된 코드의 파편을 이용하여 복잡한 UI를 구성하도록 돕습니다.

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

XML 과 유사한 재미있는 태그를 사용할 것입니다. 우리는 컴포넌트를 사용하여 React에게 화면에 표현하고 싶은 것이 무엇인지 알려줍니다. 데이터가 변경될때 React는 컴포넌트를 효율적으로 업데이트하고 다시 렌더링 합니다.  
  
여기에서 ShoppingList는 `React컴포넌트` 클래스 또는 `React 컴포넌트 타입`입니다. 개별 컴포넌트 `props` 라는 매개변수를 받아오고 `render` 함수를 통해 표시할 뷰 계층 구조를 반환합니다.  
  
- render 함수는 화면에서 보고자 하는 내용을 반환합니다.
- React는 설명을 전달받고 결과를 표시합니다. 특시 render 는 렌더링할 내용을 경령화한 `React Element`를 반환합니다.
- 다수의 React 개발자는 JSX 라는 특수한 문법을 사용하여 React의 구조를 보다 쉽게 작성합니다. `<div>` 구문은 빌드하는 시점에서 React.createElement('div') 로 변환됩니다. 위의 예시는 아래처럼 변환됩니다.

```js
return React.createElement('div', {className : 'shopping-List'},
  React.createElement('h1', /* h1 children */),
  React.createElement('ul', /* ul children*/)
  )
```
  
JSX는 JS의 강력한 기능을 가지고 있습니다. JSX 내부의 중괄호 안에 어떤 JS 표현식도 사용할 수 있습니다. React 엘리먼트는 JavaScript 객체이며 변수에 저장하거나 프로그램 여기저기에 전달할 수 있습니다.  
  
Shopping-list 컴포넌트는 `<div />`와`<li>` 같은 내각 DOM 컴포넌트만을 렌더링하지만 컴포넌트를 조합하여 커스텀 React 컴포넌트를 렌더링하는 것도 가능합니다. 예를들어 `<ShoppingList/>`를 작성하여 모든 쇼핑 목록을 참조할 수 있습니다. React 컴포넌트는 캡슐화되어 독립적으로 동작할 수 있습니다. 이러한 이유로 단순한 컴포넌트를 사용하여 복잡한 UI를 구현할 수 있습니다. 

<br>

## Props를 통해 데이터 전달하기

- Board 컴포넌트에서 Square 컴포넌트로 데이터를 전달해보겠습니다.
- [이곳의](https://codepen.io/gaearon/pen/oWWQNa?editors=0010) 코드를 수정합니다

Square에 value prop을 전달하기 위해 Board의 `renderSquare`함수 코드를 수정합니다
```js
class Board extends React.Component{
  renderSquare(i)}
  return <Square value={i}/>;
}
```
값을 표시하기위해 Square의 render 함수에서 `/*TODO*/` 를 `{this.props.value}` 로 수정해주세요

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

변경 후에는 렌더링 된 결과에서 각 사각형의 숫자가 표시됩니다.