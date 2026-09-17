# JSON


### JSON
-  속성값 또는 키값으로 이루어진 데이터 오브젝트를 전달하기 위해 사람이 읽을 수 있는 텍스트를 사용하는 `개방형 표준` 포멧이다.
- JSON의 공식 인터넷 미디어 타입은 `application / json` 이며, JSON의 파일 확장자는 `.json` 이다.

> JSON에서 String은 큰따옴표("")만 사용한다.


ex )

```json
{
  "string" : "Huemang",
  "number" : 123,
  "boolean" : true,
  "null " : null,
  "object" : {},
  "array" : []
}
```

```js
import MyData from './myData'

console.log(myData) // myData의 객체데이터 출력
```

> JSON은 하나의 문자데이터다.

> 통신을 하기 위한 용도나 가볍게 사용할 용도로는 적합하지않다.

### JSON.stringify()

- 자바스크립트의 객체데이터를 문자 데이터로 바꾸어 주는 메서드

### JSON.parse()

- 자바스크립트의 데이터를 분석하는 메서드