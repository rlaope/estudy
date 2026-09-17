# Systems Manager Session Manager

## Motivation for Session Manager

![](https://user-images.githubusercontent.com/28394879/141436264-dceb30d4-7304-420c-859d-45064cef4fee.png)

As the number of EC2 instances increases, managing different `pem` files becomes difficult.

Even when managed with a Bastion Host, there's the inconvenience of having to go through the Bastion Host every time you connect.

Session Manager was introduced to solve this problem.

## Session Manager
**Systems Manager Session Manager**
- A fully managed AWS service that allows you to manage EC2 and on-premises instances, as well as virtual machines, via a browser-based shell or the AWS CLI.

### Systems Manager

![](https://user-images.githubusercontent.com/28394879/141436794-93d1c12c-4a05-4a48-a739-d023204e2ecd.png)

AWS Systems Manager allows you to manage things that were originally managed in a very complex way, as shown in the top image, in the simpler way shown in the bottom image.

![](https://user-images.githubusercontent.com/28394879/141436884-929f4ceb-2697-4958-ba84-9d273fe14fea.png)

### Advantages of Session Manager
1. A managed service that provides one-click access to instances.
2. Allows logging into instances without an SSH connection, without needing to open ports, and without maintaining a bastion host.
3. Controllable at the IAM user level (no need to control with key files).
   - E.g., when you need to individually manage key files for logging into hundreds of instances.
   - When you want to allow developers to log in only to instances designated for their team.
4. Browser-based, usable regardless of OS.
5. Logging and Auditing.
   - You can check when, where, and who connected (CloudTrail).
   - Connection logs and all used commands and output can be sent to S3 or CloudWatch.
   - Integrates with AWS services, enabling various scenarios.
     - E.g., receiving real-time notifications for access by integrating with EventBridge, etc.
