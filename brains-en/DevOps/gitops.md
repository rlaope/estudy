# GitOps

The fundamental idea behind GitOps originated from automating operations for the entire system based on a model external to the system.

Git was chosen as the place to store this model.

In other words, GitOps involves synchronizing and automating production resources through an external model, with Git chosen as the repository for managing it.

### What is GitOps?

It is one of the methods for implementing CD (Continuous Deployment) based on Git.

It extends to a developer-centric operational infrastructure using familiar tools like Git, IaC, and CI.

![](https://oopy.lazyrockets.com/api/v2/notion/image?src=https%3A%2F%2Fs3-us-west-2.amazonaws.com%2Fsecure.notion-static.com%2Fe23e305e-9d15-4c2b-972d-84e6bac9bcbf%2FUntitled.png&blockId=fe59f9eb-a3a2-4119-a750-40e0623c0742)

> The difference is that Continuous Deployment automates deployment to production from Continuous Delivery.

Not only CI but also CD is applied to Git, extending the workflows used by developers to the operations team.

Git serves as the SSOT (Single Source of Truth) (using a single source for all data) and also acts as an interface for environments (e.g., staging, production).

<br>

### GitOps Principles

GitOps emphasizes the following principles.

1.  **Declarative**: Systems managed by GitOps must be expressed declaratively.
2.  **Versioned and Immutable**: The desired state is immutable, versioned, and stored with a complete version history enforced.
3.  **Pulled Automatically**: Software agents must automatically pull the declaration of the desired state from the source.
4.  **Continuously Reconciled**: Software agents must continuously observe the actual system state and attempt to apply the desired state.

<br>

### Workflow

![](https://oopy.lazyrockets.com/api/v2/notion/image?src=https%3A%2F%2Fs3-us-west-2.amazonaws.com%2Fsecure.notion-static.com%2F402a5c20-a430-4fe6-b2cc-fac93f15b2a3%2FUntitled.png&blockId=15c6307b-6a57-4d4f-b549-e99cfb6c702f)

The GitOps flow is as described above. The CI process remains the same as before. Code is committed to Git, and the image is pushed to the Container Registry.

Then, for changes to the image, an update request (Pull Request) is made to the manifest code repository (the connection process from top to bottom - Update image in staging config).

The bottom part is the CD process, where the submitted PR is reviewed and merged into Git. The Deploy Operator detects this and performs the DEPLOY.
