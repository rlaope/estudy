# Interface2

## Function Types
Interfaces can also be used to define function types.
```ts
interface login {
  (username : string, password : string) : boolean;
}
```
They define the types of function arguments and return values.

```ts
let loginUser : login;
loginUser = function(id : string, pw : string){
  console.log('로그인 했습니다.');
  return true;
}
```

<br>

## Class Types

Similar to C# or Java, TypeScript also allows defining type rules for classes to satisfy certain conditions.

```ts
interface CraftBeer {
  beerName : string;
  nameBeer(beer : string ) : void;
}

class myBeer implements CraftBeer {
  beerName : string = 'Babu Guinness';
  nameBeer(b : string) {
    this.beerName = b;
  }
  constructor() {}
}
```

<br>

## Interface Extension
Just like classes, interfaces can also extend other interfaces.
```ts
interface Person {
  name : string;
}
interface Developer extends Person {
  skill : string;
}

let fe = {} as Developer;
fe.name = "huemang";
fe.skill = "TypeScript";
```
Alternatively, you can inherit and use multiple interfaces as shown below.
```ts
interface Person {
  name : string;
}
interface Drinker extends Person{
  drink : string;
}
interface Developer extends Drinker{
  skill : string;
}
let fe = {} as Developer;
fe.name = 'Huemang';
fe.skill = "TypeScript";
fe.drink = "Beer";
```

<br>

## Hybrid Types
Due to JavaScript's flexible and dynamic typing characteristics, interfaces can also be created by combining various types. For example, there are interfaces that can define both function types and object types, as shown below.

```ts
interface CraftBeer {
  (beer : string) : string;
  brand : string;
  brew() : void;
}

function myBeer() : CraftBeer {
  let my = (function(beer : string) {})  as CraftBeer;
  my.brand = 'Beer Kitchen';
  my.brew = function() {};
  return my;
}

let brewedBeer = myBeer();
brandBeer('My First Beer');
brewedBeer.brand = 'Pangyo Craft';
brewedBeer.brew();
```
