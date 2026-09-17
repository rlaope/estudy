# Basic Js
` I've quickly skipped over the basics `

## Objects
- The basic data type in JavaScript is the object.
- An object is an unordered collection of properties, each consisting of a name and a value.
- A property's value can also be a function, in which case such a property is called a method.

Let's create my object, e.g.:

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

- Object Method Reference
The way to reference an object method is `objectName.methodName()`.

## Object Creation

1. Using literal notation

2. Using a constructor function

3. Using the Object.create() method
<br/> <br/>

#### Literal Notation
- The easiest way to create an object in JavaScript
```
let objectName = {

    propertyName1 : property1Value,

    propertyName2 : property2Value,

    ...

};
```
<br/>

#### Object Creation Using a Constructor
- The easiest way to create an object in JavaScript
```js
let objectName = {

   let day = new Date(); // Creates a Date type object using the new operator

  document.write("올해는 " + day.getFullYear() + "년입니다.");

};
```

#### Object Creation Using the Object.create() Method
- The Object.create() method creates a new object with the specified prototype object and properties.
```
let objectName = {
Object.create(prototypeObject[, newObjectProperty1, newObjectProperty2, ...]);
};
```
<br>

We will look at prototypes in more detail later.
