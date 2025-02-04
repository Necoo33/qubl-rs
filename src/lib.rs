/// Struct that benefits to build queries for interactions with rdbms's.
#[derive(Debug, Clone)]
pub struct QueryBuilder<'a> {
    pub query: String,
    pub table: String,
    pub qtype: QueryType,
    pub list: Vec<KeywordList>,
    pub hq: Option<[&'a str; 26]>
}

/// Implementations For QueryBuilder.
impl<'a> QueryBuilder<'a> {
    /// Select constructor. Use it if you want to build a Select Query.
    /// 
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType};
    /// 
    /// fn main(){
    ///     let query = QueryBuilder::select(vec!["*"]).unwrap();
    /// }
    /// 
    /// ```
    pub fn select(fields: Vec<&str>) -> std::result::Result<Self, std::io::Error> {
        match fields.len() {
            0 => panic!("you cannot pass an empty vector to the fields argument"),
            _ => ()
        }

        let hq = Self::load_hqs();
        match Self::sanitize_columns(&fields, hq) {
            Ok(_) => {
                if fields.len() > 1 && fields[0] == "*" {
                    let query = "SELECT * FROM".to_string();
    
                    return Ok(QueryBuilder {
                        query,
                        table: "".to_string(),
                        qtype: QueryType::Select,
                        list: vec![KeywordList::Select],
                        hq: Some(hq)
                    })
                } else {
                    let mut query = "SELECT ".to_string();

                    let length_of_fields = fields.len();
    
                    for (i , field) in fields.into_iter().enumerate() {
                        if i + 1 == length_of_fields {
                            query = format!("{}{} ", query, field);
                        } else {
                            query = format!("{}{}, ", query, field);
                        }
                    }
    
                    let query = format!("{}FROM", query);
    
                    return Ok(QueryBuilder {
                        query,
                        table: "".to_string(),
                        qtype: QueryType::Select,
                        list: vec![KeywordList::Select],
                        hq: Some(hq)
                    })
                }
            },
            Err(_) => {
                return Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "query cannot build because inserted arbitrary query."))
            }
        }
    }

    /// Delete constructor. Use it if you want to build a Delete Query.
    /// 
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType};
    /// 
    /// fn main(){
    ///     let query = QueryBuilder::delete().unwrap();
    /// }
    /// 
    /// ```
    pub fn delete() -> std::result::Result<Self, std::io::Error> {
        return Ok(QueryBuilder {
            query: "DELETE FROM".to_string(),
            table: "".to_string(),
            qtype: QueryType::Delete,
            list: vec![KeywordList::Delete],
            hq: None
        })
    }

    /// Update constructor. Use it if you want to build a Update Query.
    /// 
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType};
    /// 
    /// fn main(){
    ///     let query = QueryBuilder::update().unwrap();
    /// }
    /// 
    /// ```
    pub fn update() -> std::result::Result<Self, std::io::Error> {
        return Ok(QueryBuilder {
            query: "UPDATE".to_string(),
            table: "".to_string(),
            qtype: QueryType::Update,
            list: vec![KeywordList::Update],
            hq: None
        })
    }

    /// Insert constructor. Use it if you want to build a Insert Query.
    ///     
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType};
    /// 
    /// fn main(){
    ///     let fields = vec!["id", "age", "name"];
    ///     let values = vec![ValueType::Int32(5), ValueType::Int64(25), ValueType::String("necdet".to_string())]
    /// 
    ///     let query = QueryBuilder::insert(fields, values).unwrap();
    /// }
    /// 
    /// ```
    pub fn insert(columns: Vec<&str>, values: Vec<ValueType>) -> std::result::Result<Self, std::io::Error> {
        match values.len() {
            0 => panic!("you cannot pass an empty vector to the values argument"),
            _ => ()
        }

        let mut query = "INSERT INTO".to_string();

        let hq = Self::load_hqs();

        match QueryBuilder::sanitize_columns(&columns, hq) {
            Ok(_) => (),
            Err(_) => {
                return Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "query cannot build on insert constructor: because inserted arbitrary query on columns parameter."))
            }
        }

        match QueryBuilder::sanitize_inputs(&values, hq) {
            Ok(_) => (),
            Err(_) => {
                return Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "query cannot build on insert constructor: because inserted arbitrary query in values parameter."))

            }
        }

        let mut columns_string = "(".to_string();
        let mut values_string = "(".to_string();

        for (i, column) in columns.into_iter().enumerate() {
            for (p, value) in values.iter().enumerate() {
                match i == p {
                    true => {
                        if i == 0 {
                            columns_string = format!("{}{}", columns_string, column);        
                        } else {
                            columns_string = format!("{}, {}", columns_string, column);
                        }

                        if p == 0 {
                            values_string = format!("{}{}", values_string, value);
                        } else {
                            values_string = format!("{}, {}", values_string, value);
                        }
                    },
                    false => continue
                }
            }
        }

        query = format!("{} {}) VALUES {})", query, columns_string, values_string);

        return Ok(Self {
            query,
            table: "".to_string(),
            qtype: QueryType::Insert,
            list: vec![KeywordList::Insert],
            hq: Some(hq)
        })
    }

    /// define the table. It should came after the constructors.
    /// 
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType};
    /// 
    /// fn main(){
    ///     let query = QueryBuilder::select(vec!["*"]).unwrap().table("users");
    /// }
    /// 
    /// ```
    pub fn table(&mut self, table: &str) -> &mut Self {
        match self.qtype {
            QueryType::Select => {
                self.query = format!("{} {}", self.query, table);
                self.table = table.to_string();
            },
            QueryType::Delete => {
                self.query = format!("{} {}", self.query, table);
                self.table = table.to_string()
            },
            QueryType::Insert => {
                let split_the_query = self.query.split(" INTO ").collect::<Vec<&str>>();
    
                self.query = format!("INSERT INTO {} {}", table, split_the_query[1]);
                self.table = table.to_string();
            }
            QueryType::Update => {
                self.query = format!("{} {}", self.query, table);
                self.table = table.to_string()
            },
            QueryType::Count => {
                self.query = format!("{} {}", self.query, table);
                self.table = table.to_string()
            }
            QueryType::Null => panic!("You cannot add a table before you start a query"),
            QueryType::Create => panic!("You cannot use create keyword with a QueryBuilder instance")
        }
    
        self.list.push(KeywordList::Table);
    
        self
    }
    

    /// Count constructor. Use it if you want to learn to length of a table.
    ///     
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType};
    /// 
    /// fn main(){
    ///     let query = QueryBuilder::count("*", Some("length")).table("users");
    /// }
    /// 
    /// ```
    pub fn count(condition: &str, _as: Option<&str>) -> Self {
        let query;

        match _as {
            Some(_as) => query = format!("SELECT COUNT({}) AS {} FROM", condition, _as),
            None => query = format!("SELECT COUNT({}) FROM", condition)
        };

        return Self {
            query,
            table: "".to_string(),
            qtype: QueryType::Count,
            list: vec![KeywordList::Count],
            hq: Some(Self::load_hqs())
        }
    }
    /// add the "WHERE" keyword with it's synthax.
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType};
    /// 
    /// fn main(){
    ///     let query = QueryBuilder::select(vec!["*"]).unwrap().table("users").where_("id", "=", ValueType::Int32(5)).finish();
    /// 
    ///     assert_eq!(query, "SELECT * FROM users WHERE id = 5;")
    /// }
    /// 
    /// ```
    pub fn where_(&mut self, column: &str, mark: &str, value: ValueType) -> &mut Self {
        match Self::sanitize_mark(mark) {
            Ok(_) => (),
            Err(error) => panic!("{}", error)
        }

        match self.hq {
            Some(_) => (),
            None => self.hq = Some(Self::load_hqs())
        }

        match self.sanitize_column(&column) {
            Ok(_) => (),
            Err(error) => panic!("{}", error)
        }

        match self.sanitize_input(&value) {
            Ok(_) => (),
            Err(error) => panic!("{}", error)
        }

        self.query = format!("{} WHERE {} {} {}", self.query, column, mark, value);

        self.list.push(KeywordList::Where);

        self
    }

    /// It adds the "IN" keyword with it's synthax. Don't use ".where_cond()" method if you use it.
    ///     
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType};
    /// 
    /// fn main(){
    ///     let ins = vec![ValueType::Int16(1), ValueType::Int64(5), ValueType::Int32(10)];
    ///     let query = QueryBuilder::select(vec!["*"]).unwrap().table("users").where_in("id", &ins).finish();
    /// 
    ///     assert_eq!(query, "SELECT * FROM users WHERE id IN (1, 5, 10);")
    /// }
    pub fn where_in(&mut self, column: &str, ins: &Vec<ValueType>) -> &mut Self {
        match ins.len() {
            0 => panic!("you cannot pass an empty vector to the ins argument"),
            _ => ()
        }

        self.query = format!("{} WHERE {} IN (", self.query, column);

        let length_of_ins = ins.len();

        for (index, value) in ins.into_iter().enumerate() {
            if index + 1 == length_of_ins {
                self.query = format!("{}{})", self.query, value);
                    
                continue;
            }

            self.query = format!("{}{}, ", self.query, value);
        }

        self.list.push(KeywordList::In);
        self
    }

    /// It adds the "NOT IN" keyword with it's synthax. Don't use ".where_cond()" method if you use it.
    ///     
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType};
    /// 
    /// fn main(){
    ///     let ins = vec![ValueType::Int16(1), ValueType::Int64(5), ValueType::Int32(10)];
    ///     let query = QueryBuilder::select(vec!["*"]).unwrap().table("users").where_not_in("id", &ins).finish();
    /// 
    ///     assert_eq!(query, "SELECT * FROM users WHERE id NOT IN (1, 5, 10);")
    /// }
     pub fn where_not_in(&mut self, column: &str, ins: &Vec<ValueType>) -> &mut Self {
        match ins.len() {
            0 => panic!("you cannot pass an empty vector to the ins argument"),
            _ => ()
        }

        self.query = format!("{} WHERE {} NOT IN (", self.query, column);

        let length_of_ins = ins.len();

        for (index, value) in ins.into_iter().enumerate() {
            if index + 1 == length_of_ins {
                self.query = format!("{}{})", self.query, value);
                    
                continue;
            }

            self.query = format!("{}{}, ", self.query, value);
        }

        self.list.push(KeywordList::NotIn);
        self
    }

    /// It adds the "IN" keyword with it's synthax and an empty condition, use it if you want to give more complex condition to "IN" keyword. Don't use ".where_cond()" with it.
    ///
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType};
    /// 
    /// fn main(){
    ///     let ins = vec![ValueType::Int16(1), ValueType::Int64(5), ValueType::Int32(10)];
    ///     let query = QueryBuilder::select(vec!["*"]).unwrap().table("users").where_in_custom("id", "1, 5, 10").finish();
    /// 
    ///     assert_eq!(query, "SELECT * FROM users WHERE id IN (1, 5, 10);")
    /// }
    pub fn where_in_custom(&mut self, column: &str, query: &str) -> &mut Self {
        self.query = format!("{} WHERE {} IN ({})", self.query, column, query);

        self.list.push(KeywordList::In);
        self
    }

    /// It adds the "NOT IN" keyword with it's synthax and an empty condition, use it if you want to give more complex condition to "NOT IN" keyword. Don't use ".where_cond()" with it.
    ///    
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType};
    /// 
    /// fn main(){
    ///     let ins = vec![ValueType::Int16(1), ValueType::Int64(5), ValueType::Int32(10)];
    ///     let query = QueryBuilder::select(vec!["*"]).unwrap().table("users").where_not_in_custom("id", "1, 5, 10").finish();
    /// 
    ///     assert_eq!(query, "SELECT * FROM users WHERE id NOT IN (1, 5, 10);")
    /// }
    pub fn where_not_in_custom(&mut self, column: &str, query: &str) -> &mut Self {
        self.query = format!("{} WHERE {} NOT IN ({})", self.query, column, query);

        self.list.push(KeywordList::NotIn);

        self
    }

    /// It adds the "OR" keyword with it's synthax. Warning: It's not ready yet to chaining "AND" and "OR" keywords, for now, applying that kind of complex query use ".append_custom()" method instead.
    ///
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType};
    /// 
    /// fn main(){
    ///     let ins = vec![ValueType::Int16(1), ValueType::Int64(5), ValueType::Int32(10)];
    ///     let query = QueryBuilder::select(vec!["*"]).unwrap().table("users").where_("id", "=", ValueType::Int32(10)).or("name", "=", ValueType::String("necdet".to_string())).finish();
    /// 
    ///     assert_eq!(query, "SELECT * FROM users WHERE id = 10 OR name = 'necdet';")
    /// }
   pub fn or(&mut self, column: &str, mark: &str, value: ValueType) -> &mut Self {
        match self.sanitize_column(column) {
            Ok(_) => (),
            Err(error) => panic!("{}", error)
        }

        match Self::sanitize_mark(mark) {
            Ok(_) => (),
            Err(error) => panic!("{}", error)
        }

        match self.sanitize_input(&value) {
            Ok(_) => (),
            Err(error) => panic!("{}", error)
        }

        self.query = format!("{} OR {} {} {}", self.query, column, mark, value);


        self.list.push(KeywordList::Or);

        self
    }

    /// It adds the "SET" keyword with it's synthax.
    /// 
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType};
    /// 
    /// fn main(){
    ///     let query = QueryBuilder::update().unwrap()
    ///                              .table("users")
    ///                              .set("name", ValueType::String("arda".to_string()))
    ///                              .where_("id", "=", ValueType::Int32(1))
    ///                              .finish();
    /// 
    ///     assert_eq!(query, "UPDATE users SET name = 'arda' WHERE id = 1;")
    /// }
    /// 
    /// ```
    pub fn set(&mut self, column: &str, value: ValueType) -> &mut Self {
        match self.hq {
            Some(_) => (),
            None => self.hq = Some(Self::load_hqs())
        }

        match self.sanitize_column(column) {
            Ok(_) => (),
            Err(error) => panic!("{}", error)
        }

        match self.sanitize_input(&value) {
            Ok(_) => (),
            Err(error) => panic!("{}", error)
        }

        match self.list.last() {
            Some(keyword) => {
                match keyword {
                    KeywordList::Set => self.query = format!("{}, {} = {}", self.query, column, value),
                    _ => self.query = format!("{} SET {} = {}", self.query, column, value)
                }
            },
            None => panic!("that's impossible to come here.")
        }

        self.list.push(KeywordList::Set);

        self
    }

    /// It adds the "AND" keyword with it's synthax. Warning: It's not ready yet to chaining "OR" and "AND" keywords, for now, applying that kind of complex query use ".append_custom()" method instead.
    ///
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType};
    /// 
    /// fn main(){
    ///     let ins = vec![ValueType::Int16(1), ValueType::Int64(5), ValueType::Int32(10)];
    ///     let query = QueryBuilder::select(vec!["*"]).unwrap().table("users").where_("id", "=", ValueType::Int32(10)).and("name", "=", ValueType::String("necdet".to_string())).finish();
    /// 
    ///     assert_eq!(query, "SELECT * FROM users WHERE id = 10 AND name = 'necdet';")
    /// }
    pub fn and(&mut self, column: &str, mark: &str, value: ValueType) -> &mut Self {
        match self.sanitize_column(column) {
            Ok(_) => (),
            Err(error) => panic!("{}", error)
        }

        match Self::sanitize_mark(mark) {
            Ok(_) => (),
            Err(error) => panic!("{}", error)
        }

        match self.sanitize_input(&value) {
            Ok(_) => (),
            Err(error) => panic!("{}", error)
        }

        self.query = format!("{} AND {} {} {}", self.query, column, mark, value);

        self.list.push(KeywordList::And);

        self
    }

    /// It adds the "OFFSET" keyword with it's synthax. Be careful about it's alignment with "LIMIT" keyword.
    ///     
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType};
    /// 
    /// fn main(){
    ///     let ins = vec![ValueType::Int16(1), ValueType::Int64(5), ValueType::Int32(10)];
    ///     let query = QueryBuilder::select(vec!["*"]).unwrap().table("users").where_("id", "=", ValueType::Int32(10)).limit(5).offset(0).finish();
    /// 
    ///     assert_eq!(query, "SELECT * FROM users WHERE id = 10 LIMIT 5 OFFSET 0;")
    /// }
    pub fn offset(&mut self, offset: i32) -> &mut Self {
        self.query = format!("{} OFFSET {}", self.query, offset);

        self.list.push(KeywordList::Offset);

        self
    }

    /// It adds the "LIMIT" keyword with it's synthax.
    /// 
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType};
    /// 
    /// fn main(){
    ///     let ins = vec![ValueType::Int16(1), ValueType::Int64(5), ValueType::Int32(10)];
    ///     let query = QueryBuilder::select(vec!["*"]).unwrap().table("users").where_("id", "=", ValueType::Int32(10)).limit(5).offset(0).finish();
    /// 
    ///     assert_eq!(query, "SELECT * FROM users WHERE id = 10 LIMIT 5 OFFSET 0;")
    /// }
    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.query = format!("{} LIMIT {}", self.query, limit);

        self.list.push(KeywordList::Limit);

        self
    }

    /// It adds the "LIKE" keyword with it's synthax.
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType};
    /// 
    /// fn main(){
    ///     let query = QueryBuilder::select(vec!["*"]).unwrap()
    ///                              .table("blogs")
    ///                              .like(vec!["description", "title"], "qubl is awesome!")
    ///                              .finish();
    /// 
    ///     assert_eq!(query, "SELECT * FROM blogs WHERE description LIKE '%qubl is awesome!%' OR title LIKE '%qubl is awesome!%';")
    /// }
    /// 
    /// // it has more niche and different usages, for them check the tests.
    pub fn like(&mut self, columns: Vec<&str>, operand: &str) -> &mut Self {
        match columns.len() {
            0 => panic!("you cannot pass an empty vector to the columns"),
            _ => ()
        }

        let hqs = match self.hq {
            Some(hqs) => hqs,
            None => {
                let load_hqs = Self::load_hqs();
                self.hq = Some(load_hqs);

                load_hqs
            }
        };

        match Self::sanitize_columns(&columns, hqs) {
            Ok(_) => {
                match self.sanitize_str(operand){
                    Ok(_) => (),
                    Err(error) => {
                        println!("That Error Occured in like method: {}", error);
                
                        self.list.push(KeywordList::Like);
        
                        return self
                    }
                }
        
                match self.list.last() {
                    Some(keyword) => {
                        if keyword == &KeywordList::Where || keyword == &KeywordList::In || keyword == &KeywordList::NotIn {
                            let length_of_columns = columns.len();
        
                            for (i, column) in columns.into_iter().enumerate() {
                                match length_of_columns {
                                    1 => {
                                        if i == 0 {
                                            self.query = format!("{} AND {} LIKE '%{}%'", self.query, column, operand)
                                        }  
                                    },
                                    _ => {
                                        if i == 0 {
                                            self.query = format!("{} AND ({} LIKE '%{}%'", self.query, column, operand)
                                        } else if i + 1 == length_of_columns {
                                            self.query = format!("{} OR {} LIKE '%{}%')", self.query, column, operand)
                                        } else {
                                            self.query = format!("{} OR {} LIKE '%{}%'", self.query, column, operand)
                                        }
                                    }
                                }
                            }
                        } else {
                            for (i, column) in columns.into_iter().enumerate() {
                                if i == 0 {
                                    self.query = format!("{} WHERE {} LIKE '%{}%'", self.query, column, operand);
                                } else {
                                    self.query = format!("{} OR {} LIKE '%{}%'", self.query, column, operand);
                                }
                            }
                        }
                    },
                    None => panic!("Our current implementation does not support to use '.like()' later not other than WHERE, IN or NOT IN queries.")
                }

                return self
            },
            Err(error) => panic!("That error occured in '.like()' method: {}", error)
        }
    }

    /// It adds the "ORDER BY" keyword with it's synthax. It only accepts "ASC", "DESC", "asc", "desc" values.
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType};
    /// 
    /// fn main(){
    ///     let query = QueryBuilder::select(vec!["*"]).unwrap()
    ///                              .table("users")
    ///                              .where_("age", ">", ValueType::Int32(25))
    ///                              .order_by("id", "ASC")
    ///                              .limit(5)
    ///                              .offset(0)
    ///                              .finish();
    /// 
    ///     assert_eq!(query, "SELECT * FROM users WHERE age > 25 ORDER BY id ASC LIMIT 5 OFFSET 0;")
    /// }
    pub fn order_by(&mut self, column: &str, mut ordering: &str) -> &mut Self {
        match self.sanitize_column(column) {
            Ok(_) => (),
            Err(error) => {
                println!("{}", error);

                self.list.push(KeywordList::OrderBy);

                return self
            }
        }

        match ordering {
            "asc" => ordering = "ASC",
            "desc" => ordering = "DESC",
            "ASC" => ordering = "ASC",
            "DESC" => ordering = "DESC",
            &_ => panic!("Panicking in order_by method: There is no other ordering options than ASC or DESC.")
        }

        match self.list.last() {
            Some(keyword) => match keyword {
                KeywordList::OrderBy | KeywordList::Field => self.query = format!("{}, {} {}", self.query, column, ordering),
                _ => self.query = format!("{} ORDER BY {} {}", self.query, column, ordering)
            },
            None => panic!("It's almost impossible you to come here.")
        }

        self.list.push(KeywordList::OrderBy);

        self
    }

    /// A practical method that adds a query for shuffling the lines.
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType};
    /// 
    /// fn main(){
    ///     let query = QueryBuilder::select(vec!["*"]).unwrap()
    ///                              .table("users")
    ///                              .where_("age", ">", ValueType::Int32(25))
    ///                              .order_random()
    ///                              .limit(5)
    ///                              .offset(0)
    ///                              .finish();
    /// 
    ///     assert_eq!(query, "SELECT * FROM users WHERE age > 25 ORDER BY RAND() LIMIT 5 OFFSET 0;")
    /// }
    pub fn order_random(&mut self) -> &mut Self {
        if self.query.contains("ORDER BY") {
            panic!("Error in order_random method: you cannot add ordering option twice on a query.");
        }

        self.query = format!("{} ORDER BY RAND()", self.query);
        self.list.push(KeywordList::OrderBy);

        self
    }

    /// Adds "FIELD()" function with it's synthax. It's used on ordering depending on strings.
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType};
    /// 
    /// fn main(){
    ///     let query = QueryBuilder::select(vec!["*"]).unwrap()
    ///                              .table("users")
    ///                              .where_("age", ">", ValueType::Int32(25))
    ///                              .order_by_field("role", vec!["admin", "member", "observer"])
    ///                              .limit(5)
    ///                              .offset(0)
    ///                              .finish();
    /// 
    ///     assert_eq!(query, "SELECT * FROM users WHERE age > 25 ORDER BY FIELD(role, 'admin', 'member', 'observer') LIMIT 5 OFFSET 0;")
    /// }
    pub fn order_by_field(&mut self, column: &str, ordering: Vec<&str>) -> &mut Self {
        match ordering.len() {
            0 => panic!("you cannot pass an empty vector to the ordering argument"),
            _ => ()
        }

        match self.list.last() {
            Some(keyword) => match keyword {
                KeywordList::OrderBy => {
                    let mut split_the_query = self.query.split(" ORDER BY ");
                
                    self.query = format!("{} ORDER BY {}, FIELD({}", split_the_query.nth(0).unwrap(), split_the_query.nth(0).unwrap(), column);

                    for item in ordering {
                        self.query = format!("{}, '{}'", self.query, item)
                    }

                    self.query = format!("{})", self.query);
                },
                KeywordList::Field => {
                    self.query = format!("{}, FIELD({}", self.query, column);

                    for item in ordering {
                        self.query = format!("{}, '{}'", self.query, item)
                    }

                    self.query = format!("{})", self.query);
                },
                _ => {
                    let mut new_part_of_query = format!("ORDER BY FIELD({}", column);

                    for item in ordering {
                        new_part_of_query = format!("{}, '{}'", new_part_of_query, item)
                    }

                    self.query = format!("{} {})", self.query, new_part_of_query);
                }
            },
            None => panic!("It's almost impossible you to come here.")
        }

        self.list.push(KeywordList::Field);

        self
    }

    /// It adds the "GROUP BY" keyword with it's Synthax.
    pub fn group_by(&mut self, column: &str) -> &mut Self {
        self.query = format!("{} GROUP BY {}", self.query, column);

        self.list.push(KeywordList::GroupBy);

        self
    }

    pub fn having(&mut self, column: &str, mark: &str, value: ValueType) -> &mut Self {
        match self.sanitize_column(column) {
            Ok(_) => (),
            Err(error) => panic!("{}", error)
        }

        match Self::sanitize_mark(mark) {
            Ok(_) => (),
            Err(error) => panic!("{}", error)
        }

        match self.sanitize_input(&value) {
            Ok(_) => (),
            Err(error) => panic!("{}", error)
        }

        self.query = format!("{} HAVING {} {} {}", self.query, column, mark, value);

        self.list.push(KeywordList::Having);

        self
    }


    /// it adds the `UNION` keyword and its synthax. You can pass multiple queries to union with:
    /// 
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType};
    /// 
    /// fn main(){
    ///     let mut union_1 = QueryBuilder::select(vec!["name", "age", "id"]).unwrap();
    ///     union_1.table("users").where_("age", ">", ValueType::Int32(7));
    ///
    ///     let union_2 = QueryBuilder::select(vec!["name", "age", "id"]).unwrap()
    ///                                .table("users")
    ///                                .where_("age", "<", ValueType::Int32(15))
    ///                                .union(vec![union_1])
    ///                                .finish();
    ///
    ///     assert_eq!(union_2, "(SELECT name, age, id FROM users WHERE age < 15) UNION (SELECT name, age, id FROM users WHERE age > 7);");
    /// }
    /// 
    /// ```
    pub fn union(&mut self, others: Vec<QueryBuilder<'_>>) -> &mut Self {
        match self.list.last() {
            Some(keyword) => {
                match keyword {
                    KeywordList::Union | KeywordList::UnionAll => {
                        for other in others {
                            self.query = format!("{} UNION ({})", self.query, other.query)
                        }
                    },
                    _ => {
                        self.query = format!("({})", self.query);
                        
                        for other in others {
                            self.query = format!("{} UNION ({})", self.query, other.query)
                        }
                    }
                }
            },
            None => panic!("it's impossible to came here!")
        }

        self.list.push(KeywordList::Union);

        self
    }


    /// it adds the `UNION` keyword and its synthax. You can pass multiple queries to union with:
    /// 
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType};
    /// 
    /// fn main(){
    ///     let mut union_1 = QueryBuilder::select(vec!["id", "title", "description", "published"]).unwrap();
    ///     union_1.table("blogs").like(vec!["title"], "text");
    ///
    ///     let mut union_2 = QueryBuilder::select(vec!["id", "title", "description", "published"]).unwrap();
    ///     union_2.table("blogs").like(vec!["description"], "some text");
    ///
    ///     let union_3 = QueryBuilder::select(vec!["id", "title", "description", "published"]).unwrap()
    ///                                .table("blogs")
    ///                                .where_("published", "=", ValueType::Boolean(true))
    ///                                .union_all(vec![union_1, union_2])
    ///                                .finish();
    ///
    ///     assert_eq!(union_3, "(SELECT id, title, description, published FROM blogs WHERE published = true) UNION ALL (SELECT id, title, description, published FROM blogs WHERE title LIKE '%text%') UNION ALL (SELECT id, title, description, published FROM blogs WHERE description LIKE '%some text%');");
    /// }
    /// 
    /// ```
    /// 
    pub fn union_all(&mut self, others: Vec<QueryBuilder<'_>>) -> &mut Self {
        match self.list.last() {
            Some(keyword) => {
                match keyword {
                    KeywordList::Union | KeywordList::UnionAll => {
                        for other in others {
                            self.query = format!("{} UNION ALL ({})", self.query, other.query)
                        }
                    },
                    _ => {
                        self.query = format!("({})", self.query);
                        
                        for other in others {
                            self.query = format!("{} UNION ALL ({})", self.query, other.query)
                        }
                    }
                }
            },
            None => panic!("it's impossible to came here!")
        }

        self.list.push(KeywordList::UnionAll);

        self
    }

    /// A wildcard method that gives you the chance to write a part of your query. Warning, it does not add any keyword to builder, i'll encourage to add proper keyword to it with `.append_keyword()` method for your custom query, otherwise you should continue building your query by yourself with that function, or you've to be prepared to encounter bugs.  
    /// 
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType};
    /// 
    /// fn main(){
    ///     let query = QueryBuilder::select(vec!["*"]).unwrap()
    ///                              .table("users")
    ///                              .append_custom("WHERE age > 25 ORDER BY FIELD(role, 'admin', 'member', 'observer') LIMIT 5 OFFSET 0")
    ///                              .finish();
    /// 
    ///     assert_eq!(query, "SELECT * FROM users WHERE age > 25 ORDER BY FIELD(role, 'admin', 'member', 'observer') LIMIT 5 OFFSET 0;")
    /// }
    /// ```
    /// 
    pub fn append_custom(&mut self, query: &str) -> &mut Self {
        self.query = format!("{} {}", self.query, query);

        self
    }

    /// A wildcard method that benefits you to append a keyword to the keyword list, so the QueryBuilder can build your queries properly, later than you appended your custom string to your query. It should be used with `.append_custom()` method. 
    /// 
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType, KeywordList};
    /// 
    /// fn main(){
    ///     let query = QueryBuilder::select(vec!["*"]).unwrap()
    ///                              .table("users")
    ///                              .append_custom("WHERE age > 25 ORDER BY FIELD(role, 'admin', 'member', 'observer') LIMIT 5 OFFSET 0")
    ///                              .append_keyword(KeywordList::Where)
    ///                              .append_keyword(KeywordList::Field)
    ///                              .append_keyword(KeywordList::Limit)
    ///                              .append_keyword(KeywordList::Offset)
    ///                              .finish();
    /// 
    ///     assert_eq!(query, "SELECT * FROM users WHERE age > 25 ORDER BY FIELD(role, 'admin', 'member', 'observer') LIMIT 5 OFFSET 0;")
    /// }
    /// ```
    /// 
    pub fn append_keyword(&mut self, keyword: KeywordList) -> &mut Self {
        self.list.push(keyword);

        self
    }
    
    /// It applies "JSON_EXTRACT()" mysql function with it's Synthax. If you encounter any syntactic bugs or deficiencies about that function, please report it via opening an issue.
    /// 
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType};
    /// 
    /// fn main(){
    ///     let query = QueryBuilder::select(["*"].to_vec()).unwrap()
    ///                              .json_extract("articles", "[0]", Some("blog1"))
    ///                              .json_extract("articles", "[1]", Some("blog2"))
    ///                              .json_extract("articles", "[2]", Some("blog3"))
    ///                              .table("users")
    ///                              .where_("published", "=", ValueType::Int32(1))
    ///                              .finish();
    /// 
    ///     assert_eq!(query, "SELECT JSON_EXTRACT(articles, '$[0]') AS blog1, JSON_EXTRACT(articles, '$[1]') AS blog2, JSON_EXTRACT(articles, '$[2]') AS blog3 FROM users WHERE published = 1;")
    /// }
    /// 
    /// ```
    pub fn json_extract(&mut self, haystack: &str, needle: &str, _as: Option<&str>) -> &mut Self {
        match self.list.last() {
            Some(keyword) => {
                match keyword {
                    KeywordList::Where => {
                        if _as.is_some() {
                            println!("Warning: You've gave _as value to some variant and used it later than 'WHERE' keyword on .json_extract() method. In that usage, that value has no effect, you should gave it none value.");
                        }

                        match self.table.as_str() == haystack {
                            true => {
                                let mut split_the_query = self.query.split(haystack);
                                let string_for_replace = format!("JSON_EXTRACT({}, '${}')", haystack, needle);

                                self.query = format!("SELECT{}{}{}", self.table, string_for_replace, split_the_query.nth(2).unwrap()) 
                            },
                            false => {
                                let string_for_replace = format!("JSON_EXTRACT({}, '${}')", haystack, needle);

                                self.query = self.query.replace(haystack,&string_for_replace)
                            }
                        }
                    },
                    KeywordList::And => {
                        if _as.is_some() {
                            println!("Warning: You've gave _as value to some variant and used it later than 'AND' keyword on .json_extract() method. In that usage, that value has no effect, you should gave it none value.");
                        }

                        let query_to_comp = format!("AND {}", haystack);

                        match self.table.as_str() == haystack {
                            true => {
                                let mut split_the_query = self.query.split(&query_to_comp);
                                let string_for_replace = format!("JSON_EXTRACT({}, '${}')", haystack, needle);

                                self.query = format!("{}AND {}{}", split_the_query.nth(0).unwrap(), string_for_replace, split_the_query.nth(0).unwrap()) 
                            },
                            false => {
                                match self.query.matches(&query_to_comp).count() {
                                    0 => (),
                                    1 => {
                                        let string_for_replace = format!("AND JSON_EXTRACT({}, '${}')", haystack, needle);

                                        self.query = self.query.replace(&query_to_comp,&string_for_replace)
                                    }
                                    _ => {
                                        let split_the_query = self.query.split(&query_to_comp).collect::<Vec<&str>>();

                                        let mut last_chunk = "".to_string();
                                        let mut new_chunk = "".to_string();
                                        let length_of_split = split_the_query.len();
                                        
                                        for (index, chunk) in split_the_query.into_iter().enumerate() {
                                            if index + 1 == length_of_split {
                                                last_chunk = chunk.to_string()
                                            } else if index == 0 {
                                                new_chunk = format!("{}", chunk);
                                            } else {
                                                new_chunk = format!("{}{}{}", new_chunk, query_to_comp, chunk)
                                            }
                                        }

                                        let string_for_replace = format!("AND JSON_EXTRACT({}, '${}')", haystack, needle);

                                        self.query = format!("{} {} {}", new_chunk, string_for_replace, last_chunk)
                                    }
                                }
                            }
                        }
                    },
                    KeywordList::Or => {
                        if _as.is_some() {
                            println!("Warning: You've gave _as value to some variant and used it later than 'OR' keyword on .json_extract() method. In that usage, that value has no effect, you should gave it none value.");
                        }

                        let query_to_comp = format!("OR {}", haystack);

                        match self.table.as_str() == haystack {
                            true => {
                                let mut split_the_query = self.query.split(&query_to_comp);
                                let string_for_replace = format!("JSON_EXTRACT({}, '${}')", haystack, needle);

                                self.query = format!("{}OR {}{}", split_the_query.nth(0).unwrap(), string_for_replace, split_the_query.nth(0).unwrap()) 
                            },
                            false => {
                                match self.query.matches(&query_to_comp).count() {
                                    0 => (),
                                    1 => {
                                        let string_for_replace = format!("OR JSON_EXTRACT({}, '${}')", haystack, needle);

                                        self.query = self.query.replace(&query_to_comp,&string_for_replace)
                                    }
                                    _ => {
                                        let split_the_query = self.query.split(&query_to_comp).collect::<Vec<&str>>();

                                        let mut last_chunk = "".to_string();
                                        let mut new_chunk = "".to_string();
                                        let length_of_split = split_the_query.len();
                                        
                                        for (index, chunk) in split_the_query.into_iter().enumerate() {
                                            if index + 1 == length_of_split {
                                                last_chunk = chunk.to_string()
                                            } else if index == 0 {
                                                new_chunk = format!("{}", chunk);
                                            } else {
                                                new_chunk = format!("{}{}{}", new_chunk, query_to_comp, chunk)
                                            }
                                        }

                                        let string_for_replace = format!("OR JSON_EXTRACT({}, '${}')", haystack, needle);

                                        self.query = format!("{} {} {}", new_chunk, string_for_replace, last_chunk)
                                    }
                                }
                            }
                        }
                    },
                    KeywordList::Select => {
                        let string_for_put = format!("JSON_EXTRACT({}, '${}')", haystack, needle);

                        match _as {
                            Some(_as) => self.query = format!("SELECT {} AS {} FROM", string_for_put, _as),
                            None => self.query = format!("SELECT {} FROM", string_for_put),
                        }
                    },
                    KeywordList::Table => {
                        let string_for_put = format!("JSON_EXTRACT({}, '${}')", haystack, needle);

                        match _as {
                            Some(_as) => self.query = format!("SELECT {} AS {} FROM {}", string_for_put, _as, self.table),
                            None => self.query = format!("SELECT {} FROM {}", string_for_put, self.table),
                        }
                    },
                    KeywordList::OrderBy => {
                        if _as.is_some() {
                            println!("Warning: You've gave _as value to some variant and used it later than 'ORDER BY' operator on .json_extract() method. In that usage, that value has no effect, you should gave it none value.");
                        }

                        match self.query.matches(" ORDER BY ").count() {
                            0 => (),
                            1 => {
                                let split_the_query = self.query.clone();
                                let mut split_the_query = split_the_query.split(" ORDER BY ");

                                let string_for_put = format!("ORDER BY JSON_EXTRACT({}, '${}')", haystack, needle);
        
                                match _as {
                                    Some(_as) => self.query = format!("{} {} AS {}", split_the_query.nth(0).unwrap(), string_for_put, _as),
                                    None => self.query = format!("{} {}", split_the_query.nth(0).unwrap(), string_for_put)
                                }

                                match split_the_query.nth(0) {
                                    Some(comparison) => {
                                        match comparison.ends_with("ASC") || comparison.ends_with("asc") {
                                            true => self.query = format!("{} ASC", self.query),
                                            false => match comparison.ends_with("DESC") || comparison.ends_with("desc") {
                                                true => self.query = format!("{} DESC", self.query),
                                                false => ()
                                            }
                                        }
                                    },
                                    None => ()
                                }
                            },
                            _ => ()
                        }
                    },
                    KeywordList::Count => {
                        let mut split_the_query = self.query.split(" COUNT");

                        let string_for_put = match _as {
                            Some(_as) => format!("JSON_EXTRACT({}, '${}') AS {}", haystack, needle, _as),
                            None => format!("JSON_EXTRACT({}, '${}')", haystack, needle)
                        };

                        self.query = format!("SELECT {}, COUNT{}", string_for_put, split_the_query.nth(1).unwrap())
                    },
                    KeywordList::JsonExtract => {
                        let mut split_the_query = self.query.split(" FROM");

                        match _as {
                            Some(_as) => self.query = format!("{}, JSON_EXTRACT({}, '${}') AS {} FROM", split_the_query.nth(0).unwrap(), haystack, needle, _as),
                            None => panic!("If you want to chain .json_extract() methods, you have to give them a tag.")
                        }
                    }
                    _ => ()
                }
            },
            None => ()
        }
        
        self.list.push(KeywordList::JsonExtract);
        self
    }

    /// It applies "JSON_CONTAINS()" mysql function with it's Synthax. If you encounter any syntactic bugs or deficiencies about that function, please report it via opening an issue.
    /// 
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType};
    /// 
    /// fn main(){
    ///     let query = QueryBuilder::select(["*"].to_vec()).unwrap()
    ///                              .table("users")
    ///                              .where_("pic", "=", ValueType::String("".to_string()))
    ///                              .json_contains("pic", ValueType::String("{{ \"name\": \"blablabla.jpg\"}}".to_string()), Some(".name"))
    ///                              .finish();
    /// 
    ///     assert_eq!(query, "SELECT * FROM users WHERE JSON_CONTAINS(pic, '{{ \"name\": \"blablabla.jpg\"}}', '$.name');")
    /// }
    /// 
    /// ```
    pub fn json_contains(&mut self, column: &str, needle: ValueType, path: Option<&str>) -> &mut Self {
        match self.list.last().unwrap() {
            KeywordList::Select => match path {
                Some(path) => self.query = format!("SELECT JSON_CONTAINS({}, {}, '${}') FROM", column, needle, path),
                None => self.query = format!("SELECT JSON_CONTAINS({}, {}) FROM", column, needle)
            },
            KeywordList::Where => match path {
                Some(path) => {
                    let mut split_the_query = self.query.split(" WHERE ");

                    let first_half = split_the_query.nth(0);

                    self.query = format!("{} WHERE JSON_CONTAINS({}, {}, '${}')", first_half.unwrap(), column, needle, path);
                },
                None => {
                    let mut split_the_query = self.query.split(" WHERE ");

                    let first_half = split_the_query.nth(0);

                    self.query = format!("{} WHERE JSON_CONTAINS({}, {})", first_half.unwrap(), column, needle);
                }
            },
            KeywordList::And => match path {
                Some(path) => {
                    let split_the_query = self.query.split(" AND ").collect::<Vec<&str>>();

                    let length_of_the_split_the_query = split_the_query.len();

                    match split_the_query.len() {
                        0 => panic!("There Is No AND query in QueryBuilder but The AND keyword exist in keyword list, panicking."),
                        1 => panic!("There Is No AND query in QueryBuilder but The AND keyword exist in keyword list, panicking."),
                        2 => self.query = format!("{} AND JSON_CONTAINS({}, {}, '${}')", split_the_query[0], column, needle, path),
                        _ => {
                            let mut concatenated_string = String::new();

                            for (index, chunk) in  split_the_query.into_iter().enumerate() {
                                if index == 0 {
                                    concatenated_string = chunk.to_string();
                                } else if index + 1 != length_of_the_split_the_query {
                                    concatenated_string = format!("{} AND {} ", concatenated_string, chunk)
                                }
                            }

                            self.query = format!("{}AND JSON_CONTAINS({}, {}, '${}')", concatenated_string, column, needle, path)
                        }
                    }
                },
                None => {
                    let split_the_query = self.query.split(" AND ").collect::<Vec<&str>>();

                    let length_of_the_split_the_query = split_the_query.len();

                    match split_the_query.len() {
                        0 => panic!("There Is No AND query in QueryBuilder but The AND keyword exist in keyword list, panicking."),
                        1 => panic!("There Is No AND query in QueryBuilder but The AND keyword exist in keyword list, panicking."),
                        2 => self.query = format!("{} AND JSON_CONTAINS({}, {})",  split_the_query[0], column, needle),
                        _ => {
                            let mut concatenated_string = String::new();

                            for (index, chunk) in  split_the_query.into_iter().enumerate() {
                                if index == 0 {
                                    concatenated_string = chunk.to_string();
                                } else if index + 1 != length_of_the_split_the_query {
                                    concatenated_string = format!("{} AND {} ", concatenated_string, chunk)
                                }
                            }

                            self.query = format!("{}AND JSON_CONTAINS({}, {})", concatenated_string, column, needle);
                        }
                    }
                }
            },
            KeywordList::Or => match path {
                Some(path) => {
                    let split_the_query = self.query.split(" OR ").collect::<Vec<&str>>();

                    let length_of_the_split_the_query = split_the_query.len();

                    match split_the_query.len() {
                        0 => panic!("There Is No OR query in QueryBuilder but The OR keyword exist in keyword list, panicking."),
                        1 => panic!("There Is No OR query in QueryBuilder but The OR keyword exist in keyword list, panicking."),
                        2 => self.query = format!("{} OR JSON_CONTAINS({}, {}, '${}')", split_the_query[0], column, needle, path),
                        _ => {
                            let mut concatenated_string = String::new();

                            for (index, chunk) in  split_the_query.into_iter().enumerate() {
                                if index == 0 {
                                    concatenated_string = chunk.to_string();
                                } else if index + 1 != length_of_the_split_the_query {
                                    concatenated_string = format!("{} OR {} ", concatenated_string, chunk)
                                }
                            }

                            self.query = format!("{}OR JSON_CONTAINS({}, {}, '${}')", concatenated_string, column, needle, path);
                        }
                    }
                },
                None => {
                    let split_the_query = self.query.split(" OR ").collect::<Vec<&str>>();

                    let length_of_the_split_the_query = split_the_query.len();

                    match split_the_query.len() {
                        0 => panic!("There Is No OR query in QueryBuilder but The OR keyword exist in keyword list, panicking."),
                        1 => panic!("There Is No OR query in QueryBuilder but The OR keyword exist in keyword list, panicking."),
                        2 => self.query = format!("{} OR JSON_CONTAINS({}, {})",  split_the_query[0], column, needle),
                        _ => {
                            let mut concatenated_string = String::new();

                            for (index, chunk) in  split_the_query.into_iter().enumerate() {
                                if index == 0 {
                                    concatenated_string = chunk.to_string();
                                } else if index + 1 != length_of_the_split_the_query {
                                    concatenated_string = format!("{} OR {} ", concatenated_string, chunk)
                                }
                            }

                            self.query = format!("{}OR JSON_CONTAINS({}, {})", concatenated_string, column, needle);
                        }
                    }
                }
            },
            _ => panic!("Wrong usage of '.json_contains()' method, it should be used later than either SELECT, WHERE, AND, OR keywords.")
        }

        self.list.push(KeywordList::JsonContains);

        self
    }

    /// finishes the query and returns the result as string.
    pub fn finish(&self) -> String {
        return format!("{};", self.query);
    }

    /// gives you an immutable copy of that instance, just for case if you need to share and potentially mutate it across threads.
    pub fn copy(&mut self) -> Self {
        Self {
            query: self.query.clone(),
            table: self.table.clone(),
            qtype: self.qtype.clone(),
            list: self.list.clone(),
            hq: self.hq
        }
    }

    fn load_hqs() -> [&'a str; 26] {
        [";", "; drop", "admin' #", "admin'/*", "; union", "or 1 = 1",
        "or 1 = 1#", "or 1 = 1/*", "or true = true", "or false = false", "or '1' = '1'", "or '1' = '1'#",
        "or '1' = '1'/*", "; sleep(", "--", "drop table", "drop schema", "select if", "union select",
        "union all", "exec", "master..", "masters..", "information_schema", "load_file", "alter user"]
    }

    fn sanitize_column(&mut self, column: &str)  -> std::result::Result<(), std::io::Error>  {
        match self.hq {
            Some(hqs) => {
                for _hq in hqs.iter() {
                    if &column == _hq {
                        return Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "You cannot run arbitrary queries"))
                    }
                }
            },
            None => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Your input is invalid."))
        }

        Ok(())
    }

    /// checks the inputs for potential sql injection patterns and throws error if they exist.
    fn sanitize_columns(columns: &Vec<&str>, hqs: [&'a str; 26]) -> std::result::Result<(), std::io::Error> {
        if columns.len() == 1 && columns[0] == "" {
            return Ok(());
        };

        for column in columns.iter() {
            for hq in hqs.iter() {
                if column == hq {
                    return Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "You cannot run arbitrary queries"))
                }
            }
        }

        return Ok(())
    }

    fn sanitize_inputs(inputs: &Vec<ValueType>, hqs: [&'a str; 26]) -> std::result::Result<(), std::io::Error> {
        for input in inputs.iter() {
            match input {
                ValueType::String(string) | ValueType::Datetime(string) => {
                    for hq in hqs.iter() {
                        if &string == hq {
                            return Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "You cannot run arbitrary queries"))
                        }
                    }
                },
                _ => continue
            }
        }

        return Ok(())
    }

    fn sanitize_input(&mut self, input: &ValueType) -> std::result::Result<(), std::io::Error> {
        match input {
            ValueType::String(string) | ValueType::Datetime(string) => {
                match self.hq {
                    Some(hqs) => {
                        for hq in hqs.iter() {
                            if &string == hq {
                                return Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "You cannot run arbitrary queries"))
                            }
                        }
                    },
                    None => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Your input is invalid."))
                }
            },
            _ => return Ok(())
        };

        Ok(())
    }

    fn sanitize_mark(input: &str) -> std::result::Result<(), std::io::Error> {
        return match input {
            "=" | "<" | ">" | "<=" | ">=" | "!=" | "<>" => Ok(()),
            _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "comparison operators cannot be other than =, <, >, <=,  >=, != or <>."))
        }
    }

    fn sanitize_str(&mut self, input: &str) -> std::result::Result<(), std::io::Error> {
        match self.hq {
            Some(hqs) => {
                for hq in hqs.iter() {
                    if *hq == input {
                        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "comparison operators cannot be other than =, <, >, <=,  >=, != or <>."))
                    }
                }
            },
            None => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "comparison operators cannot be other than =, <, >, <=,  >=, != or <>."))
        }

        Ok(())
    }
}

