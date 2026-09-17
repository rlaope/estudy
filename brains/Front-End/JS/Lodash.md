# Lodash

### Lodash 란 ?
- JS의 인기있는 `라이브러리` 중 하나입니다. 보통의 경우 array , collection , date 등 데이터의 필수적인 구조를 쉽게 다룰 수 있게끔 하는데에 사용됩니다.

<br>

### _.uniq()
> 배열데이터 안의 중복된 값들을 고유데이터로 남겨 놓는다.

```js
import _ from 'lodash'

const usersA = [
  { userId : '1', name : 'Huemang'},
  { userId : '2', name : "Neo"}
]

const usersB = [
  { userId : '1', name : 'Huemang'},
  { userId : '3', name : "Amy"}
]

const usersC = usersA.concat(usersB)

console.log('concat',usersC); // user의 객체가 나온다. id : 1이고 name : Huemang인 객체는 두개가 나온다
console.log('uniqBy', _.uniqBy(usersC,'user')) // 고유화되어 중복된 값이 하나가 되었다.

const userD = _.unionBy(usersA, usersB, 'userId') // uniqBy와 concat을 합쳐놓은 기능을 한다.
// 고유화되어 중복된 값이 하나가 되었고 A와 B의 값이 합쳐져있다.
```

- `uniqBy()` : 하나의 배열 데이터에서 특정한 속성의 이름으로 `고유화`를 시켜주는 메서드
- `unionBy()` : 배열데이터가 여러개일때 하나로 합치고 `고유화`를 시켜주고 반환하는 메서드

### _.find( 인수 , 조건 )

- 조건에 해당하는 객체데이터를 찾아 반환한다

### _.findIndex( 인수 , 조건 )

- 조건에 해당하는 객체의 인덱스 번호를 반환한다.

### ._remove( 인수 , 조건 )

- 조건에 해당하는 객체를 삭제한다.

```js
import _ from 'lodash'

const users - [
  { userId : '1' , name : 'Huemang'},
  { userId : '2' , name : 'Neo'},
  { userId : '3' , name : 'Amy'},
  { userId : '4' , name : 'Evan'},
  { userId : '5' , name : 'Lewis'},
]

const foundUser = _.find(users, {name : 'Amy'}) 
console.log(foundUser) // 해당하는 객체
const foundUserIndex = _.findIndex(users, {name : 'Amy'}) 
console.log(foundUserIndex) // 2

_.remove(users, { name : 'Huemang'})
// name : Huemang이라는 객체를 삭제한다.
```

> lodash 라이브러리는 프론트엔드 개발에서 데이터를 처리할때 유용하게 쓰이니 잘 알아둘 것 !