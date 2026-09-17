# Application SW_2 (Computer System General Study Notes)

### Characteristics of Relational Databases
- Tables are composed of a collection of records.
- Fast data classification, sorting, and search speeds, and data integrity is guaranteed.
- Difficult to modify existing schemas and inefficient for processing large amounts of data.

### Data Modeling
- The process of deriving a set of data to constitute a system, defining detailed attributes and identifiers for each set, and visualizing the relationships between these data sets using a defined notation.

<br>

### Q ) The following is a customer table.

Customer Number | Customer Name | Resident Registration Number
|------|---|---|
|5 |김동동|050102-4******|
|3 |정봉봉|051218-4******|
|8 |서징징|050711-4******|

- A new customer has arrived. The customer number is 7, the customer name is '서봉동', and the resident registration number is '050206-4******'. Write an INSERT statement to insert this data into the customer table.

<br>

`answer )`
```
INSERT
INTO 고객(고객번호, 고객이름, 주민번호)
VALUSE(7 , '서봉동' , 050206-4******);
```

### Q2 ) Write a command to search for customer numbers whose customer name starts with '서' (Seo). (Note: The customer name is specified as 3 characters long.)

`answer )`
```
SELECT 고객번호
FROM 고객
WHERE 고객이름 LIKE ('서__');
```
