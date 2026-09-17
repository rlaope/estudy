# export

## export

- The `export` statement is used to export functions, objects, or primitive values from a JavaScript module. The exported values can be imported and used by other programs with the `import` statement.

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

### Default Exports and Re-exports

1. Default Exports
2. Named Exports
 - `Named exports` are useful when exporting multiple values. When importing, you must use the same name as the exported value. In contrast, `default exports` can be imported with any name.
 - To avoid identifier conflicts, you can also rename named exports.

 <br>

 - Named Exports
 ```js
 // 먼저 선언한 식별자 내보내기
export { myFunction, myVariable };

// 각각의 식별자 내보내기
// (변수, 상수, 함수, 클래스)
export let myVariable = Math.sqrt(2);
export function myFunction() { ... };
 ```

 - Default Exports
 ```js
 // 먼저 선언한 식별자 내보내기
export { myFunction as default };

// 각각의 식별자 내보내기
export default function () { ... };
export default class { ... }
 ```
