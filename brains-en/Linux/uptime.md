# uptime

You can check the server's uptime, load average, and the number of users connected to the server. A key metric is the load average, which can be interpreted as CPU usage. Load average represents the number of processes in R and D states over 1, 5, and 15-minute intervals. While load average is expressed as a number of processes, it's important to remember that it's relative to the number of CPUs.

#### Load Average: Relative Concept
- 1 process, 1 CPU -> load average = 1
- 2 processes, 1 CPU -> load average = 2
- 1 process, 2 CPUs -> load average = 1
- 2 processes, 2 CPUs -> load average = 2

As shown above, even if the load average (interpretable as the number of processes) is the same, its meaning changes depending on the number of CPUs.

When there is only one CPU, the concept of processes sharing the CPU is called context switching.

If `load average > cpu`, it means there are more processes than the system can currently handle.

Even if the load average is less than the number of CPUs, it's not always fine. Appropriate action is needed based on the situation, by checking the process states (R, D).

- **R (cpu bound process)**: Running or runnable (on run queue)
    - This is the number of executable processes waiting for CPU access (or currently running).
    - If there are many R-state processes, CPU utilization is high, so you should consider increasing the number of CPUs or adjusting threads.
- **D (I/O bound process)**: uninterruptible sleep (usually I/O)
    - This is the number of processes blocked because they haven't been allocated I/O resources (i.e., processes in the queue for I/O).
    - If there are many D-state processes, it means I/O is heavily used. You should consider changing to disks with higher IOPS, improving I/O performance, reducing throughput, or using remote storage like file share/blob to increase IOPS.

In summary, when checking server load, you can use `uptime` to see how much load the server is experiencing via the load average. If `load average > cpu`, you need to identify which type of process (R or D) is the cause.

To check the process types, use `vmstat` and look at the `procs` column. If `r` is high, CPU-intensive processes are causing the load. If `b` is high, I/O-intensive processes are causing the load, and appropriate action should be taken for each type.

In other words, a high load average doesn't simply mean there are many processes trying to use the CPU; it can also mean there are many processes waiting for I/O operations due to an I/O bottleneck.

#### CPU, I/O Bound Test and `vmstat` Results

**CPU Bound Test, `uptime`, `vmstat 1 10`**
```python
test = 0
while True:
	test = test + 1
```

If you run the above task and check `uptime` and `vmstat`, you'll see the following:

![](https://miro.medium.com/v2/resize:fit:1400/format:webp/1*B2Y22SC8bRY8yEKAZO3um1.png)
The `r` value is 1 or 2, indicating that CPU-bound processes are the main cause of the increased load average.

**I/O Bound Test, `uptime`, `vmstat 1 10`**

```python
while True:  
f = open("./io_test.txt",'w')  
f.write("It's IO Test")  
f.close()
```

![](https://miro.medium.com/v2/resize:fit:1400/format:webp/1*TEo-bqWL15gRZqEyUKpuw.png)
`b` is almost always active, indicating that I/O-bound processes are the main cause of the increased load average.
