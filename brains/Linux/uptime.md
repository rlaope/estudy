# uptime

서버의 가동 시간, 서버의 load average, 서버에 접속한 user 수를 확인할 수 있음, 주요 지표로 load average가 있는데 이는 cpu usage로 해석이 가능하며 load average는 단위시간 1, 5, 15분동안 R, D 상태의 프로세스 개수로 생각이 가능하다. 프로세스 개수로 load average를 나타내지만, load average는 cpu 개수에 의해 상대적인 점을 상기해야한다.

#### Load Average 상대 개념
- process 1, cpu 1 -> load average = 1
- process 2, cpu 1 -> load average = 2
- process 1, cpu 2 -> load average = 1
- process 2, cpu 2 -> load average = 2

위처럼 부하 평균(프로세스 수로 해석 가능)이 동일하더라도 cpu 개수에 따라 의미가 달라진다.

cpu가 1개일 경우 프로세스가 cpu를 나눠쓰는 개념을 context switching이라고 한다.

`load average > cpu` 라면 현재 처리 가능한 수준에 비해 많은 수에 프로세스가 존재한다는 의미로 해석된다.

부하 평균이 cpu개수보다 작다 하더라도 항상 괜찮은 것은 아니다. 프로세스의 상태를 확인하면서 상황에 맞는 적절한 조치가 필요하다. (R, D)

- **R(cpu bound process)**: Running or runnable (on run queue)
	- cpu 접근 대기중인 실행 가능한 프로세스의 수다. (현재 실행중인 프로세스의 개수)
	- R 상태 프로세스가 많다면 cpu 사용률이 많으므로 cpu 개수를 늘리거나 스레드 조정을 해야한다.
- **D(I/O bound process)**: uninterruptible sleep (usuaaly I/O)
	- I/O 자원을 할당받지 못해 블록된 프로세스의 수다. (io를 위해 대기열에 있는 프로세스 개수)
	- D 상태의 프로세스가 많다면 io를 많이 사용하는 상태이므로 iops가 높은 디스크로 변경하거나 io 성능을 높이거나, 처리량을 줄이고 혹은 파일쉐어/blob같은 원격 스토리지를 사용해 iops를 높여야한다.

정리하면 서버 부하 체크시 `uptime`을 사용해 서버가 얼마나 많은 부하를 받고있는지 load average를 확인할 수 있으며, 이때 `load average > cpu` 일 경우 어떤 종류(R,D) 프로세스가 원인인지 확인해야한다.

프로세스 종류를 확인할 때는 `vmstat` 으로 procs 컬럼을 확인하고, r이면 cpu 사용이 많은 프로세스가 부하를 일으키는 중이고, b면 io가 많은 프로세스가 부하를 일으키는 중이므로 종류에 맞게 적절한 조치를 해야한다.

즉 부하 평균이 높다는 것은 단순히 cpu를 사용하려는 프로세스가 많다고 볼 수 없고 io에 병목이 생겨 io 작업을 대기하는 프로세스가 많을 수 있다는 경우를 생각해야한다.

#### cpu, io bound test and result vmstat

**cpu bound test, uptime, vmstat 1 10**
```python
test = 0
while True:
	test = test + 1
```

위 작업을 돌리고 uptime, vmstat을 확인해보면 다음과 같다.

![](https://miro.medium.com/v2/resize:fit:1400/format:webp/1*B2Y22SC8bRY8yEKAZO3umw.png)
r의 값이 1, 2로 cpu bound의 프로세스가 load average를 높이는 주요 원인임을 파악이 가능하다.

**io bound test, uptime, vmstat 1 10**

```python
while True:  
f = open("./io_test.txt",'w')  
f.write("It's IO Test")  
f.close()
```

![](https://miro.medium.com/v2/resize:fit:1400/format:webp/1*TEo-bqWL15gRZqEyUKpuww.png)
b에 거의 활성화 되어 있으며 io bound의 프로세스가 load average를 높이는 주요 원인임을 파악이 가능하다.

