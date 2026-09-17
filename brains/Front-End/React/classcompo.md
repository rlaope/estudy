# 클래스형 컴포넌트

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

클래스형 컴포넌트와 함수형 컴포넌트의 역할은 동일하다. 차이점이 있다면 클래스형 컴포넌트의 경우 state 기능 및 라이프 사이클 기능을 사용할 수 있으며 임의 메서드를 정의할 수 있다는 점이다. 그리고 render 함수가 꼭 있어야 하고, 그 안에서 보여 주어야 할 JSX를 반환해야 한다. 또한 prototype을 이용해서 구현하던 클래스 문법을 *ES6 문법 부터는 class 문법을 사용하여 구현할 수도 있다.*  
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

- 반면 함수형 컴포넌트는 클래스형 컨포넌트보다 선언하기가 좀 더 편하고, 메모리 자원을 덜 사용한다는 장점이 있다.
- 과거에는 함수형 컴포넌트에 state와 라이프사이클 API를 사용할 수 없다는 단점이 있었는데, 이러한 단저은 앞서 언급한 것처럼 리액트 훅이 도입되면서 해결되었다.

함수형 컴포넌트를 선언할 때 사용하는 방법으로 기존의 일반적인 함수 선언 방식이 있고 ES6화살표 함수 방식도 있다. 화살표 함수의 경우 함수를 파라미터로 전달할 때 유용하다. 비슷한점도 많지만, 두 문법이 확실하게 다라다는 점은 다음 예제를 통해 알 수가 있다.
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
function()을 사용하면 검둥이가 나타나고, () => {} 를 사용하면 흰둥이가 나타난다. **일반 함수는 자신이 종속된 객체를 this로 가리키며, 화살표 함수는 자신이 종속된 인스턴스를 가리킨다**  
  
## props
- props는 프로퍼티를 줄인 표현으로 컴포넌트 속성을 설정할 때 사용하는 표현이다.
- props 값은 해당 컴포넌트를 불러와 사용하는 부모 컴포넌트에서 설정할 수 있다. 그리고 경우에 따라서 propTypes 컴포넌트 속성을 통해 props의 타입을 지정해 줄 수도 있다. 타입스크립트를 사용한다면 굳이 propTypes를 사용하지 않아도 타입 체크를 할 수가 있다.

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