# Namespace, Container + runc syscall tracking

The official Docker documentation states that namespace = container.

What is a container? It's a containerization technology that allows you to create and use Linux containers, a software virtualization method that enables isolated execution on a shared operating system.

How does a container achieve virtualization? Why is it explained that container = namespace?

**To put it simply, Docker provides an isolated space for containers using a technology called namespace.**

It's okay if you don't understand. In fact, it's normal not to understand yet. Now, let's try to understand this statement.

```yml
volumes:
  wordpress:
    driver_opts:
      type: none
      o: bind
      device: /User/Khope/data/woredpress
  mariadb:
    driver_opts:
      type: none
      o: bind
      device: /User/Khope/data/mariadb
```

I've written a compose file that creates volumes mounted to my host space like this.

Now, if you type `id`, you'll see the `khope` (host) user and the `ctr` user for `/data/mariadb` (which maps to `/var/lib/mysql`).

Let's create a file in the container and check its owner. I created `file.txt` using `touch` and then ran `ls -al | grep "file"`.

```bash
# /var/lib/mysql (in container)
$ ls -al | grep "file"
>> ctr              ctr

# ~/data/mysql
$ ls -al | grep "file"
>> khope              khope
```

As you can see, the user who `grep`ped the file in the container and the user who `grep`ped it on the host both claim to be its owner.

What if we check the process output using `watch ps ax`? You can also confirm that the same process has different PIDs in the container and on the host. (Try it yourself)

This clearly shows the difference between Docker and a virtual machine.

It's that the Host and Container share resources. And when we use them, we enable them to run without interference through a certain level of isolation.
It is namespace that enables this functionality. **To summarize, a certain level of isolation can be defined as a container, and an independent virtual space can be defined as a namespace.**

I mentioned earlier that namespace = container, but more precisely, since a container operates within the space created by a namespace, it might be considered a dependent concept.

<br>

### containerd

If you look at the process tree structure with `pstree`, you can observe containerd, containerd-shim, etc., and you can see that `watch` is running by containerd-shim.
What is containerd here? It is a standard container runtime focused on simplicity and robustness for portability.

Haha, that's too difficult to understand. Simply put, **it manages the entire container lifecycle on the host system.**

containerd has very minimal runtime requirements, and most interactions with Linux containers are handled through runc.

Since too many concepts are coming up, let's ignore runc for now and briefly look at something simpler.

- docker build
- docker pull
- docker run

When you enter the commands above, they are passed to dockerd, and dockerd automatically (by default) starts containerd. (Although runc actually handles the practical container management.)

Ah, you might find the above sentence too difficult. That's because it contains information not covered in this article. Don't despair. Just know that these things exist.

> cli <-> dockerd -> containerd -> runc is the communication structure.

When we type a CLI command, it communicates with dockerd. Since containerd manages and creates containers before dockerd starts, it is automatically executed first by dockerd. containerd then uses runc to actually create and run containers.

Let's dig a bit deeper. To view Docker logs, type `strace -f -p $(pidof container) -o docker_log.txt`. Using the above will trace the containerd process.

Next, let's create and run a new container using `docker run -it --name debina debain:buster`.

Then, a log file will be left with an "attached" message, and if you examine it, you can track syscalls.

![](https://velog.velcdn.com/images/az0856/post/33d0de39-bc3d-43a5-83aa-35636af59628/image.png)

It looks roughly like this, and you can see `runc` being executed with the `execve` command. Let's look at the logs after `runc` is executed.

![](https://velog.velcdn.com/images/az0856/post/09c57fe0-80cd-4038-b3e8-717c8a38565d/image.png)

It calls a syscall called `unshare`. If you look at the line just above it, you'll see the command `prctl(PR_SET_NAME "runc:[1:CHILD]") = 0`. `prctl` is a command that manipulates a process's thread, and the `PR_SET_NAME` option is used to specify the name of the calling thread. However, I hadn't created a separate thread, so I looked it up to see if it creates one beforehand. It turns out that `PR_SET_NAME` **can also change the process name, and if used within a process, the process name changes and its associated threads are affected. If called from a thread, only that thread's name changes.**

Calling `unshare` here creates a new child process. There's also a `CLONE_NEWPID` option, which creates a new PID namespace (not shared with the calling process). The process that called `unshare` does not move into the new namespace, but the child created by the calling process is reflected to act as `init(1)` in the new namespace.

In short, it creates a new process and forks it from the parent process.

**This is cool, so I'll include an English document.** -> https://man7.org/linux/man-pages/man2/unshare.2.html

To summarize, a container is an isolated space from a namespace, `runc` actually creates and manages containers, and a sub-thread of `runc` creates a new namespace through the `unshare()` syscall.
