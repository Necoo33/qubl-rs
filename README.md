# Sql Query Builder

 Low level And flexible query builder that gives you full control of your queries, it's created for especially mysql but probably it's compatible for postgresql with many ways as well. It also includes sanitization for both column and value inputs.

That builder enforces you to start your sql query from ground up and build it both imperative and declaratively, however you need, without giving up from flexibility.

It currently supports most basic types of queries: `SELECT`, `INSERT`, `DELETE`, `UPDATE`, `COUNT`.

It also supports this operators for now: `WHERE`, `AND`, `OR`, `SET`, `LIMIT`, `OFFSET`, `ORDER BY`, `LIKE`, `IN`, `NOT IN`, `GROUP BY`, `HAVING`, `UNION`, `UNION ALL`, `INNER JOIN`, `LEFT JOIN`, `RIGHT JOIN`. 

It supports that mysql functions: `FIELD()`, `COUNT()`, `RAND()`

And it supports the json functions for now: `JSON_EXTRACT()`, `JSON_CONTAINS()`, `JSON_ARRAY_APPEND()`, `JSON_REMOVE()`, `JSON_SET()`, `JSON_REPLACE()`

## Current Status

This project has reached it's first major release, it never take a breaking change for a long time unless there is a breaking api change on rust standart liblary. In that release branch, we'll focused to add JSON functions mostly.

Consider to give a like that liblary on github if you find it useful: [qubl-rs](https://github.com/Necoo33/qubl-rs)

## Examples

### Initialize a query

For initializing a query, each query type has it's own constructor function. For example, if you want to start a select query, you have to initialize it with corresponding constructor function, such as: `select()`, `delete()`, `update()`, `insert()`, `count()`

```rust

use qubl::{QueryBuilder, ValueType};

fn main(){
    let fields = vec!["name", "surname", "id", "age"];
    
    let select_query = QueryBuilder::select(fields).unwrap();

    let fields = vec!["name", "surname", "age"];

    let values = vec![ValueType::String("necdet".to_string()), ValueType::String("etiman".to_string()), ValueType::Int32(21)];

    let insert_query = QueryBuilder::insert(fields, values).unwrap();

    let delete_query = QueryBuilder::delete().unwrap();

    let update_query = QueryBuilder::update().unwrap();
}

```

### Build and Execute a query

Then build your query both imperative or declarative way.

Imperative way:

```rust

// ...

select_query.table("products");

// do something:

select_query.where_("price", "<", ValueType::Int32(250));
select_query.and("price", ">", ValueType::Int32(50));
select_query.limit(10);
select_query.offset(0);

// don't forget to finish the query and assign it's value to a variable.
let finish_select_query = select_query.finish();

// ...

```

Or, do it declarative way:

```rust

// ...

let finish_the_select_query = select_query.table("products").where_("price", "<", ValueType::Int32(250)).and("price", ">", ValueType::Int32(50)).limit(10).offset(0).finish();

// ...

```

## Examples Of Other Query Types

### Insert Query

```rust

    let columns = vec!["id", "name", "surname", "age", "password", "email", "grade", "passed"];
    let values = vec![ValueType::Int32(1), ValueType::String("necdet arda".to_string()), ValueType::String("etiman".to_string()),
                      ValueType::Int32(24), ValueType::String("123456".to_string()), ValueType::String("arda_etiman_799@windowslive.com".to_string()),
                      ValueType::Float64(75.65), ValueType::Boolean(true)];

    let insert_query = QueryBuilder::insert(columns, values).unwrap().table("users").finish();

```

### Delete Query

```rust

let mut delete_query = QueryBuilder::delete().unwrap();

delete_query.table("users");
delete_query.where_("age", "<", ValueType::Int32(25));

let delete_query = delete_query.finish();

```

### Update Query

```rust

    let mut update_query = QueryBuilder::update().unwrap();

    update_query = update_query.table("users")
                               .set("name", ValueType::String("necdet".to_string()))
                               .set("passed", ValueType::Boolean(true))
                               .where_("id", "=", ValueType::Int32(1));

    let finish_update_query = update_query.finish();

```

And there is tens of examples about how you can create very specific queries with absolute flexibility and type safety on that [link](https://github.com/Necoo33/qubl-rs/blob/02ee9d232c913fd4e9fc05cca4638cda3ceb0851/src/lib.rs#L3107).

## Disclaimer

But don't forget, this liblary is just builds that queries, not executes it. For that, you have to use a database driver or orm. And also this query builder fully focused on returning returning datas from databases, it doesn't have a aim for supporting logical operations via sql and we don't give any guarantee that you can do working logical queries with possibilities of that crate.
