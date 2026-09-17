# DOM API

## DOM 소개

`DOM (Document Object Model)` 이란 ?
- 문서 객체 모델은 HTML , XML 문서의 프로그래밍 interface 이다. DOM은 문서의 구조화된 표현을 제공하며 프로그래밍 언어가 DOM 구조에 접근할 수 있는 방법을 제공하여 그들이 문서구조 , 스타일 , 내용 등을 변경할 수 있게 돕는다.
-  `DOM` 은 nodes 와 objects로 문서를 표현한다. 이들은 웹 페이지를 스크립트 또는 프로그래밍 언어들에서 사용돌 수 있게 연결시켜주는 역할을 담당한다.

### 1. DOM 을 통하여

DOM을 통해서 우리가 할 수 있는 상호작용은 `DOM구조에 접근` 해서 `문서의 내용을 변경`할 수 있다고 한다.
문서의 내용을 변경한다는 것을 쉽게 말하면 `동적`인  문서를 만들 수 있다는 것이다.  
   
정말 기본적으로 웹은 HTML CSS JS로 이루어진다고 말할 수 있다. 여기서 HTML을 바꾸는 것은 JS의 역할인데, 이것이 DOM과 연관되어진다고 생각 하면 된다.
  
  물론 JS가 이 역할만을 할 수 있는건 아니지만 대부분의 경우 브라우젱서 동작하기 때문에 넘어가겠다.

### 2. DOM 구조 , DOM 트리
- `DOM`은 트리 구조로 HTML 문서를 표현한다.
  
![tree](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FssgJG%2FbtqDO1V1e2g%2FbpBfkWNPkIluSiQfhsaxG0%2Fimg.png)


<br>

Document `Object` Model인 만큼, 태그 하나하나가 전부 **객체**이다. 이또한 인지하고 있자.

### 2.1 DOM 구조에 접근
> 스크립트를 작성할 때 인라인 요소를 사용하거나 웹 페이지 안에 있는 스크립트 로딩 명령을 사용하여, 문서 자체를 조작하거나 문서의 children을 얻기 위해 document 또는 window elements 를 위한 API 를 즉시 사용할 수 있다.

### 2.2 DOM 수정
객체에 접근했다면 , `DOM API`를 활용해 `DOM`을 수정할 수 있다.

![API](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbnAfcx%2FbtqDRNhmftL%2FK2n3pKhOfEE1NikYKzm0BK%2Fimg.png)

- document.createElement로 요소를 생성하고, appendChild로 하위 요소로 집어 넣는 과정을 진행한다.


아래는 간단한 DOM API 예시이다.
```js
const p = document.getElementByTagName("p");

p.innerText = "Hello";
```

### DOM 노드
객체지향의 개념이지만 , DOM 에서 객체들은 `클래스(프로토타입)`를 상속받는다.

![상속](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FpnQ2a%2FbtqDRLDP1Ue%2FFVBUb3q0PX1KKj9ntb0Z61%2Fimg.png)

> DOM을 구현하는 방식이 다를 수 있다. DOM을 만드는건 브라우저니까