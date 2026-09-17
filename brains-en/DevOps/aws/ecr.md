# AWS ECR (Elastic Container Registry)

AWS ECR is a secure, scalable, and reliable AWS-managed container image registry service.

It can be seen as equivalent to Docker Hub, but its advantages include managing Docker images with S3, which ensures high availability, and enabling permission management for image push/pull through AWS IAM authentication.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FcAOvUx%2FbtrX6ohUvlB%2FJo0e3EnLU7cVRoe4Gpo5O0%2Fimg.png)

### Components
- Registry: An Amazon ECR private registry is provided for each AWS account, allowing you to create one or more repositories within the registry and store images in these repositories.
- Repository: Amazon ECR repositories contain Docker images, Open Container Initiative (OCI) images, and OCI-compatible artifacts.
- Repository Policy: Manages access control for repositories or Docker images within a repository.
- Image: You can push and pull container images to and from repositories. These images can be used locally on development systems, or in ECS task definitions and Amazon EKS pod specifications.
- User Permission Token: Clients must authenticate as an AWS user to the Amazon ECR Registry to push/pull images.

## Features Provided by ECR

### Lifecycle Policies

Lifecycle policies help manage the lifecycle of images in your repositories. You define rules to clean up unused images. You can test these rules before applying them to a repository.

### Image Scanning
Image scanning helps identify software vulnerabilities in container images. Each repository can be configured to scan on push. This ensures that every new image pushed to the repository is scanned. You can then retrieve the image scan results.

### Cross-Region and Cross-Account Replication
Cross-region and cross-account replication make it easy to place images where they are needed. This is configured in the registry settings and is set up on a per-region basis.

### Pull-Through Cache Rules
Pull-through cache rules provide a way to cache repositories from remote public registries in your private ECR registry. ECR uses pull-through cache rules to periodically contact the remote registry to ensure that cached images in your Amazon ECR private registry are up-to-date.
