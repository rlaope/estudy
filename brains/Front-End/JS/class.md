# 클래스

### 클래스란 ?
- ```클래스는 객체 지향 프로그래밍에서 특정 객체를 생성하기 위해 변수와 메소드를 정의하는 일종의 틀로, 객체를 정의하기 위한 상태(멤버 변수)와 메서드(함수)로 구성됨```

<br>

실무에선 사용자나 물건같이 동일한 종류의 객체를 여러 개 생성해야 하는 경우가 잦습니다.  

이럴 때 new 연산자와 생성자를 사용한`new function`을 사용할 수 있습니다.

여기에 더하여 모던 자바스크립트에 도입된 `클래스(class)`라는 문법을 사용하면 객체 지향 프로그래밍에서 사용되는 다양한 기능을 자바스크립트에서도 사용할 수 있습니다.

<br>

### 기본 문법

```js
class Myclass{
  constructor(){...}
    method(){...}
}
```

- 이렇게 클래스를 만들고, `new MyClass()`를 호출하면 내부에서 정의한 메서드가 들어있는 객체가 생성됨

- 객체의 기본 상태를 설정해주는 메서드 `constructor()` 는 `new`에 의해 자동으로 호출되므로, 특별한 절차 없이 객체를 초기화 할 수 있습니다.

```js
class User{
  constructor(name){
    this.name = name;
  }
  sayHi(){
    console.log(this.name);
  }
}

let user = new User("Huemang");
user.sayHi();

```
<br>

`new User("huemang");`을 호출하면 이런 일이 생깁니다.  

1. 새로운 객체가 생성됩니다.
2. 넘겨받은 인수와 함께 `constructor`가 자동으로 실행됩니다. 이때 인수 `huemang`이 `this.name`에 할당됩니다 이런 과정을 거친 후에 `user.sayHi()`같은 객체 메서드를 호출할 수 있습니다.

<br>

## 상속(확장)

### extends 키워드

클래스 `Vehicle`을 만들어 보겠습니다

```js
class Vehicle{
  constructor(name, wheel){
    this.name = name
    this.wheel = wheel
  }
}
```

확장(상속)을 한번 해보겠습니다.
```js
class Vehicle{
  constructor(name, wheel){
    this.name = name
    this.wheel = wheel
  }
}

const myVehicle = new Vehicle('운송수단', 2)
console.log(myVehicle); //myVehicle 객체를 보여줌

class Bicycle extends Vehicle{
  constructor(name,wheel){
    super(name , wheel) // super의 인수가 Vehicle 으로 감
  }
}
const myBicycle = new Bicycle('삼천리',2);
const daughterBicycle = new Bicycle('세발',3);

console.log(myBicycle); // 삼천리와 2가 들어감
console.log(daughterBicycle);//세발과 3이 들어감

//확장

class Car extends Vehicle{
  constructor(name,wheel,lincese){
    super(name,wheel)
    this.license = license // 새로운 인수를 추가
  }
}

const myCar = new Car('벤츠',4,true);
const daughterCar = new Car('포르쉐',4,false);

console.log(myCar); // 벤츠 4 true license내용이 추가된것을 확인할 수 있다
console.log(daughterCar); // 포르쉐 4 false
```

<br>

- 이렇게 보면 클래스는 확장 상속이 가능하다는 것을 알 수 있다.

- 클래스는 실무에서 같은 작업이 반복되는 경우가 많기 때문에 많이쓰인다 그렇기때문에 잘 알아둬야 한다 !
