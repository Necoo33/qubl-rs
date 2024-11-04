# Changelog

## v0.4.0

- Added `TableBuilder` struct and some implementations, which is enough for creating tables and adding relations. In next releases we'll implement other methods for altering tables.
- Added `ForeignKey` and `ForeignKeyItems` structs and `ForeignKeyActions` enum and implemented `Display` trait for that enum.

## v0.3.1

- Documentation fixed.

## v0.3.0

- Added `SchemaBuilder` struct and implementation, which enables you to create schemas. You can also create `USE` queries with that.
- Now `.select()` constructor of `QueryBuilder` supports `SELECT *` queries by giving a vector of it's argument which first element is "*".
- Added additional test cases.
- Upgraded the algorithm of `sanitize()` function for better sanitizing queries. Also added `KeywordList` enum and "list" field on builder. It lets us to more aggresively format the query string for specific cases.

## v0.2.0

upgraded the algorithm of `sanitize()` function for better sanitizing queries.

## v0.1.0

qubl-rs liblary created.
