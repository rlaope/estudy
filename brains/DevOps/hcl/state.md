# [Terraform] state, remote_state, backend

테라폼은 대상이 배포한 결과를 state로 저장한다.

state는 테라폼 실행 작업(생성,수정,삭제)에 큰 영향을 미친다.

예를 들어 state가 존재하지 않으면 테라폼은 리소스 생성 작업을 진행한다.

state가 존재하면 state를 비교하여 업데이트한다 이를 terraform refresh라고 한다.

### State 관리

state는 json 포멧으로 저장되어진다. 테라폼은 tfstate라는 파일로 테라폼의 상태를 저장한다.

`terraform apply`를 해보면 `terraform.tfstate`라는 파일이 생기는 것을 확인해볼 수 있다.

예를 들어 vpc를 생성했을 때 다음과 같은 파일을 볼 수 있다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbdkXji%2FbtspeGkdesJ%2FfEbuoNuJxEsUko2gHSQpC0%2Fimg.png)

tfstate 파일에 테라폼이 관리하는 리소스 만큼 state가 생기는 것을 확인할 수 있다.

name(block label)로 각 리소스를 구분한다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fuuq5w%2FbtsppJy5lfQ%2FOlsqpj3rKRm2kRkXaJYCz0%2Fimg.png)



이 파일들은 terraform state list/show 명령어로 확인할 수 있다.

list를 통해 목록을 확인해볼 수 있다.

```bash
terraform state list
```

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbqJs66%2Fbtspgnx2VRk%2F7es8A5bVQXPAo3kOCkxzFk%2Fimg.png)

state의 값은 show 를 통해서 확인할 수 있다. json의 필드값과 동일한 값이 나온다.

```bash
terraform state show aws_vpc.main
```

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FHQbCN%2FbtspmgKWRnt%2Fdi3cK2UAkMaTme9sKV3sQk%2Fimg.png)


<br>

### 버전관리

state는 serial이라는 필드로 상태의 버전을 관리한다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fk4ves%2Fbtspg1uzQN5%2FUKjilQRn2jgEv4GPksPcs0%2Fimg.png)


<br>

### 영향도

state의 영향도는 리소스를 생성할지 수정할지 삭제할지에 대한 작업을 결정하는데에 영향이 있다.

테라폼 실행전에 state를 비교하는 활동 혹은 업데이트 하는 작업을 terraform refresh 작업이라 한다.

그리고 여기서 테라폼 state에 따라서 동작이 달라지는 예시가 있다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FswdIv%2FbtsplRdM9Uw%2Fd9WehKBWJkHHFbTk45DXt0%2Fimg.png)

s3 버킷 같은 경우는 고유해야하는 리소스이므로 state의 존재하지 않고 리소스가 존재한다면 생성이 불가능하므로 오류가 발생하게 되지만, vpc같은 경우는 중복 리소스가 있을 수 있으므로 생성되게 된다. (물론 state값에는 생성한 하나의 vpc 리소스 정보만 담기게 된다.)

<br>

### terraform import

terraform import를 통해서 이미 존재하는 리소스에 대해 state를 연결할 수도 있다.

이를 사용하기 위해서는 대상 리소스와 테라폼 코드가 원래부터 존재해야한다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FTo6NV%2FbtspkKsMpc4%2FrQEnYFI3aMvriGtkSRHvj0%2Fimg.png)

보통 테라폼으로 생성하지 않은 리소스들을 테라폼으로 관리하고 싶을 때 사용한다.

사용 방법은 아래와 같다.

```bash
terraform import block_type.block_name {대상 리소스 식별값}
```

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FwsLyR%2Fbtspg0JbC3r%2FOhpbQeKRuYOVcVmp3xQHH1%2Fimg.png)


<br>

### backend

테라폼의 state를 저장하는 곳을 백엔드(backend)라고 한다.

디폴트로 테라폼은 로컬에 state 파일을 저장한다. 그리고 로컬이 아닌 원격에 저장하는 것은 remote state라고 한다.

remote state는 여러 사람과 협업할 때 꼭 설정해야 하는 내용이며 state에 따라 결과가 달라진다.

terraform의 remote state를 설정하기 위해서는 s3와 같은 버킷을 버저닝 용도로 사용하는 것이다.

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

위와 같이 버저닝 전용 s3 버킷을 사용한다.

이후 변경 사항을 apply하게 된다면


![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fb9qvs6%2FbtspolLMWwp%2FwekLstALKngAWiCuuPT9gk%2Fimg.png)

이렇게 state가 s3 버킷에 저장되게 된다.

> 추가로 locking 방식을 통해 테라폼의 상태를 수정하기 위해서는 락을 취득하고 순차적으로 처리가 되도록 락 메커니즘을 구현할 수 있다. s3를 locking을 지원하지 않는다.. 보통 DynamoDB를 사용해서 해결할 수 있다.


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

위와 같이 설정해주고 테라폼을 적용한다.

backend 설정이 달라졌으므로 terraform init을 해주어야한다. 

```bash
terraform init -migrate-state
```

이제 s3에 있는 remote_state를 업데이트하기 이전에 locking을 통과해야한다.

vpc나 관련 리소스들을 수정해보고 apply를 하고 동시에 DynamoDB를 관찰해보면 locking필드가 생겼다가 없어지는 것을 확인할 수 있다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FMDG5U%2FbtspjWsWCC5%2F2iOjgKj0sGwhIUzKzLFy50%2Fimg.png)
![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fb5pFcI%2FbtspsUmN7hU%2FRhZCjzqgy5PkPShSktU3Yk%2Fimg.png)
