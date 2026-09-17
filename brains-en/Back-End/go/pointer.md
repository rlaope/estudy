# Pointers and Dereferencing in Golang

Similar to C, but different.
  
In C, the array name itself is the address of the first element of the array, but Go doesn't have that. You only need to remember to put an & before a variable to get its address.
  
In C, `*(array_name + index)` functioned the same as `array_name[index]`, but Go doesn't have that. If you want to directly reference a value, you only need to remember to put an * before a pointer variable.
  
When calling a function, you input `"functionName(&variableName)"` to pass an address, and when defining a parameter, you put an * before the parameter type to directly reference the value. Also, inside the function, you must put an * before all parameters.
  
If you only use Go, the concepts of pointers/dereferencing can be understood as follows:
- Values are stored in memory, and variables are a kind of alias.
- A variable that holds a memory address as its value is called a pointer.
- Retrieving the value pointed to by a pointer is called dereferencing.
- Direct assignment of memory addresses or pointer arithmetic is not allowed.

## Basic Pointer Usage

You can print memory addresses using &.

```go
func main() {
    a := 2
    b := a // Value copy for primitive types
    a = 10
    fmt.Println(&a, &b) // & gives the address. Different memory addresses.
}
```

When you assign a memory address to a variable, that's a pointer. In the code below, b is a pointer.
  
You can dereference using * and furthermore, you can put a specific value back into that memory location.

```go
func main() {
    a := 5
    b := &a
    fmt.Println(b)
    fmt.Println(*b) // Dereference
}
```

You can also create pointer variables using the `*type` syntax instead of the `:=` shorthand.

```go
func main() {
    var a *int // Pointer variable. The dereferenced value of a is an int.
    b := 3
    a = &b // Must assign an address

    fmt.Println(a) // Address
    fmt.Println(*a) // Dereference
}
```

If you've used C, the code below will be more familiar. In this case, a pointer named `numPtr` was created with `*int`.

```go
func main() {
    var numPtr *int = new(int)
    *numPtr = 10
    fmt.Println(numPtr) // Address
    fmt.Println(*numPtr) // 10
}
```

You can also check if pointers point to the same memory address using the `==` operator.

```go
func main() {
    var a int = 10
	var b int = 20

	var p1 *int = &a
	var p2 *int = &b
	var p3 *int = &a

	fmt.Printf("%v\n", p1 == p2) // false
	fmt.Printf("%v", p1 == p3) // true
}
```

Now you should be able to understand the code below.
  
If you only know memory addresses (`&`) and dereferencing (`*`), you've completed the low-level basics in Go.

```go
package main

import "fmt"

func main() {
	a := 2
	b := &a            // b is a pointer to a
	fmt.Println(&a, b) // Naturally, the same memory address

	*b = 5         // Change the value at that memory location to 5
	fmt.Println(a) // Prints 5
}
```

## Accessing Variables Outside Pointer Scope

In the code below, the print statement within the `main` function's scope will output 0.
  
I want it to output 40, so let's use pointers for this.

```go
func main() {
	x := 0
	foo(x)
	fmt.Println(x) // 0 Why? Because it's outside the scope of the foo function
}

func foo(x int) {
	fmt.Println(x)
	x = 40
	fmt.Println(x)
}
```
If you write it like the following, you can display 40 regardless of the scope by manipulating x's memory address.

```go
func main() {
	x := 0
	foo(&x)
	fmt.Println(x) // 40
}

func foo(x *int) {
	fmt.Println(*x)
	*x = 40
	fmt.Println(*x)
}

```
