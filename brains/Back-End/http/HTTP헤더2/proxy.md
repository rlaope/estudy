# 프록시 캐시
**Cache-Control**
캐시 지시어(directives) - 기타
- Cache-Control: public
  - 응답이 public 캐시에 저장되오도 됨
- Cache-Control: private
  - 응답이 해당 사용자만을 위한 것임, private 캐시에 저장해야 함(기본값)
- Cache-Control: s-maxage
  - 프록시 캐시에만 적용되는 max-age
- Age: 60 (HTTP 헤더)
  - 오리진 서버에서 응답 후 프록시 캐시 내에 머문 시간(초)

## 캐시 무효화

### Cache-Control
**확실한 캐시 무효화 응답**
- Cache-Control: no-cache, no-store, must-revalidate
- Pragma: no-cache
  - HTTP 1.0 하위호환

**캐시 지시어(directive) - 확실한 캐시 무효화**

- Cache-Control: no-cache
  - 데이터는 캐시해도 되지만, 항상 원 서버에 검증하고 사용(이름에 주의!)
- Cache-Control: no-store
  - 데이터에 민감한 정보가 있으므로 저장하면 안됨
(메모리에서 사용하고 최대한 빨리 삭제)
- Cache-Control: must-revalidate
  - 캐시 만료후 최초 조회시 원 서버에 검증해야함
  - 원 서버 접근 실패시 반드시 오류가 발생해야함 - 504(Gateway Timeout)
  - must-revalidate는 캐시 유효 시간이라면 캐시를 사용함
- Pragma: no-cache
  - HTTP 1.0 하위 호환

no-cache에서 원서버에 접근할 수 없는 경우 캐시 서버 설정에 따라 프록시에서 캐시 데이터를 반환할 수 있음 Error or 200 OK(오류 보다는 오래된 데이터라도 보여주자) must-revalidate의 경우 원 서버에 접근할 수 없는 경우 항상 오류가 발생해야 한다. 504 Gateway Timeout(매우 중요한 돈과 관련된 결과로 생각해보자)