/// Struct that benefits you to create and use schema's.
#[derive(Debug, Clone)]
pub struct SchemaBuilder {
    pub query: String,
    pub schema: String,
    pub list: Vec<KeywordList>
}

/// implementations fon SchemaBuilder
impl SchemaBuilder {
    pub fn create(name: &str) -> std::result::Result<Self, std::io::Error> {
        if name.contains("!") ||
           name.contains("-") ||
           name.contains("=") ||
           name.contains("+") ||
           name.contains("%") ||
           name.contains("$") ||
           name.contains("&") ||
           name.contains("#") ||
           name.contains("[") ||
           name.contains("]") ||
           name.contains("{") ||
           name.contains("}") ||
           name.contains(":") ||
           name.contains(";") ||
           name.contains("'") ||
           name.contains("\"") ||
           name.contains(",") ||
           name.contains(".") {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Schame name cannot include these characters: '!', '-', '=', '+', '%', '$', '&', '#', '[', ']', '{', '}', ':', ';', '\"', \"'\", '.', ','"))
        }

        Ok(Self {
            query: format!("CREATE DATABASE {}", name),
            schema: name.to_string(),
            list: vec![KeywordList::Create]
        })
    }

    pub fn use_another_schema(name: &str) -> std::result::Result<Self, std::io::Error> {
        if name.contains("!") ||
        name.contains("-") ||
        name.contains("=") ||
        name.contains("+") ||
        name.contains("%") ||
        name.contains("$") ||
        name.contains("&") ||
        name.contains("#") ||
        name.contains("[") ||
        name.contains("]") ||
        name.contains("{") ||
        name.contains("}") ||
        name.contains(":") ||
        name.contains(";") ||
        name.contains("'") ||
        name.contains("\"") ||
        name.contains(",") ||
        name.contains(".") {
             return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Schame name cannot include these characters: '!', '-', '=', '+', '%', '$', '&', '#', '[', ']', '{', '}', ':', ';', '\"', \"'\", '.', ','"))
        }

        Ok(Self {
            query: format!("USE {}", name),
            schema: name.to_string(),
            list: vec![KeywordList::Use, KeywordList::Create]
        })
    }

    pub fn if_not_exists(&mut self) -> &mut Self {
        match self.list[0] {
            KeywordList::Create => (),
            KeywordList::Table => (),
            _ => panic!("if_not_exists method cannot be used without Create or Table queries")
        }

        let split_the_query =  self.query.split(" DATABASE ").collect::<Vec<&str>>();
        self.query = format!("{} DATABASE IF NOT EXISTS {}", split_the_query[0], split_the_query[1]);

        self.list.insert(0, KeywordList::IfNotExist);
        self
    }

    pub fn use_schema(&mut self, name: Option<&str>) -> &mut Self {
        match name {
            Some(schema_name) => {
                self.query = format!("USE {}", schema_name)
            },
            None => {
                self.query = format!("USE {}", self.schema);
            }
        }

        self
    }

    pub fn finish(&self) -> String {
        return format!("{};", self.query)
    }
}

