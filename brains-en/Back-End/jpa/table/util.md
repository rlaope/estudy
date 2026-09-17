# Automatic Database Schema Generation
- Automatically generate DDL at application startup
- Table-centric -> Object-centric
- Generate appropriate DDL for the database using database dialects
- The DDL generated this way **is only used on development machines**
- The generated DDL should not be used on production servers, or should be used after appropriate refinement

### Properties
- create: Deletes existing tables and recreates them (DROP + CREATE)
- create-drop: Same as create, but drops tables on exit
- update: Applies only changes (not for use in production DBs)
- validate: Only checks if entities and tables are mapped correctly
- none: Not used

### Caution
- Never use create, create-drop, or update on production machines.
- For initial development stages, use create or update
- For test servers, use update or validate
- For staging and production, use validate or none

### DDL Generation Features
- Add constraints
- `@Column(nullable = false, legnth = 10)`
- Add unique constraints
- `@Table(uniqueConstraints = {@UniqueConstraint( name = "NAME_AGE_UNIQUE" , columnNames = {"NAME", "AGE"}) })`
- The DDL generation feature is only used for automatic DDL generation and does not affect JPA's execution logic (database impact).
