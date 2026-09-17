# Strings

## String

- The `String` global object is a constructor for strings.
- It uses literal notation.
```
" Stirng text "

' Stirng text2 '

` String text3 `
```

<br>

## String.prototype.indexOf()

The `indexOf()` method returns the first index at which a given value can be found in the `String` object on which it was called.
It returns -1 if the value is not found.

<br>

## String.prototype.slice()

The `slice()` method extracts a section of a string and returns a new string.

`slice(0,3)`: Extracts from index 0 up to (but not including) index 3 > extracts 0, 1, 2

## replace()

The `replace()` method is called as `replace('string 1', 'string 2');`. This can be interpreted as replacing 'string 1' with 'string 2'.

## trim()

The `trim()` method removes whitespace from the beginning of a string. <Often used when building websites.>

`indexOf`
```js
const result = 'Hello World'.indexOf('World');
console.log(result) // 6 found starting at index 6

result = 'Hello World'.indexOf('huemang');
console.log(result) // -1 no matching value

//String.prototype.indexOf()
// If no value is provided to indexOf, `undefined` is used as the search string
```

`slice()`
```js
const str1 = "hello world";
console.log(str.slice(0,3)); // hel
```

`replace()`
```js
const str ="hello world!";

console.log(str.replace('world','huemang')); // `replace()` replaces the content corresponding to the first argument with the content corresponding to the second argument.

// If you want to 'delete' with replace, you can provide an empty string `''` as the second argument.
console.log(str.replace('world','')); // hello

```

`tirm()`
```js
const str = '    Hello world  '
console.log(str.trim()); // Removes all leading whitespace characters from the string
```
