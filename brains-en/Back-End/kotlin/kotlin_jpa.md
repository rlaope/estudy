# Differences in the Directions Kotlin and JPA Pursue

Are you aware that Kotlin and JPA pursue different directions?

Let's explore why Kotlin and JPA pursue incompatible directions.

## The Direction Kotlin Pursues

Kotlin aims for **immutability**, striving to enhance code stability by minimizing mutable states.

To achieve this, you can declare variables using the `val keyword` and define immutable data models using data classes.

## The Direction JPA Pursues

In contrast, JPA is designed around mapping objects to a database, with its primary goal being to **change the state of the database**.

Therefore, JPA defines mutable data models using entity classes and interacts with the database using `EntityManager`.

## Conclusion

Kotlin and JPA pursue different directions regarding the mutable state of objects.

It would be best to use them considering their respective pros and cons. -> Separation of Domain and Entity
