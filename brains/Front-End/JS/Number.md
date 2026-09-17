# Math

### toFixed()

- `toFixed()`메소드가 호출될때 인수로 소수점 몇번째 자리까지 유지할지 정할 수 있다.

- 문자데이터로 반환이 된다

```js
const pi = 3.14159265359879
console.log(pi); // 3.14159265358979

const str = pi.toFixed(2);
console.log(str); // 3.14
console.log(typeof str); // Stirng
```

<br>

### parseInt() , parseFloat()

`이 함수들도 전역함수이다`  
- parseInt() 문자를 추출해서 `정수`형으로 반환한다.
- parseFloat() 문자를 추출해서 `실수`형으로 반환한다.  

```js
const pi = 3.14159265359879
console.log(pi); // 3.14159265358979

const str = pi.toFixed(2);
console.log(str); // '3.14'
console.log(typeof str); // Stirng

const integer = parseInt(str);
const float = parseFloat(str);

console.log(integer); // 3
console.log(float); // 3.14  
```

### Math.abs()
- `abs : absolute`로 `Math.abs()` 함수는 주어진 숫자의 절댓값을 반환한다.

<br>

### Math.min() , Math.max()
- min : 주어진 인수중 더 `작은 값`을 반환한다.
- max : 주어진 인수중 더 `큰 값`을 반환한다.

<br>

### Math.ceil() Math.cloor()
- ceil : 주어진 인수를 정수단위로 `올림` 처리한다.
- floor : 주어진 인수를 정수단위로 `내림` 처리한다.

### Math.round()
- 주어진 인수를 `반올림` 처리한다.  

<br>


### Math.random()
- 랜덤한 숫자를 반환한다. 

<br>

```js
console.log('abs :', Math.abs(-12)) // abs : 12

console.log('min :' ,Math.min(2,8)) // min : 2

console.log('max :', Math.max(2,8)) // max : 8

console.log('ceil :', Math.ceil(3.14)) // ceil : 4

console.log('floor :', Math.floor(3.14)) // floor : 3

console.log('round :' ,Math.round(3.5)) // round : 4

console.log('random :' ,Math.random()) // 랜덤한 수 0 ~ 1

```

<br>

### random 1~ 10 랜덤한 정수를 출력해보자

```js
let rand = Math.floor(Math.random() * 10 + 1);
console.log(rand) // 1 ~ 10
```

1. rand라는 변수에`floor()` 메서드를 실행시키고 그 안에 `random()`을 실행시킨다.  

2. `random()*10`을 실행하면 0 ~ 9까지 수가 출력되기에 + 1을 더해준다 그리고 floor로 내림처리를 해주면 정수형 1 ~ 10이 랜덤하게 나온다.

