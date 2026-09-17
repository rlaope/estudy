# Managing Someone Else's Repo

1. First, receive an invitation as a collaborator.
2. Accept the invitation in the email.

<br>

## Importing Someone Else's Repo into My Repo

1. On my GitHub, create a new repository with the same name as the repository to be imported. At this time, do not initialize README, gitignore, or license.

2. Clone the repository to be imported to your computer as follows.

```bash
git clone <url>
```

3. Connect the cloned local repository to the new repository created in step 1.
```bash
git remote add origin <url>
```

4. Then, proceed as follows.

```bash
 git branch -M main
 git push -u origin master
 ```
