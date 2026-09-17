## MongoDB CRUD


MongoDB에서 CRUD 연산을 하는 방법에 대해서 알아보겠습니다.

<br> 
## Create - Create Operation(Insert)

Create는 **새로운 Document를 Collection에 생성하거나 추가하는 동작이다.** MongoDB의 create는 insert를 통해서 이루어집니다. 만약 Collection이 존재하지 않으면 insert 명령어는 collection을 새로 생성합니다.

MongoDB는 다음과 같은 두개의 Insert 메서드를 제공하고 있어요

- db.collection.insertOne()
- db.collectioin.insertMany()

---
### insertOne()

먼저 db.collection.insertOne()은 하나의 document를 collection에 넣는 명령어입니다.

```
// Inserting one document
db.collection.insertOne({ key: "value" });
```

insert가 성공하면 삽입된 document의 \_id를 리턴합니다.

### insertMany()
db.collection.insertMany()는 여러개의 document를 collection에 넣는 명령어에요

여러개의 documents를 insert하기 위한 명령어입니다.

```
// Inserting multiple documents
db.collection.insertMany([
  { key1: "value1" },
  { key2: "value2" }
]);
```

insertMany는 테이블에 데이터를 넣기 위해 Array 형태로 입력을 하며, 리턴되는 \_id 또한 array 형태로 반환됩니다.

<br>

## Read - Read Operation(Find)

Read는 Collection의 Document 읽어오는 연산을 합니다. Find라는 함수를 통해 동작을 수행합니다.

find()는 쿼리 필터를 이용하여 document를 읽어올 수 있습니다.

```
db.collection.find({}) // 전체 조회
db.collection.find( { size: { h: 14, w: 21, uom: "cm" } }) // 쿼리 필터 적용
```

find 함수를 실행하면 id와 document를 리턴해주고 없으면 아무것도 반환하지 않습니다.

쿼리 필터 즉, 조건을 넣어 document를 찾을 때, 필드와 내부 필드를 함께 붙여 사용할 수 있습니다.

```
db.collection.find({ "size.uom": "in" })
```

여기선 예시르 작성하지 않지만 operator를 사용하여 데이터를 검색할 수도 있습니다.

<br>
### 필요한 데이터만 조회하기
위의 find는 일반적으료 \_id 를 포함한 document의 모든 데이터를 가져온다. 하지만 특정 필드들만 가지고 오고 싶다면 다음과 같이 할 수 있다.

```
db.collection.find({조건}, {가져올필드 : 1})
db.collection.find( { size : { h: 14, w: 21, uom: "cm"} }, { item: 1 } )
db.collection.find( { size : { h: 14, w: 21, uom: "cm"} }, { _id: 0, item: 1 } ) // 아이디 필드는 제외하고 가져옴
```

위에서는 size 조건에 맞춘 컬렉션의 item 필드만 가져오게 된다.

0을 지정해준다면 가져올 필드에서 제외할 수 있다.


<br>

### 배열 조건 조회

특정 필드 값이 배열로 되어있고 그 데이터 필드를 이용해 조회하려고 한다. In 절이라고 생각하면 편하다.

근데 그냥 find에서 조건절안에 배열을 넣으면 순서까지 일치해야 데이터를 가져올 수 있다.

따라서 순서 상관없이 조건을 입력하고 싶다면 다음과같이 할 수 있다.

```
db.collection.find( { 필드 : { &all : [데이터1, 데이터2] } } )
```


>  비교 연산자와 논리 연산자도 몽고디비에 존재하는데 나중에 알아보겠다.
> 추가로 Cursor 조회와 null 관련 처리도 존재하는데 나중에~!


<br>

## Update - Update Operation(Update, Replace)

Update는 collection의 document를 수정하기 위한 동작이다
- db.collection.updateOne()
- db.collection.updateMany()
- db.collection.replaceOne()


updateOne()은 하나의 document를 수정한다. update 시에는 조건을 넣어 수정할 document를 지정하는데, 여러 document가 조회될 경우 가장 처음 document를 수정한다.

```
db.collection.updateOne(
	{ item: "name" },
	{
		$set: {"size.uom": "cm", status: "P"},
		$currentDate: { lastModified: true }
	}
)
```

updateMany()는 한 번에 많은 데이터를 수정하기 위해 사용되며 사용법은 updateOne과 같다

```
db.collection.updateMany(
	{ item: "name" },
	{
		$set: {"size.uom": "cm", status: "P"},
		$currentDate: { lastModified: true }
	}
)
```

위의 업데이트연산 모두 $set을 통해서 필드 값을 업데이트 했는데. replaceOne()은 위에 update들과 다르게 들어온 인자값으로 document를 대체한다.

```
db.collection.replaceOne(
	{ item: "paper" },
	{ item: "paper", instock" [ 업데이트할 필드 ]}
)
```


<br>

## Delete - Delete Operation(Delete)

delete는 collection의 document를 삭제하는 동작이다.
- db.collection.deleteOne()
- db.collection.deleteMany()

deleteMany()는 여러개의 documents를 삭제한다. 조건 지정은 find와 같다. 전체삭제는 {}

```
db.collection.deleteMany({}) // 전체 삭제
db.collection.deleteMany({조건: 조건})
```

deleteOne()은 위와 같으나 첫번째로 조회된 데이터를 삭제한다.