# 자주사용하는 DOM API 정리

## DOM tree 구성요소

- 문서 노드(Document Node) - 최상위 트리이자 접근의 시작점이다.
- 요소 노드(Element Node) - HTML 요소를 표현한다.
- 속성 노드(Attribute Node) - HTML 요소의 속성을 표현한다.
- 텍스트노드(Text Node) - HTML 요소의 텍스트를 표현한다. 자식을 가질 수 없다.

## DOM API

`DOM`을 통해 웹페이지를 조작하려면 다음 순서에 맞게 API를 사용한다.

<br>

  
## 1. 요소 선택 및 탐색

<br>

### 1-1 하나의 요소 노드 선택
- `document.getElementById("id")`
  - id 값으로 요소 노드 한 개를 선택한다.
  - 여러 개가 선택된 경우 첫번째 요소만 반환한다.

- `document.querySelector("cssSelector")`
  - CSS 선택자로 요소 노드 한 개를 선택한다.
  - 여러 개가 선택된 경우 첫 번째 요소만 반환한다.

<br>

### 1-2 여러개의 요소 노드 선택

- `document.getElementByClassName(class)`
  - class 값으로 요소 노드를 모두 선택한다.
  - 공백을 사용하면 여러 개의 class 를 지정할 수 있다.
  - HTMLCollection 형태로 반환한다.

- `document.getElementByTagName(tagName)`
  - 태그 명으로 요소 노드를 모두 선택한다.
  - HTMLCollection 형태로 반환한다.
  
- `document.queySelectorAll(selector)`
  - CSS 선택자로 요소 노드를 모두 선택한다.
  - NodeList 형태로 반환한다.

<br>

### 1-3 탐색

- `parentNode`
  - 부모 노드를 탐색한다.

- `firstChild, lastChild`
  - 자식 노드를 탐색한다.
  - 공백, 줄 바꿈도 텍스트 노드 취급당하므로 주의한다.

- `childNodes`
  - 자식 노드의 컬렉션을 반환한다.
  - NodeList 형태로 반환한다.

- `children`
  - 자식 노드의 컬렉션을 반환한다.
  - HTMLCollection 형태로 반환한다.

<br>

HTMLCollection과 NodeList 비교

- HTMLCollection (live)
  - 유사 배열
  - 실시간으로 노드의 상태 변경을 반영한다.

- NodeList (non-live)
  - 유사 배열
  - 경우에 따라 라이브 컬렉션이 되기도 한다.


## 2.조작

### 2-1 텍스트 노도에 접근 및 수정

텍스트 노드는 부모 노드를 거쳐 탐색되어야 한다.

- `nodeValue`
  - 텍스트 노드는 문자열, 요소 노드는 null을 반환한다.
  - 텍스트 노드의 유일한 프로퍼티이다.


<br>

### 2-2. 속성 노드에 접근 및 수정

- `className`
  - class 값을 얻거나 변경한다.
  - class 값이 여러개인 경우 공백으로 구분된 문자열이 반환된다.
  - 값을 할당할 때 class 속성이 존재하지 않으면 class 속성을 생성한 후 값을 할당한다.

- `id`
  - id 값을 얻거나 변경한다.
  - 값을 할당할 때 id 속성이 존재하지 않으면 id 속성을 생성한 후 값을 할당한다.

### 2-3 HTML 콘텐츠 조작

- `textContet`
  - 요소의 텍스트 콘텐츠를 얻거나 변경한다. 마크업은 무시된다.
  - 값 변경시 마크업을 포함시키면 문자열로 인식되어 그대로 출력한다.

- `innerText`
  - 마크업을 제외한 문자열을 리턴한다.
  - 값을 변경할 땐 마크업을 그대로 추가해야 한다.
  - 마크업을 포함해 값을 추가하는 것은 XSS에 취약하다.
  - 아래의 이유로 사용하지 않는 것이 좋다.
    - 비표준이다.
    - CSS에 순종적이다
    - CSS를 고려해야 하므로 textContent 보다 느리다.


- `innerHTML`
  - 자식요소를 하나의 문자열로 얻는다.
  - 마크업을 포함하여 리턴한다.
  - 마크업을 포함해 값을 추가하는 것은 XSS에 취약하다.

<br>

### 2-4. DOM 조작 방식

innerHTML을 사용하지 않고 새로운 콘텐츠를 추가하는 방법이다.

- `createElement(tagName)`
  - 태그이름을 인자로 전달하여 요소를 생성한다.

- `createTextNode(text)`
  - 텍스트를 인자로 전달하여 텍스트 노드를 생성한다.

- `appendChild(Node)`
  - 인자로 전달한 노드를 마지막 자식 요소로 DOM 트리에 추가한다.

- `removeChild(Node)`
  - 인자로 전달한 노드를 DOM 트리에 제거한다.


<br>

## innerHTML 과 DOM 조작 방식 비교

- innerHTML
  - DOM 조작 방식에 비해 빠르고 간편하다
  - XSS공격에 취약해서 값을 추가할 때 주의해야 한다,
  - 내용을 덮어쓰는 방식이라 비효율적이다.

- DOM 조작 방식
  - 특정 노드 한 개를 추가할 때 적합하다.
  - innerHTML보다 느리고 많은 코드가 필요하다.


- 결론
  > innerHTML은 XSS에 취약하다는 단점때문에 `텍스트 추가 또는 변경`시에는 `textContet`, **새로운 요소 추가 또는 삭제**시에는 **DOM 조작 방식**을 사용하도록 한다.