# dmesg

![](https://miro.medium.com/v2/resize:fit:1400/format:webp/1*Ses2zKpNmz3XYHmVyfRe-w.png)

It's a command that outputs various kernel messages, which can be used for system performance analysis and troubleshooting.

It can analyze and detect errors such as OOME (Out Of Memory Error) and SYN flooding.

- **OOME (Out Of Memory Error)** is an error that occurs when the system runs out of available memory and can no longer allocate memory to processes. When OOME occurs, the kernel creates an OOM killer process, which selects and terminates processes based on their `oom_score` value. (A higher score indicates higher priority.)

The process is handled in the following order: `OOME occurs` -> `process selection based on OOM score` -> `process termination` -> `stabilization`.

The `oom_score` can be checked by navigating to each process's PID within the `/proc` path, where process metadata can be viewed.

![](https://miro.medium.com/v2/resize:fit:1400/format:webp/1*VUdCv5r1haL-NgqV5a-QA.png)

- **SYN flooding**, which can be checked with `dmesg -TL | grep -i "syn flooding"`, is an attack where a malicious attacker sends a large number of SYN packets to exhaust a server's sockets. Nowadays, it's rare for this to occur directly on the server because proxy services like LBs and AGWs (WAFs) are placed in front of the server.
- If a direct attack occurs, the system can defend against it using the SYN cookie feature, which prevents the use of a SYN backlog during the TCP 3-way handshake. (A SYN backlog is a queue that stores metadata for SYN packets.)
- However, using SYN cookies can degrade performance because it ignores other TCP option headers. Nevertheless, it's a better alternative than having the service become unavailable due to an attack.

In summary, `dmesg` is a command that allows you to check kernel messages to detect situations like OOME and SYN flooding attacks. If OOME occurs, you should check for applications causing memory leaks, stop the leaks, and free up memory. In the case of a SYN flooding attack, you should check the firewall. While you can activate the SYN cookie feature on the server to block it, it's more efficient to place a firewall in front of the server beforehand for defense.

### Test

**Reproducing OOME**
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
You can see in the message that OOM occurred and the OOM killer terminated the process.
