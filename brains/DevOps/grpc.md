# gRPC, RPC, 작동원리, HTTP API 비교

## RPC: Remote Procedure Calls
다른 컴퓨터의 프로그램의 프로시저를 실행하는 프로그램을 허용하는 프로토콜
  
개발자가 원격 상호 작용에 대한 세부 정보를 명시적으로 코딩하지 않아도 됨 -> 프레임워크가 자동 핸들링
  
클라이언트 코드에서는 직접 서버 코드의 함수를 호출하는 것 처럼 보임
  
클라이언트 코드 언어 != 서버 코드 언어: 다른 언어로 쓰일 수 있음

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbqlDmG%2Fbtq7uceRu8g%2FttYh7wR46nA9iHST541Wmk%2Fimg.png)

## gRPC = Communtication 프레임워크
마이크로서비스는 여러 PL들로 만들어진다. (ex: Back-End: Go, Front-End: js)
  
이 서버들 간의 소통이 필요하다.
  
마이크로서비스 간의 교환되는 메시지 수 = 엄청나게 많음 >>> 빠른 소통이 좋음
  
개발자는 핵심 로직 구현만 집중하도록 하고 소통은 프레임워크에게 맡기기
  
gRPC = High-performance Open-source Feature-rich Framework

- originally developed by Google
- 지금은 Cloud Native Computing Foundation 파트 (CNF - like 쿠버네티스)
- gRPC 메시지는 기본적으로 Protobuf(이진 형식)로 인코딩 됨 > 송수신 효율적 but 사람이 못읽음

gRPC는 Protocol Buffers를 통해 클라이언트 코드와 서버 인터페이스 코드를 생성. 옵션에 따라 생성하는 언어 변경 가능
  
-> 하나의 proto 파일을 사용해 go, python, java 등 여러 언어에서 서버/클라이언트 코드를 생성할 수 있음

<br>

## gRPC 작동 원리

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FFciKH%2Fbtq7ut1Wtof%2Fz64S0EkpD9yk4WWg048b10%2Fimg.png)

1. 클라이언트는 stub 생성(서버와 같은 메서드 제공)
2. stub은 gRPC 프레임워크를 호출(내부 네트워크를 통해서 호출)
3. 클라이언트와 서버는 서로 상호작용을 위해 stubs 사용 -> 서로의 코어 서비스 로직 권한만 필요

## gRPC vs HTTP API
기능 | gRPC | HTTP API
--|--|--
계약 | 필수(.proto) | 선택 사항(OpenAPI)
프로토콜 | HTTP/2 (빠름) | HTTP
Payload | Protobuf(소형, 이진 메시지 형식) | JSON(대형, 사람이 읽을 수 있음)
규범 | 엄격한 사양 | 느슨함. 모든 HTTP가 유효함
스트리밍 | 클라이언트, 서버, 양방향 | 클라이언트, 서버
브라우저 지원 | 아니오(gRPC-웹 필요) | 예
보안 | 전송(TLS) | 전송(TLS)
클라이언트 코드 생성 | 예 | OpenAPI + 타사 도구

.proto 파일: gRPC 서비스/메시지 계약 정의

## 한계
브라우저에서 gRPC 서비스 직접 호출 불가능

- gRPC-Web 사용 (양방향 스트리밍 불가, 서버 스트리밍 제한적)
- RESTful JSON Web API 에서 HTTP 메타 데이터로 .proto 파일에 주석으로 gRPC 서비스 사용

(애플리케이션 = json Web API, gRPC 둘다 지원)