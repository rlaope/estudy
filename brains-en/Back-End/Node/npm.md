# Npm(Node.js) - Basic Command Usage Summary

## npm(node.js) - Basic Command Description
1. npm - Automatically installed when Node.js is installed.
2. In real projects, knowing how to install, remove, and manage versions of npm-based modules is crucial.
3. Early in a project, strict version management for each module is essential to avoid issues later on.

<br>

### npm - Help Option(-h)
```bash
# Check npm commands
npm -h

# Check details for a specific npm command
npm command -h
```

<br>

### npm - list
```bash
# Check modules installed in the current project
npm list

# Check globally installed modules
npm list -g

# depth option
npm list -g --depth=0
```

<br>

### npm - view

```bash
# Check the latest version of a module
npm view module_name version
```
