# dmesg

![](https://miro.medium.com/v2/resize:fit:1400/format:webp/1*Ses2zKpNmz3XYHmVyfRe-w.png)

kenrel의 다양한 메시지를 출력해주는 명령어로 시스템 성능 분석, 트러블슈팅에 사용할 수 있다.

oome(out of memory error), syn flooding 과 같은 에러를 분석하고 검출이 가능하다.

- **oome(out of memory error**는 시스템에 가용한 메모리가 부족해 더이상 프로세스가 할당해줄 메모리가 없을때 발생하는 에러다. oome가 발생하면 커널에서 oom killer process가 생성되고 종료되어야할 프로세스를 oom_score 값을 기준으로 선택해 종료시킨다. (스코어가 높을수록 우선순위가 높음)

`oome 발생` -> `oom score기준 프로세스 선택` -> `프로세스 종료` -> `안정화` 순으로 처리된다.

oom_score는 프로세스의 메타데이터를 볼 수 있는 /proc 경로에서 각 프로세스 pid로 이동하여 확인이 가능하다.

![](https://miro.medium.com/v2/resize:fit:1400/format:webp/1*VUdCv5r19haL-NgqV5a-QA.png)

- `dmesg -TL | grep -i "syn flooding"`으로 확인 가능한 **syn flooding**은 악의적인 공격자가 syn 패킷을 대량으로 보내서 서버의 소켓을 고갈시키는 공격으로 요즘은 서버 앞단에 프록시 서비스 lb, agw(waf)를 두기 때문에 서버에서 발생하는 일은 드물다.
- 만약 직접 공격을 받는다면 system에서 syn cookie 기능으로 공격 방어가 가능하며 tcp 3 way handshake시 syn backlog를 사용하지 않도록 하는 기능이다. (syn backlog는 syn에 대한 메타데이터를 저장하는 queue)
- 다만 syn cookie를 사용시 기타 tcp option 헤더를 무시하기 때문에 성능이 저하될 수 있지만, 공격에 의해 서비스가 불통이되는 것보다는 나으니 차선책이 된ㄷ다.

정리하면 dmesg로 oome, syn flooding 공격과 같은 상황을 감지할 수 있도록 커널 메시지를 확인할 수 있는 명령어로, oome가 발생하는 경우 메모리 누수가 발생하는 애플리케이션이 있는지 점검하고 누수를 막고 메모리를 확보해야하며, syn flooding 공격의 경우 방화벽을 확인해야 하며, 서버 내에는 syn cookie 기능을 활성화해서 막을 순 있지만 그 전에 미리 서버 앞단에 방화벽을 두어 방어하는게 효율적이다.

### 테스트

**oome 상태 재현**
```c
#include <stdlib.h>  
  
int main(void) {  
void* ptr;  
while (1) {  
ptr = malloc(1024 * 1024);  
if (ptr == NULL) {  
break;  
}  
}  
return 0;  
}  
  
# Compile & Run  
gcc -o oome oome.c  
./oome
```

![](https://miro.medium.com/v2/resize:fit:1400/format:webp/1*X6JeZAQXfZ5Crdaa4vLkMg.png)
oom이 발생해 oom killer가 프로세스를 종료하는 것을 메시지에서 확인해볼 수 있다.