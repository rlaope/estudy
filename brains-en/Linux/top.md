# top

![](https://miro.medium.com/v2/resize:fit:1400/format:webp/1*TtUVAogH-IVZdTeu4AMpdg.png)

You can check the status of processes running on the server, as well as CPU and memory usage.

hotkey: 1 -> check individual CPUs, d -> change interval (can be set to 1 for 1-second intervals)

It's necessary to use the hotkey to display individual CPUs and check for any imbalance in CPU usage.

While the aggregated average usage `cpu(s)` might appear normal, if only specific CPUs are being utilized when viewed individually, it should be considered problematic (an abnormal state unless it's a single-threaded application). In such cases, you need to check for scenarios like `nginx`'s `worker=1` setting, which prevents full CPU utilization.

Among CPU usage metrics, `us`/`wa` are important.
- **us**: This represents the general CPU usage of user-level processes. A high `us` value indicates that processes are heavily utilizing the CPU.
- **wa**: This is interpreted as CPU usage caused by processes waiting for I/O operations.

Process states can be checked in the `s` column (D R S Z). (D and R affect Load Average.)
- **D**: Uninterruptible, a state where I/O is pending, corresponding to the `b` state in `vmstat`.
- **R**: Running, a state where the CPU is in use, corresponding to the `v` state in `vmstat`.
- **S**: Sleeping, a state where the process is not actively working but is dormant.
- **Z**: Zombie, a state where the process does not consume actual system resources but can cause PID exhaustion issues.

The `top` command allows you to check the status of processes, CPU, and memory usage on a server. If `us` is high among CPU usage metrics, it can be interpreted as a CPU-intensive process; if `wa` is high, it can be interpreted as an I/O-intensive process.

Servers typically have multiple cores, so it's important to check individual CPU usage to identify any imbalance in CPU utilization.

Process states include D, R, S, and Z, and you should check if there are many zombie processes on the server.
