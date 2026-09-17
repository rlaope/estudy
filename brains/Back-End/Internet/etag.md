# 캐시 검증 헤더 ETag, If-None-Match

Last-Modified, If-Modified-Since보다 좀 더 간단한 방식으로 ETag와 If-None-Match 검증 헤더가 있다.
  
서버에서 완전히 캐시를 컨트롤하고 싶은 경우 ETag를 사용할 수 있음

![](https://velog.velcdn.com/images/mmmdo21/post/61357d79-0876-4cda-8329-d99a17bb7b03/image.png)

### 작동 방식
서버에서 헤더에 ETag를 작성해 응답한다.
  
클라이언트의 캐시에서 해당 ETag 값을 저장한다.

![](https://velog.velcdn.com/images/mmmdo21/post/d7003dc2-5572-4520-8f4a-288244010491/image.png)

만약 캐시 시간이 초과돼서 다시 요청을 해야 하는 경우라면 이때 ETag 값을 검증하는 If-None-Match를 요청 헤더에 작성해서 보낸다.(조건부요청)

![](https://velog.velcdn.com/images/mmmdo21/post/bb79c5d9-a157-41ad-ad62-d04211170032/image.png)

서버에서 데이터가 변경되지 않았을 경우 ETag는 동일하기에 그래서 If-None-Match는 거짓이 된다.
  
이 경우에 서버에서 304 Not Modifed를 응답하며 이때 역시 HTTP body는 없다.
  
브라우저 캐시에서는 응답 겨로가를 재사용하고 헤더 데이터를 갱신한다.

![](https://velog.velcdn.com/images/mmmdo21/post/2ff52843-8dc5-43c1-9f8e-aedb1d5f1501/image.png)

### 정리

![](https://velog.velcdn.com/images/mmmdo21/post/1410fd25-36c6-457d-b91c-fddd56ddc415/image.png)