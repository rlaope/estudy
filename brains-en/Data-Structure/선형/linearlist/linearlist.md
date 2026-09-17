# LinearList

- A method of storing data consecutively in memory in logical order.

- A structure where the logical order of data matches its physical storage order in memory.

- Implemented using an array.

<br>

### Advantages of an Array-Based List

- Access speed is very fast because it can be accessed by index.

- Easy to manage because it resides in contiguous memory space.

<br>

### Disadvantages of an Array-Based List

- Fixed array size can lead to wasted space.
- Difficulty in deletion operations (overwriting with subsequent values).

<br>

## Implemented a LinearList in C.
```c
#include <stdio.h>

#define LIST_SIZE 10

int list[LIST_SIZE] = { 0 };
int numOfDatas = 0;

// LinearList - stores and manages data using contiguous memory space
// No pointer needed. Why? Because there's the concept of array index. 

void listInit() {
	// 초기화할 수 있다.
	numOfDatas = 0;
}

void insert(int data) {
	if (numOfDatas < LIST_SIZE) {
		list[numOfDatas++] = data;
	}
	else
		printf("List is full");
}

// Search 
int search(int searchData) {

	if (numOfDatas == 0) {
		printf("No data in list");
		return -1;
	}

	for (int i = 0; i < numOfDatas; i++) {
		if (list[i] == searchData) {
			return i;
		}

		return -1;
	}
}

// Update
void update(int targetData, int updateData) {

	if (numOfDatas == 0) {
		printf("No data in list");
		return;
	}

	for (int i = 0; i < numOfDatas; i++) {
		if (list[i] == targetData) {
			list[i] = updateData;
			return;
		}

	}
}

// Delete
void doDelete(int deleteData) {

	if (numOfDatas == 0) {
		printf("No data in list");
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
- Implemented search and delete algorithms.
