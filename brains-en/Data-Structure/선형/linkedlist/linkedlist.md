# Linked List

### Characteristics of Linked Lists
- Data is not physically contiguous, but it is managed sequentially (as if stored contiguously) using links between data.

<br>

### Node
- A single unit that stores data.
- A node must store data + 'the address of the next node'.

<br>

### Implemented in C.

```c
#include <stdio.h>
#include <stdlib.h>

typedef struct _node {
	int data;
	struct _node* next;
} Node;

// 'Variable'. The length for storing 'dynamic' data is not fixed => 'dynamic allocation' in C
Node* head, * tail;
 

void insert(int data) {

	Node* newNode = (Node*)malloc(sizeof(Node)); // malloc: Allocates space equal to the memory capacity
	Node n1;

	newNode->data = data;
	newNode->next = NULL;


	if (head == NULL) { // No node exists in the linked list
		head = newNode;
	}
	else {
		tail->next = newNode;
	}
	tail = newNode;
}

/*
 Print, Search (-> Modify, Delete)
 Insert (to insert new data at the first position of the data)
 Insert (to insert data in sorted order)
*/ 

void printAll() {
	Node* cur = head;
	while (cur != NULL) {
		printf("[%d]", cur->data); // Read cur.
		cur = cur->next; // Move cur.
	}
	printf("\n");
}

// Search (Modify -> Delete)
int find(int findData) {
	Node* cur = head;
	while (cur != NULL) {
		if (cur->data == findData) {
			return cur->data;
		}
		cur = cur->next;
	}
	return -1;
}

void update(int targetData , int updateData) {
	Node* cur = head;
	while (cur != NULL) {
		if (cur->data == targetData) {
			cur->data = updateData;
		}
		cur = cur->next;
	}
	return -1;
}

// Delete (if a specific data (node) exists, delete it)
void deleteNode(int deleteData) {
	Node* cur = head; 
	Node* prev = NULL;
	Node* delNode = NULL;

	// case 1. If the node to be deleted is unique
	// case 2. If the node to be deleted is the first node
	// case 3. If the node to be deleted is the second or later node

	if (head == tail && cur->data == deleteData) { // Delete unique node
		delNode = cur;
		free(delNode);
		head = NULL;
		tail = NULL;
		return;
	}

	while (cur != NULL) {
		delNode = cur;
		if (cur->data == deleteData) {
			if (cur == head) { // Delete the first node among multiple nodes
				head = cur->next;
				cur = cur->next;
			}
			else {  // Delete a node that is not the first among multiple nodes
				cur = cur->next;
				prev->next = cur;
				if (delNode == tail) {
					tail = prev;
				}
			}
			free(delNode);
		}
		else {
			prev = cur;
			cur = cur->next;
		}
	}
	return -1;

}

int main() {
	// Write test cases
	insert(1);
	printAll();
	deleteNode(1);
	printAll();

	insert(0); insert(1); insert(1);
	insert(3); insert(4);
	insert(5); insert(5); insert(7); 
	insert(9); insert(9); 
	insert(11);
	printAll();

	deleteNode(1);
	printAll();
	deleteNode(7);
	printAll();
	deleteNode(5);
	printAll();

	deleteNode(11);
	printAll();
	deleteNode(9);
	printAll();
}
```
