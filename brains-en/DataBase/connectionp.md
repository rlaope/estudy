# Database Connection Pool

## Connection Pool CP

The typical data integration process involves the web application connecting to the database whenever needed to perform operations.

However, this approach of connecting and working whenever needed leads to a problem where database connection takes a significant amount of time.

For example, in the case of an exchange, if thousands of people simultaneously use trading and inquiry functions, repeatedly establishing and releasing connections with the database would be highly inefficient.

To solve this problem, the current approach is to pre-configure connections with the databases to be integrated as soon as the web application starts, and then use these pre-established connections to quickly interact with the database whenever needed.

**The technology that maintains such pre-established connections with the database is called a Connection Pool (CP).**

## Connection Pool Size Configuration
Theoretically, to determine the minimum required connection pool size,

`PoolSize = Tn x (Cm - 1) + 1`

> Tn: Total number of threads
> Cm: Number of connections simultaneously required for one task

If configured using the formula above, deadlocks can be avoided, but having only one spare connection in the pool is not good for performance.

Therefore, to provide some buffer in the connection pool, it is recommended to use the following formula:

`PoolSize - Tn x (Cm - 1) + (Tn / 2)`
