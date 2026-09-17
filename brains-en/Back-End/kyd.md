# KISS, YAGNI, DRY

To improve code quality and write good source code, let's explore some key principles worth knowing and following.

### KISS - Keep It Simple Stupid

KISS is an acronym derived from phrases like Keep It Simple Stupid or Keep It Short and Simple. It describes the principle that, in software design and coding, it's best to keep things as simple and straightforward as possible. It's a principle that **warns against source code or designs becoming unnecessarily verbose or complex**.

The simpler it is, the easier it is to understand; the easier it is to understand, the easier it is to track bugs, and the lower the likelihood of them occurring. This directly leads to increased productivity, so we should always be wary of complexity.

<br>

### YAGNI - You Ain't Gonna Need It

The YAGNI principle, when expanded, means 'only do what's necessary'.

When writing a program, there might be things you've prepared in advance, not currently used, but intended for future use or for extensibility.

The principle that tells you 'don't do those things!' is YAGNI.

If you write code that isn't currently used and might or might not be used at some point in the future, the code becomes unnecessarily verbose. Furthermore, if the design or environment changes, the amount of code that needs modification increases. What if the design or environment changes, and you don't modify this pre-written code? It will cause bugs later. Therefore, the principle advises focusing on immediately necessary tasks and avoiding unnecessary work.

<br>

### DRY - Do not Repeat Yourself

The DRY principle means 'don't repeat yourself'. When the same code is repeated in source code, it increases the threat of potential bugs. If the content of repeated code needs to change, you have to find and modify all instances of that repeated code. If a mistake occurs during this process, another bug will arise.

As project size grows, the maintenance overhead caused by repeated code becomes very large, so it's important to develop the habit of not repeating code, even in small projects.

That said, don't separate everything just because it's repeated a couple of times. Make a reasonable judgment about how often it might occur, if it will be needed more in the future, or if it feels like it's been repeated too much. I personally tend to separate things if they are repeated more than three times.
