# pull request

## pull request
- A `pull request` follows the procedure below.
  1. Fork
  2. Clone, set up remote
  3. Set up branch
  4. After modification, add, commit, push
  5. Create Pull request
  6. Code review, Merge Pull Request
  7. Delete branch and synchronize after Merge

<br>

## Fork
> Fork the target project's repository to your own repository.
> Once forking is complete, a new repository will be created in your account.

<br>

## clone , remote setup
> In the repository created by forking in your account, click the `clone or download` button and copy the displayed URL. (This is the repository URL, not the browser URL.)

- Open the terminal.
- To work on your computer, clone the forked repository locally.

```bash
git clone <url>
```
- Add a remote repository to your local repository. Use the same URL obtained from the clone or download menu in the GitHub repository as in the previous step.
  - Original project repository
  - Forked local project

```bash
# Add the original project repository as a remote repository
git remote add real-blog(alias) <url>

# How to check remote repository settings
git remote -v
```

<br>

## branch creation
- When adding code on your local computer, proceed by creating a branch.

> During development, it's common to need to duplicate code multiple times. After copying the entire code, you can proceed with development independently, regardless of the original code. This independent development is what a branch is.

```bash
# Create a branch named develop
git checkout -b develop
Switched to a new branch 'develop'

git branch
* develop
  master
```

<br>

## After modification, add, commit , push
- Perform modification work using your preferred code editor.
- Once the work is complete, reflect the changes by add, commit, and push.
- `Caution`: When pushing, you must specify the branch name.

```bash
git push origin develop
```

<br>

## pull request creation
- After the push is complete, when you access your GitHub repository, the `Compare & pull request` button will be activated.
- Select this button to write a message and create a PR.

<br>

## Code review, Merge Pull Request
- The administrator of the original repository who received the PR reviews the code changes and decides whether to merge.

<br>

## Synchronization and branch deletion after Merge
- Once the merge to the original repository is complete, synchronize the local code with the original repository's code.
- Delete the local branch you were working on.

```bash
git pull real-blog(remote alias)
git branch -d develop (branch alias)
```
- If you need to do additional work later, synchronize with the original repository using the `git pull real blog(remote alias)` command and repeat steps 3 to 7.
