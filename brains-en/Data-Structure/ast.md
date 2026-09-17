## Abstract Syntax Tree

An AST is a tree representation of the abstract syntactic structure of **source code** written in a programming language.

Each node in the tree denotes a construct occurring in the source code. The term "abstract" implies that it does not represent every detail appearing in the real syntax.

For example, grouping parentheses are implicit in the tree structure and are not represented as separate nodes. Similarly, a syntactic construct like an if-condition-then expression is represented by a single node with three branches.

This distinguishes abstract syntax trees from concrete syntax trees, which are the original [parse tree](https://ko.wikipedia.org/wiki/%ED%8C%8C%EC%8A%A4_%ED%8A%B8%EB%A6%AC) concept.

Parse trees are typically built by a parser during the compilation or translation of source code. Once built, additional information is added to the AST by subsequent processing.

ASTs with these characteristics are also used in program analysis and transformation systems.

### Applications in Compilers

Abstract syntax trees are widely used data structures in **compilers** because they are properties that represent the structure of program code.

An AST is typically the result of the syntax analysis phase of a compiler. It serves as an intermediate representation of the program through various stages required by the compiler and has a strong influence on the compiler's final output.

### Motivation

ASTs have several additional stages that aid in the compilation process.

- All components can be modified and improved through their attributes or annotations. Such annotations and modifications imply changes that are not possible with source code.

- Compared to source code, ASTs do not contain punctuation or delimiters (such as parentheses, semicolons, etc.).

- ASTs typically contain only additional program information resulting from the compiler's successive analysis phases. For example, they store the position of each component in the source code, allowing the compiler to output useful error messages.

ASTs are necessary due to the inherent nature and documentation of programming languages. Programming languages often inherently contain ambiguities, and to avoid these, they are specified by context-free grammars. However, there are documented language features that CFGs cannot express; these are details that require context to determine valid behavior. For example, if a programming language allows the declaration of new types, a CFG cannot predict the name of that type or how it will be used. Even if a programming language has a predefined set of types, some context is needed to enforce their proper use.
