# TypeScript 에서의 함수

웹 애플리케이션을 구현할 때 자주 사용되는 함수는 타입스크립트로 크게 3가지 타입을 정의할 수 있습니다.  
  
  - 함수의 파라미터(매개변수) 타입
  - 함수의 반환 타입
  - 함수의 구조 타입

<br>

## 함수의 기본적인 타입 선언

1. 자바스크립트 함수 선언
```js
function su,(a,b){
  return a + b;
}
```

2. 타입스크립트 함수 선언
 
```ts
 function sum(a : number, b : number) : number {
  return a + b;
 }
```

> 기존 자바스크립트 함수의 선언 방식에서 매개변수와 함수의 반환 값에 `타입`을 추가했습니다.

> 함수의 반환 값에 타입을 정하지 않을 때는 `void` 라도 사용

## 함수의 인자

타입스크립트에서는 함수의 인자를 모두 필수 값으로 간주합니다. 따라서, 함수의 매개변수를 설정하면 `undefined` 나 `null` 이라도 인자로 넘겨야하며 컴파일러에 정의된 매개변수 값이 넘어 왔는지 확인합니다. 달리 말하면 정의된 매개변수 값이 넘어 왔는지 확인합니다. 달리 말하면 정의된 매개변수 값만 맏을 수 있고 추가로 인자를 받을 수 없다는 의미입니다.

```ts
function sum(a: number, b : number) : number {
  return a + b;
}
sum(10 , 20); // 30
sum(10 , 20 , 30); // error
sum(10); // error
```

- 위와 같은 특성은 정의된 매개변수의 갯수 만큼 임자를 넘기지 않아도 되는 자바스크립트의 특성과 반대됩니다.
-  만약 이러한 특성을 살리고 싶다면 `?`를 이용해서 정의하면 됩니다
  
```ts
function sum(a : number, b? : number) : number {
  return a + b;
}

sum(10 , 20); // 30
sum(10 , 20 , 30); // error
sum(10); // 10
```

> 자바스크립트는 정의된 매개변수 인자의 값을 넘기지 않아도 된다.


<br>

## RSET 문법이 적용된 매개변수

ES6 문법에서 지원하는 Rest 문법은 타입스크립트에서 다음과 같이 사용할 수 있습니다.

```ts
function num(a : number , ...numb : number[]) : number {
  const totalOfNums = 0;
  for(let key in nums){
    totalOfNums += nums[key];
  }
  return a + totalOfNums;
}
```

<br>

## this

타입스크립트에서 자바스크립트 `this`가 잘못 사용되었을 때 감지할 수 있습니다.

- 타입스크립트에서 `this`가 가리키는 것을 명시하려면 아래와 같은 문법을 사용합니다.

```ts
function 함수명(this : 타입){
  // ...
}
```

```ts
interface Vue {
  el : string;
  count : number;
  init(this : Vue) : () => {};
}

let vm: Vue = {
  el : '#app',
  count : 10,
  init: function(this: Vue){
    return () => {
      return this.count;
    }
  }
}

let getCount = vm.init();
let count = getCount();
console.log(count); // 10
```

> 위의 코드를 타입스크립트로 컴파일 했을 때 만일 `--noImplictThis` 옵션이 있더라도 에러가 발생하지 않습니다.

## 콜백에서의 this

앞에서 살펴본 일반적인 상황에서의 `this`와 다르게 콜백으로 함수가 전달되었을 때 `this`를 구분해줘야 할 때가 있습니다. 그럴땐 아래와 같이 강제가능합니다.

```ts
interface UIElement {
  // 아래 함수의 `this : void` 코드는 함수의 `this` 타입을 선언할 필요가 없다는 의미입니다.
  addClickListener(onclick : (this : void, e : Event) => void) : void;
}

class Handler {
  info: string,
  onClick(this : Handler, e : Event){
    // 위의 `UIElement` 인터페이스의 스펙에 `this`가 필요없다고 했지만 사용했기 때문에 에러가 발생합니다.
    this.info = e.message;
  }

  let handler = new Handler();
  uiElemnet.addClickListener(handler.onClick); // error !
}
```

만약 `UIElement` 인터페이스의 스펙에 맞춰 `Handler`를 구현하려면 아래와 같이 변경합니다.

```ts
class Handler {
  info: string;
  onClick(this : void , e : Event){
    // `this`의 타입이 void 이기 때문에 여기서 `this`를 사용할 수 없습니다.
    console.log('clicked!');
  }
}

let handler = new Handler();
uiElement.addClickListener(handler.onClick);
```