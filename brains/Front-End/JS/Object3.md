# 객체 메소드

## Object.assign()
- `Object.assign()` 메소드는 열거할 수 있는 하나 이상의 출처 객체로부터 대상 객체로 속성을 복사할 때 사용한다. 
- 대상 객체를 반환한다.

**매개변수** : 
- `target` 대상객체.

**반환 값**
- 대상 객체.

```js
const target = {a : 1, b : 2};
const source = {b : 4, c : 5};

const returnedTarget = Object.assign(target, source);

console.log(target);
// { a : 1  , b : 4 , c : 5 }

console.log(returnedTarget);
// { a : 1  , b : 4 , c : 5 }
```

> `target` 과 `returnedTarget`은 같은   데이터다.  

> 객체는 생긴게 똑같아도 데이터는 다르다.

 <br>

 ## keys()
- `keys()` 객체의 값을 추출해서 배열로 만들어준다

 ```js
 const user ={
  name : "huemang",
  age : 17,
  email: "s22043@gsm.hs.kr"
 }

 const keys = Object.Keys(user);
 console.log(keys);
 // ['name' , 'age' , 'email']

 console.log(user['email']);

 const values = keys.map(key => user[key]);
 console.log(values);
 // ['huemang' , 17 , 's22043@gsm.hs.kr ]
 ```

> 객체데이터에서 대괄호를 사용하는 `인덱싱` 방법을 사용했다. 나중에 더 알아보도록 하자.