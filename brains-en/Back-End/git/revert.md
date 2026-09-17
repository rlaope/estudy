# Reverting to a previous version,

- Viewing a previous version and returning

If you want to check the source of a previous commit, there are two ways.

1. Reverting to a specific commit by viewing commit messages
  - `git log`
    1. After entering the command, use the up and down arrow keys to find the desired version commit.
    2. Copy at least the first 4 digits of the hash code after the 'commit' phrase.
  - `git checkout copied 4+ digit hash code`
  - `git checkout branch-name`

2. Stepping back version by version
  - `git checkout head ~ 1`
    - Reverts to one step before the latest commit.
    - You can specify the desired number of steps by changing the '1'.

<br>

## Reverting to a previous version

> Even if you want to revert permanently without the option to return, there are two methods, but `revert` is generally recommended.

1. Creating a new commit for the reverted version
```bash
git revert head ~ 1 혹은
git revert head 커밋해시코드
```
  - Both `checkout` methods explained earlier can be applied to `revert` in the same way.
  - When using the `revert` command, a new commit is created for that specific commit version, so a commit message input window will appear.

2. Deleting commits after the reverted version (Not Recommended)

```bash
git reset --hard head ~ 1 혹은
git reset --hard 커밋해시코드
```

- Deleting remaining added files

### head

- `head ~` previous step
- `head ~ 2` two commits ago
- `head ~ 3` three commits ago
> Can be used in this way.

### + More detailed explanation of the difference between Revert and Reset

- `git revert` allows you to revert the state while preserving history.
- `git reset` deletes history.

> Use `reset` if you don't need the past; use `revert` if you might need to keep it.

> `revert` is more recommended.
