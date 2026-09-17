# 함수

### 함수란 ?
- `함수(function)`란 하나의 특별한 목적의 작업을 수행하도록 설계된 독립적인 블록을 의미합니다.

- 이러한 함수는 필요할 때마다 호출하여 해당 작업을 반복해서 수행할 수 있습니다.

### 함수 선언

 ```js
 //리턴값이 없는 함수
 function sayHello(){
  console.log("hello");
 }
//리턴값이 있는 함수
 function plus(a, b){
  console.log(a + b);
 }
 ```

 ### 화살표 함수
 - 함수 표현식보다 단순하고 간결한 문법으로 함수를 만들 수 있는 방법
  
  ```js
  let func = (arg1, arg2, ...argN) => expression
  ```
  >이렇게 코드를 작성하면 인자 arg1..argN를 받는 함수 func이 만들어집니다. 

  >함수 func는 화살표(=>) 우측의 표현식(expression)을 평가하고, 평가 결과를 반환합니다.

  ex)
  ```js
  // a + b 반환
  let plus = function(a, b){
    return a + b;
  }

  // a + b 반환
  let plus = (a,b)=> a + b;
  ```

  위와 같이 위에 두 함수는 같은 작업을 수행한다.  
  - 인수가 하나밖에 없다면 인수를 감싸는 괄호를 생략할 수 있다.
  -  괄호를 생략하면 코드 길이를 더 줄일 수 있다

  ### 타이머 함수
  1. setTimeout(함수 , 시간): 일정 시간 후 함수 실행
  2. setInterval(함수, 시간): 시간 간격마다 함수 실행
  3. clearTimeout(); 설정된 Timeout 함수를 종료
  4. clearInterval(); 설정된 Interval 함수를 종료

  ```js
  //3초에 한번씩 hello 출력
  const timer = setInterval( () => {
  console.log("hello");
}, 3000);

  //HTML h1태그를 클릭하면 멈춤
  const h1El = document.querySelector("h1");
  h1El.addEventListener("click",() => {
  clearTimeout(timer);
})

  ```

  ### 콜백
  - 함수의 인수로 사용되는 함수

  ```js
  function timeout(callback) {
  setTimeout(() =>{
    console.log("Hello");
    callback();
  }, 3000);
}

timeout(() => {
  console.log("done");
});
  ```

  위 처럼 작성하면 3초후에 hello가 출력된 뒤에 done이 출력된다  
  저렇게 하지 않고 그냥 밖에서 화면을 실행시키면 done이 먼저 출력이 된다.

<br>

  ### this
  - 한 객체에서 자신을 지칭 하는 것

  ```js
  const Huemang = {
    firstName : "Huemang",
    lastName : "Kim",
    getFullName: function(){
      return `${this.firstName} ${this.lastName}`
    }
  }
  console.log(Huemang.getFullName()); // Huemang Kim
  ```

  - 일반 함수는 호출 위치에서 따라 this 정의
  - 화살표 함수는 자신이 선언된 함수 범위에서 this 정의 !
  

### 생성자 함수
-  new 키워드와 함께 쓰이는 함수이다. 우리가 직접 함수를 정의하여 new 키워드로 생성자 함수를 만들어 사용할 수도 있지만, 자바스크립트에 기본적으로 내장된 생성자 함수를 사용할 수도 있다
```js
function user(first, last){
  this.firstName = first,
  this.lastName = last
}

const huemang = new user('Huemang',"Kim");

console.log(huemang); // huemang의 객체가 출력
```
