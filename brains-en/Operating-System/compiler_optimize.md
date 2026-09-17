# Compiler Optimization

Optimization refers to making something perform with maximum efficiency within given conditions or scope.

Who? -> The compiler
How? -> By removing unnecessary operations
What? -> From the written code

Simply put, it means that a tool called a compiler analyzes the written code, identifies useless or inefficient operations, and transforms them into efficient code.

```c
int main(int argc, char *argv[]){ 

	// 변수 b는 변수 a에 대해 의존적이다. 
	int a = 10; int b = a * 5; 
	
	// 변수 d는 변수 c에 대해 의존적이다. 
	int c = 20; 
	int d = c * 5;
	
	// 변수a와 변수b는 변수c와 변수d와 서로 의존적이지 않다. 
	// 즉 순서를 바꾸거나 동시에 연산을 진행해도 문제가 발생하지 않는다 즉 동시성을 가진다. 
	return 0;
}
```

All dependencies are divided based on variables, and they are grouped into a single block based on their mutual dependencies.

These various blocks become individual targets for optimization when the compiler performs it.

For example, in a statement like `a = b`, `a` and `b` become dependent on each other.

It is important to clearly distinguish these concepts and prepare for compiler optimization issues.

```c
int fuc(int a){
	for (int i = 0; i < 10; ++i){
		a = 10; 
	}
	return a; 
} 

int main(int argc, char *argv[]){ 
	int a = 10; 
	printf("%d", fuc(a));
	return 0;
}
```

The `fuc` function is defined, and it iterates 10 times, changing the value received as parameter `a` to 10 and returning the changed value of `a`.

Therefore, the result printed is 10. Let's check what happens at a low level.

Below is the low-level code without compiler optimization.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FCmDV1%2FbtqC2tqKJyN%2FNZih9QnNkbu2K9TcnkfvKK%2Fimg.png)

In reality, when the `fuc` function is called, it iterates through the for loop defined within it, assigning 10 to variable `a`.

So, what happens at a low level if compiler optimization is applied?

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fmgcna%2FbtqC3qfZu6E%2FpAkquZVSFfXfm1kecEOfK1%2Fimg.png)

As shown above, the `fuc` function is not even called, and the value 10 is not assigned to variable `a`.

Internally, it simply prints the constant 10 using `printf`.

In this way, the compiler optimization process gathers variables that have dependencies due to operators and performs optimization, significantly reducing unnecessary operations and memory usage during this process.

> At this point, if a user's indiscriminate use of variables creates unnecessary dependencies, and the program's logical structure becomes complex as a result, the compiler optimization might be applied in a way unintended by the user, potentially leading to program crashes.

To write efficient programs, one must be able to distinguish operations that have dependencies on specific variables, and write program code in a way that minimizes data dependencies, i.e., by reducing variable usage.

Therefore, unless a variable is absolutely necessary, constants should be used as much as possible, and coding should aim for `const` usage rather than hardcoding.

**While it is important for programmers to write optimized code, it is arguably more crucial to write code that is easy for the compiler to optimize. Code that is easy to optimize means minimizing data with dependencies and making the program's logical structure as simple as possible before handing it to the compiler. In other words, the main obstacles to optimization are an excessive number of variables and indiscriminate use of pointers. Therefore, using `const` for variables to reduce dependencies, and also using `const` for pointer variables to treat them as constants, is an efficient way to write programs.**
