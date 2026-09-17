# git Basic Commands Summary 2

- `pwd`: current folder
- `cd`: move directory
- `cd..`: move to parent folder

`Autocompletion: available with tab key`

```bash
ls -al
git init
git remote add <원격저장소이름>
```

- `ls -al` View hidden folders, hidden files

- `git init` Create local repository
- `git remote add <address>` Informs the master folder on my computer of the GitHub repository address.

* master: branch name
* origin: remote repository name

<br>

### git log - Check history

```bash
git log [option] [revision range] [[--] <path>..]
```

- `git log` is a feature that allows you to output logs in the desired format by combining various options.

<br>

### git reset - To previous state (remove history)

```bash
git reset [<commit>] [--soft | --mixed [-N] | --hard | --merge | --keep]
```

- Resets the history up to a specific commit. You can undo work done just before, or up to n commits ago.

- ` Use with caution as history will be erased `

- `git reset` has various options, but here we will use the `--hard` option.

#### Task
1. Check commit ID for commit #2 with `git log`
2. Reset history up to commit #2

```bash
git log
git reset {v2 커밋 아이디} -- hard # 커밋 아이디 예) 27a00b7
```

- result:
```
HEAD is now at 27a00b7 v2 commit
```

- Reset history up to commit #2 -> Confirms deletion of commit #3 history.

<br>

Current Git repository history

![Repository History](image/basic2_history.png)
