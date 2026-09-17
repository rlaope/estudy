# Terraform Custom Condition(precondition, postcondition)

Terraform provides custom conditions to ensure that Terraform behaves as intended.

For Terraform to operate as intended by the code author, **it proceeds if the condition is met and raises an error if it is not.**

> This shows that Terraform's custom conditions differ from ternary operators (regarding error generation upon unmet conditions)

### precondition, postcondition

Both are among Terraform's custom conditions.

Precondition/postcondition inspect block fields during the block lifecycle stage.

The condition determines the conditional statement, and if an error occurs, the specified error message is displayed.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FJnMuU%2FbtsmWnFhb9b%2Fg8fnKALsMfwkpBDIoxfDp0%2Fimg.png)

The difference between pre and post conditions is that they execute before/after Terraform code execution.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FFkQc0%2FbtsmP7K3riC%2F1ma0i1kh97aa8kyD1nqWP0%2Fimg.png)

If neither condition is met, Terraform raises an error.

1. If there are loops like `forEach` or `count`, `precondition` checks each custom condition individually.
2. `postcondition` can access the `self` object because it performs checks during code execution. (`precondition` cannot).

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbqaHB3%2FbtsmQeCT0MJ%2F5Hz4wVB4tQDevi5LP31vDk%2Fimg.png)

I consider `precondition` and `postcondition` to be about expectation and assurance.

This is because `postcondition` prevents other parts from referencing a resource if it is incorrectly created.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbETTZc%2FbtsmZgTqgFe%2F9fFvAknvL6NzGz0KnV9Wf0%2Fimg.png)

Can `postcondition` be checked via `terraform apply`? The answer is yes.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbWAYCP%2FbtsmQC4yklZ%2FrPPpyaCTPpKXdIyysC3bl1%2Fimg.png)

We set a condition to check if `base64sha256` is `1111`, as shown below.

If we run `apply`, an error occurs as expected (because it's not `1111`).

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbpIMIc%2FbtsmO5TWt0M%2FclBWwcqcL8Eqv0hK93xbEk%2Fimg.png)

Although the custom condition failed, the `txt` file was created. The significance lies in preventing external references to that resource.

[[Terraform 작동원리]]
