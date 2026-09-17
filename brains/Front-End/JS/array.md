# 배열

`배열`이란 ?
- 배열(array)은 1개의 변수에 여러 개의 값을 순차적으로 저장할 때 사용한다.
- 자바스크립트의 배열은 객체이며 유용한 내장 메소드를 포함하고 있다.
- 배열은 Array 생성자로 생성된 Array 타입의 객체이며 프로토타입 객체는 Array.prototype이다.

<br>

## 배열 메소드

### Array.prototype.find()
- `find()` 메서드는 주어진 판별 함수를 만족하는 `첫 번째 요소`의 값을 반환한다. 없다면 `undefined`를 반환

```js
const array1 = [5, 12, 8 , 130, 44];
const found = array1.find(element => element > 10);
console.log(found); // 12
```

1. array1 안에 0번째 인덱스부터 find 함수 인수로 화살표 함수를 사용했고 그 안에서 조건이 만족하지않으면 콜백 만족하면 반환을 한다.
2. console.log() 통해서 12가 나오는 것을 볼 수 있다.

<br>


`callback` :  배열의 각 값에 대해 실행할 함수. 아래의 세 인자를 받는다.
1. element : 콜백함수에서 처리할 현재 요소.
2. index : 콜백함수에서 처리할 현재 요소의 인덱스
3. array : `find`함수를 호출한 배열.    
  
 `thisArg` : 선택 항목. 콜백이 호출될 때 this로 사용할 객체.

 ## 배열 API

 ### .length
 - 배열의 길이를 알려준다
 
 ```js
 const numbers = [ 1 , 2 , 3 , 4];
 const fruits = ['Apple' , 'Banana' , 'Cherry'];

console.log(numbers.length); // 4
console.log(fruits.length); // 3
console.log([1,2].length]); // 2
console.log([].length); // 0
 ```

 `.length` 는 배열안에 값이 있는지 없는지 확인하는 용도로 자주 쓰인다.

 <br>

 ### .concat()

 - 두개의 배열 데이터를 `병합`해서 새로운 배열데이터를 반환한다.
 > 원본의 데이터는 손상되지 않는다.

 ```js
const numbers = [ 1 , 2 , 3 , 4];
const fruits = ['Apple' , 'Banana' , 'Cherry'];

console.log(numbers.concat(fruits)) // 1 , 2 , 3 , 4 , 'Apple' , 'Banana' , Cherry

console.log(numbers); // 1 , 2 , 3 , 4
console.log(fruits); // 'Apple' , 'Banana' , 'Cherry'
 ```

 ### .forEach()
  - 메소드가 붙어있는 배열데이터의 아이템의 갯수만큼 인수로 사용된 콜백함수가 실행되는 메소드다.  

 ```js
const numbers = [ 1 , 2 , 3 , 4];
const fruits = ['Apple' , 'Banana' , 'Cherry'];

fruits.forEach(function (element, index, array){
  console.log(element, index, array);
})
// Apple 0 (fruits의 array)
// Banana 1 (fruits의 array)
// Cherry 2 (fruits의 array)
 ```


 ### .map()

 - `forEach()`와 다르게 `return`을 사용하여 값을 새로운 배열을 만들어서 `반환`한다.


 ```js
const numbers = [ 1 , 2 , 3 , 4];
const fruits = ['Apple' , 'Banana' , 'Cherry'];

const a = fruits.forEach(function(element, index){
  console.log(`${element}-${index}`) // Apple-0 Banana-1 Cherry-2
});
consle.log(a) // undefined

const b = fruits.map(funtion(element, index){
  return `${element}-${index}`;
})

console.log(b) // ['Apple-0' , 'Banana-1' , 'Cherry-2']

 ```
- map() 은 객체데이터도 배열로 반환가능하다.

 ```js
 const numbers = [ 1 , 2 , 3 , 4];
const fruits = ['Apple' , 'Banana' , 'Cherry'];

const a = fruits.forEach(function(element, index){
  console.log(`${element}-${index}`) // Apple-0 Banana-1 Cherry-2
});
consle.log(a) // undefined

const b = fruits.map(funtion(element, index){
  return {
    id : index, // index
    name: element // element
  } // 객체데이터로 담겨져있는 것을 배열로도 만들 수 있다.
})

console.log(b) 
//      [{id : 0 , name : Apple}
//      {id : 0 , name : Apple}
//      {id : 0 , name : Apple}]
 ```