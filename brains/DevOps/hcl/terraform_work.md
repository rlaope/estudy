# Terraform 작동 원리


![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbtJ8zK%2FbtsldOrf5Oe%2FvBsFdLt02vrG4MOSb1Vrnk%2Fimg.png)

테라폼은 코드를 읽고 코드가 실행 가능한지 분석한다. 

그 이후 대상에서 지원하는 API를 사용하여 코드를 대상에 반영한다.

1. 코어는 테라폼 코드를 읽어 코드 문법 검사와 실행 순서를 결정한다. 실행 순서 형태를 리소스 종속성 그래프 (Resource Dependency Graph)라고 한다.
2. 코어는 테라폼 코드 실행을 플러그인에게 요청한다.
3. 플러그인은 provider 설정을 읽어서 적절한 API를 호출한다. API 호출 로직은 client binary에 구현한다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FxPFdz%2FbtslbP5aBJ7%2FabdwMRZ6vQ0tOiOy7ndmn0%2Fimg.png)

우리가 `terraform init`을 한다면 다음과 같은 내용을 볼 수 있다.

```bash
Initializing the backend... 

Initializing provider plugins...
- Finding hashicorp/vault versions matching "3.17.0"... 
- Installing hashicorp/vault v3.17.0...
- Installed hashicorp/vault v3.17.0 (signed by HashiCorp) 
 
Terraform has created a lock file .terraform.lock.hcl to record the provider selections it made above. Include this file in your version control repository so that Terraform can guarantee to make the same selections by default when you run "terraform init" in the future. 

Terraform has been successfully initialized!

You may now begin working with Terraform. Try running "terraform plan" to see any changes that are required for your infrastructure. All Terraform commands should now work. 

If you ever set or change modules or backend configuration for Terraform, rerun this command to reinitialize your working directory. If you forget, other commands will detect it and remind you to do so if necessary.
```

여기서 볼 수 있는 것은 `terraform init` 을 하는 과정에서 플러그인을 초기화 하고 API 호출을 위한 바이너리 클라이언트 라이브러리들을 가지고 온다.

