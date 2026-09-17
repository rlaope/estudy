# UDS, 0000.sock

### UDS

UDS (Unix Domain Socket) is also known as an IPC Socket, and it is a local file-based socket that can send and receive data using the same API as TCP sockets.

The difference from TCP sockets is that it communicates between processes on the localhost, offering the advantages of very high speed and low memory consumption.

Another advantage of using UDS is that, apart from using the `sockaddr_un` structure when creating the socket, it's handled just like a regular socket. This makes it very easy to separate the server from the client later due to performance or security issues, even after initially configuring it with UDS.

A prime example of UDS usage is `mysql.sock`, the socket for MySQL.

<br>

### Checking UDS

If you specify UDS with the `ls -l` option, you can see an 's' prepended to the entry.

```bash
$ ls -l /var/lib/mysql/mysql.sock

srwxrwxrwx. 1 mysql mysql 0 Jul 19 10:20 /var/lib/mysql/mysql.sock
```

Here, 's' signifies a socket, and even the `file` command, which tells you the file type, will show it as a socket type.

```bash
$ file /var/lib/mysql/mysql.sock 
                       
/var/lib/mysql/mysql.sock: socket
```

Running the `lsof` command with the `-U` option allows you to view a list of all Unix domain sockets on the system.

```bash
$  lsof -U 

COMMAND      PID           USER   FD   TYPE             DEVICE SIZE/OFF    NODE NAME
systemd        1           root   15u  unix 0xffff9b9f575fbf00      0t0     164 /run/systemd/private type=STREAM
systemd        1           root   17u  unix 0xffff9b9fb1acf980      0t0 1573299 /run/systemd/journal/stdout type=STREAM
systemd        1           root   46u  unix 0xffff9b9f58366780      0t0   20542 type=DGRAM
systemd        1           root   63u  unix 0xffff9ba0e8266300      0t0   33828 type=STREAM
systemd        1           root   83u  unix 0xffff9b9f56470900      0t0   16877 /run/systemd/notify type=DGRAM
systemd        1           root   84u  unix 0xffff9b9f56476780      0t0   16879 /run/systemd/cgroups-agent type=DGRAM
systemd        1           root   85u  unix 0xffff9b9f56473a80      0t0   16881 type=DGRAM
systemd        1           root   86u  unix 0xffff9b9f56473f00      0t0   16882 type=DGRAM
systemd        1           root   91u  unix 0xffff9b9f565a6300      0t0   24191 /run/libvirt/libvirt-admin-sock type=STREAM
```

<br>

### mysql.sock

Just as TCP/IP sockets use IP addresses and ports for connections, Unix Domain Sockets use files.

`mysql.sock` is precisely the file used for this purpose. Therefore, when `mysqld` is running, the `mysql.sock` file exists, but it disappears when `mysqld` is stopped.

For MySQL to communicate with other systems, the `mysql.sock` file must be accessible to both the server and the client. If the server cannot write to or read from the location where `mysql.sock` is created, the server will error out and stop. If the client cannot access that file, a connection will not be established. Typically, issues with `mysql.sock` are not caused by incorrect access permissions.

When installed via RPM, the `mysql.sock` file is typically located at `/var/lib/mysql/mysql.sock`. This path can be configured during MySQL compilation using `--with-unix-socket-path=`. With this setting, `mysqld`, `mysql`, and `libmysqlclient.a` (or `.so`) all communicate using this configuration. The location of this domain socket can be changed even after compilation using command-line arguments or by specifying it in the `/etc/my.cnf` file.
