# CI/CD Pipeline and Implementation Process

### CI/CD Pipeline
- A CI/CD pipeline is a series of steps that must be performed to deliver a new version of software.
- A `Continuous Integration/Continuous Delivery (CI/CD)` pipeline is a method focused on delivering software more effectively through DevOps or Site Reliability Engineer (SRE) approaches.
- CI/CD pipelines improve the application development process by introducing monitoring and automation, especially in the integration and testing phases, and the delivery and deployment phases.
- While each stage of a CI/CD pipeline can be executed manually, its true value emerges when automated.

### Elements of a CI/CD Pipeline
The stages of a CI/CD pipeline consist of different subsets of tasks, which are called **pipeline stages**. Common pipeline stages include:

- **Build**: The stage of compiling the application.
- **Test**: The stage of testing the code. Automating this stage can reduce time and effort.
- **Release**: The stage of delivering the application to a repository.
- **Deploy**: The stage of deploying code to production.
- **Validation & compliance**: The build validation stage is determined by the needs of the organization. Image quality can be ensured by using image scanning tools like Clair to compare against known vulnerabilities.

![](./image/cicd.png)

There are also pipeline stages not mentioned here. This list is merely an example of commonly seen stages. You can configure a unique pipeline according to your organization's needs.

<br>

### Containers and CI/CD Pipelines
Traditional CI/CD systems were designed for pipelines using virtual machines, but cloud-native application development offers numerous benefits to CI/CD pipelines.

The open-source Tekton project allows you to build Kubernetes-style delivery pipelines, which can control the entire lifecycle of microservices. This eliminates the need to maintain and manage CI (continuous integration) servers, plugins, and configurations centrally.

### OpenShift Pipelines
OpenShift Pipelines is one of the Red Hat OpenShift features built on Tekton.
Tekton is an open-source project that provides a framework for rapidly creating cloud-native CI/CD pipelines. Tekton is a CI/CD framework for Kubernetes platforms, offering a standard cloud-native CI/CD experience and containers. As a Kubernetes-native framework, Tekton makes it easier to deploy across multiple cloud providers or hybrid environments. Tekton leverages CRDs (Custom Resource Definitions) to execute pipeline tasks from the Kubernetes control plane. Furthermore, Tekton adheres to standard industry specifications, allowing it to integrate effectively with existing CI/CD tools like Jenkins, Jenkins X, Skaffold, and Knative.
