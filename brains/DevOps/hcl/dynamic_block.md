# Terraform dynamic block

테라폼 다이나믹 블럭은 테라폼의 block 인자(argument)들을 동적으로 생성해주는 기능이다.

`count/for_each` 와의 차이점으로는 count, for_each는 block 자체를 반복하여 생성하는 기능이라면  다이나믹 블럭은 argument를 동적으로 생성하는 기능이다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fc0SEBw%2FbtsnFEHgTIu%2FoXelz3EKTG3fBvH3m9dxg0%2Fimg.png)

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FuDjWs%2FbtsnFMkWVM8%2FKjSxdxd6teuSUNpTUhvU40%2Fimg.png)

<br>

### 사용방법

사용방법은 block의 인자를 dynamic 블럭으로 바꿔주면 된다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fcg3fUb%2FbtsnGZqzfbl%2F6CVurPcLtnwEKcdUWhsmAk%2Fimg.png)

1. variables로 인자로 들어갈 값들을 컬렉션 형태로 정의한다.
2. argument에서 dynamic을 붙여 인자를 생성해준다
3. for_each에서 variable로 정의한 값을 참조한다.
4. content에 내용을 반복 설정한다.

content에 접근할때는 dynamic label을 사용하게 된다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fb1QJ72%2FbtsnGZqzgTS%2FoUozx57GmLJxpCYI2HfK00%2Fimg.png)


<br>

### Example

ingress 값들을 테라폼으로 설정합니다.

```tf
variable "security_group_ingress" {
  type = map(object({
    description = string
    protocol    = string
    from_port   = string
    to_port     = string
    cidr_blocks = list(string)
  }))
}
```

변수 초기화 값은 terraform.tfvars를 사용했습니다.

```tfvars
security_group_ingress = {
  http = {
    description = "http"
    protocol    = "tcp"
    from_port   = "80"
    to_port     = "80"
    cidr_blocks = ["0.0.0.0/0"]
  },
  https = {
    description = "https"
    protocol    = "tcp"
    from_port   = "443"
    to_port     = "443"
    cidr_blocks = ["0.0.0.0/0"]
  },
  ssh = {
    description = "ssh"
    protocol    = "tcp"
    from_port   = "22"
    to_port     = "22"
    cidr_blocks = ["0.0.0.0/0"]
  }
}
```

ingress의 인자를 dynamic block으로 변경해준다.

block lable(접근을 위한)은 ingress로 설정해준다.

 for_each에는 정의한 변수를 사용하고, content에는 ingress label을 참조하여 변수 값에 접근한다..

```tf
resource "aws_security_group" "main" {
  name        = "terraform-dynamicblock-test"
  description = "terraform-dynamicblock-test"
  vpc_id      = aws_vpc.main.id

  dynamic "ingress" {
    for_each = var.security_group_ingress
    content {
      description = ingress.value["description"]
      protocol    = ingress.value["protocol"]
      from_port   = ingress.value["from_port"]
      to_port     = ingress.value["to_port"]
      cidr_blocks = ingress.value["cidr_blocks"]
    }
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    Name = "terraform-dynamicblock-test"
  }
}
```

이제 `terraform apply`를 통해 반영하면 된다.
