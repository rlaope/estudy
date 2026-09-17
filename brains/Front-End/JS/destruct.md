# 구조 분해 할당

### 구조 분해 할당이란 ?
- 배열이나 객체의 속성을 해체하여 그 값을 개별 변수에 담을 수 있게 하는 JS 표현식 입니다.

```js
const user = {
  name : `huemang`,
  age : 17,
  email : 's22043@gsm.hs.kr'
}

const { name , age , email, address} = user

console.log(name);
// humang

console.log(age);
// 17

console.log(email);
// s22043@gsm.hs.kr

console.log(address);
// undefined

```
- 배열도 구조 분해 할당을 사용할 수 있다.
- 만약 세번째 값만 추출 하고싶다면 값을 추출할 변수 뒤에 , 를 붙여주면 된다

```js
const fruits = ['apple' , 'banana' , 'Cherry']
const =[, , fruit] = fruits

console.log(fruit);
// Cherry
```

- 변수에 기본값을 할당하면, 분해한 값이 `undefined`일 때 그 값을 대신 사용합니다.

```js
let a, b;

[a=5, b=7] = [1];
console.log(a); // 1
console.log(b); // 7
```