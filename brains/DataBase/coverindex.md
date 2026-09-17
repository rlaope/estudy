# 커버링 인덱스

커버링 인덱스는, **쿼리를 충족시키는데 필요한 모든 데이터를 갖고있는 인덱스**를 말한다.

인덱스는 데이터를 효율적으로 탐색하는 방법이다. 인덱스를 사용해 조회하게 되면 실제 데이터까지 접근할 필요가 없다. 즉, 인덱스는 조회 성능을 향상시킨다.

> 쿼리를 충족시킨다 -> SELECT, WHERE, ORDER BY, LIMIT, GROUP BY 등에서 사용되는 모든 컬럼이 인덱스 컬럼 안에 다 포함하는 경우


### 예시
```SQL
SELECT * FROM user WHERE id = 1;
```

![](https://velog.velcdn.com/images/boo105/post/84e67cc0-efbf-4239-86de-d06d0f0a778e/image.png)

위는 해당 쿼리의 실행 계획이다.

- `selected_type`: SIMPLE은 UNION이나 서브쿼리가 없는 단순 SELECT 문이다.
- `type`:  ref 조인을 할 때 PK 혹은 Unique Key가 아닌 Key로 매핑된 경우. 여기서는 단일 테이블 조회이기에 동등 조건으로 검색할 경우를 가리킨다.
- `key`: 쿼리에 사용된 인덱스다, 여기서는 PK를 사용했기 때문에 PRIMARY로 나온다.
- `ref`: const 비교 조건으로 어떤 값이 사용되었는지 나타낸다. 여기서는 상수인 1이 사용되어서 const로 나와있는 것을 볼 수 있다.
- `extra`: 빈 값일 경우 일반적인 쿼리다.

그럼 다시 쿼리문을 변경해보겠다 이번에는 전부 가져오지말고 id만 가져와보겠다.

```sql
select id from user
where id = 1;
```

![](https://velog.velcdn.com/images/boo105/post/74a512ba-ef27-4570-8d86-aac8804f3572/image.png)


Extra의 값이 Using index로 바뀐것을 확인할 수 있다. 이 쿼리는 인덱스에 포함된 컬럼 id 만으로 쿼리가 생성이 가능하기 때문에 커버링 인덱스가 사용된 것이다. 

**즉 쿼리에 나타난 모든 컬럼이 인덱스이기 때문에 커버링 인덱스가 적용이 된 것이다.**

<br>

### GROUP BY

예시에서는 where만 확인해 보았다. GROUP BY에서도 어떻게 커버링 인덱스를 적용할까?

인덱스 컬럼이 a, b, c라고 가정할 때 GROUP BY 인덱스 조건은 다음과 같은 조건을 만족해야한다.

1. GROUP BY에 명시하는 컬럼의 순서는 동일해야한다.
2. 앞에있는 컬럼은 무조건 명시해야하며 뒤에 있는 컬럼은 명시하지 않아도 된다.
3. 인덱스에 없는 컬럼을 명시해서는 안된다.
4. WHERE + GROUP BY가 함께 사용될 때, WHERE에 있는 컬럼은 GROUP BY에 없어도 된다.

```sql
GROUP BY a, c, b        # 인덱스 적용 X
GROUP BY a, b, c        # 인덱스 적용 O
```

```sql
GROUP BY a              # 인덱스 적용 O
GROUP BY a,b            # 인덱스 적용 O
```

```sql
GROUP BY a, b, c, d     # 인덱스 적용 X
```

```sql
WHERE a = 1
GROUP BY b, c           # 인덱스 적용 O

WHERE a = 1 and b = '홍길동'
GROUP BY c              # 인덱스 적용 O
```

<br>

### Pagination + 커버링 인덱스

```sql
// 커버링 인덱스 적용 전
SELECT *
FROM boards
WHERE 조건문
ORDER BY id DESC
OFFSET 페이지번호
LIMIT 페이지사이즈

// 커버링 인덱스 적용 후
SELECT  *
FROM  boards as b
JOIN (SELECT id
      FROM boards
      WHERE 조건문
      ORDER BY id DESC
      OFFSET 페이지번호
      LIMIT 페이지사이즈) as cover 
on cover.id = b.id
```

JOIN 내부에서 select, where, orderby, limit 항목들이 인덱스 컬럼으로만 이루어져서 인덱스 내부에서 쿼리를 충족시키는 모든 데이터를 가지고 있다.

따라서 커버링 인덱스로 조회한 id를 통해서 실제 데이터 블록에 조회할 항목들을 빠르게 조회할 수 있다.

일반적으로 조회 쿼리는 `order by`, `limit` 이 수행될 때 데이터 블록에 접근하여 쿼리 속도의 저하로 이어진다.

![](https://velog.velcdn.com/images/minbo2002/post/9c450c6d-00f5-436b-a748-d9ab98bd001c/image.png)

커버링 인덱스가 적용된다면 조회 쿼리의 경우 where, order by, limit 인덱스 검색을 처리한 다음에 특정 로우에 대해서만 데이터 블록에 접근하므로 쿼리 속도가 향상된다.

이때 클러스터 인덱스(PK)인 id는 모든 인덱스에 자동으로 포함되므로 페이징 작업까지는 커버링 인덱스로 빠르게 처리하고 마지막에 필요로 되는 컬럼만 가져온다.

![](https://velog.velcdn.com/images/minbo2002/post/385d7767-f2da-48e8-8798-7e77584421d7/image.png)


**예시 코드 QueryDSL**
```java
public Page<Match> findList(Pageable pageable, MatchSearchRequest searchRequest) {

	// 커버링 인덱스로 PK인 id 조회
    List<Long> ids = queryFactory
            .select(match.id)
            .from(match)
            .where(eqStartAt(searchRequest.getMatchDay()),
                   eqGender(searchRequest.getGender()),
                   eqStatus(searchRequest.getMatchStatus()),
                   eqPersonnel(searchRequest.getPersonnel()),
                   eqStadiumName(searchRequest.getStadiumName()))
            .orderBy(match.id.desc())
            .offset(pageable.getOffset())           
            .limit(pageable.getPageSize())
            .fetch();

	// 대상이 없을 경우 빈페이지 반환
	if (CollectionUtils.isEmpty(ids)) {
            return Page.empty();
    }

	// 조회한 PK인 id로 select절 조회
	List<Match> matchList = queryFactory
            .selectFrom(match)
            .where(match.id.in(ids))
            .orderBy(match.startAt.asc())
            .fetch();

	// 조회한 PK인 id로 페이지수 조회
	JPAQuery<Long> countQuery = queryFactory
            .select(match.count())
            .from(match)
            .where(match.id.in(ids));

	return PageableExecutionUtils.getPage(matchList, pageable, countQuery::fetchOne);
}
```