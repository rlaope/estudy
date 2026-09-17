# Interface2

## 함수 타입
인터페이스는 함수의 타입을 정의할 때에도 사용할 수 있습니다.
```ts
interface login {
  (username : string, password : string) : boolean;
}
```
함수의 인자의 타입과 반환 값의 타입을 정합니다.

```ts
let loginUser : login;
loginUser = function(id : string, pw : string){
  console.log('로그인 했습니다.');
  return true;
}
```

<br>

## 클래스 타입

C# 이나 JAVA 처럼 TS 에서도 클래스가 일정 조건을 만족하도록 타입 규칙을 정할 수 있습니다.

```ts
interface CraftBeer {
  beerName : string;
  nameBeer(beer : string ) : void;
}

class myBeer implements CraftBeer {
  beerName : string = 'Babu Guinness';
  nameBeer(b : string) {
    this.beerName = b;
  }
  constructor() {}
}
```

<br>

## 인터페이스 확장
클래스와 마찬가지로 인터페이스도 인터페이스간 확장이 가능합니다.
```ts
interface Person {
  name : string;
}
interface Developer extends Person {
  skill : string;
}

let fe = {} as Developer;
fe.name = "huemang";
fe.skill = "TypeScript";
```
혹은 아래와 같이 여러 인터페이스를 상속받아 사용할 수 있습니다.
```ts
interface Person {
  name : string;
}
interface Drinker extends Person{
  drink : string;
}
interface Developer extends Drinker{
  skill : string;
}
let fe = {} as Developer;
fe.name = 'Huemang';
fe.skill = "TypeScript";
fe.drink = "Beer";
```

<br>

## 하이브리드 타입
자바스크립트의 유연하고 동적인 타입 특성에 따라 인터페이스 역시 여러 가지 타입을 조합하여 만들 수 있습니다. 예를 들어, 다음과 같이 함수 타입이면서 객체 타입을 정의할 수 있는 인터페이스가 있습니다.

```ts
interface CraftBeer {
  (beer : string) : string;
  brand : string;
  brew() : void;
}

function myBeer() : CraftBeer {
  let my = (function(beer : string) {})  as CraftBeer;
  my.brand = 'Beer Kitchen';
  my.brew = function() {};
  return my;
}

let brewedBeer = myBeer();
brandBeer('My First Beer');
brewedBeer.brand = 'Pangyo Craft';
brewedBeer.brew();
```