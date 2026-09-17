# 선형리스트 (LinearList)

- 데이터를 논리적인 순서대로 메모리에 연속하여 저장하는 구현하는 방식

- 데이터의 논리적인 순서와 기억 장소에 저장되는 물리적 순서가 일치하는 구조다.

- 배열을 이용해 구현한다.

<br>


### 배열을 사용한 리스트의 장점

- 인덱스(Index)로 접근할 수 있기 때문에 접근 속도가 매우 빠르다.

- 연속된 메모리 공간에 존재하기 때문에 관리하기가 편하다.


<br>

### 배열을 사용한 리스트의 단점

- 배열 사이즈가 고정되어 있기 때문에 공간의 낭비가 발생할 수 있음
- 삭제 작업의 어려움 (뒤에있는 값을 덮어쓰기)

<br>

## 선형리스트를 C언어로 구현해보았다.
```c
#include <stdio.h>

#define LIST_SIZE 10

int list[LIST_SIZE] = { 0 };
int numOfDatas = 0;

// LinearList - 데이터 ,메모리 연속적인 공간을 사용하여 저장-관리
// 포인터 필요 없다 왜 ? 배열의 index 라는 개념이 있으니까. 

void listInit() {
	// 초기화할 수 있다.
	numOfDatas = 0;
}

void insert(int data) {
	if (numOfDatas < LIST_SIZE) {
		list[numOfDatas++] = data;
	}
	else
		printf("리스트가 꽉 참");
}

// 탐색 
int search(int searchData) {

	if (numOfDatas == 0) {
		printf("리스트에 데이터가 없음");
		return -1;
	}

	for (int i = 0; i < numOfDatas; i++) {
		if (list[i] == searchData) {
			return i;
		}

		return -1;
	}
}

// 변경
void update(int targetData, int updateData) {

	if (numOfDatas == 0) {
		printf("리스트에 데이터가 없음");
		return;
	}

	for (int i = 0; i < numOfDatas; i++) {
		if (list[i] == targetData) {
			list[i] = updateData;
			return;
		}

	}
}

// 삭제
void doDelete(int deleteData) {

	if (numOfDatas == 0) {
		printf("리스트에 데이터가 없음");
		return;
	}

	for (int i = 0; i < numOfDatas; i++) {
		if (list[i] == deleteData) {
			for (int j = i; j < numOfDatas - 1; j++) {
				list[j] = list[j + 1]; 
			}
			numOfDatas -= 1;
			i -= 1;
		}

	}
}

printAll() {
	for (int i = 0; i < numOfDatas; i++) {
		printf("[%d] ", list[i]);
	}
	printf("\n\n");
}

int main() {
	listInit();

	insert(1);
	insert(2); 
	insert(4);
	insert(5);
	insert(3);
	insert(5);
	insert(5);

	printAll();

	doDelete(5);

	printAll();

	

	return 0;
}
```
- 탐색기능과 삭제기능의 알고리즘을 구현했다.