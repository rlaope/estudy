# 데이터 불변성
- 데이터는 원시데이터와 참조형 데이터로 분리할 수 있다.

### 원시 데이터 
> Stirng , Number , Boolean , undefined, null

### 참조형 데이터
> Object , Array , Function

## 데이터 불변성
- 원시 데이터들은 `불변`한다.
- 참조형 데이터들은 `가변`한다.

```js
let x = {
  name: 'huemang'
};

let y = x;

x.name = 'gyeongsu';

console.log(y.name); //  gyeongsu
console.log(x === y) // true
```
- y는 x를 참조하고 있기때문에 이름을 `gyungsu`로 바꿨을때도 y는 x를 가르키고 있어서 바뀐 값 gyungsu로 출력이 된다

> 참조형 데이터들은 값이 같아도 비교를 하면 false가 나올 수 있다.

<br>

## 얕은 복사 , 깊은 복사

```js
const user = {
  name : 'Huemang',
  age : 17,
  emails : ['thesecon@gmail.com']
}
const copyUser = Object.assign({}, user)
//const copyUser = {...user}로도 가능
console.log(user === copyUser)

user.age == 22;
console.log('user',user);// 22
console.log('copyUser',copyUser); // 17
```
- 얕은복사
> 객체를 직접 대입하는 경우 참조에 의한 할당이 이루어지므로 둘은 같은 데이터(주소)를 가지고 있다.

```js
user.emails.push('piyrw9754@gmail.com')
console.log(user.emails === copyUser.emails); // true
console.log('user',user)
console.log('copyUser',copyUser)
```

- 깊은 복사란 ?

> 데이터 자체를 통째로 복사한다.

> 복사된 두 객체는 완전히 독립적인 메모리를 차지한다.

> value type의 객체들은 깊은 복사를 하게 된다.

```js
import _ from 'lodash'

const copyUser = _.cloneDeep(user)
console.log(copyUSer === user) // false

console.log('user',user.age) // 22
console.log('copyUser',copyUser.age) // 17
```

- js로 깊은복사를 만들기는 좀 힘들기때문에 lodash의 도움을 받았다

- `_.cloneDeep(value)` : 재귀적으로 값을 복사한다