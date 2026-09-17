# Redis Fragmentation Ratio

내부 단편화란 할당한 용량보다 메모리를 적게 사용하여 메모리가 남게 되는 것을 말한다.

리눅스에서 메모리 페이징으로 메모리를 관리하기 때문에 어느정도는 발생할 수 밖에 없다.

redis는 in memory data store로, 캐시로도 많이 쓰이기 때문에 메모리 관리가 중요하다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FUTjB5%2FbtrQPC1AzEy%2FA6l9FcigGijl42aQnC9ECK%2Fimg.jpg)

여러 지표중에서 Fragmentation Ratio에 대해서 알아보자

redis-cli 에서 INFO command로 확인할 수 있다.



```bash
redis> INFO 
"# Server redis_version:7.0.5
redis_git_sha1:00000000 
redis_git_dirty:0
redis_build_id:383256aa4e712b9d
redis_mode:standalone 
os:Linux 5.15.0-1015-aws x86_64 
arch_bits:64 
monotonic_clock:POSIX clock_gettime 

# Memory ... 
mem_fragmentation_ratio:1.04 
mem_fragmentation_bytes:9757104
... 
"
```

- used_memory_rss
	- rss(resident set size), physical memory actually used로 레디스를 띄우기 위해서 os가 할당한 메모리다.
- used_memory
	- redis가 실제 사용하고 있는 메모리다.
- mem_fragmentation_ratio
	- used_memory_rss / used_memory로 이상적인 경우, 1 또는 1보다 살짝 큰 수준 (ex. 1.04)를 기대할 수 있으며 1.5를 넘어가는 경우 심각한 수준이다.
	- 레디스는 일반적으로 별도의 설정을 하지 않으면, 메모리 할당 후 따로 release를 하지 않기 때문에 peak memory가 크고 평소보다 사용량이 적은 경우 정상일 수 있다.
	- MEMORY DOCTOR 명령어로 redis에 메모리 상태를 진단할 수 있다.
- mem_fragmentation_bytes
	- used_memory_rss - used_memory
	- 단편화의 크기 뿐만 아니라 프로세스의 오버헤드(allocator_* 지표 참고)등을 포함한 크디로 이 값의 절대값이 수~수십 MB 정도로 작으면, fragmentation ratio가 1.5 이상으로 크다고 해도 별 이상은 없다.


<br>

## High Fragmentation Ratio Problem Solution

이제 Fragmentation Ratio가 높을 경우 해결 방법에 대해서 알아보겠다. 

레디스의 상태가 정상 상태인지 비정상 상태인지에 대해, 두 가지의 경우를 볼 수 있다.

### 정상 상태

peak memory가 높아서 할당을 한 번 많이 받은 상태이고, used_memory에 큰 변동이 없다면, **단편화 문제가 아니고 그냥 할당을 엄청 해놓은거니 냅둬도 상관은 없다.**

*메모리가 정 필요하다면 레디스를 껐다 키면 된다.*

### 비정상 상태

이 상태일때는 명확한 답보다는 원인 분석을 우선시해야한다. 작은 키 값을 이상하게 많이 갖고 있다거나, 애플리케이션단에서 원인을 찾아봐야하고 절대로 flushall, flushdb는 사용해서는 안된다. O(N)이기 때문에

[memory purge](https://redis.io/commands/memory-purge/)를 사용하면 dirty page reclaim(수정된 페이지인 더티페이지를 회수하고 다시 사용 가능 상태로 만드는 과정)을 시도하지만, 느리다 이것도 주의가 필요하다 (jemalloc만 사용 가능)

**allocator를 변경하여 문제를 해결해볼수도 있다.**

redis는 memory allocator로 기본적으로 jemalloc을 사용한다. libc malloc, tcmalloc등 다른 memory allocator를 사용하는 것도 가능하지만 컴파일을 해야한다.

보통은 기본 allocator를 사용해도 상관 없겠지만, 강한 의심이 든다면 바꿔보는 것도 방법일 수 있다.

**active-defrag** 일명 조각 모음인데, 레디스 런타임시에 동작하고며 메모리를 자동으로 조각내어 발생하는 단편화를 해소하는 기능이다. 우리는 조각 모음이 동작하는 조건을 직접 설정할 수 있다.

- ACTIVE-DEFRAG-IGNORE-BYTES
- ACTIVE-DEFRAG-THRESHOLD-LOWER
- ...

active-defrag는 기본적으로 비활성화 되어 있으며 redis-cli에서 `CONFIG SET activedefrag yes`를 입력해 이 기능을 사용할 수 있다.

```c
########################### ACTIVE DEFRAGMENTATION #######################
#
# What is active defragmentation?
# -------------------------------
#
# Active (online) defragmentation allows a Redis server to compact the
# spaces left between small allocations and deallocations of data in memory,
# thus allowing to reclaim back memory.
#
# Fragmentation is a natural process that happens with every allocator (but
# less so with Jemalloc, fortunately) and certain workloads. Normally a server
# restart is needed in order to lower the fragmentation, or at least to flush
# away all the data and create it again. However thanks to this feature
# implemented by Oran Agra for Redis 4.0 this process can happen at runtime
# in a "hot" way, while the server is running.
#
# Basically when the fragmentation is over a certain level (see the
# configuration options below) Redis will start to create new copies of the
# values in contiguous memory regions by exploiting certain specific Jemalloc
# features (in order to understand if an allocation is causing fragmentation
# and to allocate it in a better place), and at the same time, will release the
# old copies of the data. This process, repeated incrementally for all the keys
# will cause the fragmentation to drop back to normal values.
#
# Important things to understand:
#
# 1. This feature is disabled by default, and only works if you compiled Redis
#    to use the copy of Jemalloc we ship with the source code of Redis.
#    This is the default with Linux builds.
#
# 2. You never need to enable this feature if you don't have fragmentation
#    issues.
#
# 3. Once you experience fragmentation, you can enable this feature when
#    needed with the command "CONFIG SET activedefrag yes".
#
# The configuration parameters are able to fine tune the behavior of the
# defragmentation process. If you are not sure about what they mean it is
# a good idea to leave the defaults untouched.
```

> 구성 매개변수를 사용하여 조각 모음 프로세스의 동작을 미세 조정할 수 있지만, 이들의 의미를 잘 모르는 경우 기본값을 그대로 두는 것이 좋다.

redis.conf의 주석을 보면 중간에 단편화 문제가 없으면 이 기능을 켤 필요가 없다고 한다.

조각모음이 실행될 때 cpu 사용량이 약간 증가할 수 있을 것 같으니 잘 고려해서 활성화시키자 (active-defrag는 jemalloc에서만 사용가능하다.)
