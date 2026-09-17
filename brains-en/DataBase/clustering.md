# Clustering (Active Clustering, Standby Clustering)

## Clustering
One of the database distribution techniques that involves having multiple DB servers to prepare for when one server goes down.

## Active Clustring


![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fk.kakaocdn.net%2Fdn%2FdeVt2f%2FbtqEOPtyyNu%2FBm4k5ilo6dReFoyVVP23XK%2Fimg.png)

Multiple DB servers are configured, and each server is kept in an Active state.

### Advantages
- Even if one server goes down, other servers immediately take over its role, so there is no service interruption.
- CPU and memory utilization can be increased.

### Disadvantages
- If a single storage is shared, a bottleneck may occur.
- Operating multiple servers simultaneously incurs higher costs.

## Standby Clustering

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fk.kakaocdn.net%2Fdn%2Fded8LQ%2FbtqEO02Erjo%2Fer820balKxEB3dkaBGaGn0%2Fimg.png)

Only one server is operated, and the remaining servers are kept in a Standby state. If the operating server goes down, the Standby server is switched to an Active state.

### Advantages
- It costs less compared to Active-Active clustering.

### Disadvantages
- When a server goes down, time is required to switch the Standby server to an Active state.
