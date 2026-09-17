# Stack

- A stack is a `linear list` where insertions and deletions occur only at one end.
> LIFO (Last In, First Out) method

- Stack operations include push, pop, and peek.
  - `push`: Inserts data into the stack.
  - `pop`: Deletes data.
  - `peek`: An operation that inspects the data pointed to by the stack's current top pointer.

### Stack Implementation

```c
#include <stdio.h>
#include <stdlib.h>

// Filename: stack.c
// Goal: Implement stack (push, pop, peek) functionality
// using arrays and linked lists

// 1. Array-based implementation
#define STACK_SIZE 10

int topIdx = -1; // Where is the topmost data of the stack stored? 
int arrStack[STACK_SIZE];

void init() {
	topIdx = -1; // Define stack as empty

}

void push(int data) {
	if(topIdx < STACK_SIZE - 1)
		arrStack[++topIdx] = data;
}

int pop() {
	// Decrement topIdx by 1 + return the value that topIdx was pointing to
	if(topIdx >= 0)
		return arrStack[topIdx--];

		printf("No data to pop");
		exit(-1);
}


int peek() {
	return arrStack[topIdx];
}

int main() {

	push(1); push(2); push(3); push(4); push(5);
	push(11); push(22); push(33); push(44); push(55);
	push(111);

	printf("Performing pop() operation: Popped %d\n", pop());
	printf("Performing pop() operation: Popped %d\n", pop());
	printf("Performing pop() operation: Popped %d\n", pop());

	printf("Checking topmost data with peek() operation: %d", peek());

	return 0;
}
```

### Linked List-based Stack Implementation

```c
#include <stdio.h>
#include <stdlib.h>

// Linked list-based stack
typedef struct _node {
	int data;
	struct _node* next;
} Node;

Node* top = NULL; // Used to use a linked list as a stack (performs a similar role to a linked list's head)

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

	// 1. Empty -> Non-empty
	
	if (top == NULL) {
		top = newNode;
	}
	else {
		newNode->next = top;
		top = newNode;
	}

	/* Efficient code with removed duplication (shorter)
	* if(top != NULL)
	*	newNode ->next =top;
	  top = newNode;
	*/
}

int pop() {

	if (top == NULL) {
		printf("No data to pop.\n");
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
		printf("No data to pop, returning -1\n");
		return -1;
	}
	else
		return top->data;
}

int main() {
	init(); // Initialize stack
	push(10); push(20); push(30); push(40);

	printf("pop executed: %d returned\n", pop());
	printf("pop executed: %d returned\n", pop());
	push(100); push(200); push(300);

	printf("pop() executed: %d returned\n", pop());
	printf("peek() executed: %d returned\n", peek());

	init();
	printf("pop() executed: %d returned\n ",pop());
}

```

### Examples of Stack Usage
- Parenthesis operations
  - The shapes of the parentheses must match.
  - The opening and closing order of parentheses must be correct.
  - The number of parentheses must be equal.
