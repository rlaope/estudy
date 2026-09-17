# JSON


### JSON
- It is an `open standard` format that uses human-readable text to transmit data objects consisting of attribute-value or key-value pairs.
- JSON's official internet media type is `application/json`, and its file extension is `.json`.

> In JSON, strings must use double quotes ("") only.


e.g.)

```json
{
  "string" : "Huemang",
  "number" : 123,
  "boolean" : true,
  "null " : null,
  "object" : {},
  "array" : []
}
```

```js
import MyData from './myData'

console.log(myData) // Outputs myData's object data
```

> JSON is a single string of data.

> It is not suitable for communication purposes or for lightweight use.

### JSON.stringify()

- A method that converts JavaScript object data into string data.

### JSON.parse()

- A method that parses JavaScript data.
