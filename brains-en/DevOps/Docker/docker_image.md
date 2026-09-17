# Creating a Docker Image

After creating an Ubuntu container, I'll create a file to make changes to the existing image.

```s
$ docker run -it --name commit_test ubuntu:18:04
$ echo test_first! >> first (컨테이너 내부에서 실행)
```

After exiting the container, I'll use the `docker commit` command to turn the container into an image.

```s
$ docker commit -a "ckstn0777" -m "my first commit" commit_test commit_test:first
```

- The `-a` option stands for author and includes metadata indicating the image creator in the image.
- The `-m` option is the commit message.
- `commit_test` is the name of the container to commit.
- `commit_test:first` is the name and tag of the image to be created. If no tag is entered, it defaults to `latest`.

Check if it was created successfully with `docker images`.

![](https://velog.velcdn.com/images%2Fckstn0777%2Fpost%2F38523607-56f7-42a6-82e9-be1f6add316e%2Fimage.png)

Now you can create a container using the image you just made.

You can also confirm that the `first` file exists.

```s
$ docker run -it --name commit_test2 commit_test:first
```

![](https://velog.velcdn.com/images%2Fckstn0777%2Fpost%2Fd350b244-8505-4f7d-a67a-739ace36c1bb%2Fimage.png)

## Understanding Image Structure

```s
$ docker inspect ubuntu:18:04
```
![](https://velog.velcdn.com/images%2Fckstn0777%2Fpost%2F34672132-440f-4393-a84d-23e4bf84f1ad%2Fimage.png)

Next, let's look at the layers of `commit_test:first`.

![](https://velog.velcdn.com/images%2Fckstn0777%2Fpost%2F8f3d41a4-68bf-420d-91fb-db5ff00469bc%2Fimage.png)

The difference between the two is that `commit_test:first` has one more layer at the very bottom.

This is likely the part that records the changes.

Above, the Ubuntu image size was 64.2MB, and `commit_test` was also 64.2MB. So, would they occupy a total of 128.4MB?

When committing an image, only the changes made in the container are saved as a new layer, and a new image is created including that layer. Therefore, the actual size will be 64.2MB + the size of the `first` file.

## Deleting Images
Suppose `commit_test2` container was created from `commit_test:first`, and you try to delete `commit_test:first`. Will it work correctly? (Note: the image deletion command is `docker rmi`.)

![](https://velog.velcdn.com/images%2Fckstn0777%2Fpost%2F6e9901be-894c-46f6-aa42-b998946f6184%2Fimage.png)

It says container 1a73..(=`commit_test2`) is in use. I'll force delete it and then delete the image as well.

```s
$ docker rm -f commit_test2
$ docker rmi commit_test:first
```

By the way, I didn't mention that after creating a `second` file in `commit_test2`, I created an image named `commit_test:second`.

![](https://velog.velcdn.com/images%2Fckstn0777%2Fpost%2F962a33fc-053a-4f06-8c29-6f93ff14be9e%2Fimage.png)

So, if the `commit_test:first` image was deleted, would its layer file disappear? No. `commit_test:second` would still have it.

I'll try deleting `commit_test:second`.

![](https://velog.velcdn.com/images%2Fckstn0777%2Fpost%2F36deb2da-b14c-44b0-b69f-03537a45309f%2Fimage.png)

This time, the layers being deleted are shown.

Ultimately, what we can learn here is that an image's files are only truly deleted if its parent image no longer exists.

## Extracting Images
There are times when you need to save a Docker image as a single binary file, for example, to store it separately or move it.

However, since it's a single file rather than a layered structure, each image will occupy its full size.

## Distributing Images
In fact, image distribution is more important than image extraction.

There are two methods: using the Docker Hub image repository or using a private registry that you create yourself. I'll demonstrate how to use the Docker Hub repository.

[Docker Hub](https://hub.docker.com/)

![](https://velog.velcdn.com/images%2Fckstn0777%2Fpost%2Fc0341984-19d9-47f1-8700-bea9c1151a27%2Fimage.png)

### Creating an Image Repository
It looks somewhat similar to GitHub. Click 'Create Repository'.

![](https://velog.velcdn.com/images%2Fckstn0777%2Fpost%2F2c53c866-9834-4aa4-96cf-a908f8028bae%2Fimage.png)

### Creating an Image to Upload to the Repository

```s
$ docker commit commit_test my-image-name:0.0
```

First, create an image from the `commit_test` container.

However, this image cannot be uploaded to an image repository as is. When uploading to a specific repository, you must prefix it with the repository name (username).

```s
$ docker tag my-image-name:0.0 사용자이름/my-image-name:0.0
```

This doesn't delete the existing image; it's like copying and pasting the image.

![](https://velog.velcdn.com/images%2Fckstn0777%2Fpost%2Fde99bb20-4544-4fca-8b14-86e8f00eb143%2Fimage.png)

### Logging In

```s
$ docker login
```
```s
$ docker push ckstn0777/my-image-name:0.0
```

![](https://velog.velcdn.com/images%2Fckstn0777%2Fpost%2Faebb3e10-98f2-4ee3-aa64-4b6a40a04a19%2Fimage.png)

As you can see, only the changed layers are transferred to the image repository.

![](https://velog.velcdn.com/images%2Fckstn0777%2Fpost%2Ff027c76b-d3c6-4fd2-be1b-5f452cadad35%2Fimage.png)

### Pulling Images

```s
$ docker pull 사용자이름/my-image:0.0
```

![](https://velog.velcdn.com/images%2Fckstn0777%2Fpost%2Fb0539fd3-f0f0-49be-862d-2469ca4e1152%2Fimage.png)
