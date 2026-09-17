# Creating package.json
- Managing numerous downloaded packages can become difficult.
- Therefore, a package.json file is created for each project to manage package lists and versions.
> It is recommended to create package.json immediately after creating a project and start from there.

- Command: `npm init`

<br>

1. Enter information such as the package name. Unnecessary items can be skipped by pressing Enter.
2. At the end, type 'yes' for the "Is this OK?" prompt.

- If the file is created successfully, the following three files and folders will be generated.
> However, if no external packages have been installed yet (initial state), the node_modules folder and package-lock.json file may not be created.

<br>

## 1. package.json file
- Installed packages are managed under the `dependencies` section.

```json
{
  "name" : "node.package",
  "version" : "1.0.0",
  "description" : "my node package",
  "main" : "index.js",

  "scripts" : {
    "test" : "echo / Error: no test specifed\ && exit 1 "
  },
  "dependencise" : {
    "socket.io" : "^4.1.2"
  }
}
```

<br>

## node_modules folder
- This is the folder where installed packages are actually stored.
- Other packages that the installed package depends on are also stored here when installed.

<br>

## package-lock.json file
- It manages all installed packages + their dependencies.
