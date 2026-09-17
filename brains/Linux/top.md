# top

![](https://miro.medium.com/v2/resize:fit:1400/format:webp/1*TtUVAogH-IVZdTeu4AMpdg.png)

서버에 구동되고 있는 프로세스들의 상태와, cpu, memory 사용률을 확인할 수 있다.

hotkey: 1 -> 개별 cpu 확인, d -> 인터벌 변경(1로 하여 1초 간격으로 확인 가능)

hotkey로 개발 cpu를 나타내 cpu들의 사용 불균형이 있는지 체크할 필요가 있다.

종합된 평균 사용량 cpu(s)은 정상이 아니지만 개별로 봤을 때 특정 cpu만 사용된다면 문제가 있는 상황으로 봐야한다.(싱글 스레드로 구동되는 app이 아닌 이상 비정상 상태) 이 경우 nginx의 worker=1 세팅 처럼 cpu를 모두 사용하지 못하는 경우등을 체크해야한다.

cpu usage 중 주요 지표로 us/wa가 중요하다.
- **us**: user 수준의 프로세스의 일반적인 cpu 사용량으로 us가 높다면 프로세스가 cpu를 많이 사용하는 것으로 해석된다.
- **wa**: waiting, io작업을 대기하는 프로세스로 인해 사용되는 cpu사용량으로 해석한다.

프로세스 상태는 s 컬럼에서 (D R S Z) 확인 가능하다. (D, R은 Load Average에 영향을 준다.)
- **D**: Uninterruptible, io가 대기중인 상태로 vmstat상 b 상태다
- **R**: Running, cpu 사용중인 상태로 vmstat상태로 v상태다
- **S**: Sleeping, 실제 작업을 하지 않고 잠자는 상태다
- **Z**: Zombie, 실제 시스템 리소스를 사용하지 않지만, pid 고갈 문제를 일으킬 수 있다.

top 명령으로 서버 내 프로세스들의 상태, cpu, memory 사용량을 확인할 수 있으며 cpu 사용량 중 us가 높다면 cpu를 많이 사용하는 프로세스, wa가 높다면 io를 많이 사용하는 프로세스로 해석할 수 있다.

보통 서버들은 멀티코어기 때문에 개별 cpu의 사용량을 확인해서 cpu 사용에 불균형이 있는지 확인해야한다.

프로세스의 상태는 D R S Z가 있고 혹시 서버에 좀비 프로세스가 많지는 않은지 확인해야한다.



