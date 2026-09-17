# Prototype

### What is a Prototype?
- Every object in JavaScript has an object called `prototype`.
- All objects inherit `properties` and `methods` from their prototype.
- Thus, every object in JavaScript inherits from at least one other object, and the object that provides the inherited information is called the `prototype`.

<br>

### Prototype Chain
- The Object.prototype object does not have any prototype and does not inherit any properties.
- In JavaScript, objects of the same type created using object initializers all share the same prototype.

<br>

### Creating Prototypes
- The most basic way to create a prototype is to write an object constructor function.
- By writing a constructor function and creating objects using the `new` operator, you can create objects that share the same prototype.
```js
function Dog(color, name, age) { // 개에 관한 생성자 함수를 작성함.

    this.color = color;          // 색에 관한 프로퍼티

    this.name = name;            // 이름에 관한 프로퍼티

    this.age = age;              // 나이에 관한 프로퍼티

}

let myDog = new Dog("흰색", "희망", 17); // 이 객체는 Dog라는 프로토타입을 가짐.

document.write("우리 집 강아지는 " + myDog.name + "라는 이름의 " + myDog.color + " 털이 매력적인 강아지입니다.");
```

<br>

### Prototype Property

- It is a property that only `function objects` possess.

- When a function object is used as a constructor, it points to the object (prototype object) that acts as the parent of the objects to be created through this function.

<br>

### Adding Properties and Methods to Objects

```js
function Dog(color, name, age) {
    this.color = color;

    this.name = name;
    
    this.age = age;
}
let myDog = new Dog("흰색", "희망", 1);

myDog.family = "잡종"; // 품종에 관한 프로퍼티를 추가함.

myDog.breed = function() {        // 털색을 포함한 품종을 반환해 주는 메소드를 추가함.

    return this.color + " " + this.family;
}

console.log("우리 집 강아지는 " + myDog.breed() + "입니다.");

```

### Extending Prototype Objects

- Since a prototype object is also an object, properties can be added/deleted just like with regular objects. These added/deleted properties are immediately reflected in the prototype chain.

<br>

![Extending Prototype Objects](https://poiemaweb.com/img/extension_prototype.png)
