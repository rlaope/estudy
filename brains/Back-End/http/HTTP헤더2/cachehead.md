# 캐시와 조건부 요청 헤더

- Cache-Control : 캐시 제어
- Pragma : 캐시 제어 (하위 호환)
- Expires : 캐시 유효 기간 (하위 호환)

### Cache-Control
**캐시 지시어(directives)**
- Cache-Control: max-age
  - 캐시 유효 시간, 초 단위
- Cache-Control: no-cache
  - 데이터는 캐시해도 되지만, 항상 원(origin) 서버에 검증하고 사용
- Cache-Control: no-store
  - 데이터에 민감한 정보가 있으므로 저장하면 안됨(메모리에서 사용하고 최대한 빨리 삭제)


### Pragma
**캐시 제어(하위호환)**
- Pragma : no-cache
- HTTP/1.0 하위 호환

### Expires
**캐시 만료일 지정(하위 호환)**
- expries: Mon, 01 Jan 1990 00:00:00 GMT
- 캐시 만료일을 정확한 날짜로 지정
- HTTP 1.0 부터 사용
- 지금은 더 유연한 Cache-Control: max-age 권장
- Cache-Control: max-age와 함께 사용하면 Expires는 무시