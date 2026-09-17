# Interface
An interface signifies a mutually defined promise or rule.
In TypeScript, interfaces can typically define promises for the following categories:
- Object specifications (properties and their types)
- Function parameters
- Function specifications (parameters, return types, etc.)
- Ways to access arrays and objects
- Classes

## interface
```ts
let person = { name : 'Huemang', age : 17};
function logAge(obj : { age : number }) {
  console.log(obj.age); // 17
}
logAge(person); // 17
```

The argument received by the `logAge()` function above is an object with an `age` property. When receiving arguments like this, you can define not only simple types but also the property types of an object.

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

Now, the argument for `logAge()` has explicitly changed. The argument for `logAge` must have the `personAge` type.

> When using an interface as an argument, you don't always have to match the number of properties in the interface with the number of properties in the object passed as an argument.

> In other words, it's fine if the object has more properties, as long as it satisfies the type conditions.

<br>

## Optional Properties
When using an interface, you don't necessarily have to use all the properties defined in it. This is called an optional property.

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

Looking at the code, even though the `helloName()` function declares the `Person1` interface as the argument's type, the object passed as an argument does not have an `age` property. This is because it was declared as an optional property.

<br>

## Advantages of Optional Properties
The advantage of optional properties is not just that you can selectively apply properties when using an interface, but also that it can make you aware of properties not defined in the interface.

<br>

## Read-only Properties
A read-only property means a property whose value can only be assigned when the object is first created via an interface, and `cannot be changed` thereafter.
The syntax involves prefixing the property with `readonly` as follows.
```ts
interface IPerson {
  name : string;
  age? : number;
  readonly city : string;
}
```
If you try to modify an interface property after declaring it as an object, an error will occur.
```ts
let Huemang : IPerson = {
  name : 'huemang',
  age : 17,
  city : 'Gwangju'
}

Huemang.city = 'Seoul'; // error!!
```

<br>

## Read-only Arrays
When declaring an array, you can create a read-only array by using the `ReadonlyArray<T>` type.
```ts
let arr : ReadonlyArray<number> = [1,2,3];
arr.splice(0,1); // error
arr.push(4); //error
arr[0] = 100; // error
```
Values can only be defined at the time of declaration, so use with caution.

<br>

## Type Checking Related to Object Declaration
TypeScript performs stricter property checks when declaring objects using interfaces.
```ts
interface CraftBeer{
  brand? : string;
}

function brewBeer(beer : CraftBeer){
  //..
}
brewBeer({brandon : ' what'}); // eeror
```

The `CraftBeer` interface declares `brand`, but the `myBeer` object passed as an argument to the `brewBeer()` function declares `brandon`, resulting in an error that requires a typo check.


If you want to ignore this type inference, you can do so as follows.

```ts
let myBeer = {brandon : 'what'};
brewBeer(myBeer as CraftBeer);
```

Nevertheless, if you want to use additional properties not defined in the interface, you can use the following method.

```ts
interface CraftBeer {
  brand? : string;
  [propName : string] : any;
}
```
