# Storage

## Local Storage
- `localStorage`읽기 전용 속성을 사용하면 Document 출처의 Storage 객체에 접근할 수 있다.
저장한 데이터는 브라우저 세션 간에 공유됩니다.  

<br>

### session Storage와 차이
 > localStorage는 데이터가 만료되지않고 session Storage의 데이터는 페이지 세션이 끝날 때. 사라지는 점이 다릅니다.

 ## 예제
`Storage.setItem(Key , valuse)` : Storage에 값을 저장 
 ```js
 localStorage.setItem('myCat', 'Tom');
 ```

 `Storage.getItme()` : Storage의 아이템 확인

 ```js
 const cat = localStorage.getItem('myCat');
 ```

 `Storage.removeItem()` : Storage의 아이템 제거

 ```js
 localStorage.removeItem('myCat');
 ```

 