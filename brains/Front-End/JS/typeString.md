# 문자.문자열

## String

- `Stirng` 전역 객체는 문자열의 생성자입니다.
- 리터럴 방식을 사용합니다.
```
" Stirng text "

' Stirng text2 '

` String text3 `
```

<br>

## Stirng.prototype.indexOf()

`indexOf()`메서드는 호출한 `Stirng` 객체에서 주어진 값과 일치하는 첫 번째 인덱스를 반환합니다.   
일치하는 값이 없으면 -1을 반환합니다.  


<br>

## String.prototype.slice()

`slice()`메소드는 문자열의 일부를 추출하면서 새로운 문자열을 반환합니다.

`slice(0,3)` : 인덱스 0부터 3 직전까지만 추출한다 > 0 1 2 추출

## replace()

`replace()`메소드는 replace('1번 문자열' , '2번 문자열'); 이렇게 입력한다 1번 문자열을 2번문자열로 바꾼다 라고 해석할 수 있다.


## trim()

`trim()`메소드는 문자열 앞에있는 공백을 제거해주는 메소드이다. <웹사이트 만들때 많이 사용한다.>  

`indexOf`
```js
const result = 'Hello World'.indexOf('World');
console.log(result) // 6 index 6번째에서부터 찾았다

result = 'Hello World'.indexOf('huemang');
console.log(result) // -1 일치하는 값이 없다

//Stirng.prototype.indexOf()
// 아무 값도 indexOf에 주어지지 않으면 , undefined를 찾으려는 문자열로 사용한다
```

`slice()`
```js
const str1 = "hello world";
console.log(str.slice(0,3)); // hel
```

`replace()`
```js
const str ="hello world!";

console.log(str.replace('world','huemang')); //replace() 첫번째 인수의 해당하는 내용을 두번째 인수에 해당하는 내용으로 교체한다.

//만약 replace 삭제를 하고 싶다면 두번째 인수를 '' 공백으로 추가하면 된다.
console.log(str.replace('world','')); // hello

```

`tirm()`
```js
const str = '    Hello world  '
console.log(str.trim()); // 문자열 앞에서 시작하는 공백문자 모두 제거
```