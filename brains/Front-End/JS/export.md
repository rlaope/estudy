# export

## export

- `export`문은 JavaScript 모듈에서 함수, 객체, 원시 값을 내보낼 때 사용합니다. 내보낸 값은 다른 프로그램에서 `import`문으로 가져가 사용할 수 있습니다.

```js
// 하나씩 내보내기
export let name1, name2, …, nameN; // var, const도 동일
export let name1 = …, name2 = …, …, nameN; // var, const도 동일
export function functionName(){...}
export class ClassName {...}

// 목록으로 내보내기
export { name1, name2, …, nameN };

// 내보내면서 이름 바꾸기
export { variable1 as name1, variable2 as name2, …, nameN };

// 비구조화로 내보내기
export const { name1, name2: bar } = o;

// 기본 내보내기
export default expression;
export default function (…) { … } // also class, function*
export default function name1(…) { … } // also class, function*
export { name1 as default, … };

// 모듈 조합
export * from …; // does not set the default export
export * as name1 from …;
export { name1, name2, …, nameN } from …;
export { import1 as name1, import2 as name2, …, nameN } from …;
export { default } from …;
```

### 기본 내보내기와 다시 내보내기

1. 기본내보내기
2. 유명 내보내기
 - `유명 내보내기`는 여러 값을 내보낼 떄 유용합니다. 가져갈 때는 내보낸 이름과 동일한 이름을 사용해야 합니다. 반면 `기본 내보내기`는 어떤 이름으로도 가져올 수 있습니다
 - 식별자 충돌을 피하기 위해 유명 내보내기 중 이름을 바꿔줄 수도 있습니다.

 <br>

 - 유명 내보내기
 ```js
 // 먼저 선언한 식별자 내보내기
export { myFunction, myVariable };

// 각각의 식별자 내보내기
// (변수, 상수, 함수, 클래스)
export let myVariable = Math.sqrt(2);
export function myFunction() { ... };
 ```

 - 기본 내보내기
 ```js
 // 먼저 선언한 식별자 내보내기
export { myFunction as default };

// 각각의 식별자 내보내기
export default function () { ... };
export default class { ... }
 ```