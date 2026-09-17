# YAGNI (You Ain't Gonna Need It)

The YAGNI principle means "You Ain't Gonna Need It." In other words, it means to only do the work that is currently necessary during development.

When coding, there will inevitably be unnecessary tasks done in preparation for future scalability, even if they aren't used currently. This could be abstraction, method extraction, or refactoring in some form, or even developing code modules with unused queries or patterns applied.

Will what's built in that way truly be helpful? I've mostly seen new features or code in changed formats, created with such intentions, end up as technical debt. I've rarely seen those changes applied to other codebases or frequently used.

If there's a lot of code that isn't currently used and whose future use is uncertain, it becomes unnecessarily verbose and usually difficult to share with team members. If changes are submitted as a PR, the focus should be on the core changes, and context should be shared with the team. But if minor refactoring and various features are mixed in, team members will never grasp the changes.

Therefore, the principle is to focus on immediately necessary tasks and avoid unnecessary ones. In situations where time should be spent on core logic, other tasks consume resources, and those consumed resources only accumulate as debt.

Furthermore, focusing on various other tasks degrades development productivity. A large workload can lead to cognitive overload, delaying even the start of work.

<br>

### Refactoring Cycle

Refactoring is quite ambiguous. While the goal is to clean up existing technical debt or legacy systems and transform them into a better structure for our code's productivity and scalability, refactoring can sometimes accidentally change behavior or make code harder to read.

If you're developing with team members, it's generally recommended to schedule a refactoring day. Some companies are reportedly adopting this. Personally, I believe it's acceptable even if the code is a bit messy, as long as the team members can manage it well.

While I think changes like renames or dead code elimination can be included in small, overlapping PRs, changes such as separating domain core logic, systematizing individual UI designs, or switching tech stacks should be thoroughly discussed with team members and proceeded with after a decision is made.

If excessive abstraction or design patterns are introduced to increase scalability, it could lead to difficulties adapting from data-oriented programming to object-oriented, repetitive boilerplate code, and various side effects.

Also, some code might have hardcoded parts due to compromises, and you shouldn't think that hardcoding due to compromise is bad. While it's undeniable that it can be messy, considering the resources saved by that compromise, I believe it's a reasonable trade-off. (e.g., code like "for a user with a specific uid, behave this way..." see below)

```kotlin
fun doSomething(userUid: Int) {
    if(userUid == 12234) {
        reject()
    } else {
        validate()
        accept()
    }
}
```

While code like the above is messy, if the specific task processing for userUid 11234 is complex and difficult to identify, such an approach can be a good option. Of course, you'd need to add a comment or something to delete it after a year.

<br>

### Trade-offs

When developing, you'll often make decisions amidst many trade-offs. A common mistake is having many meetings that prioritize future scenarios over immediate needs. What's funny is that sometimes you see people who, instead of using methods like linear regression to predict the future, just heuristically think, "Wouldn't it be like that?"

Of course, detecting new edge cases or similar aspects is good. That prevents malfunctions that can occur in user flows. However, what I'm referring to here is when easy solutions or solutions familiar to team members are sufficient, but people insist that a certain approach is the correct answer simply because of its name value or articles from big tech companies. (They don't even know why it's good. If they did, they wouldn't have chosen it in the first place.)

This might stem from intellectual vanity or a desire for personal growth, but that cannot be an excuse. We develop at a company, and a good developer maximizes the use of company product resources to achieve the best results with minimal loss. (If you disagree, I recommend developing alone and starting your own business, or moving to a different organization or company. Please don't misunderstand; I'm not saying this in a negative way.)

I also want to try many new technologies and explore various techniques, but ultimately, I believe the most crucial thing is to accomplish the mission assigned by the company right now, more so than such capabilities. This isn't a disqualification as a developer, nor is it about hierarchy. First, satisfying the customer must be prioritized for a business to succeed, and only when the business succeeds do the opportunities and scope for using various technologies expand.

Unfortunately, I have yet to see a single tech-biased developer who doesn't prioritize client feedback.