/// Struct that benefits you to create Tables. Currently incomplete thoug.
#[derive(Debug, Clone)]
pub struct TableBuilder {
    pub query: String,
    pub name: String,
    pub schema: String,
    pub all: Vec<String>,
}

/// Struct that benefits to define a foreign key.
#[derive(Debug, Clone)]
pub struct ForeignKey {
    pub first: ForeignKeyItem,
    pub second: ForeignKeyItem,
    pub on_delete: Option<ForeignKeyActions>,
    pub on_update: Option<ForeignKeyActions>,
    pub constraint: Option<String>
}

/// Struct that benefits you to add a foreign key item to a foreign key.
#[derive(Debug, Clone)]
pub struct ForeignKeyItem {
    pub table: String,
    pub column: String
}

/// implementations for TableBuilder
impl TableBuilder {
    pub fn create(schema_name: &str, table_name: &str) -> Self {
        return Self {
            query: format!("CREATE TABLE {} (", table_name),
            schema: schema_name.to_string(),
            name: table_name.to_string(),
            all: vec![]
        }
    }

    pub fn if_not_exists(&mut self) -> &mut Self {
        self.query = format!("{}IF NOT EXISTS (", self.query.replace("(", ""));

        self
    }

    pub fn add_column(&mut self, column_name: &str) -> &mut Self {
        if self.query.ends_with("(") {
            self.query = format!("{}{}", self.query, column_name)
        } else {
            self.query = format!("{}, {}", self.query, column_name)
        }

        self
    }

