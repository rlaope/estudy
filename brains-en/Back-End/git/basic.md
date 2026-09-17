# What is GIT?
> `git` is a distributed version control system used to track changes in computer files and coordinate work among multiple files.

> `github` is a web hosting service that supports projects using Git.

---

### Uploading a Git Project
1. Create a repository + remember the address
<br/><br/>

2. Perform initial setup.
```bash
git config --global user.name "NAME"
git config --global user.email "EMAIL"
```

3. Prepare files

```bash
git init  # Create git files
git add . # Manage all files within the selected project folder
git status # Check status
git commit -m '내용' # Commit
```

4. Upload
```bash
git remote add origin 리퍼지토리주소
git push origin master
```

5. Done

___

<br>

### Updating a Git Repository

- Used when initial setup is complete and you want to update

```bash
git add .
git commit -m '커밋메시지'
git push origin master
```

> You can check the current status by typing `git status`.

<br>

## Collection of Git Commands (a few only)

Commands mentioned above are omitted.
<br/>
### clone
```bash
git clone URL // Clone repository, download
git clone /로컬/저장소/경로  // Clone local repository
```

### commit
```bash
$ git add <파일명>
$ git add *   // Include changes of a single file in the commit
```

### branch
```bash
$ git branch // List branches
$ git branch // <브랜치이름>	Create a new branch (locally)
$ git checkout -b // <브랜치이름>	Create & switch to a branch
$ git checkout master	// Switch back to master branch
```

### push
```bash
$ git push origin master // Upload changes to remote server
$ git push < remote > // <브랜치이름>	Upload commits to remote server
$ git push -u < remote > // <브랜치이름>	Upload commits to remote server
```
