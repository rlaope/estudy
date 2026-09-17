# 스택 (Stack)

- 스택은 삽입과 삭제가 한쪽 끝에서만 발생하는 `선형 리스트`다.  
> LIFO (Last In, First Out) 방식

- 스택의 연산에는 push pot peek 연산이 있다.
  - `push` : 스택의 연산에 자료 삽입
  - `pop` : 자료를 삭제
  - `peek` : 현재 스택의 top 포인터가 가리키는 자료의 내용을 조사하는 peek 연산자가 있다.

### 스택 구현

```c
#include <stdio.h>
#include <stdlib.h>

// 파일명 : stack.c
// 목표 : 배열과 연결리스트를 사용해서
// 스택(push, pop, peek)의 기능을 구현

// 1.배열 기반 구현
#define STACK_SIZE 10

int topIdx = -1; // 스택의 맨 위 데이터가 어디에 저장? 
int arrStack[STACK_SIZE];

void init() {
	topIdx = -1; // 스택이 비어있다고 정의

}

void push(int data) {
	if(topIdx < STACK_SIZE - 1)
		arrStack[++topIdx] = data;
}

int pop() {
	// topIdx 값이 1 감소 + topIdx가 가리키고 있던 값 반환
	if(topIdx >= 0)
		return arrStack[topIdx--];

		printf("꺼낼 데이터가 없음");
		exit(-1);
}


int peek() {
	return arrStack[topIdx];
}

int main() {

	push(1); push(2); push(3); push(4); push(5);
	push(11); push(22); push(33); push(44); push(55);
	push(111);

	printf("pop() 연산 수행 : %d 꺼냄\n", pop());
	printf("pop() 연산 수행 : %d 꺼냄\n", pop());
	printf("pop() 연산 수행 : %d 꺼냄\n", pop());

	printf("peek() 연산으로 맨위 데이터 확인: %d", peek());

	return 0;
}
```

### 연결리스트 기반 스택 구현

```c
#include <stdio.h>
#include <stdlib.h>

// 연결리스트 기반 스택
typedef struct _node {
	int data;
	struct _node* next;
} Node;

Node* top = NULL; // 연결리스트를 스택으로 쓰기 위해 사용 (연결리스트의 head와 비슷한 역할 수행)

void init() {
	if (top != NULL) {
		Node* delNode;
		while (top != NULL) {
			delNode = top;
			top = top->next;
			free(delNode);
		}
	}
}

void push(int data) {
	Node* newNode = (Node*)malloc(sizeof(Node));
	newNode->data = data;
	newNode->next = NULL;

	// 1. 무 -> 유
	
	if (top == NULL) {
		top = newNode;
	}
	else {
		newNode->next = top;
		top = newNode;
	}

	/* 중복된 코드를 제거한 효율적인 코드 (짧은)
	* if(top != NULL)
	*	newNode ->next =top;
	  top = newNode;
	*/
}

int pop() {

	if (top == NULL) {
		printf("꺼낼 데이터가 없습니다.\n");
		return -1;
	}
	else {
		Node* delNode;
		int returnData;
		delNode = top;

		top = top->next;
		returnData = delNode->data;
		free(delNode);
		return returnData;
	}
}

int peek() {
	if (top == NULL) {
		printf("꺼낼 데이터가 없어서 -1 리턴함\n");
		return -1;
	}
	else
		return top->data;
}

int main() {
	init(); // 스택 초기화
	push(10); push(20); push(30); push(40);

	printf("pop실행 : %d 반환\n", pop());
	printf("pop실행 : %d 반환\n", pop());
	push(100); push(200); push(300);

	printf("pop()실행 : %d 반환\n", pop());
	printf("peek()실행 : %d 반환\n", peek());

	init();
	printf("pop()실행 : %d 반환\n ",pop());
}

```

### 스택의 활용 예시
- 괄호의 연산
  - 괄호의 모양이 같아야한다.
  - 괄호의 열리고 닫히는 순서가 같아야한다.
  - 괄호의 갯수가 같아야한다.