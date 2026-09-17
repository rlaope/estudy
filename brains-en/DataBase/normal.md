# Database Normalization

### Normalization

The primary goal of normalization is to **prevent duplicate data between tables**. By preventing duplicate data, **data integrity can be maintained, and the storage capacity of the database can also be reduced.**
Normalization defines stages for decomposing tables. The normalization stage varies depending on how a table is decomposed. Let's explore each normalization stage in detail.

<br>

### First Normal Form
First Normal Form (1NF) means **decomposing a table so that its columns contain atomic values (a single value).** For example, let's say there's a customer hobby table like the one below.

![](./Image/제1정규화.png)

In the table above, Choo Shin-soo and Park Se-ri have multiple hobbies, so they do not satisfy the First Normal Form. Therefore, this can be decomposed through 1NF.

![](./Image/1정규화적용.png)

<br>

### Second Normal Form
Second Normal Form (2NF) means **decomposing a table that has undergone 1NF to satisfy full functional dependency.** Here, full functional dependency means that **no subset of the primary key should be a determinant.**

![](Image/제2정규화.png)

In this table, the primary key is a composite key: (Student ID, Course Name). This primary key (Student ID, Course Name) determines the Grade. (Student ID, Course Name) --> (Grade)
However, the 'Classroom' column here can be determined by 'Course Name', which is a subset of the primary key. (Course Name) --> (Classroom)
In other words, since 'Course Name', a partial key of the primary key (Student ID, Course Name), is a determinant, the table above can satisfy 2NF by decomposing 'Classroom' from the original table and managing it in a separate table, as shown below.

![](./Image/제2정규화적용.png)

### Third Normal Form
Third Normal Form (3NF) means **decomposing a table that has undergone 2NF to eliminate transitive dependencies.** Here, transitive dependency means that if A -> B and B -> C hold, then A -> C also holds.

![](./Image/제3정규화.png)

In the original table, Student ID determines Course Name, and Course Name determines Tuition Fee. Therefore, this should be decomposed into a (Student ID, Course Name) table and a (Course Name, Tuition Fee) table.

The reason for eliminating transitive dependencies is relatively simple. For example, suppose student 501's course changes to Sports Management. If a transitive dependency exists, student 501 would take the Sports Management course for a tuition fee of 20,000 won. While the tuition fee could be changed again to match the course name, 3NF is applied to resolve this inconvenience.
That is, the table should be decomposed so that Student ID references Course Name, and Course Name references Tuition Fee. The result is shown in the following figure.

![](Image/제3정규화적용.png)

<br>

### BCNF Normalization
BCNF Normalization means **decomposing a table that has undergone 3NF such that every determinant is a candidate key.** For example, let's say there's a special lecture enrollment table like the one below.

![](Image/BCNF정규화.png)

In the special lecture enrollment table, the primary key is (Student ID, Special Lecture Name). This primary key (Student ID, Special Lecture Name) determines the Professor. Furthermore, the Professor here determines the Special Lecture Name.
The problem, however, is that the Professor is a determinant for Special Lecture Name but is not a candidate key. Therefore, to satisfy BCNF Normalization, the table above must be decomposed, which can be done into a Special Lecture Enrollment table and a Special Lecture Professor table, as follows.

![](Image/BCNF적용.png)
