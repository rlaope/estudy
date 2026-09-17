# Queue

Unlike a stack, a queue has a First-In, First-Out (FIFO) structure. A stack is like a closed box where the first item in is the last item out, but a queue is a structure where the first item in is the first item out. A good example is the numbering system at a bank. If you visit a bank and are waiting, and someone who arrived after you is served first, you'd feel upset, right? To prevent such situations, banks distribute numbered tickets. This is to ensure that customers who arrived earlier receive service before those who arrived later. This bank numbering system is a queue.

### ex
Let's reconsider the bank's numbering system. Suppose I drew ticket number 619. Currently, service has been provided up to number 618. How should the numbering system internally remember the next number to be served? And when the latest customer draws a ticket, what number should they be given? How should it remember? The ticket machine is also a program implemented with programming. Should it check from number 1 to see if each customer has been served, and then determine the next number to call? This would be too inefficient. Simply put, if you only remember the first and last positions for service, it's easily solved.

> These positions are called front and rear in a queue.

![](./image/queue.png)

Looking at the image above, there are terms like Deletion and Insertion. However, many people use the terms Dequeue and Enqueue instead of these. Enqueue is the same as Insertion in the image above. It's like giving a ticket to the latest customer. Then Dequeue is removing the served ticket from the waiting list. Only then can we know the next number to call.

To summarize,
1. Enqueue: Adds an element to the very end of the queue, like issuing a ticket to the latest customer.
2. Dequeue: Deletes the element at the very front of the queue, like removing the ticket of a customer who has been served at the counter from the waiting list.
3. Peek: Reads the data located at the front, like checking who the next customer to be served is.
4. front: The position (index) at the very front of the queue, the number of the next customer to be served.
5. rear: The position (index) at the very end of the queue, the number of the last customer who arrived.

### Implementation

```java
public class ArrQueue {

	private int front;

	private int rear;

	private int size;

	private Object[] arrQueue;

	

	public ArrQueue(int size){

		this.front = 0;

		this.rear = -1;

		this.size = size;

		this.arrQueue = new Object[this.size];

	}

	

	//Queue 배열이 꽉 차있는지 확인

	public boolean isFull(){

		if(rear >= size-1) return true;

		else return false;

	}

	

	//Queue 배열이 비어있는지 확인

	public boolean isEmpty(){

		if(rear < front) return true;

		else return false;

	}

	

	//원하는 데이터를 추가하는 메쏘드

	public void enQueue(Object item){

		if(isFull()) throw new ArrayIndexOutOfBoundsException();

		

		this.rear++;

		arrQueue[rear] = item;
	}

	

	//front에 위치한 데이터가 무엇인지 확인하는 메쏘드

	public Object peek(){

		return arrQueue[front];

	}

	

	//Queue의 front에 위치한 데이터 삭제

	public Object deQueue(){

		if(isEmpty()) throw new ArrayIndexOutOfBoundsException();

		Object backUpItem = peek();

		this.front++;

		return backUpItem;

	}

}

public class main {


	public static void main(String[] args) {

		ArrQueue arrQueue = new ArrQueue(4);

		arrQueue.enQueue("Hello");

		arrQueue.enQueue(5);

		arrQueue.enQueue("queue");

		

		while(!arrQueue.isEmpty()){

			System.out.println(arrQueue.deQueue());

		}

	}


}
```
