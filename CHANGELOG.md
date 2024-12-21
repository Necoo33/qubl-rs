# Changelog

## v1.2.0

- added `.json_contains()` method to the `QueryBuilder` struct, which provides you to use "JSON_CONTAINS()" mysql function later than various keywords. Added many test cases to the tests.
- added support for `HAVING` keyword with `.having()` method.

## v1.1.1

- added proper documentation for most of the types. This release is just a documentation fix release.

## v1.1.0

- added `.json_extract()` method to the `QueryBuilder` struct, which provides you to use "JSON_EXTRACT()" mysql function later than various keywords. Added many test cases to the tests.
- added support for `GROUP BY` keyword with `.group_by()` method.
- The algorithm of `.order_by()` method slightly changed, it does not affect your existing codes though.
- The actual usage of `KeywordList` enums are started with that release. Added `GroupBy` variant to it.
- Now all methods add proper keywords for their last added query types to the `list` field.
- added `.append_keyword()` method. Since we started to use `KeywordList` enum to build queries correctly, you should append equivalent keyword with the string that you appended to original query with `.append_custom()` method. Otherwise you should continue to build your entire query with `.append_custom()` method, because you could easily encounter syntactic bugs.

## v1.0.0

- Made performance improvements by removing unnecessary copyings on `QueryBuilder` and `SchemaBuilder`.
- Project become stabilized. It continue to take more updates but it doesn't take a major one unless a breaking change happens on the synthax for very long time.
- Added proper documentation for `QueryBuilder` struct.

## v0.9.0

- Added `count()` constructor to `QueryBuilder` struct for creating count queries and added `Count` variant to `QueryType` and `KeywordList` enums. Also added `In` and `NotIn` variants to `KeywordList` enum.

## v0.8.0

- Added `.where_in()`, `.where_not_in()`, `.where_in_custom()`, `.where_not_in_custom()` methods for using `IN` and `NOT IN` operators.

## v0.7.0

- Added unix epoch times support for `Time` variant of `ValueType` enum. If you give a value that's characters are fully convertible to integer or empty strings with that `Time` variant, it applies that value to `FROM_UNIXTIME()` function and converts it to a valid `DATETIME`, so you can add them to column. If you want to store them directly as unix epoch times, then you have to set to column type as `INT` in mysql and set the `ValueType` enum as `Integer` variant.

## v0.6.0

- Added `Time` variant for `ValueType` enum. Added support for column types of time. That values and sql functions are supported to add as sql: `UNIX_TIMESTAMP`, `CURRENT_TIMESTAMP`, `CURRENT_DATE`, `CURRENT_TIME`, `NOW()`, `CURDATE()`, `CURTIME()`. You can also time values as string. For now, unix epoch times are not supported.

## v0.5.0

- Added `append_custom()` function for my urgent need for using json functions on mysql. In later releases, json functions will be implemented to `QueryBuilder`.

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
