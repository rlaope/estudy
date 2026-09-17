# Promise

1. 제작코드 : 원격에서 스크립트를 불러오는 것 같은 시간이 걸리는 일을 함.
2. 소비코드 : `제작코드`의 겨로가를 기다렸다가 이를 소비함, 이때 주체(함수)는 여럿이 될 수 있음
3. promise : `제작코드`와 `소비코드`를 연결해 주는 특별한 자바스크립트 객체이다. promise는 시간이 얼마나 걸리든 상관없이 약속한 결과를 만들어 내는 `제작 코드`가 준비되었을 때, 모든 소비 코드가 결과를 사용할 수 있도록 해줍니다.

<br>

promise 객체는 아래와 같은 문법으로 만들 수 있다.
```js
promise = new Promise(function(resolve, reject)){
  // executor (제작 코드, 가수)
}
```
> new Promise에 전달되는 함수는 executor(실행자, 실행함수) 라고 한다.
> executor는 new Promise가 만들어질 때 자동으로 실행되는데, 결과를 최종적으로 만들어내는 제작코드를 포함한다.
> resolve와 reject는 자바스크립트에서 제공하는 자체 콜백 ` 개발자는 신경쓰지않고 executor 안 코드만 작성하면 됨 `


- resolve(value) : 일이 성공적으로 끝난 경우 그 결과를 나타내는 value와 함께 호출
- reject(error) : 에러 발생 시 에러 객체를 나타내는 error와 함께 호출

<br>

- new Promise 생성자가 반환하는 promise 객체에는 다음과 같은 내부 프로퍼티를 갖는다.
    - `state` 처음엔 pending 이었다 resolve가 호출되면 "fullfilled", reject가 호출되면 "rejected" 로 변합니다.
    - `result` 처음엔 undefined 이었다가 resolve가 호출되면 value로, reject가 호출되면 error로 변합니다.


```js
let promise = new Promise(function(resolve, reject)){
  // promise가 만들어지면 executor 함수는 자동으로 싱행된다.

  // 1초 뒤에 일이 성공적으로 끝났다는 신호가 전달되면서 result는 `완료`(value)가 된다.
  setTimeout(() => resolve('완료'), 1000);
};
```
> 1. executor는 new Promise에 의해 자동으로 그리고 즉각적으로 호출됩니다.
> 2. executor는 인자로 resolve와 reject 함수를 받습니다. 이 함수들은 자바스크립트 엔진이 미리 정의한 함수이므로 개발자가 따로 만들 필요가 없습니다. 다만, resolve나 reject중 하나는 반드시 호출해야 합니다.

<br>

이처럼 일이 성공적으로 처리되었을 때의 promise는 'fulfilled promise' 라고 불립니다.   
이번에는 executor가 에러와 함께 약속한 작업을 거부하는 경우에 대해 알아보겠습니다.

```js
let promise = new Promise(function(resolve, reject){
  // 1초 뒤에 에러와 함께 실행이 종료되었다는 신호를 보냅니다.
  setTimeout(() => reject(new Error("에러 !발생")), 1000);
});
```

1초후 `reject`가 호출되면 promise의 상태가 "rejected" 로 변합니다.

<br>

## 요약
excutor는 보통 시간이 걸리는 일을 수행합니다. 일이 끝나면 `resolve`나 `reject` 함수를 호출하는데, 이때 promise 객체의 상태가 변화합니다.

<br>

### promise는 성공 또는 실패만 합니다.

- executor는 resolve나 reject중 반드시 하나를 호출해야합니다. 이때 변경된 상태는 더 이상 변하지 않습니다.

```js
let promise = new Promise(function(resolve, reject)){
  resolve("완료");

  reject(new Error(",,,")); // 무시됨
}; 
```

<br>

### Error 객체와 함께 거부하기

- 무언가 잘못된 경우, executor 는 reject를 호출해야 합니다. 이때 인수는 resolve 와 마찬가지로 어떤 타입도 가능하지만 Error 객체 또는 Error를 상속받은 객체를 사용할 것을 추천합니다. 이유는 뒤에서 설명하겠습니다. 

### resolve , reject 함수 즉시 호출

- executor는 대개 무언가를 비동기적으로 수행하고, 약간의 시간이 지난 후에 resolve , reject를 호출하는데, 꼭 이렇게 할 필요는 없습니다. 아래와 같이 resolve나 reject를 즉시 호출할 수도 있습니다.

```js
let promise = new Promise(function(resolve, reject){
  // 일을 끝마치는 데 시간이 들지 않음
  resolve(123); // 결과 123을 즉시 resolve에 전달
});
```

