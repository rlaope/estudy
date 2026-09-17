# Type

## 타입스크립트의 기본 타입
- 타입스크립트로 변수나 함수와 같은 자바스크립트 코드에 타입을 정의할 수 있습니다.
- 타입스크립트의 기본 타입에는 크게 다음 12가지가 있습니다.

  1. Boolean
  2. Number
  1. String
  1. Object
  1. Array
  1. Tuple
  1. Enum
  1. Any
  1. Void
  1. Null
  1. Undefined
  1. Never

## Primitive Type
- 오브젝트와 레퍼런스 형태가 아닌 실제 값을 저장하는 자료형입니다.
- 프리미티브 형의 내장 함수를 사용 가능한것은 자바스크립트 처리 방식 덕분
- (ES2015 기준) 6가지
  * boolean
  * number
  * string
  * symbol
  * null
  * undifined

### boolean
- 가장 기본적인 데이터 타입
- 단순산 true 혹은 false 값 입니다.
-  js / ts에서 'boolean' 이라고 부른다.

### number
- JavaScript와 같이, TypeScript 의 모든 숫자는 부동 소수점 값 입니다.
-  TypeScript는 16진수 및 10진수 리터럴 외에도, ECMAScript 2015에 도입된 2진수 및 8진수를 지원한다.
-  NaN
-  1_000_000 과 같은 표기 가능

### string
- 다른 언어에서와 마찬가지로 이 텍스트 형식을 참조하기 위해 `string` 형식을 사용한다.
- JavaScript 와 마찬가지로, TypeScript는 문자열 데이터를 둘러싸기 위해 큰 따옴표 "", 작은 따옴표 '' 를 사용합니다.

### Template String
- 행이 걸쳐 있거나, 표현식을 넣을 수 있는 문자열
- 이 문자열은 backtick 기호에 둘러쌓여 있다.
- 포함된 표현식은 `${expr}` 와 같은 형태로 사용한다.

### symbol
- ECMAScript 2015 의 Symbol 입니다.
- new Symbol 로 사용할 수 없습니다.
-  Symbol 을 함수로 사용해서 symbol 타입을 만들어낼 수 있습니다.
-  프리미티브 타입의 값을 담아서 사용합니다.
-  고유하고 수정불가능한 값으로 만들어줍니다.
-  그래서 주로 접근을 제어하는데 쓰는 경우가 많습니다.

### null & undefined
- TypeScript 에서, undefined 와 null은 실제로 각각 undefined 및 null이라는 타입을 가집니다
- void와 마찬가지로, 그 자체로는 그다지 유용하지 않습니다.
- 둘다 소문자만 존재합니다.
- undefined와 null을 할당할 수 있게 하려면 `union type`을 이용해야합니다.
  
### object
- `primitive type 이 아닌 것` 을 나타내고 싶을 때 사용하는 타입

### array
- 원래 자바스크립트에서 array는 객체입니다.
- 사용법
  * Array<타입>
  * 타입[]
  
### Tuple
- 배열인데 타입이 한가지가 아닌 경우
- 마찬가지로 객체입니다.
- 꺼내 사용할때 주의가 필요합니다
    * 배열을 Destructuting 하면 타입이 제대로 얻어집니다.
  
### any
- 어떤 타입이어도 상관없는 타입입니다.
- 이걸 최대한 쓰지 않는게 핵심입니다.
- 왜냐면 컴파일 타임에 타입 체크가 정상적으로 이뤄지지 않기 때문입니다.
- 그래서 컴파일 옵션 중에서는 any를 써야하는데 쓰지 않으면 오류를 뱉도록 하는 옵션도 있습니다.
    * nolmplicitAny

### unknown
- 응용 프로그램을 작성할 때 모르는 변수의 타입을 묘사해야 할 수도 있습니다.
- 이러한 값은 동적 콘텐츠의 모든 값을 의도적으로 수락하기를 원할 수 있습니다.
- typeof 검사, 비교 검사 또는 `고급 타입 카드`를 수행하여 보다 구체적인 변수로 좁힐 수 있습니다.

### never
- 리턴에 사용됩니다.
- 리턴에 사용되는 경우, 아래 3가지 정도의 경우가 대부분입니다.
> never 타입은 모든 타입의 subtype이며, 모든 타입에 할당 할 수 있습니다.
> 하지만 never에는 그 어떤 것도 할당할 수 없습니다.
> any 조차도 never 에게 할당 할 수 없습니다.
> 잘못된 타입을 넣는 실수를 막고자 할 때 사용하기도 합니다.

### void
- 어떤 타입도 가지지 않는 빈 상태를 의미합니다.
- 값은 없고 타입만 있습니다.
- 소문자 입니다.
- 일반적으로 값을 반환하지 않는 함수의 리턴 타입으로 사용합니다.
- 할당이 가능한 값은 undefined 입니다.



## [예제](https://github.com/KIMHUEMANG/Study_TypeScript/tree/main/TypeScript_Essentials)