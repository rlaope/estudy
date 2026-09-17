# Basic Js
` 기초적인 것들은 빠르게 생략했습니다 `

## 객체
- 자바스크립트의 기본 타입(data type)은 객체(object)입니다.
- 객체란 이름(name)과 값(value)으로 구성된 프로퍼티(property)의 정렬되지 않은 집합입니다.
- 프로퍼티의 값으로 함수가 올 수도 있는데, 이러한 프로퍼티를 메소드(method)라고 합니다.

나의 객체를 만들어보자 ex )

```js
const hope = {
  name : '김희망',
  age : 17,
  Introduce : function(){
    console.log(`안녕하세요 저는 ${this.name} ${this.age}살 입니다);
  }
};

hope.name //김희망
hope.age // 17
hope.Introduce(); // 안녕하세요 저는 김희망 17살 입니다.
```

- 객체 메소드 참조  
객체 메소드 참조하는 방법 `객체이름.메소드이름()` 

## 객체의 생성

1. 리터럴 표기(literal notation)를 이용한 방법

2. 생성자 함수(constructor function)를 이용한 방법

3. Object.create() 메소드를 이용한 방법
<br/> <br/> 

#### 리터럴 표기
- 자바스크립트에서 객체를 생성하는 가장 쉬운 방법 
```
let 객체이름 = {

    프로퍼티1이름 : 프로퍼티1의값,

    프로퍼티2이름 : 프로퍼티2의값,

    ...

}; 
```
<br/> 

#### 생성자를 이용한 객체의 생성
- 자바스크립트에서 객체를 생성하는 가장 쉬운 방법 
```js
let 객체이름 = {

   let day = new Date(); // new 연산자를 사용하여 Date 타입의 객체를 생성

  document.write("올해는 " + day.getFullYear() + "년입니다.");

}; 
```

#### Object.create() 메소드를 이용한 객체의 생성
- Object.create() 메소드는 지정된 프로토타입(prototype) 객체와 프로퍼티를 가지고 새로운 객체를 만들어 줍니다.
```
let 객체이름 = {
Object.create(프로토타입객체[, 새로운객체의프로퍼티1, 새로운객체의프로퍼티2, ...]);
}; 
```
<br>

프로토타입에 대해서 더 자세한건 다음에서 보겠습니다.