    pub fn col_type(&mut self, type_name: &str) -> &mut Self {
        if self.query.ends_with("(") {
            panic!("Cannot add type before defining a column name.")
        }

        self.query = format!("{} {}", self.query, type_name);

        self
    }

    pub fn null(&mut self) -> &mut Self {
        self.query = format!("{} NULL", self.query);

        self
    }

    pub fn not_null(&mut self) -> &mut Self {
        self.query = format!("{} NOT NULL", self.query);

        self
    }

    pub fn auto_increment(&mut self) -> &mut Self {
        self.query = format!("{} AUTO_INCREMENT", self.query);

        self
    }

    pub fn primary_key(&mut self) -> &mut Self {
        if self.query.contains("PRIMARY KEY") {
            panic!("A table cannot have two primary keys.")
        }

        self.query = format!("{} PRIMARY KEY", self.query);

        self
    }

    pub fn default(&mut self, value: ValueType) -> &mut Self {
        let split_the_query = self.query.clone();
        let split_the_query = split_the_query.split(", ").collect::<Vec<&str>>();

        let last_query = split_the_query[split_the_query.len() - 1];

        if last_query.contains("INT") || 
           last_query.contains("TINYINT") ||
           last_query.contains("SMALLINT") ||
           last_query.contains("MEDIUMINT") ||
           last_query.contains("BIGINT") ||
           last_query.contains("BIT") ||
           last_query.contains("SERIAL") {
            match value {
                ValueType::Int8(int) => self.query = format!("{} DEFAULT {}", self.query, int),
                ValueType::Int16(int) => self.query = format!("{} DEFAULT {}", self.query, int),
                ValueType::Int32(int) => self.query = format!("{} DEFAULT {}", self.query, int),
                ValueType::Int64(int) => self.query = format!("{} DEFAULT {}", self.query, int),
                ValueType::Uint8(int) => self.query = format!("{} DEFAULT {}", self.query, int),
                ValueType::Uint16(int) => self.query = format!("{} DEFAULT {}", self.query, int),
                ValueType::Uint32(int) => self.query = format!("{} DEFAULT {}", self.query, int),
                ValueType::Uint64(int) => self.query = format!("{} DEFAULT {}", self.query, int),
                ValueType::Usize(int) => self.query = format!("{} DEFAULT {}", self.query, int),
                ValueType::Float32(int) => self.query = format!("{} DEFAULT {}", self.query, int),
                ValueType::Float64(int) => self.query = format!("{} DEFAULT {}", self.query, int),
                _ => panic!("Error: If your column has the of the types of INT, TINYINT, SMALLINT, MEDIUMINT, BIGINT, BIT or SERIAL, it has to be an i8, i16, i32, i64, i128, usize, u8,  u16, u32, u64.")
            }
        }

        if last_query.contains("BOOL") || 
           last_query.contains("BOOLEAN") {
            match value {
                ValueType::Boolean(boolean) => self.query = format!("{} DEFAULT {}", self.query, boolean),
                _ => panic!("If your column type is BOOLEAN, you have to write either true or false.")
            }    
        }

        if last_query.contains("CHAR") ||
           last_query.contains("VARCHAR") ||
           last_query.contains("TEXT") ||
           last_query.contains("TINYTEXT") ||
           last_query.contains("MEDIUMTEXT") ||
           last_query.contains("LONGTEXT") ||
           last_query.contains("BINARY") ||
           last_query.contains("VARBINARY") {
            match value {
                ValueType::String(ref text) => self.query = format!("{} DEFAULT '{}'", self.query, text),
                _ => panic!("Error: if your column type is one of the types of CHAR, VARCHAR, TEXT, TINYTEXT, MEDIUMTEXT, LONGTEXT, BINARY or VARBINARY, your value type has to be String.")
            }
        }

        if last_query.contains("DATETIME") ||
           last_query.contains("TIMESTAMP") {
            match value {
                ValueType::Datetime(datetime) => self.query = format!("{} DEFAULT {}", self.query, datetime),
                _ => panic!("Error: if your column type is one of the types of CHAR, VARCHAR, TEXT, TINYTEXT, MEDIUMTEXT, LONGTEXT, BINARY or VARBINARY, your value type has to be String.")
            }
        }

        self
    }

