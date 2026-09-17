# Service - Cluster IP, Node Port, Load Balancer

## Service

In a Kubernetes environment, a Service is a **virtual component that exposes applications running through Pods to the network.**

It also helps external applications connect with other external applications or users.

### Why Service?

First, let's understand why we use Services instead of just Pods.

Pods, in a Kubernetes environment, can fail at any time due to various factors.

In such cases, Controllers (which we'll learn about later) will recreate the Pod.

The recreated Pod will have a different IP address assigned. This means its reliability decreases.

In contrast, a Service is not deleted unless explicitly removed by the user. (It has a non-ephemeral characteristic).

Therefore, a Pod configured with a Service gains more reliability, and clients can access the Pod by connecting to the Service.

> You can think of Pods as ephemeral and Services as persistent.

![](https://miro.medium.com/v2/resize:fit:1400/format:webp/1*tnK94zrEwyNe1hL-PhJXOA.png)

The necessity of Service

## Cluster IP
As the name suggests, this object is inaccessible from outside and can only be accessed within the cluster.

It can connect multiple Pods, and when multiple Pods are connected, the Service distributes traffic to them.

It is typically used by authorized users (operators) to check the status of Pods.

## Node Port

In this option, a Cluster IP is also assigned by default.

However, the difference is that while Cluster IP connects Pods and the Service, this option connects the Service at the Node level.

The same port is assigned to all Nodes connected to the Kubernetes cluster, so when you connect to that Node and that port, you access the Service, and the Service then forwards traffic to the Pods connected to it.

If you set the `externalTrafficPolicy` option to `Local`, you can specify that traffic is only allocated to Pods on the accessed Node.

## Load Balancer

It retains the characteristics of Node Port.

It distributes traffic to each Node, and a separate external access IP is allocated for accessing the load balancer.

In a real production environment, you should use a Load Balancer instead of NodePort. This is because exposing the direct IP of a Node can pose security risks.

Since an external access IP must be allocated separately, you need to use products from external vendors (AWS, GCP, Azure).

We will continue with ExternalName next time.
