# RESTful API 의미와 설게 규칙

### RESTful API
REST 란 REpresentational State Trasfer 의 약어로 웹을 이용할 때 제약 조건들을 정의하는 소프트웨어 아키텍처 스타일입니다. HTTP URL을 통해 자원을 명시하고 HTTP Method를 통해서 해당 자원에 대한 CRUD를 적용하는 것을 의미합니다. 한마디로 HTTP의 장점을 `살리고자 하는 통신규약` 이라 할 수 있습니다. 로이 필딩의 2000년 박사학위 논문에서 소개되었으며 RESTful API 는 이러한 규약을 바탕으로 리소스 중심으로 설계하고 기능에 맞게 HTTP Method를 사용하여 설계된 API 입니다.  
  
- GET : 지정된 URL에서 리소스의 표현을 조회
- POST : 지정된 URL에 신규 리소스를 생성
- PUT : 지정된 URL에 리소스를 생성하거나 업데이트
- PATCH : 리소스의 부분 업데이트
- DELETE : 지정된 URL의 리소스를 제거

<br>

### REST 특징

**REST 아키텍처에 적용되는 6가지 제한 조건**
- 인터페이스 일관성 : 일관적인 인터페이스로 분리되어야 합니다.
- 무상태 : 각 요청 간 클라이언트의 context, 세션과 같은 상태 정보를 서버에 저장하지 않습니다.
- 캐시 처리 가능 : 클라이언트는 응답을 캐싱할 수 있어야 합니다. 캐시를 통해 대량의 요청을 효율적으로 처리할 수 있습니다.
- 계층화 : 클라이언트는 대상 서버에 직접 연결되어있는지, Proxy를 통해서 연결되었는지 알 수 없습니다.
- Code on demand : 자바 애플릿이나 자바스크립트의 제공을 통해 서버가 클라이언트를 실행시킬 수 있는 로직을 전송하여 기능을 확장시킬 수 있습니다.
- 클라이언트/서버 구조 : 아키텍처를 단순화시키고 작은 단위로 분리함으로써 클라이언트-서버의 각 파트가 독립적으로 구분하고 서로 간의 의존성을 줄입니다.

<br>

### REST 구성 요소
REST는 다음과 같은 3가지로 구성되어 있습니다.
1. 자원 : HTTP URL
2. 자원에 대한 행위 : HTTP Method
3. 자원에 대한 표현(Representations)

<br>

### REST API 설계 Rulse 및 예시

> 1. 소문자를 사용한다.

```
❌ http://cocoon1787.tistory.com/users/Post-Comments
```
```
⭕ http://cocoon1787.tistory.com/users/post-comments
```
**대문자는 때로 문제를 일으키는 경우가 있기 때문에 소문자로 작성합니다**  
  
> 2. 언더바 대신 하이픈을 사용한다.

```
❌ http://cocoon1787.tistory.com/users/post_comments
```
```
⭕ http://cocoon1787.tistory.com/users/post-comments
```
**정확한 의미나 단어 결합이 불가피한 경우 ("-")을 사용하여 하이픈("-") 사용도 최소한으로 설계합니다. 언더바("_")는 사용하지 않습니다.**
  
> 3. 마지막에 슬래시를 포함하지 않는다.

```
❌ http://cocoon1787.tistory.com/users/
```
```
⭕ http://cocoon1787.tistory.com/users
```
**슬래시("/")는 계층 관계를 나타낼 때 사용됩니다.**  

> 4. 행위를 포함하지 않는다.

```
❌ POST http://cocoon1787.tistory.com/users/post/1
```
```
⭕ DELETE http://cocoon1787.tistory.com/users/1
```
**자원에 대한 행위는 HTTP Method로 표현합니다.(GET, POST, DELETE, PUT)**  
  
> 5. 파일 확장자는 URL에 포함시키지 않는다.

```
❌ http://cocoon1787.tistory.com/users/photo.jpg
```

```
⭕ GET http://cocoon1787.tistory.com/users/photo
   HTTP/1.1 Host: cocoon1787.tistory.com Accept: image/jpg
```

**URL에 메시지 body 내용의 포맷을 나타내기 위한 파일 확장자를 적지 않습니다. 대신 Accept header를 사용합니다.**  
  
> 6. 자원에는 형용사, 동사가 아닌 명사를 사용하며, 컨트롤 자원을 의미하는 경우 예외적으로 동사를 사용한다.

```
❌ http://cocoon1787.tistory.com/duplicating
```

```
⭕ http://cocoon1787.tistory.com/duplicate
```
**URL은 자원을 표현하는데 중점을 두기 때문에 동사, 형용사보다는 명사를 사용하여야 합니다.**