# Sql Query Builder

 Low level And flexible query builder that gives you full control of your queries, it's created for especially mysql but probably it's compatible for postgresql with many ways as well. It also includes sanitization for both column and value inputs.

That builder enforces you to start your sql query from ground up and build it both imperative and declaratively, however you need, without giving up from flexibility.

It currently supports most basic types of queries: `SELECT`, `INSERT`, `DELETE`, `UPDATE`, `COUNT`.

It also supports this operators for now: `WHERE`, `AND`, `OR`, `SET`, `LIMIT`, `OFFSET`, `ORDER BY`, `LIKE`, `IN`, `NOT IN`, `GROUP BY`, `HAVING`.

And it supports the json functions for now: `JSON_EXTRACT()`, `JSON_CONTAINS()`

## Current Status

This project has reached it's first major release, it never take a breaking change for a long time unless there is a breaking api change on rust standart liblary. In that release branch, we'll focused to add JSON functions mostly.

## Examples

### Initialize a query

For initializing a query, each query type has it's own constructor function. For example, if you want to start a select query, you have to initialize it with corresponding constructor function, such as: `select()`, `delete()`, `update()`, `insert()`, `count()`

```rust

use qubl::{QueryBuilder, ValueType};

fn main(){
    let fields = vec!["name", "surname", "id", "age"];
    
    let select_query = QueryBuilder::select(fields).unwrap();

    let fields = vec!["name", "surname", "age"];

    let values = vec![("necdet", ValueType::String), ("etiman", ValueType::String), ("21", ValueType::Integer)];

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

select_query.where_cond("price", "<", ("250", ValueType::Integer));
select_query.and("price", ">", ("50", ValueType::Integer));
select_query.limit(10);
select_query.offset(0);

// don't forget to finish the query and assign it's value to a variable.
let finish_select_query = select_query.finish();

// ...

```

Or, do it declarative way:

```rust

// ...

let finish_the_select_query = select_query.table("products").where_cond("price", "<", ("250", ValueType::Integer)).and("price", ">", ("50", ValueType::Integer)).limit(10).offset(0).finish();

// ...

```

## Examples Of Other Query Types

### Insert Query

```rust

    let columns = vec!["id", "name", "surname", "age", "password", "email", "grade", "passed"];
    let values = vec![("1", ValueType::Integer), ("necdet arda", ValueType::String), ("etiman", ValueType::String),
                      ("25", ValueType::Integer), ("123456", ValueType::String), ("arda_etiman_799@windowslive.com", ValueType::String),
                      ("75.65", ValueType::Float), ("true", ValueType::Boolean)];

    let insert_query = QueryBuilder::insert(columns, values).unwrap().table("users").finish();

```

### Delete Query

```rust

let mut delete_query = QueryBuilder::delete().unwrap();

delete_query.table("users");
delete_query.where_cond("age", "<", ("25", ValueType::Integer));

let delete_query = delete_query.finish();

```

### Update Query

```rust

    let mut update_query = QueryBuilder::update().unwrap();

    update_query = update_query.table("users")
                               .set("name", ("necdet", ValueType::String))
                               .set("passed", ("true", ValueType::Boolean))
                               .where_cond("id", "=", ("1", ValueType::Integer));

    let finish_update_query = update_query.finish();

```

## Disclaimer

But don't forget, this liblary is just builds that queries, not executes it. For that, you have to use a database driver or orm.
