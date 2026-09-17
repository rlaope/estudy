# 객체와 테이블 매핑

### 엔티티 매핑
- 객체와 테이블 매핑 : @Entity, @Table
- 필드와 컬럼 매핑 : @Column
- 기본 키 매핑 : @Id
- 연관관계 매핑 : @ManyToOne, @JoinColumn

### 객체와 테이블 매핑

#### Entity
- @Entity가 붙은 클래스는 JPA가 관리하면 엔티티라고한다.
- JPA를 사용해서 테이블과 매핑할 클래스는 @Entity 필수

주의
- 기본 생성자 필수 (파라미터가 없는 public 또는 protected 생성자)
- final 클래스, enum, interface, inner 클래스 사용 X
- 저장할 필드에 final 사용 X

### @Table
- @Table은 엔티티와 매핑할 테이블 지정]
- name : 매핑할 테이블 이름 (기본값 : 엔티티 이름을 사용)
- catalog : 데이터베이스 catalog 매핑
- schema : 데이터베이스 schema 매핑
- uniqueConstraints(DDL) : DDL 생성 시에 유니크 제약 조건 생성