    pub fn unique(&mut self) -> &mut Self {
        self.query = format!("{} UNIQUE", self.query);

        self
    }

    pub fn check(&mut self, condition: &str) -> &mut Self {
        self.query = format!("{} CHECK({})", self.query, condition);

        self
    }

    pub fn character_set(&mut self, character_set: &str) -> &mut Self {
        self.query = format!("{} CHARACTER SET {}", self.query, character_set);

        self
    }

    pub fn foreign_key(&mut self, opts: ForeignKey) -> &mut Self {
        if self.query.starts_with("ALTER TABLE") {
            match opts.constraint {
                Some(constraint) => self.query = format!("{}, ADD CONSTRAINT {} FOREIGN KEY ({})", self.query, constraint, opts.first.column),
                None => self.query = format!("{}, ADD FOREIGN KEY ({})", self.query, opts.first.column)
            }
            
        } else {
            match opts.constraint {
                Some(constraint) => self.query = format!("{}, CONSTRAINT {} FOREIGN KEY ({})", self.query, constraint, opts.first.column),
                None => self.query = format!("{}, FOREIGN KEY ({})", self.query, opts.first.column)
            }
        }

        self.query = format!("{} REFERENCES {}({})", self.query, opts.second.table, opts.second.column);

        match opts.on_delete {
            Some(on_delete_opt) => self.query = format!("{} ON DELETE {}", self.query, on_delete_opt),
            None => ()
        }

        match opts.on_update {
            Some(on_update_opt) => self.query = format!("{} ON UPDATE {}", self.query, on_update_opt),
            None => ()
        }

        self
    }

    pub fn unsigned(&mut self) -> &mut Self {
        self.query = format!("{} UNSIGNED", self.query);

        self
    }

    pub fn zerofill(&mut self) -> &mut Self {
        self.query = format!("{} ZEROFILL", self.query);

        self
    }

    pub fn enum_sql(&mut self, enum_vec: Vec<&str>) -> &mut Self {
        match enum_vec.len() {
            0 => panic!("enum_vec argument cannot be an empty vector"),
            _ => ()
        }
        
        self.query = format!("{} ENUM(", self.query);

        let length_of_enum_vec = enum_vec.len();
        for (index, item) in enum_vec.into_iter().enumerate() {
            if index + 1 == length_of_enum_vec {
                self.query = format!("{}'{}'", self.query, item)
            } else {
                self.query = format!("{}'{}', ", self.query, item)
            }
        }

        self
    }

    pub fn generated_always(&mut self, condition: &str) -> &mut Self {
        self.query = format!("{} GENERATED ALWAYS AS {}", self.query, condition);

        self
    }

    pub fn virtual_sql(&mut self) -> &mut Self {
        self.query = format!("{} VIRTUAL", self.query);

        self
    }

    pub fn stored(&mut self) -> &mut Self {
        self.query = format!("{} STORED", self.query);

        self
    }

    pub fn spatial(&mut self) -> &mut Self {
        self.query = format!("{} SPATIAL", self.query);

        self
    }

    pub fn generated(&mut self) -> &mut Self {
        self.query = format!("{} GENERATED", self.query);

        self
    }

    pub fn index(&mut self, indexes: Vec<&str>) -> &mut Self {
        let length_of_indexes = indexes.len();

        match length_of_indexes {
            0 => panic!("There is no index here."),
            1 => self.query = format!("{}, INDEX({})", self.query, indexes[0]),
            _ => {
                for (i, index) in indexes.into_iter().enumerate() {
                    if i + 1 == length_of_indexes {
                        self.query = format!("{}{}", self.query, index);

                        continue;
                    }

                    if i == 0 {
                        self.query = format!("{}, INDEX ({}, ", self.query, index);

                        continue;
                    }

                    self.query = format!("{}{}, ", self.query, index)
                }
            }
        }

        self
    }

    pub fn comment(&mut self, comment: &str) -> &mut Self {
        self.query = format!("{} COMMENT '{}'", self.query, comment);

        self
    }

    pub fn default_on_null(&mut self, value: ValueType) -> &mut Self {
        match value {
            ValueType::String(text) => self.query = format!("{} DEFAULT {} ON NULL", self.query, text),
            _ => self.query = format!("{} DEFAULT {} ON NULL", self.query, value),
        }

        self
    }

    pub fn invisible(&mut self) -> &mut Self {
        self.query = format!("{} INVISIBLE", self.query);

        self
    }

    pub fn custom_query(&mut self, query: &str) -> &mut Self {
        self.query = format!("{} {}", self.query, query);

        self
    }

    pub fn finish(&mut self) -> String {
        return format!("{});", self.query)
    }
}

/// KeywordList enum. It helps to syntactically correcting the queries. 
#[derive(Debug, Clone, PartialEq)]
pub enum KeywordList {
    Select, Update, Delete, Insert, Count, Table, Where, Or, And, Set, 
    Finish, OrderBy, GroupBy, Having, Like, Limit, Offset, IfNotExist, Create, Use, In, 
    NotIn, JsonExtract, JsonContains, Field, Union, UnionAll
}

/// QueryType enum. It helps to detect the type of a query with more optimized way when is needed.
#[derive(Debug, Clone)]
pub enum QueryType {
    Select, Update, Delete, Insert, Null, Create, Count
}

/// ValueType enum. It benefits to detect and format the value with optimized way when you have to work with exact column values. 

#[derive(Debug, Clone)]
pub enum ValueType {
    String(String), Datetime(String), Null, Boolean(bool), Int32(i32), Int16(i16), Int8(i8), Int64(i64), Int128(i128),
    Uint8(u8), Uint16(u16), Uint32(u32), Uint64(u64), Usize(usize), Float32(f32), Float64(f64),
    EpochTime(i64),
}

impl std::fmt::Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::String(string) => write!(f, "'{}'", string),
            ValueType::Datetime(datetime) => match datetime.as_str() {
                "CURRENT_TIMESTAMP" | "UNIX_TIMESTAMP" | "CURRENT_DATE" | "CURRENT_TIME" | "NOW()" | "CURDATE()" | "CURTIME()" => write!(f, "{}", datetime),
                _ => write!(f, "'{}'", datetime)
            },
            ValueType::Null => write!(f, "NULL"),
            ValueType::Boolean(val) => write!(f, "{}", val),
            ValueType::Int8(val) => write!(f, "{}", val),
            ValueType::Int16(val) => write!(f, "{}", val),
            ValueType::Int32(val) => write!(f, "{}", val),
            ValueType::Int64(val) => write!(f, "{}", val),
            ValueType::Int128(val) => write!(f, "{}", val),
            ValueType::Usize(val) => write!(f, "{}", val),
            ValueType::Uint8(val) => write!(f, "{}", val),
            ValueType::Uint16(val) => write!(f, "{}", val),
            ValueType::Uint32(val) => write!(f, "{}", val),
            ValueType::Uint64(val) => write!(f, "{}", val),
            ValueType::Float32(val) => write!(f, "{}", val),
            ValueType::Float64(val) => write!(f, "{}", val),
            ValueType::EpochTime(val) => write!(f, "FROM_UNIXTIME({})", val),
        }
    }
}

impl From<String> for ValueType { fn from(value: String) -> Self { ValueType::String(value) } }
impl From<bool> for ValueType { fn from(value: bool) -> Self { ValueType::Boolean(value) } }
impl From<i8> for ValueType { fn from(value: i8) -> Self { ValueType::Int8(value) } }
impl From<i16> for ValueType { fn from(value: i16) -> Self { ValueType::Int16(value) } }
impl From<i32> for ValueType { fn from(value: i32) -> Self { ValueType::Int32(value) } }
impl From<i64> for ValueType { fn from(value: i64) -> Self { ValueType::Int64(value) } }
impl From<i128> for ValueType { fn from(value: i128) -> Self { ValueType::Int128(value) } }
impl From<usize> for ValueType { fn from(value: usize) -> Self { ValueType::Usize(value) } }
impl From<u8> for ValueType { fn from(value: u8) -> Self { ValueType::Uint8(value) } }
impl From<u16> for ValueType { fn from(value: u16) -> Self { ValueType::Uint16(value) } }
impl From<u32> for ValueType { fn from(value: u32) -> Self { ValueType::Uint32(value) } }
impl From<u64> for ValueType { fn from(value: u64) -> Self { ValueType::Uint64(value) } }
impl From<f32> for ValueType { fn from(value: f32) -> Self { ValueType::Float32(value) } }
impl From<f64> for ValueType { fn from(value: f64) -> Self { ValueType::Float64(value) } }


impl Into<String> for ValueType {
    fn into(self) -> String {
        match self {
            ValueType::String(text) => text,
            ValueType::Datetime(datetime) => datetime,
            _ => panic!("you cannot convert a ValueType to string unless it's not a String or Datetime variant.")
        }
    }
}

/*
            ValueType::Boolean(boolean) => boolean,
            ValueType::Float32(float) => float,
            ValueType::Float64(float) => float,
*/

impl Into<bool> for ValueType {
    fn into(self) -> bool {
        match self {
            ValueType::Boolean(val) => val,
            ValueType::String(text) => match text.as_str() {
                "false" | "" | "\0" | "0" => false,
                _ => true,
            }
            ValueType::Null => false,
            ValueType::Int8(val) => match val == 0 {
                false => true,
                true => false
            },
            ValueType::Int16(val) => match val == 0 {
                false => true,
                true => false
            },
            ValueType::Int32(val) => match val == 0 {
                false => true,
                true => false
            },
            ValueType::Int64(val) => match val == 0 {
                false => true,
                true => false
            },
            ValueType::Int128(val) => match val == 0 {
                false => true,
                true => false
            },
            ValueType::Uint8(val) => match val == 0 {
                false => true,
                true => false
            },
            ValueType::Uint16(val) => match val == 0 {
                false => true,
                true => false
            },
            ValueType::Uint32(val) => match val == 0 {
                false => true,
                true => false
            },
            ValueType::Uint64(val) => match val == 0 {
                false => true,
                true => false
            },
            ValueType::Float32(val) => match val == 0.0 {
                false => true,
                true => false
            },
            ValueType::Float64(val) => match val == 0.0 {
                false => true,
                true => false
            },
            _ => panic!("invalid conversion")
        }
    }
}

