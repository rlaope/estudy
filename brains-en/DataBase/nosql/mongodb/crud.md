## MongoDB CRUD

Let's learn about how to perform CRUD operations in MongoDB.

<br>
## Create - Create Operation(Insert)

Create is **the operation of creating or adding new documents to a collection.** In MongoDB, create operations are performed through inserts. If a collection does not exist, the insert command will create a new collection.

MongoDB provides the following two insert methods:

- db.collection.insertOne()
- db.collectioin.insertMany()

---
### insertOne()

First, `db.collection.insertOne()` is a command to insert a single document into a collection.

```
// Inserting one document
db.collection.insertOne({ key: "value" });
```

If the insert is successful, it returns the `_id` of the inserted document.

### insertMany()
`db.collection.insertMany()` is a command to insert multiple documents into a collection.

This command is used to insert multiple documents.

```
// Inserting multiple documents
db.collection.insertMany([
  { key1: "value1" },
  { key2: "value2" }
]);
```

`insertMany` takes an array as input to insert data into the table, and the returned `_id` is also in array form.

<br>

## Read - Read Operation(Find)

Read performs the operation of reading documents from a collection. This operation is carried out using the `find` function.

`find()` can read documents using query filters.

```
db.collection.find({}) // 전체 조회
db.collection.find( { size: { h: 14, w: 21, uom: "cm" } }) // 쿼리 필터 적용
```

Executing the `find` function returns the `_id` and document; if none are found, it returns nothing.

When searching for documents using query filters, i.e., conditions, you can use fields and nested fields together.

```
db.collection.find({ "size.uom": "in" })
```

Although an example is not provided here, you can also search for data using operators.

<br>
### Retrieving Only Necessary Data
The `find` operation above generally retrieves all data of a document, including its `_id`. However, if you want to retrieve only specific fields, you can do so as follows.

```
db.collection.find({조건}, {가져올필드 : 1})
db.collection.find( { size : { h: 14, w: 21, uom: "cm"} }, { item: 1 } )
db.collection.find( { size : { h: 14, w: 21, uom: "cm"} }, { _id: 0, item: 1 } ) // 아이디 필드는 제외하고 가져옴
```

In the example above, only the `item` field of the collection matching the `size` condition is retrieved.

If you specify `0`, you can exclude a field from being retrieved.

<br>

### Querying Array Conditions

You want to query using a data field where a specific field's value is an array. It's convenient to think of it as an "IN clause."

However, if you simply put an array in the condition clause of `find`, the data can only be retrieved if the order matches exactly.

Therefore, if you want to input conditions regardless of order, you can do so as follows.

```
db.collection.find( { 필드 : { &all : [데이터1, 데이터2] } } )
```


> Comparison operators and logical operators also exist in MongoDB, which I will explore later.
> Additionally, there are Cursor queries and null-related handling, which I'll cover later!

<br>

## Update - Update Operation(Update, Replace)

Update is the operation for modifying documents in a collection.
- db.collection.updateOne()
- db.collection.updateMany()
- db.collection.replaceOne()


`updateOne()` modifies a single document. When updating, you specify the document to modify using conditions; if multiple documents are found, it modifies the first one.

```
db.collection.updateOne(
	{ item: "name" },
	{
		$set: {"size.uom": "cm", status: "P"},
		$currentDate: { lastModified: true }
	}
)
```

`updateMany()` is used to modify many documents at once, and its usage is the same as `updateOne`.

```
db.collection.updateMany(
	{ item: "name" },
	{
		$set: {"size.uom": "cm", status: "P"},
		$currentDate: { lastModified: true }
	}
)
```

All the update operations above updated field values using `$set`. `replaceOne()`, unlike the other update methods, replaces the document with the provided arguments.

```
db.collection.replaceOne(
	{ item: "paper" },
	{ item: "paper", instock" [ 업데이트할 필드 ]}
)
```


<br>

## Delete - Delete Operation(Delete)

Delete is the operation for removing documents from a collection.
- db.collection.deleteOne()
- db.collection.deleteMany()

`deleteMany()` deletes multiple documents. Specifying conditions is similar to `find`. To delete all, use `{}`.

```
db.collection.deleteMany({}) // 전체 삭제
db.collection.deleteMany({조건: 조건})
```

`deleteOne()` is similar to the above but deletes the first document found.
