# Constructor, Access Modifiers, Getter and Setter

## Constructor

> Object-oriented languages have constructors.

- Every `Class` has a method called `constructor`. It is called when an object is created from the class and is responsible for `initializing` the object.

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

// Create a new instance
let user1 = new User('Huemang', 17 , "광주");
user1.printUserInfo();
```

<br>

## Access Modifiers
> Access modifiers are keywords that can be applied to member variable properties and methods within a class. They can control access from outside the class.

- `public`: Accessible from outside the class (default)
- `private`: Accessible only within the class, not from outside the class (private member)
- `protected`: Accessible within the class and by inherited child classes

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
console.log(user1.name); // error: Access from outside the class
```

<br>

## Using Getters and Setters
```ts
class User{
    private _name : string; // _ is an implicit convention to indicate a private member
  age : number;
  address : string;

  constructor(name : string , age : number , address : string) {
    this.name = name;
    this.age = age;
    this.address = address;
  }

  // getter
  get name () {
    return this._name;
  }

  // setter
  set name () {
    return this._name = value;
  }

   printUserInfo = () : void => {
    console.log(`${name}의 나이는 ${age} 이고 ${address}에 삽니다.`);
  }
}

let user1 = new User('Huemang' 17, "Gwangju");
console.log(user1.name) // "Huemang", retrieved using the getter
// This is because `user1.name` calls the getter and setter, not the `_name` member directly.

let user2 = new User('Huemang' 17, "Gwangju");
user.name = "Hope"; // Set the member's name using the setter
console.log(user2.name) // Hope, retrieved using the getter
```

<br>

## Constructor and Access Modifiers

> In TypeScript, you can directly apply Access Modifiers to the parameters of a Constructor, allowing for a more concise representation as shown in the code below.

```ts
class User {

  // That is, when an object is created, the values passed as parameters to the constructor
  // are automatically initialized and assigned as property values of the object.
  constructor(private name : string, private age : number, public address : string) {}

  get name () {
    return this._name;
  }

  set name() {
    this._name = value;
  }

  // A function defined within a class is called a method.
  printUserInfo = (): void => {
    console.log(`${name}의 나이는 ${age} 이고 ${address}에 삽니다.`);
  }
}

let user1 = new User('Huemang' 17, "Gwangju");
console.log(user1.name) // "Huemang", retrieved using the getter
// This is because `user1.name` calls the getter and setter, not the `_name` member directly.

let user2 = new User('Huemang' 17, "Gwangju");
user.name = "Hope"; // Set the member's name using the setter
console.log(user2.name) // Hope, retrieved using the getter
```
