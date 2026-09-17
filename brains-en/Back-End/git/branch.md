# Branch Branch


### What is a Branch?
- A branch is a concept for independently carrying out specific work.
- Each branch, created as needed, is not affected by other branches, allowing multiple tasks to be performed simultaneously.

### Master Branch

- When you first create a repository, Git immediately creates a branch named `master`.
- Unless you create another new branch other than 'master' and declare 'I'm going to use this branch from now on!' (checkout), all work at this time is done on the `master` branch.

## Creating and Switching Branches
1. Let's create a branch named huemang. You can write it like this: ` git branch <username> `

```bash
git branch huemang
```

If you type `git branch`, you can see that a branch named huemang has been created.

2. Now, let's move from the master branch to the huemang branch. The command used for this is `checkout`.
Type ` git checkout <username> `.

```bash
git checkout huemang
```
After typing this, the branch has changed to huemang.

<br>
  
## Merging Branches

- Let's merge the huemang branch and the master branch. The command used for this is `merge`.
`git merge <commit> `

- To incorporate 'huemang' into the 'master' branch, you first need to make sure 'HEAD' is on the 'master' branch. At this point, use the checkout command to switch the currently active branch to 'master'.

```bash
git checkout master
```

- Then, you can merge using `merge`.
```bash
git merge huemang
```
Merge the huemang branch into the master branch

<br>

## Deleting a Branch

- Since all the content of the huemang branch has been integrated into 'master', the huemang branch is no longer needed.

- To delete a branch, run the `branch` command with the `-d` option.

```bash
$ git branch -d huemang
```

This way, the huemang branch has been deleted.