impl Into<f32> for ValueType {
    fn into(self) -> f32 {
        match self {
            ValueType::Float32(num) => num,
            ValueType::Float64(num) => num as f32,
            _ => panic!("invalid conversion")
        }
    }
}

impl Into<f64> for ValueType {
    fn into(self) -> f64 {
        match self {
            ValueType::Float32(num) => num as f64,
            ValueType::Float64(num) => num,
            _ => panic!("invalid conversion")
        }
    }
}

impl Into<i8> for ValueType {
    fn into(self) -> i8 {
        match self {
            ValueType::Int8(num) => num,
            ValueType::Int16(num) => match num > 128 || num < -128 {
                true => panic!("you cannot convert i16's into a value which is bigger than the capacity of 32 bit values."),
                false => num as i8
            },
            ValueType::Int32(num) => match num > 128 || num < -128 {
                true => panic!("you cannot convert i16's into a value which is bigger than the capacity of 32 bit values."),
                false => num as i8
            },
            ValueType::Int64(num) => match num > 128 || num < -128 {
                true => panic!("you cannot convert i16's into a value which is bigger than the capacity of 32 bit values."),
                false => num as i8
            },
            ValueType::Int128(num) => match num > 128 || num < -128 {
                true => panic!("you cannot convert i16's into a value which is bigger than the capacity of 32 bit values."),
                false => num as i8
            }
            ValueType::Uint8(num) => match num > 128 {
                true => panic!("you cannot convert i16's into a value which is bigger than the capacity of 32 bit values."),
                false => num as i8
            },
            ValueType::Uint16(num) => match num > 128 {
                true => panic!("you cannot convert i16's into a value which is bigger than the capacity of 32 bit values."),
                false => num as i8
            },
            ValueType::Uint32(num) => match num > 128 {
                true => panic!("you cannot convert i16's into a value which is bigger than the capacity of 32 bit values."),
                false => num as i8
            },
            ValueType::Usize(num) => match num > 128 {
                true => panic!("you cannot convert i16's into a value which is bigger than the capacity of 32 bit values."),
                false => num as i8
            },
            ValueType::Uint64(num) => match num > 128 {
                true => panic!("you cannot convert i16's into a value which is bigger than the capacity of 32 bit values."),
                false => num as i8
            },
            _ => panic!("you cannot convert non numeric values into numeric ones.")
        }
    }
}

impl Into<i16> for ValueType {
    fn into(self) -> i16 {
        match self {
            ValueType::Int8(num) => num as i16,
            ValueType::Int16(num) => num,
            ValueType::Int32(num) => match num > 32_768 || num < -32_768 {
                true => panic!("you cannot convert i16's into a value which is bigger than the capacity of 32 bit values."),
                false => num as i16
            },
            ValueType::Int64(num) => match num > 32_768 || num < -32_768 {
                true => panic!("you cannot convert i16's into a value which is bigger than the capacity of 32 bit values."),
                false => num as i16
            },
            ValueType::Int128(num) => match num > 32_768 || num < -32_768 {
                true => panic!("you cannot convert i16's into a value which is bigger than the capacity of 32 bit values."),
                false => num as i16
            }
            ValueType::Uint8(num) => num as i16,
            ValueType::Uint16(num) => match num > 32_768 {
                true => panic!("you cannot convert i16's into a value which is bigger than the capacity of 32 bit values."),
                false => num as i16
            },
            ValueType::Uint32(num) => match num > 32_768 {
                true => panic!("you cannot convert i16's into a value which is bigger than the capacity of 32 bit values."),
                false => num as i16
            },
            ValueType::Usize(num) => match num > 32_768 {
                true => panic!("you cannot convert i16's into a value which is bigger than the capacity of 32 bit values."),
                false => num as i16
            },
            ValueType::Uint64(num) => match num > 32_768 {
                true => panic!("you cannot convert i16's into a value which is bigger than the capacity of 32 bit values."),
                false => num as i16
            },
            _ => panic!("you cannot convert non numeric values into numeric ones.")
        }
    }
}

impl Into<i32> for ValueType {
    fn into(self) -> i32 {
        match self {
            ValueType::Int8(num) => num as i32,
            ValueType::Int16(num) => num as i32,
            ValueType::Int32(num) => num,
            ValueType::Int64(num) => match num > 2_147_483_647 || num < -2_147_483_647 {
                true => panic!("you cannot convert i32's into a value which is bigger than the capacity of 32 bit values."),
                false => num as i32
            },
            ValueType::Int128(num) => match num > 2_147_483_647 || num < -2_147_483_647 {
                true => panic!("you cannot convert i32's into a value which is bigger than the capacity of 32 bit values."),
                false => num as i32
            }
            ValueType::Uint8(num) => num as i32,
            ValueType::Uint16(num) => num as i32,
            ValueType::Uint32(num) => match num > 2_147_483_647 {
                true => panic!("you cannot convert i32's into a value which is bigger than the capacity of 32 bit values."),
                false => num as i32
            },
            ValueType::Usize(num) => match num > 2_147_483_647 {
                true => panic!("you cannot convert i32's into a value which is bigger than the capacity of 32 bit values."),
                false => num as i32
            },
            ValueType::Uint64(num) => match num > 2_147_483_647 {
                true => panic!("you cannot convert i32's into a value which is bigger than the capacity of 32 bit values."),
                false => num as i32
            }
            _ => panic!("you cannot convert non numeric values into numeric ones.")
        }
    }
}

impl Into<i64> for ValueType {
    fn into(self) -> i64 {
        match self {
            ValueType::EpochTime(epoch) => epoch as i64,
            ValueType::Int8(num) => num as i64,
            ValueType::Int16(num) => num as i64,
            ValueType::Int32(num) => num as i64,
            ValueType::Int64(num) => num,
            ValueType::Usize(num) => num as i64,
            ValueType::Uint8(num) => num as i64,
            ValueType::Uint16(num) => num as i64,
            ValueType::Uint32(num) => num as i64,
            ValueType::Uint64(num) => num as i64,
            _ => panic!("you cannot convert non numeric values into numeric ones.")
        }
    }
}

impl Into<u8> for ValueType {
    fn into(self) -> u8 {
        match self {
            ValueType::Uint8(num) => num,
            ValueType::Uint16(num) => match num > 255 {
                true => panic!("you cannot convert u16's if it's value is bigger than capacity of 8 bit values"),
                false => num as u8
            },
            ValueType::Uint32(num) => match num > 255 {
                true => panic!("you cannot convert u32's if it's value is bigger than capacity of 8 bit values"),
                false => num as u8
            },
            ValueType::Uint64(num) => match num > 255 {
                true => panic!("you cannot convert u64's if it's value is bigger than capacity of 8 bit values"),
                false => num as u8
            },
            ValueType::Usize(num) => match num > 255 {
                true => panic!("you cannot convert usizes if it's value is bigger than capacity of 8 bit values"),
                false => num as u8
            },
            ValueType::Int8(num) => match num < 0 {
                true => panic!("you cannot convert i8's if it's value is lower than 0"),
                false => num as u8
            },
            ValueType::Int16(num) => match num < 0 || num > 255 {
                true => panic!("you cannot convert i16's if it's value is lower than 0 or has a value which is bigger than capacity of 8 bit values."),
                false => num as u8
            },
            ValueType::Int32(num) => match num < 0 || num > 255 {
                true => panic!("you cannot convert i32's if it's value is lower than 0 or has a value which is bigger than capacity of 8 bit values."),
                false => num as u8
            },
            ValueType::Int64(num) => match num < 0 || num > 255 {
                true => panic!("you cannot convert 64's if it's value is lower than 0 or has a value which is bigger than capacity of 8 bit values."),
                false => num as u8
            },
            ValueType::Int128(num) => match num < 0 || num > 255 {
                true => panic!("you cannot convert i128's if it's value is lower than 0 or has a value which is bigger than capacity of 8 bit values."),
                false => num as u8
            }
            _ => panic!("you cannot convert non numeric values into numeric ones.")
        }
    }
}

impl Into<u16> for ValueType {
    fn into(self) -> u16 {
        match self {
            ValueType::Uint8(num) => num as u16,
            ValueType::Uint16(num) => num,
            ValueType::Uint32(num) => match num > 65_535 {
                true => panic!("you cannot convert u32's if it's value is bigger than capacity of 32 bit values"),
                false => num as u16
            },
            ValueType::Uint64(num) => match num > 65_535 {
                true => panic!("you cannot convert u64's if it's value is bigger than capacity of 32 bit values"),
                false => num as u16
            },
            ValueType::Usize(num) => match num > 65_535 {
                true => panic!("you cannot convert usizes if it's value is bigger than capacity of 32 bit values"),
                false => num as u16
            },
            ValueType::Int8(num) => match num < 0 {
                true => panic!("you cannot convert i8's if it's value is lower than 0"),
                false => num as u16
            },
            ValueType::Int16(num) => match num < 0 {
                true => panic!("you cannot convert i16's if it's value is lower than 0"),
                false => num as u16
            },
            ValueType::Int32(num) => match num < 0 || num > 65_535 {
                true => panic!("you cannot convert i32's if it's value is lower than 0 or has a value which is bigger than capacity of 16 bit values."),
                false => num as u16
            },
            ValueType::Int64(num) => match num < 0 || num > 65_535 {
                true => panic!("you cannot convert i64's if it's value is lower than 0 or has a value which is bigger than capacity of 16 bit values."),
                false => num as u16
            },
            ValueType::Int128(num) => match num < 0 || num > 65_535 {
                true => panic!("you cannot convert i128's if it's value is lower than 0 or has a value which is bigger than capacity of 16 bit values."),
                false => num as u16
            }
            _ => panic!("you cannot convert non numeric values into numeric ones.")
        }
    }
}

impl Into<u32> for ValueType {
    fn into(self) -> u32 {
        match self {
            ValueType::Uint8(num) => num as u32,
            ValueType::Uint16(num) => num as u32,
            ValueType::Uint32(num) => num,
            ValueType::Uint64(num) => match num > 4_294_967_295 {
                true => panic!("you cannot convert u64's if it's value is bigger than capacity of 32 bit values"),
                false => num as u32
            },
            ValueType::Usize(num) => match num > 4_294_967_295 {
                true => panic!("you cannot convert usizes if it's value is bigger than capacity of 32 bit values"),
                false => num as u32
            },
            ValueType::Int8(num) => match num < 0 {
                true => panic!("you cannot convert i8's if it's value is lower than 0"),
                false => num as u32
            },
            ValueType::Int16(num) => match num < 0 {
                true => panic!("you cannot convert i16's if it's value is lower than 0"),
                false => num as u32
            },
            ValueType::Int32(num) => match num < 0 {
                true => panic!("you cannot convert i32's if it's value is lower than 0"),
                false => num as u32
            },
            ValueType::Int64(num) => match num < 0 || num > 4_294_967_295 {
                true => panic!("you cannot convert i64's if it's value is lower than 0 or has a value which is bigger than capacity of 32 bit values."),
                false => num as u32
            },
            ValueType::Int128(num) => match num < 0 || num > 4_294_967_295 {
                true => panic!("you cannot convert i128's if it's value is lower than 0 or has a value which is bigger than capacity of 32 bit values."),
                false => num as u32
            }
            _ => panic!("you cannot convert non numeric values into numeric ones.")
        }
    }
}

impl Into<u64> for ValueType {
    fn into(self) -> u64 {
        match self {
            ValueType::Usize(num) => num as u64,
            ValueType::Uint8(num) => num as u64,
            ValueType::Uint16(num) => num as u64,
            ValueType::Uint32(num) => num as u64,
            ValueType::Uint64(num) => num,
            ValueType::Int8(num) => match num < 0 {
                true => panic!("you cannot turn a negative value into u64"),
                false => num as u64
            },
            ValueType::Int16(num) => match num < 0 {
                true => panic!("you cannot turn a negative value into u64"),
                false => num as u64
            },
            ValueType::Int32(num) => match num < 0 {
                true => panic!("you cannot turn a negative value into u64"),
                false => num as u64
            },
            ValueType::Int64(num) => match num < 0 {
                true => panic!("you cannot turn a negative value into u64"),
                false => num as u64
            },
            ValueType::Int128(num) => match num < 0 {
                true => panic!("you cannot turn a negative value into u64"),
                false => num as u64
            },
            _ => panic!("you cannot convert non numeric values into numeric ones.")
        }
    }
}

impl Into<usize> for ValueType {
    fn into(self) -> usize {
        match self {
            ValueType::Int8(num) => match num < 0 {
                true => panic!("you cannot convert negative numbers to usize"),
                false => num as usize
            },
            ValueType::Int16(num) => match num < 0 {
                true => panic!("you cannot convert negative numbers to usize"),
                false => num as usize
            },
            ValueType::Int32(num) => match num < 0 {
                true => panic!("you cannot convert negative numbers to usize"),
                false => num as usize
            },
            ValueType::Int64(num) => match num < 0 {
                true => panic!("you cannot convert negative numbers to usize"),
                false => num as usize
            },
            ValueType::Int128(num) => match num < 0 {
                true => panic!("you cannot convert negative numbers to usize"),
                false => num as usize
            },
            ValueType::Usize(num) => num,
            ValueType::Uint8(num) => num as usize,
            ValueType::Uint16(num) => num as usize,
            ValueType::Uint32(num) => num as usize,
            ValueType::Uint64(num) => num as usize,
            _ => panic!("you cannot convert non numeric values into numeric ones.")
        }
    }
}

