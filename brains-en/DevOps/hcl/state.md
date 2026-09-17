# [Terraform] state, remote_state, backend

Terraform stores the results of its deployments as state.

State significantly impacts Terraform's execution operations (creation, modification, deletion).

For example, if state does not exist, Terraform proceeds with resource creation.

If state exists, it compares and updates the state, which is called `terraform refresh`.

### State Management

State is stored in JSON format. Terraform saves its state in a file named `tfstate`.

If you run `terraform apply`, you can see that a `terraform.tfstate` file is created.

For example, when a VPC is created, you can see a file like this:

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbdkXji%2FbtspeGkdesJ%2FfEbuoNuJxEsUko2gHSQpC0%2Fimg.png)

You can see that state is created in the tfstate file for each resource managed by Terraform.

Each resource is distinguished by its name (block label).

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fuuq5w%2FbtsppJy5lfQ%2FOlsqpj3rKRm2kRkXaJYCz0%2Fimg.png)

These files can be checked using the `terraform state list/show` commands.

You can view the list using `list`.

```bash
terraform state list
```

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbqJs66%2Fbtspgnx2VRk%2F7es8A5bVQXPAo3kOCkxzFk%2Fimg.png)

The value of the state can be checked using `show`. It displays the same value as the JSON field.

```bash
terraform state show aws_vpc.main
```

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FHQbCN%2FbtspmgKWRnt%2Fdi3cK2UAkMaTme9sKV3sQk%2Fimg.png)

<br>

### Version Control

State manages its version through a field called `serial`.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fk4ves%2Fbtspg1uzQN5%2FUKjilQRn2jgEv4GPksPcs0%2Fimg.png)

<br>

### Impact

The impact of state influences whether resources are created, modified, or deleted.

The activity of comparing or updating state before Terraform execution is called `terraform refresh`.

Here's an example of how Terraform's behavior changes based on its state.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FswdIv%2FbtsplRdM9Uw%2Fd9WehKBWJkHHFbTk45DXt0%2Fimg.png)

For an S3 bucket, which must be unique, if the state does not exist but the resource does, creation is impossible, leading to an error. However, for a VPC, duplicate resources can exist, so it will be created. (Of course, the state will only contain information for the single VPC resource that was created.)

<br>

### terraform import

You can also link state to existing resources using `terraform import`.

To use this, the target resource and Terraform code must already exist.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FTo6NV%2FbtspkKsMpc4%2FrQEnYFI3aMvriGtkSRHvj0%2Fimg.png)

It's typically used when you want to manage resources not created by Terraform with Terraform.

The usage is as follows:

```bash
terraform import block_type.block_name {대상 리소스 식별값}
```

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FwsLyR%2Fbtspg0JbC3r%2FOhpbQeKRuYOVcVmp3xQHH1%2Fimg.png)

<br>

### backend

The location where Terraform's state is stored is called the backend.

By default, Terraform stores state files locally. Storing them remotely, rather than locally, is called remote state.

Remote state is essential when collaborating with multiple people, and the results vary depending on the state.

To configure Terraform's remote state, you use a bucket like S3 for versioning.

```tf
resource "aws_s3_bucket" "main" {
  bucket = var.bucket_name

  tags = {
    Name = "terraform test"
  }
}

resource "aws_s3_bucket_versioning" "main" {
  bucket = aws_s3_bucket.main.id

  versioning_configuration {
    status = "Enabled"
  }
}
```

As shown above, a versioning-enabled S3 bucket is used.

If you then apply the changes,

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fb9qvs6%2FbtspolLMWwp%2FwekLstALKngAWiCuuPT9gk%2Fimg.png)

The state will then be stored in the S3 bucket like this.

> Additionally, to modify Terraform's state using a locking mechanism, you can implement a lock mechanism to acquire a lock and ensure sequential processing. S3 does not support locking directly; this is usually resolved using DynamoDB.

```tf
resource "aws_dynamodb_table" "terraform_state_lock" {
  name           = "terraform-lock" # table이름
  hash_key       = "LockID" # key 이름
  billing_mode   = "PAY_PER_REQUEST"

  attribute {
    name = "LockID"
    type = "S" # key 타입
  }
}
```

Configure it as above and apply Terraform.

Since the backend configuration has changed, you need to run `terraform init`.

```bash
terraform init -migrate-state
```

Now, before updating the remote state in S3, you must pass through the locking mechanism.

If you modify VPC or related resources, apply the changes, and simultaneously observe DynamoDB, you will see the locking field appear and then disappear.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FMDG5U%2FbtspjWsWCC5%2F2iOjgKj0sGwhIUzKzLFy50%2Fimg.png)  
![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fb5pFcI%2FbtspsUmN7hU%2FRhZCjzqgy5PkPShSktU3Yk%2Fimg.png)
