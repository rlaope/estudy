# fetch API

## XML Http Request 방식
xmlhttprequest 객체를 이용한 정통적인 초창기 비동기 서버 요청 방식이다.  
성능에는 문제는 없지만 코드가 복잡하고 가독성이 좋지 않다는 단점이 있었다.
```js
let httpRequest = new XMLHttpRequest();
httpRequest.onreadystatechange = function (){
  if(httpRequest.readyState === XMLHttpRequest.DONE && httpRequest.status === 200){
    document.getElementById('text').innerHTML = httpRequest.responseText;
  }
}

httpRequest.open("GET", "ajax_intro_data.txt",true);
httpRequest.send();
```

<br>

## fetch API 방식
이벤트 기반인 XMLHTTpRequest과는 달리 fetch API는 Promise 기반으로 구성되어 있어 **비동기 처리 프로그래밍 방식**에 잘 맞는 형태이다.  
그래서 then이나 catch와 같은 체이닝으로 작성할 수 있다는 장점이 있다.

```js
fetch('ajax_intro.data.txt').then(response  => response.text()).then(text => {document.getElementById("#t').innerHTML = text;})

// fetch('서버주소')는 웹 브라우저에게 '이 서버주소로 요청해줘' 라는 의미이고
// 뒤에 .then이 붙으면 '요청 끝나고 나서 이 할일을 해줘!' 라는 뜻이다.
```
또한 fetch API JS 기본 기능이기 때문에 , JQuery와 같이 CDN과 같은 다른 작업을 하지 않아도 바로 사용가능하다.

<br>

## fetch 문법 & 사용법

```js
fetch("https://jsonplaceholder.typicode.com/posts",option).then(res => res.text()).then(text => console.log(text));
```
1. fetch에는 기본적으로 첫 번째 인자에 요청할 `url`이 들어간다
2. 기본적으로 http 메소드 중 GET으로 동작한다
3. fetch를 통해 ajax를 호출 시 해당 주소에 요청을 보낸 다음. 응답 객체를 받는다
4. 그러면 첫 번째 then에서 그 응답을 받게되고, res.text() 메서드로 파싱한 text값을 리턴한다,
5. 그러면 다음 then 에서 리턴받은 text 값을 받고, 원하는 처리를 할 수 있게 된다.

<br>

## response 프로퍼티와 메서드
fetch를 통해 요청을 하고 서버로부터 값을 응답 받으면 .then을 통해 함수의 인자에 담기게 되는데 이 값은 `Response 객체`로서 여러가지 정보를 담고 있다.  
필요한 변수값이나 메서드를 뽑아내서 값을 얻으면 된다.

### 기본 문법
- response.status >> HTTP 상태코드
- response.ok >> HTTP 상태 코드가 200과 299 사이일 경우 true
- response.body >> 내용
- response.text() >> 응답을 읽고 텍스트를 반환한다.
- response.json() >> 응답을 JSON 형태로 파싱한다.
- response.formData() >> 응답을 FormData 객체 형태로 반환한다
- response.blob() >> 응답을 Blob(타입이 있는 바이너리 데이터) 형태로 반환한다.
- response.arrayBuffer() >> 응답을 ArrayBuffer(바이너리 데이터를 로우 레벨 형식으로 표현한 것)형태로 반환한다.

<br>

