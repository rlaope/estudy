# Object-Oriented

### What is Object-Oriented?
- Everything we use in real life is called an object, and object-oriented programming involves identifying the objects needed for program implementation, defining the role of each object, and creating programs through the interaction of these objects.

  ### Components of Object-Oriented Programming Languages
  1. `Class`
  > Defines the attributes and behaviors belonging to the same group.

  2. `Object`
  > An instance of a class that holds its own unique data and can perform the behaviors defined in the class.

  3. `Method`
  > A way to use an object created from a class.

  4. `Attribute`
  > Data values held by objects within a class, defined unit by unit.

<br>

  ### What is Overloading?
  - Using the same function name but with different parameters or data types.

  ex )
  ```java
  class ExJava {
    int value = 0;
    int exov(int a){
      value = a;
    }
    int exov(int a , int b){
      value = a + b;
    }
    int exov(int a , int b , int c){
      value = a + b * c;
    }
  }
  ```
<br>

  ### What is Method Overriding?
  - Redefining a method inherited from a superclass (or parent class).

  <br>

  ### Access Specifiers

  1. public: An access specifier that allows all access.
  2. protected: An access specifier that allows access from its own class and inherited child classes.
  3. private: An access specifier that allows access only from methods within its own class.
  4. default: An access specifier that allows access from all classes within the same package, but not from other packages.

  <br>

  ###
