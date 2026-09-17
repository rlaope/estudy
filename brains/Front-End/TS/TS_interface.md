# Interface
인터페이스는 상호 간에 정의한 약속 혹은 규칙을 의미합니다.  
타입스크립트에서의 인터페이스는 보통 다음과 같은 범주에 대한 약속을 정의할 수 있습니다.
- 객체의 스펙(속성과 속성의 타입)
- 함수의 파라미터
- 함수의 스펙(파라미터, 반환 타입 등)
- 배열과 객체를 접근하는 방식
- 클래스

## interface
```ts
let person = { name : 'Huemang', age : 17};
function logAge(obj : { age : number }) {
  console.log(obj.age); // 17
}
logAge(person); // 17
```

위 logAge() 함수에서 받는 인자의 형태는 age를 속성으로 갖는 객체입니다. 이렇게 인자를 받을 때 단순한 타입 뿐만 아니라 객체의 속성 타입까지 정의 가능합니다.

```ts
interface personAge {
  age : number;
}

function logAge(obj : personAge) {
  console.log(obj.age);
}
let person = {name : 'Huemang' , age : 17};
logAge(person);
```

이제는 `logAge()`의 인자가 명시적을 바뀌었습니다. logAge의 인자는 personAge 라는 타입을 가져야합니다.

> 인터페이스를 인자로 받아 사용할 때 항상 인터페이스의 속성 갯수와 인자로 받는 객체의 속성 갯수를 일치시키지 않아도 됩니다.

> 즉 타입의 조건만 만족한다면 객체의 속성 갯수가 더 많아도 상관없다는 의미입니다.

<br>

## 옵션 속성
인터페이스를 사용할 때 인터페이스에 정의되어 있는 속성을 모두 다 꼭 사용하지 않아도 됩니다. 이를 옵션 속성이라고 합니다.

```ts
interface Person1{
  name : string;
  age? : number;
}

let myName = {
  name : 'Huemang'
}
function helloName(name : Person1){
  console.log(this.name); //Huemang
}

helloName(myName);
```

코드를 보면 helloName() 함수에서 Person1 인터페이스를 인자의 타입으로 선언했음에도 불구하고, 인자로 넘긴 객체에는 age 속성이 없습니다. 왜냐하면 옵션 속성으로 선언했기 때문입니다.

<br>

## 옵션 속성의 장점
옵션 속성의 장점은 단순히 인터페이스를 사용할 때 속성을 선택적으로 적용할 수 있다는 것 뿐만 아니라 인터페이스에 정의되어 있지 않은 속성에 대해서 인지시켜줄 수 있다는 점입니다.

<br>

## 읽기 전용 속성
읽기 전용 속성은 인터페이스로 객체를 처음 생성할 때만 값을 할당하고 그 이후에는 `변경할 수 없는 속성`을 의미합니다.  
문법은 다음과 같이 `readonly` 속성을 앞에 붙입니다.
```ts
interface IPerson {
  name : string;
  age? : number;
  readonly city : string;
}
```
인터페이스를 객체로 선언하고 나서 수정을 하려고하면 오류가 납니다.
```ts
let Huemang : IPerson = {
  name : 'huemang',
  age : 17,
  city : 'Gwangju'
}

Huemang.city = 'Seoul'; // error!!
```

<br>

## 읽기 전용 배열
배열을 선언할 때 `ReadonlyArray<T>` 타입을 사용하면 읽기 전용 배열을 생성할 수 있습니다.
```ts
let arr : ReadonlyArray<number> = [1,2,3];
arr.splice(0,1); // error
arr.push(4); //error
arr[0] = 100; // error
```
선언하는 시점에만 값을 정의할 수 있으니 주의해서 사용해야합니다.

<br>

## 객체 선언과 관련된 타입 체킹
타입스크립트는 인터페이스를 이용하여 객체를 선언할 때 좀 더 엄밀한 속성 검사를 진행합니다.
```ts
interface CraftBeer{
  brand? : string;
}

function brewBeer(beer : CraftBeer){
  //..
}
brewBeer({brandon : ' what'}); // eeror
```

`CraftBeer` 인터페이스에는 `brand`라고 선언되어 있지만 `brewBeer()` 함수에 인자로 넘기는 `myBeer` 객체에는 `brandon`이 선언되어 있어 오탈자 점검을 요하는 오류가 납니다.  
  
  
만약 이런 타입 추론을 무시하고 싶다면 아래와 같이 하면 됩니다.

```ts
let myBeer = {brandon : 'what'};
brewBeer(myBeer as CraftBeer);
```

그럼에도 불구하고 만약 인터페이스에 정의하지 않은 속성들을 추가로 사용하고 싶을 때는 아래와 같은 방법을 사용합니다.


```ts
interface CraftBeer {
  brand? : string;
  [propName : string] : any;
}
```