/// Enum that benefits you to define what you want with a foreign key.
#[derive(Debug, Clone)]
pub enum ForeignKeyActions {
    Cascade, Restrict, SetNull, NoAction, SetDefault
}

impl std::fmt::Display for ForeignKeyActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &ForeignKeyActions::Cascade => write!(f, "CASCADE"),
            &ForeignKeyActions::NoAction => write!(f, "NO ACTION"),
            &ForeignKeyActions::Restrict => write!(f, "RESTRICT"),
            &ForeignKeyActions::SetNull => write!(f, "SET NULL"),
            &ForeignKeyActions::SetDefault => write!(f, "SET DEFAULT")
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_schema_query_declarative(){
        let schema = SchemaBuilder::create("blog_website").unwrap().if_not_exists().finish();

        assert_eq!("CREATE DATABASE IF NOT EXISTS blog_website;", schema);
    }

    #[test]
    pub fn test_schema_query_imperative(){
        let mut schema = SchemaBuilder::create("blog_website").unwrap();
        schema.if_not_exists();
        let schema_query = schema.finish();

        assert_eq!("CREATE DATABASE IF NOT EXISTS blog_website;", schema_query);
    }

    #[test]
    pub fn test_use_another_schema(){
        let schema = SchemaBuilder::use_another_schema("chat_website").unwrap().finish();

        assert_eq!("USE chat_website;", schema);
    }

    #[test]
    pub fn test_insert_query(){
        let columns = vec!["title", "author", "description"];
        let values = vec![ValueType::String("What's Up?".to_string()), ValueType::String("John Doe".to_string()), ValueType::String("Lorem ipsum dolor sit amet, consectetur adipiscing elit.".to_string())];
    
        let insert_query = QueryBuilder::insert(columns, values).unwrap().table("blogs").finish();

        println!("{}", insert_query);
        assert_eq!("INSERT INTO blogs (title, author, description) VALUES ('What's Up?', 'John Doe', 'Lorem ipsum dolor sit amet, consectetur adipiscing elit.');".to_string(), 
                    insert_query);
    }

    #[test]
    pub fn test_update_query(){
        let update_query = QueryBuilder::update().unwrap().table("blogs").set("title", ValueType::String("Hello Rust!".to_string())).set("author", ValueType::String("Necdet".to_string())).finish();

        assert_eq!("UPDATE blogs SET title = 'Hello Rust!', author = 'Necdet';", update_query);
    }

    #[test]
    pub fn test_delete_query(){
        let delete_query = QueryBuilder::delete().unwrap().table("blogs").where_("id", "=", ValueType::String("1".to_string())).finish();

        assert_eq!("DELETE FROM blogs WHERE id = '1';", delete_query);
    }

    #[test]
    pub fn test_select_query_declarative(){
        let mut select = QueryBuilder::select(["id", "title", "description", "point"].to_vec()).unwrap();

        let select_query = select.table("blogs")
                                    .where_("id", "=", ValueType::Int32(10))
                                    .and("point", ">", ValueType::Int8(90))
                                    .or("id", "=", ValueType::Int64(20))
                                    .finish();

        assert_eq!("SELECT id, title, description, point FROM blogs WHERE id = 10 AND point > 90 OR id = 20;", select_query)
    }

    #[test]
    pub fn test_select_query_imperative(){
        let mut select = QueryBuilder::select(["*"].to_vec()).unwrap();

        let select_query = select.table("blogs");
        select_query.where_("id", "=", ValueType::Uint8(5));
        select_query.or("id", "=", ValueType::Usize(25));

        let finish_the_select_query = select_query.finish();

        assert_eq!("SELECT * FROM blogs WHERE id = 5 OR id = 25;", finish_the_select_query);
    }

    #[test]
    pub fn test_create_table() {
        let mut table_builder_2 = TableBuilder::create("blabla", "projects");
        let table_builder_2 = table_builder_2.if_not_exists();
    
        table_builder_2.add_column("id").col_type("INT").primary_key().auto_increment();
        table_builder_2.add_column("name").col_type("VARCHAR(40)").not_null();
        table_builder_2.add_column("owner_id").col_type("INT").not_null();
        
        // if we create a table, the first ForeignKeyItem's table field is not necessary.
        let opts = ForeignKey {
            first: ForeignKeyItem { table: "".to_string(), column: "owner_id".to_string() },
            second: ForeignKeyItem { table: "users".to_string(), column: "id".to_string() },
            constraint: None,
            on_delete: Some(ForeignKeyActions::Cascade),
            on_update: None
        };
        
        table_builder_2.foreign_key(opts);
    
        let table_builder_2 = table_builder_2.finish();

        let raw_query = "CREATE TABLE projects IF NOT EXISTS (id INT PRIMARY KEY AUTO_INCREMENT, name VARCHAR(40) NOT NULL, owner_id INT NOT NULL, FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE CASCADE);".to_string();

        assert_eq!(raw_query, table_builder_2);
    }

    #[test]
    pub fn test_time_value_type(){
        let columns = ["name", "password", "last_login"].to_vec();
        let values = [ValueType::String("necoo33".to_string()), ValueType::String("123456".to_string()), ValueType::Datetime("CURRENT_TIMESTAMP".to_string())].to_vec();
    
        let time_insert_test = QueryBuilder::insert(columns, values).unwrap().table("users").finish();

        assert_eq!(time_insert_test, "INSERT INTO users (name, password, last_login) VALUES ('necoo33', '123456', CURRENT_TIMESTAMP);");

        let time_update_test = QueryBuilder::update().unwrap().table("users").set("last_login", ValueType::Datetime("CURRENT_TIMESTAMP".to_string())).where_("name", "=", ValueType::String("necoo33".to_string())).finish();

        assert_eq!(time_update_test, "UPDATE users SET last_login = CURRENT_TIMESTAMP WHERE name = 'necoo33';")
    }

    #[test]
    pub fn test_unix_epoch_times(){
        let columns = ["name", "password", "last_login"].to_vec();
        let values = [ValueType::String("necoo33".to_string()), ValueType::String("123456".to_string()), ValueType::EpochTime(134523452)].to_vec();
    
        let time_insert_with_unix_epoch_times_test = QueryBuilder::insert(columns, values).unwrap().table("users").finish();
        assert_eq!(time_insert_with_unix_epoch_times_test, "INSERT INTO users (name, password, last_login) VALUES ('necoo33', '123456', FROM_UNIXTIME(134523452));");
    
        let time_update_with_unix_epoch_times_test = QueryBuilder::update().unwrap().table("users").set("last_login", ValueType::EpochTime(3456436)).where_("name", "=", ValueType::String("necoo33".to_string())).finish();

        assert_eq!(time_update_with_unix_epoch_times_test, "UPDATE users SET last_login = FROM_UNIXTIME(3456436) WHERE name = 'necoo33';");

        let columns = ["name", "password", "last_login", "created_at"].to_vec();

        let unix_epoch_times_test_3 = QueryBuilder::select(columns).unwrap().table("users").where_("created_at", ">", ValueType::EpochTime(3234534)).or("last_login", ">=", ValueType::EpochTime(2134432)).offset(0).limit(20).finish();

        assert_eq!(unix_epoch_times_test_3, "SELECT name, password, last_login, created_at FROM users WHERE created_at > FROM_UNIXTIME(3234534) OR last_login >= FROM_UNIXTIME(2134432) OFFSET 0 LIMIT 20;")
    }

    #[test]
    pub fn test_where_ins(){
        let columns = ["name", "age", "id", "last_login"].to_vec();

        let ids = [ValueType::Int32(1), ValueType::Int16(12), ValueType::Int64(8)].to_vec();

        let test_where_in = QueryBuilder::select(columns).unwrap().table("users").where_in("id", &ids).finish();

        assert_eq!(test_where_in, "SELECT name, age, id, last_login FROM users WHERE id IN (1, 12, 8);");

        let columns = ["name", "age", "id", "last_login"].to_vec();

        let ids = [ValueType::Int32(1), ValueType::Int16(12), ValueType::Int32(8)].to_vec();

        let test_where_not_in = QueryBuilder::select(columns).unwrap().table("users").where_not_in("id", &ids).finish();

        assert_eq!(test_where_not_in, "SELECT name, age, id, last_login FROM users WHERE id NOT IN (1, 12, 8);");

        let columns = ["name", "age", "id", "last_login"].to_vec();

        let test_where_in_custom = QueryBuilder::select(columns).unwrap().table("users").where_in_custom("id", "1, 12, 8").finish();

        assert_eq!(test_where_in_custom, "SELECT name, age, id, last_login FROM users WHERE id IN (1, 12, 8);");

        let columns = ["name", "age", "id", "last_login"].to_vec();

        let test_where_not_in_custom = QueryBuilder::select(columns).unwrap().table("users").where_not_in_custom("id", "1, 12, 8").finish();

        assert_eq!(test_where_not_in_custom, "SELECT name, age, id, last_login FROM users WHERE id NOT IN (1, 12, 8);")
    }

    #[test]
    pub fn test_count() {
        let count_of_users = QueryBuilder::count("*", None).table("users").where_("age", ">", ValueType::Int32(25)).finish();

        assert_eq!(count_of_users, "SELECT COUNT(*) FROM users WHERE age > 25;".to_string());

        let count_of_users_as_length = QueryBuilder::count("*", Some("length")).table("users").finish();

        assert_eq!(count_of_users_as_length, "SELECT COUNT(*) AS length FROM users;".to_string());
    }

    #[test]
    pub fn test_json_extract(){
        // tests with "select()" constructor

        let select_query_1 = QueryBuilder::select(["*"].to_vec()).unwrap().json_extract("data", ".age", Some("student_age")).table("students").finish();

        assert_eq!(select_query_1, "SELECT JSON_EXTRACT(data, '$.age') AS student_age FROM students;".to_string());
        
        let select_query_2 = QueryBuilder::select(["*"].to_vec()).unwrap().json_extract("data", ".age", Some("student_age")).table("students").where_("successfull", "=", ValueType::Int8(1)).finish();

        assert_eq!(select_query_2, "SELECT JSON_EXTRACT(data, '$.age') AS student_age FROM students WHERE successfull = 1;".to_string());
        
        let select_query_3 = QueryBuilder::select(["*"].to_vec()).unwrap().json_extract("data", ".age", Some("student_age")).table("students").where_("points", ">", ValueType::Int32(85)).json_extract("points", ".name", None).finish();

        assert_eq!(select_query_3, "SELECT JSON_EXTRACT(data, '$.age') AS student_age FROM students WHERE JSON_EXTRACT(points, '$.name') > 85;".to_string());

        // tests with ".where_cond()" method

        let with_where = QueryBuilder::delete().unwrap().table("users").where_("id", ">", ValueType::Int32(200)).json_extract("id", ".user_id", None).finish();

        assert_eq!(with_where, "DELETE FROM users WHERE JSON_EXTRACT(id, '$.user_id') > 200;".to_string());

        // tests with ".table()" method
        
        let fields = ["name", "age"].to_vec();
        
        let with_table = QueryBuilder::select(fields).unwrap().table("users").json_extract("id", ".user_id", None).finish();

        assert_eq!(with_table, "SELECT JSON_EXTRACT(id, '$.user_id') FROM users;".to_string());

        // tests with ".and()" method

        let fields = ["name", "age"].to_vec();

        let with_and_1 = QueryBuilder::select(fields).unwrap().table("height").where_("weight", ">", ValueType::Int32(60)).and("height", ">", ValueType::Float64(1.70)).json_extract("height", ".student_height", None).finish();

        assert_eq!(with_and_1, "SELECT name, age FROM height WHERE weight > 60 AND JSON_EXTRACT(height, '$.student_height') > 1.7;".to_string());
    
        let with_and_2 = QueryBuilder::select(["*"].to_vec()).unwrap().table("students").where_("weight", ">", ValueType::Int32(60)).and("height", ">", ValueType::Float64(1.70)).json_extract("height", ".student_height", None).finish();

        assert_eq!(with_and_2, "SELECT * FROM students WHERE weight > 60 AND JSON_EXTRACT(height, '$.student_height') > 1.7;".to_string());

        // tests with ".or()" method

        let fields = ["name", "age"].to_vec();

        let with_or_1 = QueryBuilder::select(fields).unwrap().table("height").where_("weight", ">", ValueType::Int32(60)).or("height", ">", ValueType::Float64(1.71)).json_extract("height", ".student_height", None).finish();

        assert_eq!(with_or_1, "SELECT name, age FROM height WHERE weight > 60 OR JSON_EXTRACT(height, '$.student_height') > 1.71;".to_string());
    
        let with_or_2 = QueryBuilder::select(["*"].to_vec()).unwrap().table("students").where_("weight", ">", ValueType::Int32(60)).or("height", ">", ValueType::Float64(1.71)).json_extract("height", ".student_height", None).finish();

        assert_eq!(with_or_2, "SELECT * FROM students WHERE weight > 60 OR JSON_EXTRACT(height, '$.student_height') > 1.71;".to_string());

        // tests with "count()" constructor

        let count_query_1 = QueryBuilder::count("*", None).json_extract("age", ".student_age", Some("value")).table("students").group_by("points").having("points", ">", ValueType::Int32(75)).finish();

        assert_eq!(count_query_1, "SELECT JSON_EXTRACT(age, '$.student_age') AS value, COUNT(*) FROM students GROUP BY points HAVING points > 75;".to_string());

        // tests with ".order_by()" method
        
        let fields = ["title", "desc", "created_at", "updated_at", "keywords", "pics", "likes"].to_vec();

        let order_by_query_1 = QueryBuilder::select(fields).unwrap().table("contents").where_("published", "=", ValueType::Int32(1)).order_by("likes", "ASC").json_extract("likes", ".name", None).finish();

        assert_eq!(order_by_query_1, "SELECT title, desc, created_at, updated_at, keywords, pics, likes FROM contents WHERE published = 1 ORDER BY JSON_EXTRACT(likes, '$.name') ASC;".to_string());
    
        // tests with ".json_extract()" method

        let json_extract_chaining = QueryBuilder::select(["*"].to_vec()).unwrap().json_extract("articles", "[0]", Some("blog1")).json_extract("articles", "[1]", Some("blog2")).json_extract("articles", "[2]", Some("blog3")).table("users").where_("published", "=", ValueType::Int32(1)).finish();

        assert_eq!(json_extract_chaining, "SELECT JSON_EXTRACT(articles, '$[0]') AS blog1, JSON_EXTRACT(articles, '$[1]') AS blog2, JSON_EXTRACT(articles, '$[2]') AS blog3 FROM users WHERE published = 1;".to_string());
    }

    #[test]
    pub fn test_json_contains(){
        // test with "select()" constructor:

        let ins = [ValueType::Int32(1), ValueType::Int32(5), ValueType::Int64(11)].to_vec();
        let select_query = QueryBuilder::select(["*"].to_vec()).unwrap().json_contains("pic", ValueType::String("\"/files/hello.jpg\"".to_string()), Some(".path")).table("users").where_in("id", &ins).finish();

        assert_eq!(select_query, "SELECT JSON_CONTAINS(pic, '\"/files/hello.jpg\"', '$.path') FROM users WHERE id IN (1, 5, 11);".to_string());
        
        // test with ".where_cond()" method:

        let where_query = QueryBuilder::select(["*"].to_vec()).unwrap().table("users").where_("pic", "=", ValueType::String("".to_string())).json_contains("pic", ValueType::String("\"blablabla.jpg\"".to_string()), Some(".name")).finish();

        assert_eq!(where_query, "SELECT * FROM users WHERE JSON_CONTAINS(pic, '\"blablabla.jpg\"', '$.name');".to_string());

        // tests with ".and()" method:

        let and_query_1 = QueryBuilder::select(["*"].to_vec()).unwrap().table("users").where_("age", ">", ValueType::Int32(15)).and("asdfasdf", ">", ValueType::String("".to_string())).json_contains("graduation_stats", ValueType::Float64(80.11), Some(".average_point")).finish();

        assert_eq!(and_query_1, "SELECT * FROM users WHERE age > 15 AND JSON_CONTAINS(graduation_stats, 80.11, '$.average_point');".to_string());

        let and_query_2 = QueryBuilder::select(["*"].to_vec()).unwrap().table("users").where_("age", ">", ValueType::Int32(15)).and("class", "=", ValueType::String("5/c".to_string())).and("asdfasdf", ">", ValueType::String("".to_string())).json_contains("graduation_stats", ValueType::Float64(80.11), Some(".average_point")).finish();

        assert_eq!(and_query_2, "SELECT * FROM users WHERE age > 15 AND class = '5/c' AND JSON_CONTAINS(graduation_stats, 80.11, '$.average_point');".to_string());
        
        let and_query_3 = QueryBuilder::select(["*"].to_vec()).unwrap().table("users").where_("age", ">", ValueType::Int32(15)).and("class", "=", ValueType::String("5/c".to_string())).and("surname", "=", ValueType::String("etiman".to_string())).and("asdfasdf", ">", ValueType::String("".to_string())).json_contains("graduation_stats", ValueType::Float64(80.11), Some(".average_point")).finish();
    
        assert_eq!(and_query_3, "SELECT * FROM users WHERE age > 15 AND class = '5/c'  AND surname = 'etiman' AND JSON_CONTAINS(graduation_stats, 80.11, '$.average_point');".to_string());

        let and_query_4 = QueryBuilder::select(["*"].to_vec()).unwrap().table("users").where_("age", ">", ValueType::Int32(15)).and("sdfgsdfg", "=", ValueType::String("".to_string())).json_contains("parents", ValueType::Int32(50), Some(".age")).and("surname", "=", ValueType::String("etiman".to_string())).and("asdfasdf", ">", ValueType::String("".to_string())).json_contains("graduation_stats", ValueType::Float32(80.11), Some(".average_point")).finish();
    
        assert_eq!(and_query_4, "SELECT * FROM users WHERE age > 15 AND JSON_CONTAINS(parents, 50, '$.age')  AND surname = 'etiman' AND JSON_CONTAINS(graduation_stats, 80.11, '$.average_point');".to_string());

        let and_query_5 = QueryBuilder::select(["*"].to_vec()).unwrap().table("users").where_("age", ">", ValueType::Int32(15)).and("sdfgsdfg", "=", ValueType::String("".to_string())).json_contains("parents", ValueType::Int32(50), Some(".age")).and("asdfasdf", ">", ValueType::String("".to_string())).json_contains("graduation_stats", ValueType::Float64(80.11), Some(".average_point")).finish();
    
        assert_eq!(and_query_5, "SELECT * FROM users WHERE age > 15 AND JSON_CONTAINS(parents, 50, '$.age') AND JSON_CONTAINS(graduation_stats, 80.11, '$.average_point');".to_string());
        
        // tests with ".or()" method:

        let or_query_1 = QueryBuilder::select(["*"].to_vec()).unwrap().table("users").where_("age", ">", ValueType::Int32(15)).or("asdfasdf", ">", ValueType::String("".to_string())).json_contains("graduation_stats", ValueType::Float32(80.11), Some(".average_point")).finish();

        assert_eq!(or_query_1, "SELECT * FROM users WHERE age > 15 OR JSON_CONTAINS(graduation_stats, 80.11, '$.average_point');".to_string());
        
        let or_query_2 = QueryBuilder::select(["*"].to_vec()).unwrap().table("users").where_("age", ">", ValueType::Int32(15)).or("class", "=", ValueType::String("5/c".to_string())).or("asdfasdf", ">", ValueType::String("".to_string())).json_contains("graduation_stats", ValueType::Float64(80.11), Some(".average_point")).finish();
        
        assert_eq!(or_query_2, "SELECT * FROM users WHERE age > 15 OR class = '5/c' OR JSON_CONTAINS(graduation_stats, 80.11, '$.average_point');".to_string());
                
        let or_query_3 = QueryBuilder::select(["*"].to_vec()).unwrap().table("users").where_("age", ">", ValueType::Int32(15)).or("class", "=", ValueType::String("5/c".to_string())).or("surname", "=", ValueType::String("etiman".to_string())).or("asdfasdf", ">", ValueType::String("".to_string())).json_contains("graduation_stats", ValueType::Float64(80.11), Some(".average_point")).finish();
            
        assert_eq!(or_query_3, "SELECT * FROM users WHERE age > 15 OR class = '5/c'  OR surname = 'etiman' OR JSON_CONTAINS(graduation_stats, 80.11, '$.average_point');".to_string());
        
        let or_query_4 = QueryBuilder::select(["*"].to_vec()).unwrap().table("users").where_("age", ">", ValueType::Int32(15)).or("sdfgsdfg", "=", ValueType::String("".to_string())).json_contains("parents", ValueType::Int32(50), Some(".age")).or("surname", "=", ValueType::String("etiman".to_string())).or("asdfasdf", ">", ValueType::String("".to_string())).json_contains("graduation_stats", ValueType::Float64(80.11), Some(".average_point")).finish();
            
        assert_eq!(or_query_4, "SELECT * FROM users WHERE age > 15 OR JSON_CONTAINS(parents, 50, '$.age')  OR surname = 'etiman' OR JSON_CONTAINS(graduation_stats, 80.11, '$.average_point');".to_string());
        
        let or_query_5 = QueryBuilder::select(["*"].to_vec()).unwrap().table("users").where_("age", ">", ValueType::Int32(15)).or("sdfgsdfg", "=", ValueType::String("".to_string())).json_contains("parents", ValueType::Int64(50), Some(".age")).or("asdfasdf", ">", ValueType::String("".to_string())).json_contains("graduation_stats", ValueType::Float32(80.11), Some(".average_point")).finish();
            
        assert_eq!(or_query_5, "SELECT * FROM users WHERE age > 15 OR JSON_CONTAINS(parents, 50, '$.age') OR JSON_CONTAINS(graduation_stats, 80.11, '$.average_point');".to_string());
    }

    #[test]
    pub fn test_like_later_than_where_keywords(){
        let mut like_query_1 = QueryBuilder::select(["*"].to_vec()).unwrap();

        let like_query_1 = like_query_1.table("blogs")
                                                      .where_("id", "=", ValueType::Int32(5))
                                                      .like(["title", "description"].to_vec(), "hello")
                                                      .finish();

        assert_eq!(like_query_1, "SELECT * FROM blogs WHERE id = 5 AND (title LIKE '%hello%' OR description LIKE '%hello%');");
    
        let mut like_query_2 = QueryBuilder::select(["*"].to_vec()).unwrap();

        let ins = vec![ValueType::Int32(1), ValueType::Int32(2), ValueType::Int32(3)];
        let like_query_2 = like_query_2.table("blogs")
                                                          .where_in("id", &ins)
                                                          .like(["title", "description", "keywords"].to_vec(), "necdet")
                                                          .limit(10)
                                                          .offset(0)
                                                          .finish();

        assert_eq!(like_query_2, "SELECT * FROM blogs WHERE id IN (1, 2, 3) AND (title LIKE '%necdet%' OR description LIKE '%necdet%' OR keywords LIKE '%necdet%') LIMIT 10 OFFSET 0;")
    }

    #[test]
    pub fn test_ordering_functions(){
        let order_by_query = QueryBuilder::select(["*"].to_vec()).unwrap().table("users").order_by("id", "asc").order_by("weight", "desc").order_by("point", "asc").finish();

        assert_eq!(order_by_query, "SELECT * FROM users ORDER BY id ASC, weight DESC, point ASC;");

        let order_by_random_query = QueryBuilder::select(["*"].to_vec()).unwrap().table("users").order_random().finish();

        assert_eq!(order_by_random_query, "SELECT * FROM users ORDER BY RAND();");

        let roles = ["admin", "moderator", "member", "guest"].to_vec();
        let field_query_1 = QueryBuilder::select(["*"].to_vec()).unwrap().table("users").order_by_field("role", roles.clone()).finish();

        assert_eq!(field_query_1, "SELECT * FROM users ORDER BY FIELD(role, 'admin', 'moderator', 'member', 'guest');");

        let field_query_2 = QueryBuilder::select(["*"].to_vec()).unwrap().table("users").order_by("id", "asc").order_by_field("role", roles.clone()).finish();

        assert_eq!(field_query_2, "SELECT * FROM users ORDER BY id ASC, FIELD(role, 'admin', 'moderator', 'member', 'guest');");

        let field_query_3 = QueryBuilder::select(["*"].to_vec()).unwrap().table("users").order_by_field("role", roles.clone()).order_by("id", "asc").finish();

        assert_eq!(field_query_3, "SELECT * FROM users ORDER BY FIELD(role, 'admin', 'moderator', 'member', 'guest'), id ASC;");
        
        let field_query_4 = QueryBuilder::select(["*"].to_vec()).unwrap().table("users").order_by_field("role", roles).order_by_field("status", vec!["active", "banned", "unverified"]).finish();

        assert_eq!(field_query_4, "SELECT * FROM users ORDER BY FIELD(role, 'admin', 'moderator', 'member', 'guest'), FIELD(status, 'active', 'banned', 'unverified');");
    }

    #[test]
    pub fn test_unions(){
        let mut union_1 = QueryBuilder::select(vec!["name", "age", "id"]).unwrap();
        union_1.table("users").where_("age", ">", ValueType::Int32(7));

        let union_2 = QueryBuilder::select(vec!["name", "age", "id"]).unwrap()
                                                          .table("users")
                                                          .where_("age", "<", ValueType::Int32(15))
                                                          .union(vec![union_1])
                                                          .finish();

        assert_eq!(union_2, "(SELECT name, age, id FROM users WHERE age < 15) UNION (SELECT name, age, id FROM users WHERE age > 7);");

        let mut union_1 = QueryBuilder::select(vec!["id", "title", "description", "published"]).unwrap();
        union_1.table("blogs").like(vec!["title"], "text");

        let mut union_2 = QueryBuilder::select(vec!["id", "title", "description", "published"]).unwrap();
        union_2.table("blogs").like(vec!["description"], "some text");

        let union_3 = QueryBuilder::select(vec!["id", "title", "description", "published"]).unwrap()
                                                              .table("blogs")
                                                              .where_("published", "=", ValueType::Boolean(true))
                                                              .union_all(vec![union_1, union_2])
                                                              .finish();

        assert_eq!(union_3, "(SELECT id, title, description, published FROM blogs WHERE published = true) UNION ALL (SELECT id, title, description, published FROM blogs WHERE title LIKE '%text%') UNION ALL (SELECT id, title, description, published FROM blogs WHERE description LIKE '%some text%');");
    }
}
