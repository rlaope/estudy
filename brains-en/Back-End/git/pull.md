# Remote Repository

## pull, bringing remote repository data to the local repository and merging it

- Running `pull` allows you to fetch changed data from the remote repository.
- `git pull` is a command that fetches commits from the Remote Repository (git fetch) and then merges them into the current Working Directory.

> git pull = git fetch + git merge

### pull
- Downloads necessary files from the remote repository + merges them
- The local branch and the remote repository's origin/master point to the same location.

### fetch

- Downloads necessary files from the remote repository (merging must be done separately)
- The local branch points to the latest commit in the local repository it originally had, while the remote repository's origin/master points to the latest fetched commit.
- Use when caution is needed.

- Reasons for use
> You can see the difference between the original content and the changed content.
> You can see how many commits there are.
> After checking these details, `git merge origin/master` will result in the same state as `git pull`.

## Difference between pull and fetch

git pull
- Fetches and merges the latest content from a remote repository connected via the `git remote` command into the local repository.

git fetch
- When changes differ between the local and remote repositories, it compares them and, along with the `git merge` command, applies the latest data or resolves conflicts.

<br>

## Resolving Git Push Errors

```
! [rejected] master -> master (fetch first)
error: failed to push some refs to 'https://github.com/dalso~~'
hint: Updates were rejected because the remote contains work that you do
hint: not have locally. This is usually caused by another repository pushing
hint: to the same ref. You may want to first integrate the remote changes
hint: (e.g., 'git pull …') before pushing again.
hint: See the 'Note about fast-forwards' in 'git push --help' for details.
```

- The cause of the above error is that the Git remote repository and the current local repository are not synchronized. The solution is simple: just synchronize them.

```bash
git pull --rebase 원격저장소별칭 master
```

If you successfully synchronize using the command above, you should have no issues pushing again.
