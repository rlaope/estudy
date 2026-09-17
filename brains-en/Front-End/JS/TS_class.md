# Class
- A blueprint for creating objects
- Before classes, the basic way to create objects was using functions.
- Classes are available in JavaScript starting from ES6.
- A cornerstone for OOP.
- In TypeScript, classes are also one of the types created by the user. More powerful than JS.
- class = one of the types created by the user

<br>

## class
- You can create a class using the `class` keyword.
- Names usually start with an uppercase letter.
- You can create an object using `new`.
- You can pass values while creating an object using a constructor.
- You can refer to the created object using `this`.
- When compiled to JS, it changes to a function in the case of ES5.

```ts
class Person {
  name;
  constructor(name : string){
    this.name = name;
  }
}

const p1 = new Person("Minuuuk");
```

<br>

## constructor & initialize
- If there is no constructor function, the default constructor is called.
- If there is at least one constructor created by the programmer, the default constructor disappears.
- In strict mode, values must be assigned either where properties are declared or in the constructor.
- If a class property is defined but no value is assigned, it is `undefined`.
- You cannot set `async` on a constructor.
  
```ts
class Person{
  name : string = "mark"
  age : number;

  constructor(age? :number){
    if(age === undefined){
      this.age = 20;
    } else {
      this.age = age;
    }
  }
}

const p1 : Person = new Person(17);
const p2 : Person = new Person();
```

<br>

## 접근 제어자
- Sets the accessibility of properties within a class.
- Access modifiers include `public`, `private`, and `protected`.
- If not set, it is `public`.
- Can be set anywhere within the class (constructor, properties, methods).
- If set to `private`, it cannot be accessed from outside the class.
- Since JavaScript did not support `private`, for a long time, it was expressed by prefixing property or method names with an underscore (`_`).

```ts
class Person {
  public name : string = "Huemang";
  private age : nuber;

  public constructor(age? : number){
    if(age === undefined){
      this.age = 20;
    }
    else {
      this.age = age;
    }
  }

  public async init(){

  }
}

const p1: Person = new Person(17);
```
<br>

## initialization in constructor parameters

- A method to initialize class properties by receiving parameters in the constructor.
  
```ts
class Person {
  public constructor(public name : string , private age : number){}
}

const p1 : Person = new Person("Mark",39);
console.log(p1);
```

<br>

## Getters & Setters
```ts
class Person {
  public constructor(private _name : string , private age : number){}
  get name(){
    // Return is mandatory
    return this._name + "Kim"
  }

  set name(n : string){
    // Can check how many times it has been set using a count.
    this._name = n;
  }
}


const p1: Person = new Person("Mark", 39);
console.log(p1.name); // Get, function that gets: getter
p1.name = 'Woongjae'; // Set, function that sets: setter
console.log(p1.name);

```

<br>

## readonly properties
- A method that can only `get` but not `set`.
- `readonly`: Used to prevent modification.
  
```ts
class Person{
  public readonly name : string = 'Mark';
  private readonly country : string;
  public constructor(private _name : string, private age : number){
    this.country = "Korea";
  }
  hello(){
    // Since it's `readonly` and `private`, modification is not possible.
    //this.country = "japan";
  }
}
const p1 : Person = new Person("huemang" , 17);
```

<br>

## 8. Index Signatures in class
- A point to consider when properties are not fixed but change dynamically.

```ts
class Students {
  [index : string] : 'male' | 'female';

  mark : "male" = "male";
}

const a = new Students();
a.mark = "male";
a.jade = "male";

const b = new Students();
b.chloe = "female";
b.alex = "male";
b.anna = "femal
```

<br>

## Static Properties & Methods
```ts
class Person {
  public static CITY = "Seoul";
  public hello(){
    console.log("Hello! " , Person.CITY);
  }
  public change() {
    Person.CITY = "LA";
  }
}

const p1 = new Person();
p1.hello(); // Hello Seoul

const p2 = new Person();
p2.hello(); // Hello Seoul
p1.change(); // 'Change to LA'
p2.hello(); // Hello LA

// `static`: Parts to be used in common.
// Person.hello();
// Person.CITY;
```

<br>

## Singletons
- A pattern where only a single object is created and executed from a single class during application runtime.
  
```ts
class ClassName{
  private static instance : ClassName | null = null;
  // Set to `private` to prevent direct creation with `new`.
  public static getInstance(){
    // If an object created from `ClassName` exists, return it.
    // Otherwise, create and return it.
    if(ClassName.instance === null){
      ClassName.instance = new ClassName();
    }

    return ClassName.instance;
  }
  private constructor(){}
}

const a = ClassName.getInstance();
const b = ClassName.getInstance();

console.log(a === b); // true
```

<br>

## 상속
- When a class takes another class and adds its own properties to use it.

```ts
class Parent{
  // `protected`: Cannot be used externally, but can be used when inherited.
  constructor(protected _name: string , private _age : number){}

  public print() : void {
    console.log(`My name is ${this._name} and I am ${this._age} years old. `);
  }

  protected printName() : void {
    console.log(this._name, this._age)
  }
}

// const p = new Parent("Makr", 17);
// p.print(); My name is Mark and I am 39 years old.

class Child extends Parent {
  // Inherits the constructor of `Parent` as is.
  public gender = 'male';
  // override
  // public _name = "Mark Jr";
  // Constructor override
  constructor(age : number){
    super('Mark Jr', age)
    this.printName();
  }
}

const c = new Child(1);
c.print()
// Mark Jr. 1
// My name is Mark Jr. and I am 1 year old.

// As applications become complex, it's good to plan and distinguish what each of the parent and child areas can do.
```

<br>

## Abstract Classes
- Using an incomplete object by making it complete through inheritance.
  
```ts
abstract class AbstractPerson{
  protected _name : string = 'Mark';

  abstract setName(name : string) : void;

  // new AbstractPerson() x
}

class Person extends AbstractPerson{
  setName(name : string) : void {
    this._name = name;
  }
}

const p = new Person();
```
