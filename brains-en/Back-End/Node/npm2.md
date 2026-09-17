# Other Commands

### npm -version
- Checks the version of npm itself
> Can be abbreviated as `npm -v`

<br>

### npm install -g npm
- Upgrades the version of npm itself
> For Mac/Linux, you might need to use `sudo`.

<br>

### npm outdated
- Checks if there are any packages that can be updated

<br>

### npm update <package_name>
- Updates the package to the latest version

<br>

### npm info <package_name>
- Gets detailed information about the package

<br>

### npm search <keyword>
- Searches for packages on the npm server

<br>

### npm help
- Displays a list of commands

<br>

### npm ls
- Shows the structure of installed packages.
> `npm ls <package_name>`: Only shows the structure related to the specified package.
> `npm ll`: Shows more detailed package structure information.

<br>

### npx <package_name> [options]
- A command that allows you to use an uninstalled package from the console as if it were global.
- In other words, it finds and executes the binary file of that package in the `node_modules/bin` folder.
- If the package is not found, it temporarily downloads, installs, and executes it.
> Details about npx can be lengthy, so they are omitted here.
> Example usage: `npx pm2 list`


<br><br>

## How Node.js References Packages
- When Node.js uses external modules, it searches for packages in the following order:

  1. Searches in the `currently executing folder` (highest priority)
  2. Moves to the `parent folder` and searches.
  3. Moves to the `folder where global packages are installed` and searches.
