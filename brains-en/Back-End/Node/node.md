# Node.js

## What is Node?
`JavaScript Runtime`: A JS runtime that can be used without a browser.
`Usage`: Servers, desktop web, mobile web, games, but `it's slow`, so it's not used in operating systems.

<br>

### Development Process
Originally, JS could only be used within web browsers, running through a JS engine called `V8 engine` developed by Google. However, after Google open-sourced it, `Node.js` emerged, allowing JS to be used outside the browser.

> Limitation: However, because it's outside the browser, `document` / `window` objects cannot be used.

### Installation
I used the `LTS` version from the [Node.js installation page](https://nodejs.org/ko/).
The `LTS version` stands for Long Term Support, which is a stable version that provides maintenance and updates for 3 years.

<br><br>

## Compiled Languages vs. Runtime (Interpreted) Languages

### 1. Compiled Languages
- It's a language that translates and executes the `entire` code into something the CPU can understand. Once translated, the app can run without an executor. `It's fast`.
- C / C++

<br>

### 2. Runtime (Interpreted) Languages
- It can be executed in parts and `immediately` runs commands as they come. True to its name, 'runtime,' which refers to an `environment` containing everything needed for execution, a program can only run if an executor is present. It is relatively slower compared to compiled languages.
- Python / JS

<br>

### REPL
- It refers to Node's Shell, a `terminal` that reads, evaluates, prints, and loops results when a user inputs a value. It's an abbreviation for Read Eval Print Loop.

<br>

### Express
- It is a representative `web framework` for Node.js, designed to easily build web servers. For building web servers, it's better to use Express, which allows for detailed manipulation, rather than Node.js, which is designed for broader capabilities.

<br>
