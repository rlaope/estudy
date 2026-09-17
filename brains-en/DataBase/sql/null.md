# SQL NULL-Related Functions - NVL, NVL2, NULLIF, COALESCE

## SQL NULL-Related Functions
Let's explore SQL's NULL-related functions: NVL, NVL2, NULLIF, and COALESCE.

### NVL
A function that changes a value if it is NULL.

NVL(K, 0) changes the K column to 0 if it is NULL.

### NVL2
A function that combines the NVL and DECODE functions.
NVL2(K, 1, -1) returns 1 if the K column is not NULL, and -1 if it is NULL.

### NULLIF
A function that returns NULL if two values are equal, and the first value if they are not equal.

NULLIF(exp1, exp2) returns NULL if exp1 and exp2 are equal, and exp1 if they are not equal.

### COALESCE
A function that returns the first non-NULL argument value.

COALESCE(exp1, exp2, exp3, ...) checks from exp1 onwards for NULL values in order. If exp2 is the first non-NULL value, it returns exp2.
