# 배열2

### .filter()

- map과 다르게 어떠한 배열데이터 안에 들어있는 각각의 아이템들을 기준에 대해서 기준에 맞는 `값`을 반환한다.
- 원본의 영향을 미치지않는다 `map()과 동일`
```js
 const numbers = [ 1 , 2 , 3 , 4];
const fruits = ['Apple' , 'Banana' , 'Cherry'];

const a = numbers.map(number => number < 3)
console.log(a); // [true ,true, false, false]

const b = numbers.filter(number => number < 3)
console.log(b); // [1,2]

console.log(numbers); // [ 1 , 2 , 3 , 4]

```

<br>

## .find() , findIndex()

- find()
  - 특정한 조건의 맞는 배열 `element`를 찾아서 반환한다.

- findIndex()
  - 특정한 조건의 맞는 배열 `element`를 찾아서 반환하고 몇번째 인덱스에 있는지까지 반환한다.
```js
const numbers = [ 1 , 2 , 3 , 4];
const fruits = ['Apple' , 'Banana' , 'Cherry'];

const a = fruits.find(fruit => {
  return /^B/.test(fruit) // '/^B/' 이건 정규표현식이라는 것이다 이건 나중에 알아보도록 하겠다. ( B로 시작하는걸 찾는 의미라고만 먼저 알아두자.)
})

console.log(a); // Banana

const b = fruits.findIndex(fruit => {
  return /^C/.test(fruit) 
})

console.log(b); // Cherry , 2
```

<br>

## .includes()

- 불린형으로 특정 조건의 값이 배열안에 element 값이 포함되어있는지 아닌지 반환한다.
```js
const numbers = [ 1 , 2 , 3 , 4];
const fruits = ['Apple' , 'Banana' , 'Cherry'];

const a = numbers.includes(3)
console.log(a) // true

const b = fruits.includes('Huemang');
console.log(b) // false
```

<br>

## .push() .unpush()

- 이 메서드는 원본이 수정된다. 
1. `push()` : 배열에 가장 뒷쪽에 특정한 인수를 삽입한다.
2. `unshift()` : 배열의 맨 앞쪽에 특정한 인수를 삽입한다.

```js
const numbers = [ 1 , 2 , 3 , 4];
const fruits = ['Apple' , 'Banana' , 'Cherry'];

numbers.push(5);
console.log(numbers) // [ 1 , 2 , 3 , 4 , 5 ]

numbers.unshift(0)
console.log(numbers) // [ 0 , 1 , 2 , 3 , 4 , 5 ]
```

<br>

## .reverse()
- 이 메소드는 원본이 수정된다.
- `reverse` 번역하면 뒤집다 말 그대로 배열의 순서를 반대로 뒤집는다.


```js
const numbers = [ 1 , 2 , 3 , 4];
const fruits = ['Apple' , 'Banana' , 'Cherry'];

numbers.reverse();
fruits.reverse();

console.log(numbers) // 4 , 3 , 2 , 1
console.log(fruits) // 'Cherry' , 'Banana' , 'Apple'
```

<br>

## .splice()
- 이 메소드는 원본이 수정된다.
- 사용법 : `splice(index번호, 지울 아이템 갯수 , index번호에 새로추가할 수)`
- 배열의 아이템을 지우는 메소드이다.
- 배열의 아이템을 끼워 넣을수도 있다.

```js
const numbers = [ 1 , 2 , 3 , 4];
const fruits = ['Apple' , 'Banana' , 'Cherry'];

numbers.splice(2,1) //index 2번부터 1개를 지워라.

console.log(numbers) // 1 , 2, 4
```

```js
const numbers = [ 1 , 2 , 3 , 4];
const fruits = ['Apple' , 'Banana' , 'Cherry'];

numbers.splice(2,0,999) // 지우지않고 2번인덱스에 999를 집어넣어라

console.log(numbers) // 1 , 2 , 999 , 3 , 4
```

## 전개 연산자

- 하나의 배열 데이터를 쉼표로 구분하는 각각의 아이템을 전개해서 출력한다.  

> 변수옆에 ...을 붙여 사용 

```js
const fruits = ['Apple' , 'Banana' , 'Cherry'];

console.log(...fruits)
// Apple Banana Cherry

function toObject(a,b,c){
  return {
    a: a,
    b: b,
    c: c
  }
}
console.log(toObject(...fruits))
```

- 전개 연산자를 사용하면 아이템들을 쉽게 출력할 수 있다.
- 매개변수 부분에도 전개연산자를 사용할 수 있다. `rest parameter` 이라고 불림
