# 서버 구축하기

### 1. server 폴더 생성

### 2 server.js 파일 생성
- server .js

```js
// 서버 사용을 위해 http 모듈을 http 변수에 담는다
let http = require('http');

// http 모듈로 서버를 생성한다
// 아래와 같이 작성하면 서버를 생성한 후 , 사용자로 부터 http 요청이 들어오면 function 블럭 내부의 코드를 실행해서 응답한다.
let server = http.createServer(function(request,response){
  response.writeHead(200, {'Content=Type':'text/html'});
  response.end('Hello node.js!');
});

// 3. listen 함수로 8080 포트를 가진 서버를 실행한다.
server.listen(8000, function(){
  console.log('Server is running...');
});
```

### server.js 실행 및 접속

터미널 창
```
$ node server
```
실행 후 http://localhost:8080 이 서버에 Hello node.js! 라고 잘 나온다면 성공한 것이다.

<br>

### 코드 분석

```js
let http = require('http');
```
웹 서버를 실행하기 위해 http 모듈을 require로 불러온다. (require은 import와 유사한 기능이다.) node.js는 require 후에 해당 모듈을 http라는 변수에 담은 후 하나의 독립적인 객체로 사용합니다.  
  
http 모듈에 정의되어 있는 createServer 함수로 서버를 생성합니다.  
일반적인 함수는 이렇게 사용하는데
```js
function nameOfFunction(parameter){
  // 실행 로직
}
```
createServer() 파라미터로 입력되는 function(request , response){}은 함수명이 없습니다.  
  
함수명이 없이 function이라고만 작성된 파라미터는 이벤트 발생시에 callback이 됩니다. 즉, 생성된 서버로 어떤 요청이 들어오면 function 내부의 로직이 실행되면서 function내부에 선언한 request와 response라는 이름으로 사용할 수 있는 값을 넘겨줍니다. 그래서 function 블럭 {} 내부에 서는 request와 response로 넘어오는 어떤 값을 사용할 수 있습니다.

```js
let server.createServer(function(request,response){
  response.writeHead(200,{'Content-Type':'text/html'});
  response.end('Hello node.js!');
})
```

아래 코드에서는 response 객체를 사용해 사용자 측으로 반환값을 넘겨주는데, 먼저 writeHead( ) 라는 함수에  
  
첫번째는 200 이라는 숫자값을,  
  
두번째는 { } 중괄호 안에 { '키' : '값' } 형태의 값을 담습니다.

```js
response.writeHead(200,{'Content-Type':'text/html'});
```
첫번째 200이라는 숫자값은 웹서버에 들어오는 어떤 요청에 대해 정상적으로 값을 리턴할 때 사용하는 http 상태 코드입니다. 오류가 없이 서버에서 정상적으로 처리가 완료되면 200 코드를 담아서 응답헤더를 설정해 주게 됩니다.

<br>

두번째 {'Content-Type':'text/html'} 값은 서버측에서 보내주는 컨텐츠의 타입이 텍스트이고, html형태라는 것을 정의합니다