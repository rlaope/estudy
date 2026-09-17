# 연결리스트 (linkedList)

### 연결리스트의 특징
- 데이터가 물리적으로 인접해있지는 않으나 데이터간의 연결고리를 사용하여 연속적으로 (저장하는 것 처럼) 관리

<br>

### 노드
-  데이터를 저장하는 하나의 단위
-  노드에는 데이터를 저장해야하고 + '다음 노드에 대한 주소'를 저장

<br>

### c언어로 구현해 보았다.

```c
#include <stdio.h>
#include <stdlib.h>

typedef struct _node {
	int data;
	struct _node* next;
} Node;

// '가변적' . '동적' 데이터를 저장하는 길이가 정해지 있지 않음 => c에서의 '동적 할당'
Node* head, * tail;
 

void insert(int data) {

	Node* newNode = (Node*)malloc(sizeof(Node)); // malloc : 메모리 용량 만큼 공간을 할당시켜줌
	Node n1;

	newNode->data = data;
	newNode->next = NULL;


	if (head == NULL) { // 연결리스트에 노드 존재 x
		head = newNode;
	}
	else {
		tail->next = newNode;
	}
	tail = newNode;
}

/*
 출력, 조회(-> 변경 , 삭제)
 삽입(데이터의 첫 번째 위치에 새로운 데이터가 삽입되도록 하기)
 삽입(데이터가 정렬되어서 삽입되도록 하기)
*/ 

void printAll() {
	Node* cur = head;
	while (cur != NULL) {
		printf("[%d]", cur->data); // cur을 읽는다.
		cur = cur->next; // cur을 이동시킨다.
	}
	printf("\n");
}

// 조회 (변경 -> 삭제)
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

// 삭제(특정한 데이터(노드)가 존재한다면 삭제)
void deleteNode(int deleteData) {
	Node* cur = head; 
	Node* prev = NULL;
	Node* delNode = NULL;

	// case 1 . 지우고 싶은 노드가 유일할 경우
	// case 2 . 지우고 싶은 노드가 첫 번째 노드인 경우
	// case 3.  지우고 싶은 노드가 두 번째 이후 노드인 경우

	if (head == tail && cur->data == deleteData) { // 유일한 노드 삭제
		delNode = cur;
		free(delNode);
		head = NULL;
		tail = NULL;
		return;
	}

	while (cur != NULL) {
		delNode = cur;
		if (cur->data == deleteData) {
			if (cur == head) { // 다수의 노드중 첫번째 노드 삭제
				head = cur->next;
				cur = cur->next;
			}
			else {  // 여러 노드중 첫번재가 아닌 노드 삭제
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
	// 테스트 케이스 작성
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