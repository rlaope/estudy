# Blue/Green Deployment and Architecture

### Zero-Downtime Deployment

As the name suggests, it refers to deploying an application without any downtime.

## Blue/Green Deployment

It's a deployment strategy that allows new updates or changes to an application to be deployed without downtime.

It involves creating two identical environments: the previously operational environment (Blue) and a staging environment (Green). Updates are deployed to the Green environment, and after thorough testing, traffic is switched from the Blue environment to the Green environment.

This ensures that users do not experience any downtime or interruptions during the deployment process.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FdAUFUz%2FbtrjjQNguQL%2FIFp7c0CXy5IS7Mrzhkbji1%2Fimg.png)

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FcSUemw%2FbtrjkD0YcME%2FL3gplugLTxlfkbGaLMy36K%2Fimg.png)

Prepare the new version in an environment identical to the old version.

Switch the load balancer's routing all at once.

### Advantages
- It's easy to reuse the old version's environment or roll back.
- It ensures that users do not experience any downtime or interruptions during the deployment process.

### Disadvantages
- Requires double the system resources.

## Blue/Green Architecture

I've designed a Blue/Green architecture.

![](./image/blue_green.png)
