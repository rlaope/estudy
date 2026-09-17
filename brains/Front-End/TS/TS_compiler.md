# TypeScript 컴파일러

TypeScript는 babel로 JavaScript로 컴파일이 되는 과정을 수반한다. 이 때 어떻게 컴파일할 지에 대한 option이 다양한데, `tsconfig` 파일이 그 역할을 하게 된다.

## 1. Compiler Option
- tsconfig파일 내에서 컴파일이 필요한 여러 option을 설정할 수 있다. option은 아주 다양하다
- 대표적으로 : target, module, rootDir, allowJS, SourceMap 등이 있다.

<br>

### 1. target
- target에서 지정한 버전으로 TypeScript를 컴파일하게 된다. 일반적으로 `ES5` , `ES6`를 쓴다.

### 2. outDir
- TypeScript에서 컴파일된 파일을 어떤 디렉토리에 넣을 지 경로를 정한다.

```json
outDir : "./build"
```

### 3. rootDir
- TypeScript파일이 생성될 수 있는 최상위 디렉토리를 정하게 된다. 예를들어

```json
rootDir : "./src"
```

라고 지정하게 되면, 모든 ts파일은 src 디렉토리 안에 생성해야만 한다.

### 4. sourceMap
- `SourceMap : True` 는 디버깅에 유용하다.
  
디버깅은 브라우저에서 `개발자 도구 -source` 에서 주로 한다. 이 때 확인 가능한 파일은 컴파일된 js파일이므로 직접 작성한 ts파일과 상이하다. 이는 개발자에게 혼동을 야기한다.  
SourceMap은 컴파일 시 각 ts파일에 map파일을 생성하고, 이는 브라우저에서 디버그할 때 ts파일을 보며 디버깅이 가능하게 한다.

<br>

## 2. exclude && include
컴파일할 때, 특정 ts파일을 제외하거나 특정 ts파일만 한정지을 수 있다. exclude와 include가 그 기능을 한다
```json
exclude : ["./src/login.ts"], // login.ts 만 제외하고 모든 ts 파일 컴파일
include : ["./src/shopping.ts", "./str/buy.ts"] // shopping.ts , buy.ts만 컴파일
```

<br>


## 3. strict
- 엄격하게 타입을 확인하는 옵션들을 한번에 제어 
> 반드시 true로 설정할 것

### nolmplicitAny
- 컴파일러가 타입 추론중 `any`라고 판단하게 되면 컴파일 에러를 발생시켜 명시적으로 지정하도록 유도
  
```ts
function test(a){ // nolmplicitAny로 인한 에러 발생
  //Todo..
}

test(10) // 사용 불가
```

- `suppressImplicitAnyIndexErrors` : noImplicitAny 사용할 때, 인덱스 객체에 인덱스 signature 가 없는 경우 오류를 발생 하는데 이를 예외 처리

<br>

### nolmplicitThis 
- 명시적으로 `any` 타입을 사용하지 않고, this표현식을 사용시 에러발생
  
```ts
function test(name : string){
  this.name = name // 에러발생
}
```

> 해결 방안
> ```ts
> function test(this: any, name : string){
> this.name : name
> }
> ```
 
첫번째 매개변수 자리에 `this`를 넣고 타입 지정  
`TypeScript` 에서만 허용한 문법

> Class 에서는 this를 사용하면서 `noImplicitThis` 에러가 발생하지 않는다.
> 메소드에 자동으로 클래스 타입이 this로 지정된다.
> 단, `constructor` 함수의 첫번째 매개변수로 `this` 사용불가

<br>

### strictNullCheck
- 모든 타입에 자동으로 포함되어 있는 `null`과 `undefined` 제거
> 예외, undefined에 void 할당 가능

<br>

- strictNullCheck 설정 X
  
```ts
function test(a : number){
  if(a > 0){
    return a * 5;
  }
}
test(5) // 25
test(-5) // NaN 컴파일 에러 발생 x
```

-strictNullCheck 설정 o
```ts
function test(a : number){
  if(a > 0){
    return a * 5;
  }
}
test(5) // 25
test(-5) // 컴파일 에러 발생 o (undefined)
```

<br>

### strictFunctionTypes 옵션
- 함수를 할당할 시에 함수의 매개변수 타입이 같거나 슈퍼타입인 경우(반공변)가 아닌 경우 컴파일 에러 발생

```ts
class Person{}
class Man extends Person{}
class Boy extends Man{}

function tset(f : (p : Man) => Man){}

test((p : Boy) : Man => {}) // 매개변수가 하위타입인 경우 컴파일 에러
```

> `반공변` : 함수의 매개변수 타입만 같거나 슈퍼타입인 경우 할당 가능
> `공변` : 같거나 서브 타입인 경우 할당 가능
> ```ts
> let sub : string = ''
> let sup : string | number = sub // 가능
> ```
> 함수의 매개변수는 반공변(같거나 상위)이여야 하고 반환값은 공변(같거나 하위)이여야 한다.


<br>

## 4. Conclusion
tsconfig 파일 내에 option들을 머리에 다 외우고 다닐 수 없다. 그래서 파일 가장 상단에 링크를 타고가서 option들을 확인해보고 나에게 필요한 것만 이용할 수 있도록 해야한다.
