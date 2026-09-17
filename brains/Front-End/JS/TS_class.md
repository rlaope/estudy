# Class
- object를 만드는 blueprint
- 클래스 이전에 object를 만드는 기본적인 방법은 function
- JavaScript에도 class는 ES6 부터 사용가능
- OOP 을 위한 초석
- TypeScript 에서는 클래스도 사용자가 만드는 타입의 하나. JS 보다 강력.
- class = 사용자가 만드는 type의 하나

<br>

## class
- class 키워드를 사용하여 클래스를 만들 수 있다.
- 이름은 보통 대문자를 이용한다.
- new를 이용해서 object를 만들 수 있다.
- constructor를 이용하여 object를 생성하면서 값을 전달할 수 있다.
- this 를 이용해서 만들어진 object를 가리킬 수 있다.
- js로 컴파일되면 es5의 경우 function으로 변경된다.

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
- 생성자 함수가 없으면 , 디폴트 생성자가 불린다.
- 프로그래머가 만든 생성자가 하나라도 있으면 , 디폴드 생성자는 사라진다.
- strict 모드에서는 프로퍼티를 선언하는 곳 또는 생성자에서 값을 할당해야 한다.
- 클래스의 프로퍼티가 정의되어 있지만, 값을 대입하지 않으면 undefined 이다.
- 생성자에는 async를 설정할 수 없다.
  
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
- 클래스 내 프로퍼티의 접근 가능성 설정.
- 접근 제어자에는 public, private , protected가 있다.
- 설정하지 않으면 public 이다.
- 클래스 내부의 모든 곳에 (생성자 , 프로퍼티 , 메서드) 설정 가능하다.
- pricate 으로 설정하면 클래스 외부에서 접근할 수 없다.
- 자바스크립트에서 private를 지원하지 않아 오랫동안 프로퍼티나 메서드 이름 앞에 _를 붙여서 표현했다.

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

- 생성자의 파라미터를 받아서 그 클래스의 프로퍼티로 초기화 하는 방법.
  
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
    // return 필수
    return this._name + "Kim"
  }

  set name(n : string){
    // count를 사용해 몇번 set되었는지 체크가능.
    this._name = n;
  }
}


const p1: Person = new Person("Mark", 39);
console.log(p1.name); // Get, get을 하는 함수 : getter
p1.name = 'Woongjae'; // Set, set을 하는 함수 : setter
console.log(p1.name);

```

<br>

## readonly properties
- set은 할 수 없고 get만 할 수 있는 메서드
- readonly : 수정불가능하게 막아 놓을 때 사용한다.
  
```ts
class Person{
  public readonly name : string = 'Mark';
  private readonly country : string;
  public constructor(private _name : string, private age : number){
    this.country = "Korea";
  }
  hello(){
    // readonly 이고 private이므로 수정 불가
    //this.country = "japan";
  }
}
const p1 : Person = new Person("huemang" , 17);
```

<br>

## 8. Index Signatures in class
- property가 고정된 것이 아닌 동적으로 바뀌는 상황에서 고려해볼 만한 부분.

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
    console.log("안녕하세요 ! " , Person.CITY);
  }
  public change() {
    Person.CITY = "LA";
  }
}

const p1 = new Person();
p1.hello(); // 안녕하세요 Seoul

const p2 = new Person();
p2.hello(); // 안녕하세요 Seoul
p1.change(); // 'LA로 변경'
p2.hello(); // 안녕하세요 LA

// static : 공통적으로 사용할 부분들.
// Person.hello();
// Person.CITY;
```

<br>

## Singletons
- 어플리케이션이 실행되는 중간에 단 클래스로부터 단 하나의 오브젝트만 생성해서 실행하는 패턴
  
```ts
class ClassName{
  private static instance : ClassName | null = null;
  // private 로 new 로 직접 생성할 수 없도록 막음.
  public static getInstance(){
    // ClassName 으로부터 만든 Object가 있으면 그걸 리턴
    // 없으면, 만들어서 리턴.
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
- 클래스가 다른 클래스를 가져다가 자신만의 속성을 추가해서 사용하는 것.

```ts
class Parent{
  // protected : 외부에서는 사용 불가하지만 상속하면 사용 가능.
  constructor(protected _name: string , private _age : number){}

  public print() : void {
    console.log(`내이름은 ${this._name} 이고, 나이는 ${this._age} 입니다. `);
  }

  protected printName() : void {
    console.log(this._name, this._age)
  }
}

// const p = new Parent("Makr", 17);
// p.print(); 이름은 Mark이고 , 나이는 39입니다.

class Child extends Parent {
  // Parent의 생성자를 그대로 가져옴.
  public gender = 'male';
  // override
  // public _name = "Mark Jr";
  // 생성자 override
  constructor(age : number){
    super('Mark Jr', age)
    this.printName();
  }
}

const c = new Child(1);
c.print()
// Mark Jr. 1
// 이름은 Mark Jr. 이고 , 나이는 1 입니다.

// 애플리케이션이 복잡해지면 부모와 자식의 영역에서 각각 할 수 있는 것을 계획하고 구분하는 것이 좋음.
```

<br>

## Abstract Classes
- 완전하지 않는 객체를 상속을 통해서 완전하게 하여 사용.
  
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