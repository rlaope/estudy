# Spring Package Structure - Layered vs. Domain-Driven

There are broadly two types of package organization:
1. Layered
2. Domain-Driven

### Layered Package Structure
- A method of designing packages in a layered fashion.
- It has the advantage of quickly grasping the overall structure, but the disadvantage is that too many classes accumulate in a directory.

**Advantages**
1. Even with relatively low understanding of the project, the overall structure can be quickly grasped.

**Disadvantages**
1. Too many classes accumulate in a directory.
2. Difficult to separate into modules.

### Domain-Driven Package Structure
- Organizes directories by domain.
- The domain structure has the advantage of cohesive related code, but it can be difficult to grasp the overall structure if one has a low understanding of the project.

**Advantages**
- Related code is cohesive.
- Advantageous when separating into modules.

**Disadvantages**
- Difficult to grasp the overall structure if one has a low understanding of the project.
- The criteria for distinguishing domains can vary depending on the developer's preference, and it can be hard to find related code if it's in a package different from what's expected.
- Potential for circular dependencies between packages.
  - Even files that could reside within the same package might end up in different packages due to module-based separation, leading to mutual references.

<br>

### Layered and Domain-Driven Examples

![Structure](./image/패키지구조.png)

<br>

### Conclusion

If complexity is high, many features are provided, leading to a large number of classes belonging to a single layer,
features can be separated by clear criteria,
and there's a possibility of splitting into separate services by module in the future,
-> **Domain-Driven Structure**

If complexity is low, few features are provided, leading to a small number of classes belonging to a single layer,
it's ambiguous to separate features by clear criteria,
and it's a small-scale project unlikely to be split into modules in the future,
-> **Layered Structure**

I would lean towards this direction.
