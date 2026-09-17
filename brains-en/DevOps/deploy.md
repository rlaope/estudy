# Types of Deployment Strategies (Rolling/Blue-Green/Canary)

## Blue-Green Deployment

![](https://velog.velcdn.com/images/jingrow/post/57f5e51e-6dc7-4f44-ada2-231d0e444d32/image.png)

The previous version can be called the blue environment, and the new version the green environment.

Only one version is published at a time. This method involves pre-building identical servers and then instantaneously switching routing to deploy the new version.

It has the advantage of allowing fast rollbacks and enabling testing of the new version while maintaining the production environment. However, it also has the disadvantage of requiring double the resources, leading to higher costs.

## Rolling Deployment

![](https://velog.velcdn.com/images/jingrow/post/57f5e51e-6dc7-4f44-ada2-231d0e444d32/image.png)

Rolling deployment is a strategy that gradually replaces the previous version of an application with a new version by completely replacing the infrastructure where the application is running.

Instances are replaced with the new version sequentially, in predefined units.

It is used when available resources are limited.

This method is useful when there are constraints on the number of servers, but since the number of instances decreases during deployment, server processing capacity must be considered in advance.

## Canary Deployment

![](https://velog.velcdn.com/images/jingrow/post/0e926b33-6f41-4c75-bca2-29d03ca1abc9/image.png)

It is a technique that reduces the risk of introducing a new software version into production by slowly releasing changes before making the new software version available to all users across the entire infrastructure.

This deployment method allows for quick, early detection and response to risks, much like a canary bird. It is said to be useful for monitoring error rates and performance because it configures old and new versions of servers and distributes a portion of traffic to the new version to determine if there are any errors.
