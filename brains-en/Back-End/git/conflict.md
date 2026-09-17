# conflict - Resolving Conflicts

- There are inevitably situations where the same file must be modified, leading to conflicts. Let's create a conflict scenario and learn how to resolve it.

Let's create a conflict by modifying the `red` file, which was changed in the `update-red` branch, also in the `main` branch.

<br>

### Steps
1. Modify the `red` file in the `main` branch
2. Add all changes to the index
3. Create a commit
4. Merge the `update-red` branch into `main`

```bash
echo "빨간색" > red
git add -A # gaa
git commit -m "update red color"
git merge update-red
```

`result` : Failure with a conflict message

> ### CONFLICT
> Conflicts are situations you really want to avoid. However, if they occur, you must resolve them.
> You can resolve the conflict and commit, or you can cancel the merge operation.


```
<<<<<<< HEAD
빨간색
======
붉은색
>>>>>>> update-red
```
- When a conflict occurs, it shows the changes made simultaneously in both branches.
- `<<<<<<<` , `=======` , `>>>>>>>` indicate the point where the conflict occurred.

<br>

The content above is from the `main` branch, and the content below is from the `update-red` branch. The developer must decide and choose which content is correct.

- Here, we'll keep only the "붉은색" (reddish color) line and delete the others.
- Once the conflict is resolved, proceed with the commit in the same way as before. The difference here is that you only `commit` without entering a `-m` message.
- Since the message for the resolved conflict is automatically generated, you don't need to enter it.

```bash
git add -A
git commit
```

> If a vi window opens prompting you to enter a message, don't panic. Press the esc key, then type :x in sequence, and press enter.
