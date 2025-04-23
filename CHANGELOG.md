# Changelog

## v2.16.1

- Added documentation for `.inner_join()` method.

## v2.16.0

- `.inner_join()` method added to `QueryBuilder` type.
- Added support for all remaining Pacific Continent Timezonse to `Timezone` enum.

## v2.15.0

- `.set_timezone()` and `.set_global_timezone()` methods are changed, in previous version their way of work is failed last test, now it passes.
- Way of work of `NULL` variant of `ValueType` enum with `mark` arguments are changed, now it changes all `=` operands with `IS` and `!=` & `<>` operands with `IS NOT`. If you add this kind of query: `query.where_("pic", "!=", ValueType::Null)` it transforms it to that: `... WHERE pic IS NOT NULL`.

## v2.14.0

- Added support for Arabic and all remaining American Continent provinces to `Timezone` enum.

## v2.13.0

- `.and_in()`, `.and_not_in()`, `.and_in_custom()`, `.and_not_in_custom()`, `.or_in()`, `.or_not_in()`, `.or_in_custom()`, `.or_not_in_custom()` methods added to `QueryBuilder` struct. That works same as their counterparts, such as `.where_in()`, `.where_not_in()` etc.

## v2.12.0

- `Timezone` Enum added. It benefits to set query's timezone to a specific timezone. For now, it only supports American, Russian, European, Turkish And Chinese Timezones.
- For setting timezones, `.time_zone()` and `".global_time_zone()` methods are added to  `QueryBuilder` struct.

## v2.11.2

- after i saw the some of the breaking aspects of that code, that fix removed for `ValueType::String()` and `ValueType::Datetime()` instances. If you want a text like '"something"' Than go with: `JsonValue::Initial(ValueType::JsonString("blabla"))`. Otherwise, Go with others.

## v2.11.1

- Bugfix on previous update was unsuccessfull, this time it really fixed and put an example on tests.

## v2.11.0

- A bug fixed: In previous versions, when you pass `JsonValue::Initial()` with `ValueType::JsonString()` to the json function's arguments that has `JsonValue` type, the `QueryBuilder` were produce invalid query when you pass it to a string. With that release, that is fixed. Now when you pass either `ValueType::JsonString()`, `ValueType::String()` or `ValueType::Datetime()` to that functions with `JsonValue::Initial()` enum, your string will be wrapped by single quotes.

## v2.10.0

- Added `.json_set()` and `.json_replace()` methods with it's synthax. It's meant to be used with `update()` constructor for update something inside of the columns that has Json type.

## v2.9.1

- Some documentation fixes on Readme file.

## v2.9.0

- Added `.json_remove()` method to the `QueryBuilder` struct. It's meant to be used with `update()` constructor for removing something from the columns that has Json type.
- Some documentation fixes.

## v2.8.1

- documentation update, nothing changed.

## v2.8.0

- `JsonValue` enum added. It's meant to bu used to create json objects for using with json functions.
- Breaking change: `.json_contains()` and `.not_json_contains()` method's arguments are changed. Now their second argument's type is `JsonValue`. Check the documentation examples about that.
- `.json_array_append()` method added to the `QueryBuilder` struct. It's meant to be used with `update()` constructor for appending something to the columns that has Json type.

## v2.7.0

- Added `.not_json_contains()` method to `QueryBuilder` struct.

## v2.6.0

- Implemented `Into` trait to `ValueType` enum.

## v2.5.0

- Support for ordering methods are improved: Now it supports `.order_by_field()` method before than `.order_by()` method and chaining of `.order_by_field()` methods.

## v2.4.0

- Added `.union()` and `.union_all()` methods to `QueryBuilder` Struct. Added support for `Union` and `UnionAll` variants for `Keywordlist` Enum.

## v2.3.0

- Length check added for every vector that passed on builders. Now if you pass an empty vector to any of the builders any method, builders will be panicked.

## v2.2.0

- Added Null value support for `QueryBuilder` struct.

## v2.1.0

- `FROM` trait implemented to `ValueType` enum.
- `v2rc1` branch merged to main.

## v2.0.0

- `ValueType` Enum had breaking changes: Now we implemented `Display` trait to it, now we don't have to give values with old cumbersome way. You can reach it's details on [README.md](https://github.com/Necoo33/qubl-rs) or [Official Documentation](https://docs.rs/qubl-rs/latest/qubl/).
- documentation updated, basic usages are shown for each query.

## v1.6.0

- added `.order_by_field()` method, which benefits to apply `FIELD()` mysql function to query with it's synthax.
- `.order_by()` function is modified, now it supports ORDER BY chaining.

## v1.5.0

- the implementation of `.like()` method modified, now it supports to come later than `.where_cond()`, `.where_in()` or `.where_not_in()` methods. Previously, it was not produce correct queries when it used later then that methods because it also applies `WHERE` keywords for it's synthax. Now, you can use a different condition with `WHERE` methods and later than use that method for searching for rows that both provide the `WHERE` keywords condition. It searchs the needle on all the given columns seperately and takes the row if it exists on any of them.

## v1.4.0

- added `.copy()` method for getting the immutable copy of the `QueryBuilder` struct.
- `.json_extract()` method modified, now it supports multiple chaining of `JSON_EXTRACT()` method.

## v1.3.0

- `.json_extract()` and `.json_contains()` functions are modified and their algorithm improved. Before that update, we assume you want to work with a column that is a json object and pointing a key. With that update, you can choose whether you want to work with a json array or json object, by simply putting a dot in the beginning of that function(for example, if you want to put "name" on the needle, you just have to make it ".name") or just select the which nth of that array with it's synthax.

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