어떤 일을 시작했는데 알고 보니 일이 이미 끝나 저장까지 되어있는 경우, 이렇게 resolve나 reject를 즉시 호출하는 방식을 사용할 수 있습니다.  
이렇게 하면 promise는 즉시 이행 상태가 됩니다.  



<br>

###  state와 result는 내부에 있습니다
- promise 객체의 state, result 프로퍼티는 내부 프로퍼티이므로 개발자가 직접 접근할 수 없습니다.
- `.then` / `.catch` / `.finally` 메서드를 사용하면 접근 가능한데, 자세한 내용은 아래에서 살펴보겠습니다.

<br>

## 소비자 : then , catch, finally
- 프라미스 객체는 executor와 결과나 에러를 받을 소비 함수(`팬`)를 이어주는 역할을 합니다. 소비함수는 `.then` , `.catch` , `finally` 메서드를 사용해 등록됩니다.

<br>

### then
> 프라미스에서 가장 중요하고 기본이 되는 메서드

```js
promise.then(
  function(result){/* 결과를 다룹니다 */}.
  function(error){/* 에러를 다룹니다. */}
);
```
`.then`의 첫 번째 인수는 프라미스가 이행되었을 때 실행되는 함수이고, 여기서 실행 결과를 받습니다.  
`.then`의 두번째 인수는 프라미스가 거부되었을 때 실행되는 함수이고, 여기서 에러를 받습니다.  

```js
let promise = new Promise(function(resolve,reject){
  set Timeout(() => resolve("완료!"),1000);
});
// resolve 함수는 .then의 첫 번째 함수를 실행합니다.
promise.then(
  result => alert(result); // 1초 후에 완료를 출력
  error => alert(error); // 실행되지 않음.
)
```

<br>

### catch
에러가 발생한 경우만 다루고 싶다면 .`then(null, errorHandlingFunction)` 같이 `null` 을 첫 번째 인수로 전달하면 됩니다. `.catch(errorHandlingFunction)`를 써도 되는데, `.catch`는 `.then` 에 `null` 을 전달하는 것과 동일하게 작동합니다.
```js
let promise = new Promise((resolve, reject) => {
  setTimeout(() => reject(new Error("에러 발생!")), 1000);
});
// .catch(f)는 promise.then(null , f)와 동일하게 작동
promise.catch(alert); // 1초뒤 에러 발생! 출력
```
`.catch(f)` 는 문법이 간결하다는 점만 빼고 `.then(null,f)`과 완벽하게 같습니다.

<br>

### finally
- `try{...} carch{...}` 에 finally 절이 있는 것처럼, promise에도 finally 가 있습니다.
- `.finally(f)` 호출은 `.then(f,f)`와 유사합니다.
- 쓸모가 없어진 로딩 인디케이터를 멈추는 경우같이, 결과가 어떻든 마무리가 필요하면 `finally`가 유용합니다.

```js
new Promise((resolve,reject) => {
  /* 시간이 걸리는 어떤 일을 수행하고, 그 후 resolve , reject를 호출함 */
});
// 성공 실패 여부와 상관없이 promise가 처리되면 실행됨
.finally(() => 로딩 인디케이터 중지)
.then(result => result와 err 보여줌 => error 보여줌)
```

<br>

### then(f,f) 와 finally 의 차이
1. `finally` 핸들러엔 인수가 없습니다. promise가 이행되었는지 거부되었는지 알 수 없습니다.
> 보편적 동작을 수행하기 때문에 성공 실패 여부를 몰라도 됩니다.

2. `finally` 핸들러를 자동으로 다음 핸들러에 결과와 에러를 전달합니다.
> result가 finally를 거쳐 then까지 전달되는 것을 확인해봅시다.

```js
new Promise((resolve, reject) => {
  setTimeout(() => resolve('결과'), 2000);
});
  .finally(() => alert("promise가 준비됨"));
  .then(result => alert(result)); // then에서 result를 다룰 수 있음
```
프라미스에서 에러가 발생하고 이 에러가 finally를 거쳐 catch까지 전달되는 것을 확인해봅시다.
```js
new Promise((resolve, reject) => {
  throw new Error("에러 발생");
});
  .finally(() => alert("프라미스가 준비되었습니다."));
  .catch(err => alert(err)); // catch에서 에러 객체를 다룰 수 있음
```
`finally`는 프라미스 결과를 처리하기 위해 만들어 진 게 아닙니다. 프라미스 결과는 `finally`를 통과해서 전달되죠, 이런 특징은 아주 유용하게 사용됩니다.
