# Class

### What is a Class?
- A class is a blueprint in object-oriented programming that defines variables and methods to create specific objects. It consists of states (member variables) and methods (functions) to define an object.

<br>

In practice, it's often necessary to create multiple objects of the same type, such as users or items.

In such cases, you can use the `new` operator and a constructor with `new function`.

Additionally, by using the `class` syntax introduced in modern JavaScript, you can utilize various features of object-oriented programming in JavaScript.

<br>

### Basic Syntax

```js
class Myclass{
  constructor(){...}
    method(){...}
}
```

- When you create a class like this and call `new MyClass()`, an object containing the methods defined within it is created.

- The `constructor()` method, which sets the initial state of an object, is automatically called by `new`, allowing you to initialize the object without special procedures.

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

When `new User("huemang");` is called, the following happens:

1. A new object is created.
2. The `constructor` is automatically executed with the passed arguments. At this point, the argument `huemang` is assigned to `this.name`. After this process, object methods like `user.sayHi()` can be called.

<br>

## Inheritance (Extension)

### The `extends` keyword

Let's create a `Vehicle` class.

```js
class Vehicle{
  constructor(name, wheel){
    this.name = name
    this.wheel = wheel
  }
}
```

Now, let's try extending (inheriting).
```js
class Vehicle{
  constructor(name, wheel){
    this.name = name
    this.wheel = wheel
  }
}

const myVehicle = new Vehicle('운송수단', 2)
console.log(myVehicle); //Shows the myVehicle object

class Bicycle extends Vehicle{
  constructor(name,wheel){
    super(name , wheel) // super's arguments go to Vehicle
  }
}
const myBicycle = new Bicycle('삼천리',2);
const daughterBicycle = new Bicycle('세발',3);

console.log(myBicycle); // Contains Samcheolli and 2
console.log(daughterBicycle);//Contains Sebal and 3

//Extension

class Car extends Vehicle{
  constructor(name,wheel,lincese){
    super(name,wheel)
    this.license = license // Adds a new argument
  }
}

const myCar = new Car('벤츠',4,true);
const daughterCar = new Car('포르쉐',4,false);

console.log(myCar); // Benz 4 true, you can see the license content has been added
console.log(daughterCar); // Porsche 4 false
```

<br>

From this, we can see that classes support extension and inheritance.

Classes are widely used in practice because the same tasks are often repeated, so it's important to understand them well!
