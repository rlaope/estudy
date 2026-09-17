# Package Installation / Uninstallation Commands

- These are commands for installing/uninstalling external module packages.

## 1. Package Installation
- Command: `npm install` <package_name>
- Installs the package into the current project's node_modules folder.
- It is also automatically added to the `dependencies` section of the `package.json` file.
  > In the past, to add a package to `dependencies`, you would install it with `npm install --save <package_name>`. However, since npm@5, this is the default behavior, so `--save` is no longer necessary.
- Can be abbreviated as `npm i` <package_name>.
- You can specify the version with `npm install <package_name>@<version>`.
- You can specify the package installation address with `npm install <address>`.

<br>

## 2. Installing Multiple Packages
- Command: `npm install <package1> <package2> <package3>...`
- Installs multiple packages simultaneously.

<br>

## 3. Global Installation
- Command: `npm install -global 패키지명`
- Installs the package in the folder where npm itself is installed, making it accessible from anywhere.
> On Mac and Linux, you might need to install it with `sudo`, like `sudo npm i --global <package_name>`.

> Can be abbreviated as `npm i -g 패키지명`.

<br>

## 4. Development Installation
- Command: `npm install --save-dev 패키지명`
- Installs it so that it's added to the `devDependencies` section of the `package.json` file.
> Can be abbreviated as `npm i -D`.

<br>

## 5. Installation Using `package.json` File
- Command: `npm install`
- When you run the command in the folder where the `package.json` file is saved, it automatically installs all packages listed in the `dependencies` section.

<br>

## 6. Package Uninstallation
- Command: `npm uninstall 패키지명`
- Deletes the package installed in the `node_modules` folder. It is also removed from the `dependencies` section of `package.json`.
> Can be abbreviated as `npm rm 패키지명`.
