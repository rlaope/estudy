# Prototype Pattern

A prototype is a sample product created for testing before manufacturing the actual product; this sample product is referred to as a prototype.

The Prototype Pattern is one of the creational patterns used when object creation is costly and similar objects already exist.

In other words, the Prototype Pattern provides a mechanism to copy an existing object to a new object and modify it as needed.

The Prototype Pattern uses Java's built-in `clone` method for copying.

![](https://velog.velcdn.com/images%2Fnewtownboy%2Fpost%2Fc06b7237-e6d4-4061-af91-fc875c425745%2Fimage.png)

## The Prototype Pattern is a Creational Pattern

Creational patterns abstract the process of instance creation.

Patterns belonging to the creational category decouple the system from how objects are created, composed, or represented.

Creational patterns are becoming increasingly important as systems evolve towards using composition over inheritance.

There are two key issues in creational patterns:
1. Creational patterns encapsulate information about which concrete classes the system uses.
2. Creational patterns completely hide how instances of these classes are created and combined.

In other words, using creational patterns provides flexibility in deciding what gets created, who creates it, how it's created, and when it's created.

## Prototype Pattern Implementation
The Prototype Pattern is used when object creation is costly and similar objects already exist.

Since it uses Java's `clone` method, it requires overriding the `clone` method for the object to be created. A crucial point here is that the `clone` method must be defined in the class of the object to be created.

For example, if there's a requirement to modify data fetched from a DB multiple times within a program, creating a new object using the `new` keyword and fetching all data from the DB each time is not a good idea.

The reason is that accessing the DB to fetch data is a high-cost operation.
Therefore, it's a better approach to fetch data from the DB once, then copy that object to new objects as needed for data modification tasks.

```java
// Employees.java
public class Employees implements Cloneable {
    private List <String> empList;

    public Employees() {
        empList = new ArrayLis<>();
    }

    public Employees(List < String > list) {
        this.empList = list;
    }

    public void loadData() {
        empList.add("Ann");
        empList.add("David");
        empList.add("John");
        empList.add("Methew");
    }

    @Override
    public Object clone() throws CloneNotSupportedException {
        List<String> temp = new ArrayList<>();
        for (String str: this.empList) {
            temp.add(str);
        }
        return new Employees(temp);
    }
}
```

The code above distinguishes between two types of constructors.

The first is a default constructor with no parameters, which creates a new object.

The second is a constructor with a list parameter, which stores the passed `list` in the currently defined `empList` object.

The `loadData` method simulates fetching data from a DB, and the `getEmpList` method returns the `empList`.

As mentioned earlier, the `clone` method is implemented via an override; it creates a new `temp` object, adds the existing data from `empList` to this `temp` list, and then returns it to the second constructor.

```java
// PrototypePattern.java
public class PrototypePattern {
    public static void main(String[] args) throws CloneNotSupportedException {
        Employees emps = new Employees();
        emps.loadData(); // Ann, John, Methew...

        Employees emps1 = (Employees) emps.clone();
        Employees emps2 = (Employees) emps.clone();

        List<String> list1 = emps1.getEmpList();
        list1.add("Peter");

        List<String> list2 = emps2.getEmpList();
        list2.remove("John");

        System.out.println("emps: " + emps.getEmpList());
        System.out.println("emps1: " + list1.getEmpList());
        System.out.println("emps2: " + list2.getEmpList());
    }
}
```

```
// 결과화면
emps List  : [Ann, David, John, Methew]
emps1 List : [Ann, David, John, Methew, Peter]
emps2 List : [Ann, David, Methew]
```

If the `Employees` class did not provide a `clone` method, the employee list would have to be fetched from the DB every time, which would lead to significant cost.

However, by using the Prototype Pattern, data fetched through a single DB access can be copied to other objects and used, thereby saving costs.
