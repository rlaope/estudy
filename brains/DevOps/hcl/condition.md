# Terraform Custom Condition(precondition, postcondition)


테라폼은 커스텀 조건문을 제공하여 테라폼이 의도된 동작을 하도록 보장해준다.

테라폼은 코드 작성자가 의도한대로 동작하기 위하여 **조건에 만족한다면 동작하고 만족하지 않는다면 에러를 발생시킨다.**

> 이로 인해 테라폼의 커스텀 조건문은 삼항연산자와 다르다는 것을 알 수 있다 (조건 불만족시 에러 발생 여부)


### precondition, postcondition

둘 다 테라폼의 커스텀 조건문중 하나다.

precondition/postcondition은 block lifecycle(생명주기)단계에서 block필드를 검사한다.

condition을 통해 조건문을 결정하며 에러가 발생시 지정한 에러 메세지가 출력된다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FJnMuU%2FbtsmWnFhb9b%2Fg8fnKALsMfwkpBDIoxfDp0%2Fimg.png)


pre, post condition의 차이는 테라폼 코드 실행 전/후에 실행된다는 차이가 존재한다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FFkQc0%2FbtsmP7K3riC%2F1ma0i1kh97aa8kyD1nqWP0%2Fimg.png)

두 조건이 모두 만족하지 않으면 테라폼은 에러를 발생시킨다.

1. precondition은 forEach, count와 같은 반복문이 있을 경우, 각각 커스텀 조건을 검사한다.
2. postcondition은 코드 실행중에 검사를 하므로 self object에 접근이 가능하다. (precondition은 불가능하다.)

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbqaHB3%2FbtsmQeCT0MJ%2F5Hz4wVB4tQDevi5LP31vDk%2Fimg.png)

precondition과 postcondition은 기대와 보장이라고 생각한다.

postcondition은 리소스가 잘못 생성되면 다른 곳에서 해당 리소스를 참조하지 못하게 하기 때문이다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbETTZc%2FbtsmZgTqgFe%2F9fFvAknvL6NzGz0KnV9Wf0%2Fimg.png)

terraform apply를 통해서 postcondition을 검사할 수 있을까? 정답은 할 수 있다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbWAYCP%2FbtsmQC4yklZ%2FrPPpyaCTPpKXdIyysC3bl1%2Fimg.png)

다음과 같이 base64sha256이 1111인지 검사하는 컨디션을 준다.

apply를 해보면 예상처럼 에러가 발생하게 된다. (1111아니니까)

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbpIMIc%2FbtsmO5TWt0M%2FclBWwcqcL8Eqv0hK93xbEk%2Fimg.png)

커스텀 조건은 실패했지만 txt파일을 생성이 되었다. 그렇게 해당 리소스를 외부에서 참조할 수 없게 만들어 주는 것에 의의를 둔다.




[[Terraform 작동원리]]