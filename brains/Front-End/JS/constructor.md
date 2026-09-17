# Constructor(생성자) , 접근제한자 , Getter 와 Setter


## constructor

> 객체지향 언어어에는 constructor 생성자가 있습니다.

- 모든 Class는 `constructor`라는 메소드를 가집니다. class로 부터 객체를 생성할 때 호출되며, 객체의 `초기화`를 담당하고 있습니다.

```ts
class User {
  //class 내에 정의된 변수는 property라고 함.
  name : string;
  age : number;
  address : string;

  constructor(name : string , age : number , address : string) {
    this.name = name;
    this.age = age;
    this.address = address;
  }
  //class 내에 정의된 함수는 메소드라고 함.
  printUserInfo = () : void => {
  console.log(`${name}의 나이는 ${age} 이고 ${address}에 삽니다.`)
  }
}

//새로운 인스턴스 생성
let user1 = new User('Huemang', 17 , "광주");
user1.printUserInfo();
```

<br>

## Access Modifier (접근 제한자)
> 접근 제한자에는 class 속 멤버 변수 프로퍼티와 메소드에 적용될 수 있는 키워드가 있습니다. class 외부로 부터 접근을 통제할 수 있습니다.

- `public` : 클래스의 외부에서 접근 가능 (default)
- `private` : 클래스 내에서만 접근 가능, 클래스의 외부에서 접근 불가능(비공개 멤버)
- `protected` : 클래스 내부, 그리고 상속받은 자식 클래스에 접근 가능

```ts
class User {
  private name : string;
  age : number;
  address : string;

  constructor(name : string , age : number , address : string) {
    this.name = name;
    this.age = age;
    this.address = address;
  }

  printUserInfo = () : void => {
    console.log(`${name}의 나이는 ${age} 이고 ${address}에 삽니다.`);
  }
}

let user1 = new User('huemang', 17 '광주');
console.log(user1.name); // error 클래스 외부에서의 접근
```

<br>

## Getter, Setter 사용
```ts
class User{
    private _name : string; // _ private 멤버를 나타내는 암묵적인 컨벤션
  age : number;
  address : string;

  constructor(name : string , age : number , address : string) {
    this.name = name;
    this.age = age;
    this.address = address;
  }

  //getter
  get name () {
    return this._name;
  }

  //setter
  set name () {
    return this._name = value;
  }

   printUserInfo = () : void => {
    console.log(`${name}의 나이는 ${age} 이고 ${address}에 삽니다.`);
  }
}

let user1 = new User('Huemang' 17, "Gwangju");
console.log(user1.name) // "Huemang"을 getter을 사용해 가져옴
// user1.name은 _name 멤버를 호출하는게 아닌 getter와 setter을 호출하기 때문이다.

let user2 = new User('Huemang' 17, "Gwangju");
user.name = "Hope"; //setter를 사용하여 멤버의 이름을 설정
console.log(user2.name) // Hope getter을 사용해 가져옴
```

<br>

## Constructor 와 Access Modifiers

> TypeScript에서는 Constructor의 매개변수에 Access Modifiers를 직접 적용하여 아래의 코드처럼 짧게 나타낼 수 있다.

```ts
class User {

  // 즉 , 객체가 생성될 때, 생성자의 매개변수로 전달된 값은 , 객체의 프로퍼티 값으로
  // 자동으로 그 값이 초기화되고 할당된다.
  constructor(private name : string, private age : number, public address : string) {}

  get name () {
    return this._name;
  }

  set name() {
    this._name = value;
  }

  // class 내에 정의된 함수는 method 메소드 라고 부른다.
  printUserInfo = (): void => {
    console.log(`${name}의 나이는 ${age} 이고 ${address}에 삽니다.`);
  }
}

let user1 = new User('Huemang' 17, "Gwangju");
console.log(user1.name) // "Huemang"을 getter을 사용해 가져옴
// user1.name은 _name 멤버를 호출하는게 아닌 getter와 setter을 호출하기 때문이다.

let user2 = new User('Huemang' 17, "Gwangju");
user.name = "Hope"; //setter를 사용하여 멤버의 이름을 설정
console.log(user2.name) // Hope getter을 사용해 가져옴
```