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
    pub fn where_(&mut self, column: &str, mut mark: &str, value: ValueType) -> &mut Self {
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

        if let ValueType::Null = value {
            match mark {
                "=" => mark = "IS",
                "!=" | "<>" => mark = "IS NOT",
                "IS" | "IS NOT" => (),
                _ => mark = "IS"
            }
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

        self.list.push(KeywordList::WhereIn);
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

        self.list.push(KeywordList::WhereNotIn);
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

        self.list.push(KeywordList::WhereIn);
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

        self.list.push(KeywordList::WhereNotIn);

        self
    }


    /// It adds the "IN" keyword with it's synthax, with 'AND' keyword except 'WHERE'.
    ///     
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType};
    /// 
    /// fn main(){
    ///     let ins = vec![ValueType::Int16(1), ValueType::Int64(5), ValueType::Int32(10)];
    ///     let query = QueryBuilder::select(vec!["*"]).unwrap()
    ///                              .table("users")
    ///                              .where_("class", "=", ValueType::String("10/c".to_string()))
    ///                              .and_in("id", &ins)
    ///                              .finish();
    /// 
    ///     assert_eq!(query, "SELECT * FROM users WHERE class = '10/c' AND id IN (1, 5, 10);")
    /// }
    pub fn and_in(&mut self, column: &str, ins: &Vec<ValueType>) -> &mut Self {
        match ins.len() {
            0 => panic!("you cannot pass an empty vector to the ins argument"),
            _ => ()
        }

        self.query = format!("{} AND {} IN (", self.query, column);

        let length_of_ins = ins.len();

        for (index, value) in ins.into_iter().enumerate() {
            if index + 1 == length_of_ins {
                self.query = format!("{}{})", self.query, value);
                    
                continue;
            }

            self.query = format!("{}{}, ", self.query, value);
        }

        self.list.push(KeywordList::AndIn);
        self
    }


    /// It adds the "NOT IN" keyword with it's synthax, with 'AND' keyword except 'WHERE'.
    ///     
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType};
    /// 
    /// fn main(){
    ///     let ins = vec![ValueType::Int16(1), ValueType::Int64(5), ValueType::Int32(10)];
    ///     let query = QueryBuilder::select(vec!["*"]).unwrap()
    ///                              .table("users")
    ///                              .where_("class", "=", ValueType::String("10/c".to_string()))
    ///                              .and_not_in("id", &ins)
    ///                              .finish();
    /// 
    ///     assert_eq!(query, "SELECT * FROM users WHERE class = '10/c' AND id NOT IN (1, 5, 10);")
    /// }
    pub fn and_not_in(&mut self, column: &str, ins: &Vec<ValueType>) -> &mut Self {
        match ins.len() {
            0 => panic!("you cannot pass an empty vector to the ins argument"),
            _ => ()
        }

        self.query = format!("{} AND {} NOT IN (", self.query, column);

        let length_of_ins = ins.len();

        for (index, value) in ins.into_iter().enumerate() {
            if index + 1 == length_of_ins {
                self.query = format!("{}{})", self.query, value);
                    
                continue;
            }

            self.query = format!("{}{}, ", self.query, value);
        }

        self.list.push(KeywordList::AndNotIn);
        self
    }

    /// It adds the "IN" keyword with it's synthax, with 'AND' keyword except 'WHERE'.
    ///     
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType};
    /// 
    /// fn main(){
    ///     let ins = vec![ValueType::Int16(1), ValueType::Int64(5), ValueType::Int32(10)];
    ///     let query = QueryBuilder::select(vec!["*"]).unwrap()
    ///                              .table("users")
    ///                              .where_("class", "=", ValueType::String("10/c".to_string()))
    ///                              .and_in_custom("id", "1, 5, 10")
    ///                              .finish();
    /// 
    ///     assert_eq!(query, "SELECT * FROM users WHERE class = '10/c' AND id IN (1, 5, 10);")
    /// }
    pub fn and_in_custom(&mut self, column: &str, query: &str) -> &mut Self {
        self.query = format!("{} AND {} IN ({})", self.query, column, query);

        self.list.push(KeywordList::AndIn);
        self
    }

    /// It adds the "IN" keyword with it's synthax, with 'AND' keyword except 'WHERE'.
    ///     
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType};
    /// 
    /// fn main(){
    ///     let ins = vec![ValueType::Int16(1), ValueType::Int64(5), ValueType::Int32(10)];
    ///     let query = QueryBuilder::select(vec!["*"]).unwrap()
    ///                              .table("users")
    ///                              .where_("class", "=", ValueType::String("10/c".to_string()))
    ///                              .and_not_in_custom("id", "1, 5, 10")
    ///                              .finish();
    /// 
    ///     assert_eq!(query, "SELECT * FROM users WHERE class = '10/c' AND id NOT IN (1, 5, 10);")
    /// }
    pub fn and_not_in_custom(&mut self, column: &str, query: &str) -> &mut Self {
        self.query = format!("{} AND {} NOT IN ({})", self.query, column, query);

        self.list.push(KeywordList::AndNotIn);

        self
    }

    /// It adds the "IN" keyword with it's synthax, with 'OR' keyword except 'WHERE'.
    ///     
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType};
    /// 
    /// fn main(){
    ///     let ins = vec![ValueType::Int16(1), ValueType::Int64(5), ValueType::Int32(10)];
    ///     let query = QueryBuilder::select(vec!["*"]).unwrap()
    ///                              .table("users")
    ///                              .where_("class", "=", ValueType::String("10/c".to_string()))
    ///                              .or_in("id", &ins)
    ///                              .finish();
    /// 
    ///     assert_eq!(query, "SELECT * FROM users WHERE class = '10/c' OR id IN (1, 5, 10);")
    /// }
    pub fn or_in(&mut self, column: &str, ins: &Vec<ValueType>) -> &mut Self {
        match ins.len() {
            0 => panic!("you cannot pass an empty vector to the ins argument"),
            _ => ()
        }

        self.query = format!("{} OR {} IN (", self.query, column);

        let length_of_ins = ins.len();

        for (index, value) in ins.into_iter().enumerate() {
            if index + 1 == length_of_ins {
                self.query = format!("{}{})", self.query, value);
                    
                continue;
            }

            self.query = format!("{}{}, ", self.query, value);
        }

        self.list.push(KeywordList::AndIn);
        self
    }

    /// It adds the "NOT IN" keyword with it's synthax, with 'OR' keyword except 'WHERE'.
    ///     
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType};
    /// 
    /// fn main(){
    ///     let ins = vec![ValueType::Int16(1), ValueType::Int64(5), ValueType::Int32(10)];
    ///     let query = QueryBuilder::select(vec!["*"]).unwrap()
    ///                              .table("users")
    ///                              .where_("class", "=", ValueType::String("10/c".to_string()))
    ///                              .or_not_in("id", &ins)
    ///                              .finish();
    /// 
    ///     assert_eq!(query, "SELECT * FROM users WHERE class = '10/c' OR id NOT IN (1, 5, 10);")
    /// }
    pub fn or_not_in(&mut self, column: &str, ins: &Vec<ValueType>) -> &mut Self {
        match ins.len() {
            0 => panic!("you cannot pass an empty vector to the ins argument"),
            _ => ()
        }

        self.query = format!("{} OR {} NOT IN (", self.query, column);

        let length_of_ins = ins.len();

        for (index, value) in ins.into_iter().enumerate() {
            if index + 1 == length_of_ins {
                self.query = format!("{}{})", self.query, value);
                    
                continue;
            }

            self.query = format!("{}{}, ", self.query, value);
        }

        self.list.push(KeywordList::AndNotIn);
        self
    }

    /// It adds the "IN" keyword with it's synthax, with 'AND' keyword except 'WHERE'.
    ///     
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType};
    /// 
    /// fn main(){
    ///     let ins = vec![ValueType::Int16(1), ValueType::Int64(5), ValueType::Int32(10)];
    ///     let query = QueryBuilder::select(vec!["*"]).unwrap()
    ///                              .table("users")
    ///                              .where_("class", "=", ValueType::String("10/c".to_string()))
    ///                              .or_in_custom("id", "1, 5, 10")
    ///                              .finish();
    /// 
    ///     assert_eq!(query, "SELECT * FROM users WHERE class = '10/c' OR id IN (1, 5, 10);")
    /// }
    pub fn or_in_custom(&mut self, column: &str, query: &str) -> &mut Self {
        self.query = format!("{} OR {} IN ({})", self.query, column, query);

        self.list.push(KeywordList::AndIn);
        self
    }

    /// It adds the "IN" keyword with it's synthax, with 'AND' keyword except 'WHERE'.
    ///     
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType};
    /// 
    /// fn main(){
    ///     let ins = vec![ValueType::Int16(1), ValueType::Int64(5), ValueType::Int32(10)];
    ///     let query = QueryBuilder::select(vec!["*"]).unwrap()
    ///                              .table("users")
    ///                              .where_("class", "=", ValueType::String("10/c".to_string()))
    ///                              .or_not_in_custom("id", "1, 5, 10")
    ///                              .finish();
    /// 
    ///     assert_eq!(query, "SELECT * FROM users WHERE class = '10/c' OR id NOT IN (1, 5, 10);")
    /// }
    /// 
    /// ```
    pub fn or_not_in_custom(&mut self, column: &str, query: &str) -> &mut Self {
        self.query = format!("{} OR {} NOT IN ({})", self.query, column, query);

        self.list.push(KeywordList::AndNotIn);

        self
    }

    /// it opens a parenthesis without declaring it's first parameter. It adds this string to the query: "... (WHERE / AND / OR) (" It's suitable for more custom approach with parenthesis, if you only want to use WHERE, AND & OR queries, we suggest you to check `.open_parenthesis_with()` method.
    /// 
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType};
    /// 
    /// fn main {
    ///    let query = QueryBuilder::select(vec!["*"]).unwrap()
    ///                             .table("users")
    ///                             .where_("grades", ">", ValueType::Int32(80))
    ///                             .open_parenthesis(BracketType::And)
    ///                             .and("height", ">", ValueType::Int32(170))
    ///                             .or("weight", ">", ValueType::Int32(60))
    ///                             .close_parenthesis() // you have to close the parenthesis, otherwise it'll panic.
    ///                             .finish();
    ///
    ///    assert_eq!(query, "SELECT * FROM users WHERE grades > 80 AND ( AND height > 170 OR weight > 60);");
    /// 
    /// }
    /// 
    /// ```
    pub fn open_parenthesis(&mut self, parenthesis_type: BracketType) -> &mut Self {
        match self.list.last() {
            Some(keyword) => match keyword {
                _ => {
                    self.query = format!("{} {} (", self.query, parenthesis_type);
                    
                    match parenthesis_type {
                        BracketType::Where => self.list.push(KeywordList::LeftBracketWhere),
                        BracketType::And => self.list.push(KeywordList::LeftBracketAnd),
                        BracketType::Or => self.list.push(KeywordList::LeftBracketOr),
                    }
                }
            },
            None => panic!("that's impossible to come here.")
        };

        self
    }

    /// it opens a parenthesis with declaring it's first parameter, it suits very well the most common use cases of parenthesis.
    /// 
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType};
    /// 
    /// fn main {
    /// 
    ///    let query = QueryBuilder::select(vec!["*"]).unwrap()
    ///                                    .table("users")
    ///                                    .where_("grades", ">", ValueType::Int32(80))
    ///                                    .open_parenthesis_with(BracketType::And, "height", ">", ValueType::Int32(170))
    ///                                    .open_parenthesis_with(BracketType::Or, "weight", ">", ValueType::Int32(50))
    ///                                    .and("weight", "<", ValueType::Int32(70))
    ///                                    .close_parenthesis()
    ///                                    .close_parenthesis()
    ///                                    .finish();          
    ///
    ///    assert_eq!(query, "SELECT * FROM users WHERE grades > 80 AND (height > 170 OR (weight > 50 AND weight < 70));");    
    /// 
    /// }
    /// 
    /// ```
    pub fn open_parenthesis_with(&mut self, parenthesis_type: BracketType, column: &str, mut mark: &str, value: ValueType) -> &mut Self {
        if let ValueType::Null = value {
            match mark {
                "=" => mark = "IS",
                "!=" | "<>" => mark = "IS NOT",
                "IS" | "IS NOT" => (),
                _ => mark = "IS"
            }
        }

        match self.list.last() {
            Some(keyword) => match keyword {
                _ => {
                    self.query = format!("{} {} ({} {} {}", self.query, parenthesis_type, column, mark, value);
                    
                    match parenthesis_type {
                        BracketType::Where => self.list.push(KeywordList::LeftBracketWhere),
                        BracketType::And => self.list.push(KeywordList::LeftBracketAnd),
                        BracketType::Or => self.list.push(KeywordList::LeftBracketOr),
                    }
                }
            },
            None => panic!("that's impossible to come here.")
        };

        self
    }

    /// Parenthesis closer for open parenthesis. The combined amount of `.open_parenthesis()` and `.open_parenthesis_with()` must match to be the query produced correctly. It'll panic if there is no call of these methods or query don't have "(" character.
    pub fn close_parenthesis(&mut self) -> &mut Self {
        if !self.list.iter().any(|keyword| keyword == &KeywordList::LeftBracketWhere || keyword == &KeywordList::LeftBracketAnd || keyword == &KeywordList::LeftBracketOr) {
            if !self.query.contains("(") {
                panic!("There is no left bracket exists on that query, panicking....")
            }
        } else {
            self.query = format!("{})", self.query)
        }

        self
    }

    /// it benefits to set timezone when you make your query. It's very flexible, always put on very beginning of the query, you can use it later than any other method.
    pub fn time_zone(&mut self, timezone: Timezone) -> &mut Self {
        self.query = format!("SET time_zone = {}; {}", timezone, self.query);

        self.list.insert(1, KeywordList::Timezone);
        self
    }

    /// it benefits to set global timezone when you make your query. It's very flexible, always put on very beginning of the query, you can use it later than any other method.
    pub fn global_time_zone(&mut self, timezone: Timezone) -> &mut Self {
        self.query = format!("SET GLOBAL time_zone = {}; {}", timezone, self.query);

        self.list.insert(1, KeywordList::GlobalTimezone);
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
   pub fn or(&mut self, column: &str, mut mark: &str, value: ValueType) -> &mut Self {
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

        if let ValueType::Null = value {
            match mark {
                "=" => mark = "IS",
                "!=" | "<>" => mark = "IS NOT",
                "IS" | "IS NOT" => (),
                _ => mark = "IS"
            }
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
    pub fn and(&mut self, column: &str, mut mark: &str, value: ValueType) -> &mut Self {
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

        if let ValueType::Null = value {
            match mark {
                "=" => mark = "IS",
                "!=" | "<>" => mark = "IS NOT",
                "IS" | "IS NOT" => (),
                _ => mark = "IS"
            }
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
                        if keyword == &KeywordList::Where || keyword == &KeywordList::WhereIn || keyword == &KeywordList::WhereNotIn {
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
                        } else if keyword == &KeywordList::LeftBracketWhere || keyword == &KeywordList::LeftBracketAnd || keyword == &KeywordList::LeftBracketOr {
                            for (i, column) in columns.into_iter().enumerate() {
                                if i == 0 {
                                    self.query = format!("{}{} LIKE '%{}%'", self.query, column, operand)
                                } else {
                                    self.query = format!("{}, AND {} LIKE '%{}%'", self.query, column, operand)
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

    pub fn having(&mut self, column: &str, mut mark: &str, value: ValueType) -> &mut Self {
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

        if let ValueType::Null = value {
            match mark {
                "=" => mark = "IS",
                "!=" | "<>" => mark = "IS NOT",
                "IS" | "IS NOT" => (),
                _ => mark = "IS"
            }
        }

        self.query = format!("{} HAVING {} {} {}", self.query, column, mark, value);

        self.list.push(KeywordList::Having);

        self
    }

    /// it adds the `INNER JOIN` keyword with it's synthax.
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType};
    /// 
    /// fn main(){ 
    ///    let query = QueryBuilder::select(vec!["*"]).unwrap()
    ///                             .table("students s")
    ///                             .inner_join("grades g", "s.id", "=", "g.student_id")
    ///                             .where_("id", "=", ValueType::Int32(10))
    ///                             .finish();
    ///
    ///    assert_eq!(query, "SELECT * FROM students s INNER JOIN grades g ON s.id = g.student_id WHERE id = 10;");
    /// }
    /// 
    /// ```
    pub fn inner_join(&mut self, table: &str, left: &str, mark: &str, right: &str) -> &mut Self {
        self.query = format!("{} INNER JOIN {} ON {} {} {}", self.query, table, left, mark, right);
        self.list.push(KeywordList::InnerJoin);
        self
    }

     /// it adds the `LEFT JOIN` keyword with it's synthax.
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType};
    /// 
    /// fn main(){ 
    ///    let query = QueryBuilder::select(vec!["*"]).unwrap()
    ///                             .table("students s")
    ///                             .left_join("grades g", "s.id", "=", "g.student_id")
    ///                             .where_("id", "=", ValueType::Int32(10))
    ///                             .finish();
    ///
    ///    assert_eq!(query, "SELECT * FROM students s LEFT JOIN grades g ON s.id = g.student_id WHERE id = 10;");
    /// }
    /// 
    /// ```
    pub fn left_join(&mut self, table: &str, left: &str, mark: &str, right: &str) -> &mut Self {
        self.query = format!("{} LEFT JOIN {} ON {} {} {}", self.query, table, left, mark, right);
        self.list.push(KeywordList::LeftJoin);
        self
    }

    /// it adds the `RIGHT JOIN` keyword with it's synthax.
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType};
    /// 
    /// fn main(){ 
    ///    let query = QueryBuilder::select(vec!["*"]).unwrap()
    ///                             .table("students s")
    ///                             .right_join("grades g", "s.id", "=", "g.student_id")
    ///                             .where_("id", "=", ValueType::Int32(10))
    ///                             .finish();
    ///
    ///    assert_eq!(query, "SELECT * FROM students s RIGHT JOIN grades g ON s.id = g.student_id WHERE id = 10;");
    /// }
    /// 
    /// ```
    pub fn right_join(&mut self, table: &str, left: &str, mark: &str, right: &str) -> &mut Self {
        self.query = format!("{} RIGHT JOIN {} ON {} {} {}", self.query, table, left, mark, right);
        self.list.push(KeywordList::RightJoin);
        self
    }

    /// it adds the `CROSS JOIN` keyword with it's synthax.
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType};
    /// 
    /// fn main(){ 
    ///    let query = QueryBuilder::select(vec!["*"]).unwrap()
    ///                             .table("students s")
    ///                             .cross_join("grades g")
    ///                             .where_("id", "=", ValueType::Int32(10))
    ///                             .finish();
    ///
    ///    assert_eq!(query, "SELECT * FROM students s RIGHT JOIN grades g ON s.id = g.student_id WHERE id = 10;");
    /// }
    /// 
    /// ```
    pub fn cross_join(&mut self, table: &str) -> &mut Self {
        self.query = format!("{} CROSS JOIN {}", self.query, table);
        self.list.push(KeywordList::RightJoin);
        self
    }

    /// it adds the `NATURAL JOIN` keyword with it's synthax.
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType};
    /// 
    /// fn main(){ 
    ///    let query = QueryBuilder::select(vec!["*"]).unwrap()
    ///                             .table("students s")
    ///                             .natural_join("grades g")
    ///                             .where_("id", "=", ValueType::Int32(10))
    ///                             .finish();
    ///
    ///    assert_eq!(query, "SELECT * FROM students s RIGHT JOIN grades g ON s.id = g.student_id WHERE id = 10;");
    /// }
    /// 
    /// ```
    pub fn natural_join(&mut self, table: &str) -> &mut Self {
        self.query = format!("{} NATURAL JOIN {}", self.query, table);
        self.list.push(KeywordList::RightJoin);
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
                    KeywordList::LeftBracketWhere | KeywordList::LeftBracketAnd | KeywordList::LeftBracketOr => {
                        panic!("you cannot use .json_extract() method with bracket methods for now, this will be implemented on future updates.")
                    },
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
    /// use qubl::{QueryBuilder, ValueType, JsonValue};
    /// 
    /// fn main(){
    ///     let value = ValueType::String("blablabla.jpg".to_string());
    ///     let prop = vec![("name", &value)];
    /// 
    ///     let object = JsonValue::MysqlJsonObject(&prop);
    /// 
    ///     let query = QueryBuilder::select(["*"].to_vec()).unwrap()
    ///                              .table("users")
    ///                              .where_("pic", "=", ValueType::String("".to_string()))
    ///                              .json_contains("pic", object, Some(".name"))
    ///                              .finish();
    /// 
    ///     assert_eq!(query, "SELECT * FROM users WHERE JSON_CONTAINS(pic, JSON_OBJECT('name', 'blablabla.jpg'), '$.name');")
    /// }
    /// 
    /// ```
    pub fn json_contains(&mut self, column: &str, needle: JsonValue, path: Option<&str>) -> &mut Self {
        match self.list.last().unwrap() {
            KeywordList::Select => match path {
                Some(path) => match needle {
                    JsonValue::Initial(initial) => match initial {
                        ValueType::JsonString(needle) => self.query = format!("SELECT JSON_CONTAINS({}, '\"{}\"', '${}') FROM", column, needle, path),
                        ValueType::String(needle) => self.query = format!("SELECT JSON_CONTAINS({}, '{}', '${}') FROM", column, needle, path),
                        ValueType::Datetime(needle) => self.query = format!("SELECT JSON_CONTAINS({}, '{}', '${}') FROM", column, needle, path),
                        _ => self.query = format!("SELECT JSON_CONTAINS({}, {}, '${}') FROM", column, needle, path),
                    },
                    _ => self.query = format!("SELECT JSON_CONTAINS({}, {}, '${}') FROM", column, needle, path),
                }
                None => match needle {
                    JsonValue::Initial(initial) => match initial {
                        ValueType::JsonString(needle) => self.query = format!("SELECT JSON_CONTAINS({}, '\"{}\"') FROM", column, needle),
                        ValueType::String(needle) => self.query = format!("SELECT JSON_CONTAINS({}, '{}') FROM", column, needle),
                        ValueType::Datetime(needle) => self.query = format!("SELECT JSON_CONTAINS({}, '{}') FROM", column, needle),
                        _ => self.query = format!("SELECT JSON_CONTAINS({}, {}) FROM", column, needle)
                    },
                    _ => self.query = format!("SELECT JSON_CONTAINS({}, {}) FROM", column, needle)
                }
            },
            KeywordList::Where => match path {
                Some(path) => {
                    let mut split_the_query = self.query.split(" WHERE ");

                    let first_half = split_the_query.nth(0);

                    match needle {
                        JsonValue::Initial(initial) => match initial {
                            ValueType::JsonString(needle) => self.query = format!("{} WHERE JSON_CONTAINS({}, '\"{}\"', '${}')", first_half.unwrap(), column, needle, path),
                            ValueType::String(needle) => self.query = format!("{} WHERE JSON_CONTAINS({}, '{}', '${}')", first_half.unwrap(), column, needle, path),
                            ValueType::Datetime(needle) => self.query = format!("{} WHERE JSON_CONTAINS({}, '{}', '${}')", first_half.unwrap(), column, needle, path),
                            _ => self.query = format!("{} WHERE JSON_CONTAINS({}, {}, '${}')", first_half.unwrap(), column, needle, path)
                        },
                        _ => self.query = format!("{} WHERE JSON_CONTAINS({}, {}, '${}')", first_half.unwrap(), column, needle, path)
                    }
                },
                None => {
                    let mut split_the_query = self.query.split(" WHERE ");

                    let first_half = split_the_query.nth(0);

                    match needle {
                        JsonValue::Initial(initial) => match initial {
                            ValueType::JsonString(needle) => self.query = format!("{} WHERE JSON_CONTAINS({}, '\"{}\"')", first_half.unwrap(), column, needle),
                            ValueType::String(needle) => self.query = format!("{} WHERE JSON_CONTAINS({}, '{}')", first_half.unwrap(), column, needle),
                            ValueType::Datetime(needle) => self.query = format!("{} WHERE JSON_CONTAINS({}, '{}')", first_half.unwrap(), column, needle),
                            _ => self.query = format!("{} WHERE JSON_CONTAINS({}, {})", first_half.unwrap(), column, needle)
                        },
                        _ => self.query = format!("{} WHERE JSON_CONTAINS({}, {})", first_half.unwrap(), column, needle)
                    }
                }
            },
            KeywordList::LeftBracketWhere => match path {
                Some(path) => {
                    let mut split_the_query = self.query.split(" WHERE ");

                    let first_half = split_the_query.nth(0);

                    match needle {
                        JsonValue::Initial(initial) => match initial {
                            ValueType::JsonString(needle) => self.query = format!("{} WHERE (JSON_CONTAINS({}, '\"{}\"', '${}')", first_half.unwrap(), column, needle, path),
                            ValueType::String(needle) => self.query = format!("{} WHERE (JSON_CONTAINS({}, '{}', '${}')", first_half.unwrap(), column, needle, path),
                            ValueType::Datetime(needle) => self.query = format!("{} WHERE (JSON_CONTAINS({}, '{}', '${}')", first_half.unwrap(), column, needle, path),
                            _ => self.query = format!("{} WHERE (JSON_CONTAINS({}, {}, '${}')", first_half.unwrap(), column, needle, path)
                        },
                        _ => self.query = format!("{} WHERE (JSON_CONTAINS({}, {}, '${}')", first_half.unwrap(), column, needle, path)
                    }
                },
                None => {
                    let mut split_the_query = self.query.split(" WHERE ");

                    let first_half = split_the_query.nth(0);

                    match needle {
                        JsonValue::Initial(initial) => match initial {
                            ValueType::JsonString(needle) => self.query = format!("{} WHERE (JSON_CONTAINS({}, '\"{}\"')", first_half.unwrap(), column, needle),
                            ValueType::String(needle) => self.query = format!("{} WHERE (JSON_CONTAINS({}, '{}')", first_half.unwrap(), column, needle),
                            ValueType::Datetime(needle) => self.query = format!("{} WHERE (JSON_CONTAINS({}, '{}')", first_half.unwrap(), column, needle),
                            _ => self.query = format!("{} WHERE (JSON_CONTAINS({}, {})", first_half.unwrap(), column, needle)
                        },
                        _ => self.query = format!("{} WHERE (JSON_CONTAINS({}, {})", first_half.unwrap(), column, needle)
                    }
                }
            },
            KeywordList::And => match path {
                Some(path) => {
                    let split_the_query = self.query.split(" AND ").collect::<Vec<&str>>();

                    let length_of_the_split_the_query = split_the_query.len();

                    match split_the_query.len() {
                        0 => panic!("There Is No AND query in QueryBuilder but The AND keyword exist in keyword list, panicking."),
                        1 => panic!("There Is No AND query in QueryBuilder but The AND keyword exist in keyword list, panicking."),
                        2 => match needle {
                            JsonValue::Initial(initial) => match initial {
                                ValueType::JsonString(needle) => self.query = format!("{} AND JSON_CONTAINS({}, '\"{}\"', '${}')", split_the_query[0], column, needle, path),
                                ValueType::String(needle) => self.query = format!("{} AND JSON_CONTAINS({}, '{}', '${}')", split_the_query[0], column, needle, path),
                                ValueType::Datetime(needle) => self.query = format!("{} AND JSON_CONTAINS({}, '{}', '${}')", split_the_query[0], column, needle, path),
                                _ => self.query = format!("{} AND JSON_CONTAINS({}, {}, '${}')", split_the_query[0], column, needle, path)
                            },
                            _ => self.query = format!("{} AND JSON_CONTAINS({}, {}, '${}')", split_the_query[0], column, needle, path)
                        },
                        _ => {
                            let mut concatenated_string = String::new();

                            for (index, chunk) in  split_the_query.into_iter().enumerate() {
                                if index == 0 {
                                    concatenated_string = chunk.to_string();
                                } else if index + 1 != length_of_the_split_the_query {
                                    concatenated_string = format!("{} AND {} ", concatenated_string, chunk)
                                }
                            }

                            match needle {
                                JsonValue::Initial(initial) => match initial {
                                    ValueType::JsonString(needle) => self.query = format!("{}AND JSON_CONTAINS({}, '\"{}\"', '${}')", concatenated_string, column, needle, path),
                                    ValueType::String(needle) => self.query = format!("{}AND JSON_CONTAINS({}, '{}', '${}')", concatenated_string, column, needle, path),
                                    ValueType::Datetime(needle) => self.query = format!("{}AND JSON_CONTAINS({}, '{}', '${}')", concatenated_string, column, needle, path),
                                    _ => self.query = format!("{}AND JSON_CONTAINS({}, {}, '${}')", concatenated_string, column, needle, path)
                                },
                                _ => self.query = format!("{}AND JSON_CONTAINS({}, {}, '${}')", concatenated_string, column, needle, path)
                            }
                        }
                    }
                },
                None => {
                    let split_the_query = self.query.split(" AND ").collect::<Vec<&str>>();

                    let length_of_the_split_the_query = split_the_query.len();

                    match split_the_query.len() {
                        0 => panic!("There Is No AND query in QueryBuilder but The AND keyword exist in keyword list, panicking."),
                        1 => panic!("There Is No AND query in QueryBuilder but The AND keyword exist in keyword list, panicking."),
                        2 => match needle {
                            JsonValue::Initial(initial) => match initial {
                                ValueType::JsonString(needle) => self.query = format!("{} AND JSON_CONTAINS({}, '\"{}\"')",  split_the_query[0], column, needle),
                                ValueType::String(needle) => self.query = format!("{} AND JSON_CONTAINS({}, '{}')",  split_the_query[0], column, needle),
                                ValueType::Datetime(needle) => self.query = format!("{} AND JSON_CONTAINS({}, '{}')",  split_the_query[0], column, needle),
                                _ => self.query = format!("{} AND JSON_CONTAINS({}, {})",  split_the_query[0], column, needle)
                            },
                            _ => self.query = format!("{} AND JSON_CONTAINS({}, {})",  split_the_query[0], column, needle)
                        },
                        _ => {
                            let mut concatenated_string = String::new();

                            for (index, chunk) in  split_the_query.into_iter().enumerate() {
                                if index == 0 {
                                    concatenated_string = chunk.to_string();
                                } else if index + 1 != length_of_the_split_the_query {
                                    concatenated_string = format!("{} AND {} ", concatenated_string, chunk)
                                }
                            }

                            match needle {
                                JsonValue::Initial(initial) => match initial {
                                    ValueType::JsonString(needle) => self.query = format!("{}AND JSON_CONTAINS({}, '\"{}\"')", concatenated_string, column, needle),
                                    ValueType::String(needle) => self.query = format!("{}AND JSON_CONTAINS({}, '{}')", concatenated_string, column, needle),
                                    ValueType::Datetime(needle) => self.query = format!("{}AND JSON_CONTAINS({}, '{}')", concatenated_string, column, needle),
                                    _ => self.query = format!("{}AND JSON_CONTAINS({}, {})", concatenated_string, column, needle)
                                },
                                _ => self.query = format!("{}AND JSON_CONTAINS({}, {})", concatenated_string, column, needle)
                            }
                        }
                    }
                }
            },
            KeywordList::LeftBracketAnd => match path {
                Some(path) => {
                    let split_the_query = self.query.split(" AND ").collect::<Vec<&str>>();

                    let length_of_the_split_the_query = split_the_query.len();

                    match split_the_query.len() {
                        0 => panic!("There Is No AND query in QueryBuilder but The AND keyword exist in keyword list, panicking."),
                        1 => panic!("There Is No AND query in QueryBuilder but The AND keyword exist in keyword list, panicking."),
                        2 => match needle {
                            JsonValue::Initial(initial) => match initial {
                                ValueType::JsonString(needle) => self.query = format!("{} AND (JSON_CONTAINS({}, '\"{}\"', '${}')", split_the_query[0], column, needle, path),
                                ValueType::String(needle) => self.query = format!("{} AND 8JSON_CONTAINS({}, '{}', '${}')", split_the_query[0], column, needle, path),
                                ValueType::Datetime(needle) => self.query = format!("{} AND (JSON_CONTAINS({}, '{}', '${}')", split_the_query[0], column, needle, path),
                                _ => self.query = format!("{} AND (JSON_CONTAINS({}, {}, '${}')", split_the_query[0], column, needle, path)
                            },
                            _ => self.query = format!("{} AND (JSON_CONTAINS({}, {}, '${}')", split_the_query[0], column, needle, path)
                        },
                        _ => {
                            let mut concatenated_string = String::new();

                            for (index, chunk) in  split_the_query.into_iter().enumerate() {
                                if index == 0 {
                                    concatenated_string = chunk.to_string();
                                } else if index + 1 != length_of_the_split_the_query {
                                    concatenated_string = format!("{} AND {} ", concatenated_string, chunk)
                                }
                            }

                            match needle {
                                JsonValue::Initial(initial) => match initial {
                                    ValueType::JsonString(needle) => self.query = format!("{}AND (JSON_CONTAINS({}, '\"{}\"', '${}')", concatenated_string, column, needle, path),
                                    ValueType::String(needle) => self.query = format!("{}AND (JSON_CONTAINS({}, '{}', '${}')", concatenated_string, column, needle, path),
                                    ValueType::Datetime(needle) => self.query = format!("{}AND (JSON_CONTAINS({}, '{}', '${}')", concatenated_string, column, needle, path),
                                    _ => self.query = format!("{}AND (JSON_CONTAINS({}, {}, '${}')", concatenated_string, column, needle, path)
                                },
                                _ => self.query = format!("{}AND (JSON_CONTAINS({}, {}, '${}')", concatenated_string, column, needle, path)
                            }
                        }
                    }
                },
                None => {
                    let split_the_query = self.query.split(" AND ").collect::<Vec<&str>>();

                    let length_of_the_split_the_query = split_the_query.len();

                    match split_the_query.len() {
                        0 => panic!("There Is No AND query in QueryBuilder but The AND keyword exist in keyword list, panicking."),
                        1 => panic!("There Is No AND query in QueryBuilder but The AND keyword exist in keyword list, panicking."),
                        2 => match needle {
                            JsonValue::Initial(initial) => match initial {
                                ValueType::JsonString(needle) => self.query = format!("{} AND (JSON_CONTAINS({}, '\"{}\"')",  split_the_query[0], column, needle),
                                ValueType::String(needle) => self.query = format!("{} AND (JSON_CONTAINS({}, '{}')",  split_the_query[0], column, needle),
                                ValueType::Datetime(needle) => self.query = format!("{} AND (JSON_CONTAINS({}, '{}')",  split_the_query[0], column, needle),
                                _ => self.query = format!("{} AND (JSON_CONTAINS({}, {})",  split_the_query[0], column, needle)
                            },
                            _ => self.query = format!("{} AND (JSON_CONTAINS({}, {})",  split_the_query[0], column, needle)
                        },
                        _ => {
                            let mut concatenated_string = String::new();

                            for (index, chunk) in  split_the_query.into_iter().enumerate() {
                                if index == 0 {
                                    concatenated_string = chunk.to_string();
                                } else if index + 1 != length_of_the_split_the_query {
                                    concatenated_string = format!("{} AND {} ", concatenated_string, chunk)
                                }
                            }

                            match needle {
                                JsonValue::Initial(initial) => match initial {
                                    ValueType::JsonString(needle) => self.query = format!("{}AND (JSON_CONTAINS({}, '\"{}\"')", concatenated_string, column, needle),
                                    ValueType::String(needle) => self.query = format!("{}AND (JSON_CONTAINS({}, '{}')", concatenated_string, column, needle),
                                    ValueType::Datetime(needle) => self.query = format!("{}AND (JSON_CONTAINS({}, '{}')", concatenated_string, column, needle),
                                    _ => self.query = format!("{}AND (JSON_CONTAINS({}, {})", concatenated_string, column, needle)
                                },
                                _ => self.query = format!("{}AND (JSON_CONTAINS({}, {})", concatenated_string, column, needle)
                            }
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
                        2 => match needle {
                            JsonValue::Initial(initial) => match initial {
                                ValueType::JsonString(needle) => self.query = format!("{} OR JSON_CONTAINS({}, '\"{}\"', '${}')", split_the_query[0], column, needle, path),
                                ValueType::String(needle) => self.query = format!("{} OR JSON_CONTAINS({}, '{}', '${}')", split_the_query[0], column, needle, path),
                                ValueType::Datetime(needle) => self.query = format!("{} OR JSON_CONTAINS({}, '{}', '${}')", split_the_query[0], column, needle, path),
                                _ => self.query = format!("{} OR JSON_CONTAINS({}, {}, '${}')", split_the_query[0], column, needle, path)
                            },
                            _ => self.query = format!("{} OR JSON_CONTAINS({}, {}, '${}')", split_the_query[0], column, needle, path)
                        },
                        _ => {
                            let mut concatenated_string = String::new();

                            for (index, chunk) in  split_the_query.into_iter().enumerate() {
                                if index == 0 {
                                    concatenated_string = chunk.to_string();
                                } else if index + 1 != length_of_the_split_the_query {
                                    concatenated_string = format!("{} OR {} ", concatenated_string, chunk)
                                }
                            }

                            match needle {
                                JsonValue::Initial(initial) => match initial {
                                    ValueType::JsonString(needle) => self.query = format!("{}OR JSON_CONTAINS({}, '\"{}\"', '${}')", concatenated_string, column, needle, path),
                                    ValueType::String(needle) => self.query = format!("{}OR JSON_CONTAINS({}, '{}', '${}')", concatenated_string, column, needle, path),
                                    ValueType::Datetime(needle) => self.query = format!("{}OR JSON_CONTAINS({}, '{}', '${}')", concatenated_string, column, needle, path),
                                    _ => self.query = format!("{}OR JSON_CONTAINS({}, {}, '${}')", concatenated_string, column, needle, path)
                                },
                                _ => self.query = format!("{}OR JSON_CONTAINS({}, {}, '${}')", concatenated_string, column, needle, path)
                            }
                        }
                    }
                },
                None => {
                    let split_the_query = self.query.split(" OR ").collect::<Vec<&str>>();

                    let length_of_the_split_the_query = split_the_query.len();

                    match split_the_query.len() {
                        0 => panic!("There Is No OR query in QueryBuilder but The OR keyword exist in keyword list, panicking."),
                        1 => panic!("There Is No OR query in QueryBuilder but The OR keyword exist in keyword list, panicking."),
                        2 => match needle {
                            JsonValue::Initial(initial) => match initial {
                                ValueType::JsonString(needle) => self.query = format!("{} OR JSON_CONTAINS({}, '\"{}\"')",  split_the_query[0], column, needle),
                                ValueType::String(needle) => self.query = format!("{} OR JSON_CONTAINS({}, '{}')",  split_the_query[0], column, needle),
                                ValueType::Datetime(needle) => self.query = format!("{} OR JSON_CONTAINS({}, '{}')",  split_the_query[0], column, needle),
                                _ => self.query = format!("{} OR JSON_CONTAINS({}, {})",  split_the_query[0], column, needle)
                            },
                            _ => self.query = format!("{} OR JSON_CONTAINS({}, {})",  split_the_query[0], column, needle)
                        },
                        _ => {
                            let mut concatenated_string = String::new();

                            for (index, chunk) in  split_the_query.into_iter().enumerate() {
                                if index == 0 {
                                    concatenated_string = chunk.to_string();
                                } else if index + 1 != length_of_the_split_the_query {
                                    concatenated_string = format!("{} OR {} ", concatenated_string, chunk)
                                }
                            }

                            match needle {
                                JsonValue::Initial(initial) => match initial {
                                    ValueType::JsonString(needle) => self.query = format!("{}OR JSON_CONTAINS({}, '\"{}\"')", concatenated_string, column, needle),
                                    ValueType::String(needle) => self.query = format!("{}OR JSON_CONTAINS({}, '{}')", concatenated_string, column, needle),
                                    ValueType::Datetime(needle) => self.query = format!("{}OR JSON_CONTAINS({}, '{}')", concatenated_string, column, needle),
                                    _ => self.query = format!("{}OR JSON_CONTAINS({}, {})", concatenated_string, column, needle)
                                },
                                _ => self.query = format!("{}OR JSON_CONTAINS({}, {})", concatenated_string, column, needle)
                            }
                        }
                    }
                }
            },
            KeywordList::LeftBracketOr => match path {
                Some(path) => {
                    let split_the_query = self.query.split(" OR ").collect::<Vec<&str>>();

                    let length_of_the_split_the_query = split_the_query.len();

                    match split_the_query.len() {
                        0 => panic!("There Is No OR query in QueryBuilder but The OR keyword exist in keyword list, panicking."),
                        1 => panic!("There Is No OR query in QueryBuilder but The OR keyword exist in keyword list, panicking."),
                        2 => match needle {
                            JsonValue::Initial(initial) => match initial {
                                ValueType::JsonString(needle) => self.query = format!("{} OR (JSON_CONTAINS({}, '\"{}\"', '${}')", split_the_query[0], column, needle, path),
                                ValueType::String(needle) => self.query = format!("{} OR (JSON_CONTAINS({}, '{}', '${}')", split_the_query[0], column, needle, path),
                                ValueType::Datetime(needle) => self.query = format!("{} OR (JSON_CONTAINS({}, '{}', '${}')", split_the_query[0], column, needle, path),
                                _ => self.query = format!("{} OR (JSON_CONTAINS({}, {}, '${}')", split_the_query[0], column, needle, path)
                            },
                            _ => self.query = format!("{} OR (JSON_CONTAINS({}, {}, '${}')", split_the_query[0], column, needle, path)
                        },
                        _ => {
                            let mut concatenated_string = String::new();

                            for (index, chunk) in  split_the_query.into_iter().enumerate() {
                                if index == 0 {
                                    concatenated_string = chunk.to_string();
                                } else if index + 1 != length_of_the_split_the_query {
                                    concatenated_string = format!("{} OR {} ", concatenated_string, chunk)
                                }
                            }

                            match needle {
                                JsonValue::Initial(initial) => match initial {
                                    ValueType::JsonString(needle) => self.query = format!("{}OR (JSON_CONTAINS({}, '\"{}\"', '${}')", concatenated_string, column, needle, path),
                                    ValueType::String(needle) => self.query = format!("{}OR (JSON_CONTAINS({}, '{}', '${}')", concatenated_string, column, needle, path),
                                    ValueType::Datetime(needle) => self.query = format!("{}OR (JSON_CONTAINS({}, '{}', '${}')", concatenated_string, column, needle, path),
                                    _ => self.query = format!("{}OR (JSON_CONTAINS({}, {}, '${}')", concatenated_string, column, needle, path)
                                },
                                _ => self.query = format!("{}OR (JSON_CONTAINS({}, {}, '${}')", concatenated_string, column, needle, path)
                            }
                        }
                    }
                },
                None => {
                    let split_the_query = self.query.split(" OR ").collect::<Vec<&str>>();

                    let length_of_the_split_the_query = split_the_query.len();

                    match split_the_query.len() {
                        0 => panic!("There Is No OR query in QueryBuilder but The OR keyword exist in keyword list, panicking."),
                        1 => panic!("There Is No OR query in QueryBuilder but The OR keyword exist in keyword list, panicking."),
                        2 => match needle {
                            JsonValue::Initial(initial) => match initial {
                                ValueType::JsonString(needle) => self.query = format!("{} OR (JSON_CONTAINS({}, '\"{}\"')",  split_the_query[0], column, needle),
                                ValueType::String(needle) => self.query = format!("{} OR (JSON_CONTAINS({}, '{}')",  split_the_query[0], column, needle),
                                ValueType::Datetime(needle) => self.query = format!("{} OR (JSON_CONTAINS({}, '{}')",  split_the_query[0], column, needle),
                                _ => self.query = format!("{} OR (JSON_CONTAINS({}, {})",  split_the_query[0], column, needle)
                            },
                            _ => self.query = format!("{} OR (JSON_CONTAINS({}, {})",  split_the_query[0], column, needle)
                        },
                        _ => {
                            let mut concatenated_string = String::new();

                            for (index, chunk) in  split_the_query.into_iter().enumerate() {
                                if index == 0 {
                                    concatenated_string = chunk.to_string();
                                } else if index + 1 != length_of_the_split_the_query {
                                    concatenated_string = format!("{} OR {} ", concatenated_string, chunk)
                                }
                            }

                            match needle {
                                JsonValue::Initial(initial) => match initial {
                                    ValueType::JsonString(needle) => self.query = format!("{}OR (JSON_CONTAINS({}, '\"{}\"')", concatenated_string, column, needle),
                                    ValueType::String(needle) => self.query = format!("{}OR (JSON_CONTAINS({}, '{}')", concatenated_string, column, needle),
                                    ValueType::Datetime(needle) => self.query = format!("{}OR (JSON_CONTAINS({}, '{}')", concatenated_string, column, needle),
                                    _ => self.query = format!("{}OR (JSON_CONTAINS({}, {})", concatenated_string, column, needle)
                                },
                                _ => self.query = format!("{}OR (JSON_CONTAINS({}, {})", concatenated_string, column, needle)
                            }
                        }
                    }
                }
            },
            _ => panic!("Wrong usage of '.json_contains()' method, it should be used later than either SELECT, WHERE, AND, OR keywords.")
        }

        self.list.push(KeywordList::JsonContains);

        self
    }

    /// It applies "NOT JSON_CONTAINS()" mysql function with it's Synthax. If you encounter any syntactic bugs or deficiencies about that function, please report it via opening an issue.
    /// 
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType, JsonValue};
    /// 
    /// fn main(){
    ///     let value = ValueType::String("blablabla.jpg".to_string());
    ///     let prop = vec![("name", &value)];
    /// 
    ///     let object = JsonValue::MysqlJsonObject(&prop);
    /// 
    ///     let query = QueryBuilder::select(["*"].to_vec()).unwrap()
    ///                              .table("users")
    ///                              .where_("pic", "=", ValueType::String("".to_string()))
    ///                              .not_json_contains("pic", object, Some(".name"))
    ///                              .finish();
    /// 
    ///     assert_eq!(query, "SELECT * FROM users WHERE NOT JSON_CONTAINS(pic, JSON_OBJECT('name', 'blablabla.jpg'), '$.name');")
    /// }
    /// 
    /// ```
    pub fn not_json_contains(&mut self, column: &str, needle: JsonValue, path: Option<&str>) -> &mut Self {
        match self.list.last().unwrap() {
            KeywordList::Select => match path {
                Some(path) => match needle {
                    JsonValue::Initial(initial) => match initial {
                        ValueType::JsonString(needle) => self.query = format!("SELECT NOT JSON_CONTAINS({}, '\"{}\"', '${}') FROM", column, needle, path),
                        ValueType::String(needle) => self.query = format!("SELECT NOT JSON_CONTAINS({}, '{}', '${}') FROM", column, needle, path),
                        ValueType::Datetime(needle) => self.query = format!("SELECT NOT JSON_CONTAINS({}, '{}', '${}') FROM", column, needle, path),
                        _ => self.query = format!("SELECT NOT JSON_CONTAINS({}, {}, '${}') FROM", column, needle, path),
                    },
                    _ => self.query = format!("SELECT NOT JSON_CONTAINS({}, {}, '${}') FROM", column, needle, path),
                }
                None => match needle {
                    JsonValue::Initial(initial) => match initial {
                        ValueType::JsonString(needle) => self.query = format!("SELECT NOT JSON_CONTAINS({}, '\"{}\"') FROM", column, needle),
                        ValueType::String(needle) => self.query = format!("SELECT NOT JSON_CONTAINS({}, '{}') FROM", column, needle),
                        ValueType::Datetime(needle) => self.query = format!("SELECT NOT JSON_CONTAINS({}, '{}') FROM", column, needle),
                        _ => self.query = format!("SELECT NOT JSON_CONTAINS({}, {}) FROM", column, needle)
                    },
                    _ => self.query = format!("SELECT NOT JSON_CONTAINS({}, {}) FROM", column, needle)
                }
            },
            KeywordList::Where => match path {
                Some(path) => {
                    let mut split_the_query = self.query.split(" WHERE ");

                    let first_half = split_the_query.nth(0);

                    match needle {
                        JsonValue::Initial(initial) => match initial {
                            ValueType::JsonString(needle) => self.query = format!("{} WHERE NOT JSON_CONTAINS({}, '\"{}\"', '${}')", first_half.unwrap(), column, needle, path),
                            ValueType::String(needle) => self.query = format!("{} WHERE NOT JSON_CONTAINS({}, '{}', '${}')", first_half.unwrap(), column, needle, path),
                            ValueType::Datetime(needle) => self.query = format!("{} WHERE NOT JSON_CONTAINS({}, '{}', '${}')", first_half.unwrap(), column, needle, path),
                            _ => self.query = format!("{} WHERE NOT JSON_CONTAINS({}, {}, '${}')", first_half.unwrap(), column, needle, path)
                        },
                        _ => self.query = format!("{} WHERE NOT JSON_CONTAINS({}, {}, '${}')", first_half.unwrap(), column, needle, path)
                    }
                },
                None => {
                    let mut split_the_query = self.query.split(" WHERE ");

                    let first_half = split_the_query.nth(0);

                    match needle {
                        JsonValue::Initial(initial) => match initial {
                            ValueType::JsonString(needle) => self.query = format!("{} WHERE NOT JSON_CONTAINS({}, '\"{}\"')", first_half.unwrap(), column, needle),
                            ValueType::String(needle) => self.query = format!("{} WHERE NOT JSON_CONTAINS({}, '{}')", first_half.unwrap(), column, needle),
                            ValueType::Datetime(needle) => self.query = format!("{} WHERE NOT JSON_CONTAINS({}, '{}')", first_half.unwrap(), column, needle),
                            _ => self.query = format!("{} WHERE NOT JSON_CONTAINS({}, {})", first_half.unwrap(), column, needle)
                        },
                        _ => self.query = format!("{} WHERE NOT JSON_CONTAINS({}, {})", first_half.unwrap(), column, needle)
                    }
                }
            },
            KeywordList::LeftBracketWhere => match path {
                Some(path) => {
                    let mut split_the_query = self.query.split(" WHERE ");

                    let first_half = split_the_query.nth(0);

                    match needle {
                        JsonValue::Initial(initial) => match initial {
                            ValueType::JsonString(needle) => self.query = format!("{} WHERE (NOT JSON_CONTAINS({}, '\"{}\"', '${}')", first_half.unwrap(), column, needle, path),
                            ValueType::String(needle) => self.query = format!("{} WHERE (NOT JSON_CONTAINS({}, '{}', '${}')", first_half.unwrap(), column, needle, path),
                            ValueType::Datetime(needle) => self.query = format!("{} WHERE (NOT JSON_CONTAINS({}, '{}', '${}')", first_half.unwrap(), column, needle, path),
                            _ => self.query = format!("{} WHERE (NOT JSON_CONTAINS({}, {}, '${}')", first_half.unwrap(), column, needle, path)
                        },
                        _ => self.query = format!("{} WHERE (NOT JSON_CONTAINS({}, {}, '${}')", first_half.unwrap(), column, needle, path)
                    }
                },
                None => {
                    let mut split_the_query = self.query.split(" WHERE ");

                    let first_half = split_the_query.nth(0);

                    match needle {
                        JsonValue::Initial(initial) => match initial {
                            ValueType::JsonString(needle) => self.query = format!("{} WHERE (NOT JSON_CONTAINS({}, '\"{}\"')", first_half.unwrap(), column, needle),
                            ValueType::String(needle) => self.query = format!("{} WHERE (NOT JSON_CONTAINS({}, '{}')", first_half.unwrap(), column, needle),
                            ValueType::Datetime(needle) => self.query = format!("{} WHERE (NOT JSON_CONTAINS({}, '{}')", first_half.unwrap(), column, needle),
                            _ => self.query = format!("{} WHERE (NOT JSON_CONTAINS({}, {})", first_half.unwrap(), column, needle)
                        },
                        _ => self.query = format!("{} WHERE (NOT JSON_CONTAINS({}, {})", first_half.unwrap(), column, needle)
                    }
                }
            },
            KeywordList::And => match path {
                Some(path) => {
                    let split_the_query = self.query.split(" AND ").collect::<Vec<&str>>();

                    let length_of_the_split_the_query = split_the_query.len();

                    match split_the_query.len() {
                        0 => panic!("There Is No AND query in QueryBuilder but The AND keyword exist in keyword list, panicking."),
                        1 => panic!("There Is No AND query in QueryBuilder but The AND keyword exist in keyword list, panicking."),
                        2 => match needle {
                            JsonValue::Initial(initial) => match initial {
                                ValueType::JsonString(needle) => self.query = format!("{} AND NOT JSON_CONTAINS({}, '\"{}\"', '${}')", split_the_query[0], column, needle, path),
                                ValueType::String(needle) => self.query = format!("{} AND NOT JSON_CONTAINS({}, '{}', '${}')", split_the_query[0], column, needle, path),
                                ValueType::Datetime(needle) => self.query = format!("{} AND NOT JSON_CONTAINS({}, '{}', '${}')", split_the_query[0], column, needle, path),
                                _ => self.query = format!("{} AND NOT JSON_CONTAINS({}, {}, '${}')", split_the_query[0], column, needle, path)
                            },
                            _ => self.query = format!("{} AND NOT JSON_CONTAINS({}, {}, '${}')", split_the_query[0], column, needle, path)
                        },
                        _ => {
                            let mut concatenated_string = String::new();

                            for (index, chunk) in  split_the_query.into_iter().enumerate() {
                                if index == 0 {
                                    concatenated_string = chunk.to_string();
                                } else if index + 1 != length_of_the_split_the_query {
                                    concatenated_string = format!("{} AND {} ", concatenated_string, chunk)
                                }
                            }

                            match needle {
                                JsonValue::Initial(initial) => match initial {
                                    ValueType::JsonString(needle) => self.query = format!("{}AND NOT JSON_CONTAINS({}, '\"{}\"', '${}')", concatenated_string, column, needle, path),
                                    ValueType::String(needle) => self.query = format!("{}AND NOT JSON_CONTAINS({}, '{}', '${}')", concatenated_string, column, needle, path),
                                    ValueType::Datetime(needle) => self.query = format!("{}AND NOT JSON_CONTAINS({}, '{}', '${}')", concatenated_string, column, needle, path),
                                    _ => self.query = format!("{}AND NOT JSON_CONTAINS({}, {}, '${}')", concatenated_string, column, needle, path)
                                },
                                _ => self.query = format!("{}AND NOT JSON_CONTAINS({}, {}, '${}')", concatenated_string, column, needle, path)
                            }
                        }
                    }
                },
                None => {
                    let split_the_query = self.query.split(" AND ").collect::<Vec<&str>>();

                    let length_of_the_split_the_query = split_the_query.len();

                    match split_the_query.len() {
                        0 => panic!("There Is No AND query in QueryBuilder but The AND keyword exist in keyword list, panicking."),
                        1 => panic!("There Is No AND query in QueryBuilder but The AND keyword exist in keyword list, panicking."),
                        2 => match needle {
                            JsonValue::Initial(initial) => match initial {
                                ValueType::JsonString(needle) => self.query = format!("{} AND NOT JSON_CONTAINS({}, '\"{}\"')",  split_the_query[0], column, needle),
                                ValueType::String(needle) => self.query = format!("{} AND NOT JSON_CONTAINS({}, '{}')",  split_the_query[0], column, needle),
                                ValueType::Datetime(needle) => self.query = format!("{} AND NOT JSON_CONTAINS({}, '{}')",  split_the_query[0], column, needle),
                                _ => self.query = format!("{} AND NOT JSON_CONTAINS({}, {})",  split_the_query[0], column, needle)
                            },
                            _ => self.query = format!("{} AND NOT JSON_CONTAINS({}, {})",  split_the_query[0], column, needle)
                        },
                        _ => {
                            let mut concatenated_string = String::new();

                            for (index, chunk) in  split_the_query.into_iter().enumerate() {
                                if index == 0 {
                                    concatenated_string = chunk.to_string();
                                } else if index + 1 != length_of_the_split_the_query {
                                    concatenated_string = format!("{} AND {} ", concatenated_string, chunk)
                                }
                            }

                            match needle {
                                JsonValue::Initial(initial) => match initial {
                                    ValueType::JsonString(needle) => self.query = format!("{}AND NOT JSON_CONTAINS({}, '\"{}\"')", concatenated_string, column, needle),
                                    ValueType::String(needle) => self.query = format!("{}AND NOT JSON_CONTAINS({}, '{}')", concatenated_string, column, needle),
                                    ValueType::Datetime(needle) => self.query = format!("{}AND NOT JSON_CONTAINS({}, '{}')", concatenated_string, column, needle),
                                    _ => self.query = format!("{}AND NOT JSON_CONTAINS({}, {})", concatenated_string, column, needle)
                                },
                                _ => self.query = format!("{}AND NOT JSON_CONTAINS({}, {})", concatenated_string, column, needle)
                            }
                        }
                    }
                }
            },
            KeywordList::LeftBracketAnd => match path {
                Some(path) => {
                    let split_the_query = self.query.split(" AND ").collect::<Vec<&str>>();

                    let length_of_the_split_the_query = split_the_query.len();

                    match split_the_query.len() {
                        0 => panic!("There Is No AND query in QueryBuilder but The AND keyword exist in keyword list, panicking."),
                        1 => panic!("There Is No AND query in QueryBuilder but The AND keyword exist in keyword list, panicking."),
                        2 => match needle {
                            JsonValue::Initial(initial) => match initial {
                                ValueType::JsonString(needle) => self.query = format!("{} AND (NOT JSON_CONTAINS({}, '\"{}\"', '${}')", split_the_query[0], column, needle, path),
                                ValueType::String(needle) => self.query = format!("{} AND (NOT JSON_CONTAINS({}, '{}', '${}')", split_the_query[0], column, needle, path),
                                ValueType::Datetime(needle) => self.query = format!("{} AND (NOT JSON_CONTAINS({}, '{}', '${}')", split_the_query[0], column, needle, path),
                                _ => self.query = format!("{} AND (NOT JSON_CONTAINS({}, {}, '${}')", split_the_query[0], column, needle, path)
                            },
                            _ => self.query = format!("{} AND (NOT JSON_CONTAINS({}, {}, '${}')", split_the_query[0], column, needle, path)
                        },
                        _ => {
                            let mut concatenated_string = String::new();

                            for (index, chunk) in  split_the_query.into_iter().enumerate() {
                                if index == 0 {
                                    concatenated_string = chunk.to_string();
                                } else if index + 1 != length_of_the_split_the_query {
                                    concatenated_string = format!("{} AND {} ", concatenated_string, chunk)
                                }
                            }

                            match needle {
                                JsonValue::Initial(initial) => match initial {
                                    ValueType::JsonString(needle) => self.query = format!("{}AND (NOT JSON_CONTAINS({}, '\"{}\"', '${}')", concatenated_string, column, needle, path),
                                    ValueType::String(needle) => self.query = format!("{}AND (NOT JSON_CONTAINS({}, '{}', '${}')", concatenated_string, column, needle, path),
                                    ValueType::Datetime(needle) => self.query = format!("{}AND (NOT JSON_CONTAINS({}, '{}', '${}')", concatenated_string, column, needle, path),
                                    _ => self.query = format!("{}AND (NOT JSON_CONTAINS({}, {}, '${}')", concatenated_string, column, needle, path)
                                },
                                _ => self.query = format!("{}AND (NOT JSON_CONTAINS({}, {}, '${}')", concatenated_string, column, needle, path)
                            }
                        }
                    }
                },
                None => {
                    let split_the_query = self.query.split(" AND ").collect::<Vec<&str>>();

                    let length_of_the_split_the_query = split_the_query.len();

                    match split_the_query.len() {
                        0 => panic!("There Is No AND query in QueryBuilder but The AND keyword exist in keyword list, panicking."),
                        1 => panic!("There Is No AND query in QueryBuilder but The AND keyword exist in keyword list, panicking."),
                        2 => match needle {
                            JsonValue::Initial(initial) => match initial {
                                ValueType::JsonString(needle) => self.query = format!("{} AND (NOT JSON_CONTAINS({}, '\"{}\"')",  split_the_query[0], column, needle),
                                ValueType::String(needle) => self.query = format!("{} AND (NOT JSON_CONTAINS({}, '{}')",  split_the_query[0], column, needle),
                                ValueType::Datetime(needle) => self.query = format!("{} AND (NOT JSON_CONTAINS({}, '{}')",  split_the_query[0], column, needle),
                                _ => self.query = format!("{} AND (NOT JSON_CONTAINS({}, {})",  split_the_query[0], column, needle)
                            },
                            _ => self.query = format!("{} AND (NOT JSON_CONTAINS({}, {})",  split_the_query[0], column, needle)
                        },
                        _ => {
                            let mut concatenated_string = String::new();

                            for (index, chunk) in  split_the_query.into_iter().enumerate() {
                                if index == 0 {
                                    concatenated_string = chunk.to_string();
                                } else if index + 1 != length_of_the_split_the_query {
                                    concatenated_string = format!("{} AND {} ", concatenated_string, chunk)
                                }
                            }

                            match needle {
                                JsonValue::Initial(initial) => match initial {
                                    ValueType::JsonString(needle) => self.query = format!("{}AND (NOT JSON_CONTAINS({}, '\"{}\"')", concatenated_string, column, needle),
                                    ValueType::String(needle) => self.query = format!("{}AND (NOT JSON_CONTAINS({}, '{}')", concatenated_string, column, needle),
                                    ValueType::Datetime(needle) => self.query = format!("{}AND (NOT JSON_CONTAINS({}, '{}')", concatenated_string, column, needle),
                                    _ => self.query = format!("{}AND (NOT JSON_CONTAINS({}, {})", concatenated_string, column, needle)
                                },
                                _ => self.query = format!("{}AND (NOT JSON_CONTAINS({}, {})", concatenated_string, column, needle)
                            }
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
                        2 => match needle {
                            JsonValue::Initial(initial) => match initial {
                                ValueType::JsonString(needle) => self.query = format!("{} OR NOT JSON_CONTAINS({}, '\"{}\"', '${}')", split_the_query[0], column, needle, path),
                                ValueType::String(needle) => self.query = format!("{} OR NOT JSON_CONTAINS({}, '{}', '${}')", split_the_query[0], column, needle, path),
                                ValueType::Datetime(needle) => self.query = format!("{} OR NOT JSON_CONTAINS({}, '{}', '${}')", split_the_query[0], column, needle, path),
                                _ => self.query = format!("{} OR NOT JSON_CONTAINS({}, {}, '${}')", split_the_query[0], column, needle, path)
                            },
                            _ => self.query = format!("{} OR NOT JSON_CONTAINS({}, {}, '${}')", split_the_query[0], column, needle, path)
                        },
                        _ => {
                            let mut concatenated_string = String::new();

                            for (index, chunk) in  split_the_query.into_iter().enumerate() {
                                if index == 0 {
                                    concatenated_string = chunk.to_string();
                                } else if index + 1 != length_of_the_split_the_query {
                                    concatenated_string = format!("{} OR {} ", concatenated_string, chunk)
                                }
                            }

                            match needle {
                                JsonValue::Initial(initial) => match initial {
                                    ValueType::JsonString(needle) => self.query = format!("{}OR NOT JSON_CONTAINS({}, '\"{}\"', '${}')", concatenated_string, column, needle, path),
                                    ValueType::String(needle) => self.query = format!("{}OR NOT JSON_CONTAINS({}, '{}', '${}')", concatenated_string, column, needle, path),
                                    ValueType::Datetime(needle) => self.query = format!("{}OR NOT JSON_CONTAINS({}, '{}', '${}')", concatenated_string, column, needle, path),
                                    _ => self.query = format!("{}OR NOT JSON_CONTAINS({}, {}, '${}')", concatenated_string, column, needle, path)
                                },
                                _ => self.query = format!("{}OR NOT JSON_CONTAINS({}, {}, '${}')", concatenated_string, column, needle, path)
                            }
                        }
                    }
                },
                None => {
                    let split_the_query = self.query.split(" OR ").collect::<Vec<&str>>();

                    let length_of_the_split_the_query = split_the_query.len();

                    match split_the_query.len() {
                        0 => panic!("There Is No OR query in QueryBuilder but The OR keyword exist in keyword list, panicking."),
                        1 => panic!("There Is No OR query in QueryBuilder but The OR keyword exist in keyword list, panicking."),
                        2 => match needle {
                            JsonValue::Initial(initial) => match initial {
                                ValueType::JsonString(needle) => self.query = format!("{} OR NOT JSON_CONTAINS({}, '\"{}\"')",  split_the_query[0], column, needle),
                                ValueType::String(needle) => self.query = format!("{} OR NOT JSON_CONTAINS({}, '{}')",  split_the_query[0], column, needle),
                                ValueType::Datetime(needle) => self.query = format!("{} OR NOT JSON_CONTAINS({}, '{}')",  split_the_query[0], column, needle),
                                _ => self.query = format!("{} OR NOT JSON_CONTAINS({}, {})",  split_the_query[0], column, needle)
                            },
                            _ => self.query = format!("{} OR NOT JSON_CONTAINS({}, {})",  split_the_query[0], column, needle)
                        },
                        _ => {
                            let mut concatenated_string = String::new();

                            for (index, chunk) in  split_the_query.into_iter().enumerate() {
                                if index == 0 {
                                    concatenated_string = chunk.to_string();
                                } else if index + 1 != length_of_the_split_the_query {
                                    concatenated_string = format!("{} OR {} ", concatenated_string, chunk)
                                }
                            }

                            match needle {
                                JsonValue::Initial(initial) => match initial {
                                    ValueType::JsonString(needle) => self.query = format!("{}OR NOT JSON_CONTAINS({}, '\"{}\"')", concatenated_string, column, needle),
                                    ValueType::String(needle) => self.query = format!("{}OR NOT JSON_CONTAINS({}, '{}')", concatenated_string, column, needle),
                                    ValueType::Datetime(needle) => self.query = format!("{}OR NOT JSON_CONTAINS({}, '{}')", concatenated_string, column, needle),
                                    _ => self.query = format!("{}OR NOT JSON_CONTAINS({}, {})", concatenated_string, column, needle)
                                },
                                _ => self.query = format!("{}OR NOT JSON_CONTAINS({}, {})", concatenated_string, column, needle)
                            }
                        }
                    }
                }
            },
            KeywordList::LeftBracketOr => match path {
                Some(path) => {
                    let split_the_query = self.query.split(" OR ").collect::<Vec<&str>>();

                    let length_of_the_split_the_query = split_the_query.len();

                    match split_the_query.len() {
                        0 => panic!("There Is No OR query in QueryBuilder but The OR keyword exist in keyword list, panicking."),
                        1 => panic!("There Is No OR query in QueryBuilder but The OR keyword exist in keyword list, panicking."),
                        2 => match needle {
                            JsonValue::Initial(initial) => match initial {
                                ValueType::JsonString(needle) => self.query = format!("{} OR (NOT JSON_CONTAINS({}, '\"{}\"', '${}')", split_the_query[0], column, needle, path),
                                ValueType::String(needle) => self.query = format!("{} OR (NOT JSON_CONTAINS({}, '{}', '${}')", split_the_query[0], column, needle, path),
                                ValueType::Datetime(needle) => self.query = format!("{} OR (NOT JSON_CONTAINS({}, '{}', '${}')", split_the_query[0], column, needle, path),
                                _ => self.query = format!("{} OR (NOT JSON_CONTAINS({}, {}, '${}')", split_the_query[0], column, needle, path)
                            },
                            _ => self.query = format!("{} OR (NOT JSON_CONTAINS({}, {}, '${}')", split_the_query[0], column, needle, path)
                        },
                        _ => {
                            let mut concatenated_string = String::new();

                            for (index, chunk) in  split_the_query.into_iter().enumerate() {
                                if index == 0 {
                                    concatenated_string = chunk.to_string();
                                } else if index + 1 != length_of_the_split_the_query {
                                    concatenated_string = format!("{} OR {} ", concatenated_string, chunk)
                                }
                            }

                            match needle {
                                JsonValue::Initial(initial) => match initial {
                                    ValueType::JsonString(needle) => self.query = format!("{}OR (NOT JSON_CONTAINS({}, '\"{}\"', '${}')", concatenated_string, column, needle, path),
                                    ValueType::String(needle) => self.query = format!("{}OR (NOT JSON_CONTAINS({}, '{}', '${}')", concatenated_string, column, needle, path),
                                    ValueType::Datetime(needle) => self.query = format!("{}OR (NOT JSON_CONTAINS({}, '{}', '${}')", concatenated_string, column, needle, path),
                                    _ => self.query = format!("{}OR (NOT JSON_CONTAINS({}, {}, '${}')", concatenated_string, column, needle, path)
                                },
                                _ => self.query = format!("{}OR (NOT JSON_CONTAINS({}, {}, '${}')", concatenated_string, column, needle, path)
                            }
                        }
                    }
                },
                None => {
                    let split_the_query = self.query.split(" OR ").collect::<Vec<&str>>();

                    let length_of_the_split_the_query = split_the_query.len();

                    match split_the_query.len() {
                        0 => panic!("There Is No OR query in QueryBuilder but The OR keyword exist in keyword list, panicking."),
                        1 => panic!("There Is No OR query in QueryBuilder but The OR keyword exist in keyword list, panicking."),
                        2 => match needle {
                            JsonValue::Initial(initial) => match initial {
                                ValueType::JsonString(needle) => self.query = format!("{} OR (NOT JSON_CONTAINS({}, '\"{}\"')",  split_the_query[0], column, needle),
                                ValueType::String(needle) => self.query = format!("{} OR (NOT JSON_CONTAINS({}, '{}')",  split_the_query[0], column, needle),
                                ValueType::Datetime(needle) => self.query = format!("{} OR (NOT JSON_CONTAINS({}, '{}')",  split_the_query[0], column, needle),
                                _ => self.query = format!("{} OR (NOT JSON_CONTAINS({}, {})",  split_the_query[0], column, needle)
                            },
                            _ => self.query = format!("{} OR (NOT JSON_CONTAINS({}, {})",  split_the_query[0], column, needle)
                        },
                        _ => {
                            let mut concatenated_string = String::new();

                            for (index, chunk) in  split_the_query.into_iter().enumerate() {
                                if index == 0 {
                                    concatenated_string = chunk.to_string();
                                } else if index + 1 != length_of_the_split_the_query {
                                    concatenated_string = format!("{} OR {} ", concatenated_string, chunk)
                                }
                            }

                            match needle {
                                JsonValue::Initial(initial) => match initial {
                                    ValueType::JsonString(needle) => self.query = format!("{}OR (NOT JSON_CONTAINS({}, '\"{}\"')", concatenated_string, column, needle),
                                    ValueType::String(needle) => self.query = format!("{}OR (NOT JSON_CONTAINS({}, '{}')", concatenated_string, column, needle),
                                    ValueType::Datetime(needle) => self.query = format!("{}OR (NOT JSON_CONTAINS({}, '{}')", concatenated_string, column, needle),
                                    _ => self.query = format!("{}OR (NOT JSON_CONTAINS({}, {})", concatenated_string, column, needle)
                                },
                                _ => self.query = format!("{}OR (NOT JSON_CONTAINS({}, {})", concatenated_string, column, needle)
                            }
                        }
                    }
                }
            },
            _ => panic!("Wrong usage of '.not_json_contains()' method, it should be used later than either SELECT, WHERE, AND, OR keywords.")
        }

        self.list.push(KeywordList::NotJsonContains);

        self
    }

    /// it adds `JSON_ARRAY_APPEND()` mysql function with it's synthax. It's intended to used with only update constructor, don't use it with any other kind of query.
    /// 
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType, JsonValue};
    /// 
    /// fn main () {
    ///     let lesson = ("lesson", &ValueType::String("math".to_string()));
    ///     let point = ("point", &ValueType::Int32(100));
    ///
    ///     let values = vec![lesson, point];
    ///
    ///     let object = JsonValue::MysqlJsonObject(&values);
    ///
    ///     let query = QueryBuilder::update().unwrap()
    ///                                 .table("users")
    ///                                 .json_array_append("points", Some(""), object.clone())
    ///                                 .where_("id", "=", ValueType::Int8(1))
    ///                                 .finish();
    ///
    ///     assert_eq!("UPDATE users SET points = JSON_ARRAY_APPEND(points, '$', JSON_OBJECT('lesson', 'math', 'point', 100)) WHERE id = 1;", query);
    /// }
    /// 
    /// ```
    pub fn json_array_append(&mut self, column: &str, path: Option<&str>, object: JsonValue) -> &mut Self {
        match self.list.last() {
            Some(keyword) => match keyword {
                KeywordList::Set => {
                    match path {
                        Some(path) => match object {
                            JsonValue::Initial(initial) => match initial {
                                ValueType::JsonString(object) => self.query = format!("{}, {} = JSON_ARRAY_APPEND({}, '${}', '\"{}\"')", self.query, column, column, path, object),
                                ValueType::String(object) => self.query = format!("{}, {} = JSON_ARRAY_APPEND({}, '${}', '{}')", self.query, column, column, path, object),
                                ValueType::Datetime(object) => self.query = format!("{}, {} = JSON_ARRAY_APPEND({}, '${}', '{}')", self.query, column, column, path, object),
                                _ => self.query = format!("{}, {} = JSON_ARRAY_APPEND({}, '${}', {})", self.query, column, column, path, object),
                            },
                            _ => self.query = format!("{}, {} = JSON_ARRAY_APPEND({}, '${}', {})", self.query, column, column, path, object),
                        }
                        None => match object {
                            JsonValue::Initial(initial) => match initial {
                                ValueType::JsonString(object) => self.query = format!("{}, {} = JSON_ARRAY_APPEND({}, '$', '\"{}\"')", self.query, column, column, object),
                                ValueType::String(object) => self.query = format!("{}, {} = JSON_ARRAY_APPEND({}, '$', '{}')", self.query, column, column, object),
                                ValueType::Datetime(object) => self.query = format!("{}, {} = JSON_ARRAY_APPEND({}, '$', '{}')", self.query, column, column, object),
                                _ => self.query = format!("{}, {} = JSON_ARRAY_APPEND({}, '$', {})", self.query, column, column, object)
                            },
                            _ => self.query = format!("{}, {} = JSON_ARRAY_APPEND({}, '$', {})", self.query, column, column, object)
                        }
                    }
                },
                _ => {
                    match path {
                        Some(path) => match object {
                            JsonValue::Initial(initial) => match initial {
                                ValueType::JsonString(object) => self.query = format!("{} SET {} = JSON_ARRAY_APPEND({}, '${}', '\"{}\"')", self.query, column, column, path, object),
                                ValueType::String(object) => self.query = format!("{} SET {} = JSON_ARRAY_APPEND({}, '${}', '{}')", self.query, column, column, path, object),
                                ValueType::Datetime(object) => self.query = format!("{} SET {} = JSON_ARRAY_APPEND({}, '${}', '{}')", self.query, column, column, path, object),
                                _ => self.query = format!("{} SET {} = JSON_ARRAY_APPEND({}, '${}', {})", self.query, column, column, path, object),
                            },
                            _ => self.query = format!("{} SET {} = JSON_ARRAY_APPEND({}, '${}', {})", self.query, column, column, path, object),
                        }
                        None => match object {
                            JsonValue::Initial(initial) => match initial {
                                ValueType::JsonString(object) => self.query = format!("{} SET {} = JSON_ARRAY_APPEND({}, '$', '\"{}\"')", self.query, column, column, object),
                                ValueType::String(object) => self.query = format!("{} SET {} = JSON_ARRAY_APPEND({}, '$', '{}')", self.query, column, column, object),
                                ValueType::Datetime(object) => self.query = format!("{} SET {} = JSON_ARRAY_APPEND({}, '$', '{}')", self.query, column, column, object),
                                _ => self.query = format!("{} SET {} = JSON_ARRAY_APPEND({}, '$', {})", self.query, column, column, object)
                            },
                            _ => self.query = format!("{} SET {} = JSON_ARRAY_APPEND({}, '$', {})", self.query, column, column, object)
                        }
                    }
                }
            },
            None => panic!("it's impossible to came here!")
        }

        self.list.push(KeywordList::JsonArrayAppend);
        self
    }

    /// it adds "JSON_REMOVE()" function with it's synthax. You cannot pass empty strings to paths.
    /// 
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType};
    /// 
    /// fn main () {
    ///   let query = QueryBuilder::update().unwrap()
    ///                            .table("blogs")
    ///                            .json_remove("likes", vec!["[10]"])
    ///                            .where_("blog_id", "=", ValueType::Int32(20))
    ///                            .finish();
    ///
    ///   assert_eq!(query, "UPDATE blogs SET likes = JSON_REMOVE(likes, '$[10]') WHERE blog_id = 20;");
    /// }
    /// 
    /// ```
    pub fn json_remove(&mut self, column: &str, paths: Vec<&str>) -> &mut Self {
        match paths.iter().any(|path| *path == "") {
            true => panic!("Error: a value in the paths cannot be empty string, panicking..."),
            false => ()
        }

        match self.list.last() {
            Some(keyword) => match keyword {
                KeywordList::Set => {
                    self.query = format!("{}, {} = JSON_REMOVE({}", self.query, column, column);

                    for path in paths {
                        if path.starts_with("$") {
                            self.query = format!("{}, '${}'", self.query, path.replace("$", ""))
                        } else {
                            self.query = format!("{}, '${}'", self.query, path)
                        }
                    }

                    self.query = format!("{})", self.query)
                },
                _ => {
                    self.query = format!("{} SET {} = JSON_REMOVE({}", self.query, column, column);

                    for path in paths {
                        if path.starts_with("$") {
                            self.query = format!("{}, '${}'", self.query, path.replace("$", ""))
                        } else {
                            self.query = format!("{}, '${}'", self.query, path)
                        }
                    }

                    self.query = format!("{})", self.query)
                }
            },
            None => panic!("it's impossible to came here!")
        }

        self.list.push(KeywordList::JsonRemove);
        self
    }

    /// It adds `JSON_SET()` function with it's synthax. It updates values with the specified path.
    /// 
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType, JsonValue};
    /// 
    /// fn main () {
    /// 
    /// let lesson = ("lesson", &ValueType::String("math".to_string()));
    /// let point = ("point", &ValueType::Int32(100));
    ///
    /// let values = vec![lesson, point];
    ///
    /// let object = JsonValue::MysqlJsonObject(&values);
    ///
    /// let query = QueryBuilder::update().unwrap()
    ///                          .table("users")
    ///                          .json_set("points", "[0]", object)
    ///                          .where_("id", "=", ValueType::Int32(1))
    ///                          .finish();
    ///
    /// assert_eq!("UPDATE users SET points = JSON_SET(points, '$[0]', JSON_OBJECT('lesson', 'math', 'point', 100)) WHERE id = 1;", query);
    /// 
    /// }
    /// 
    /// ```
    pub fn json_set(&mut self, column: &str, path: &str, value: JsonValue) -> &mut Self {
        match self.list.last() {
            Some(keyword) => match keyword {
                KeywordList::Set => match value {
                    JsonValue::Initial(initial) => match initial {
                        ValueType::JsonString(value) => self.query = format!("{}, {} = JSON_SET({}, '${}', '\"{}\"')", self.query, column, column, path, value),
                        ValueType::String(value) => self.query = format!("{}, {} = JSON_SET({}, '${}', '{}')", self.query, column, column, path, value),
                        ValueType::Datetime(value) => self.query = format!("{}, {} = JSON_SET({}, '${}', '{}')", self.query, column, column, path, value),
                        _ => self.query = format!("{}, {} = JSON_SET({}, '${}', {})", self.query, column, column, path, value),
                    },
                    _ => self.query = format!("{}, {} = JSON_SET({}, '${}', {})", self.query, column, column, path, value),
                }
                _ => match value {
                    JsonValue::Initial(initial) => match initial {
                        ValueType::JsonString(value) => self.query = format!("{} SET {} = JSON_SET({}, '${}', '\"{}\"')", self.query, column, column, path, value),
                        ValueType::String(value) => self.query = format!("{} SET {} = JSON_SET({}, '${}', '{}')", self.query, column, column, path, value),
                        ValueType::Datetime(value) => self.query = format!("{} SET {} = JSON_SET({}, '${}', '{}')", self.query, column, column, path, value),
                        _ => self.query = format!("{} SET {} = JSON_SET({}, '${}', {})", self.query, column, column, path, value)
                    },
                    _ => self.query = format!("{} SET {} = JSON_SET({}, '${}', {})", self.query, column, column, path, value)
                }
            },
            None => panic!("it's impossible to came here!")
        }

        self.list.push(KeywordList::JsonSet);
        self
    }

    /// It adds `JSON_REPLACE()` function with it's synthax. It updates values with the specified path.
    /// 
    /// ```rust
    /// 
    /// use qubl::{QueryBuilder, ValueType, JsonValue};
    /// 
    /// fn main () {
    /// 
    /// let value = ValueType::Int32(100);
    /// let value = JsonValue::Initial(&value);
    ///
    /// let query = QueryBuilder::update().unwrap()
    ///                          .table("users")
    ///                          .json_replace("points", "[0].point", value)
    ///                          .where_("id", "=", ValueType::Int32(1))
    ///                          .finish();
    ///
    /// assert_eq!("UPDATE users SET points = JSON_REPLACE(points, '$[0].point', 100) WHERE id = 1;", query)
    /// 
    /// }
    /// 
    /// ```
    pub fn json_replace(&mut self, column: &str, path: &str, value: JsonValue) -> &mut Self {
        match self.list.last() {
            Some(keyword) => match keyword {
                KeywordList::Set => match value {
                    JsonValue::Initial(initial) => match initial {
                        ValueType::JsonString(value) => self.query = format!("{}, {} = JSON_REPLACE({}, '${}', '\"{}\"')", self.query, column, column, path, value),
                        ValueType::String(value) => self.query = format!("{}, {} = JSON_REPLACE({}, '${}', '{}')", self.query, column, column, path, value),
                        ValueType::Datetime(value) => self.query = format!("{}, {} = JSON_REPLACE({}, '${}', '{}')", self.query, column, column, path, value),
                        _ => self.query = format!("{}, {} = JSON_REPLACE({}, '${}', {})", self.query, column, column, path, value),
                    },
                    _ => self.query = format!("{}, {} = JSON_REPLACE({}, '${}', {})", self.query, column, column, path, value),
                }
                _ => match value {
                    JsonValue::Initial(initial) => match initial {
                        ValueType::JsonString(value) => self.query = format!("{} SET {} = JSON_REPLACE({}, '${}', '\"{}\"')", self.query, column, column, path, value),
                        ValueType::String(value) => self.query = format!("{} SET {} = JSON_REPLACE({}, '${}', '{}')", self.query, column, column, path, value),
                        ValueType::Datetime(value) => self.query = format!("{} SET {} = JSON_REPLACE({}, '${}', '{}')", self.query, column, column, path, value),
                        _ => self.query = format!("{} SET {} = JSON_REPLACE({}, '${}', {})", self.query, column, column, path, value)
                    },
                    _ => self.query = format!("{} SET {} = JSON_REPLACE({}, '${}', {})", self.query, column, column, path, value)
                }
            },
            None => panic!("it's impossible to came here!")
        }

        self.list.push(KeywordList::JsonSet);
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
    Finish, OrderBy, GroupBy, Having, Like, Limit, Offset, IfNotExist, Create, Use, WhereIn, 
    WhereNotIn, AndIn, AndNotIn, OrIn, OrNotIn, JsonExtract, JsonContains, NotJsonContains, JsonArrayAppend, JsonRemove, JsonSet, JsonReplace, 
    Field, Union, UnionAll, Timezone, GlobalTimezone, InnerJoin, LeftJoin, RightJoin, LeftBracketWhere, LeftBracketAnd, LeftBracketOr, RightBracket
}

/// QueryType enum. It helps to detect the type of a query with more optimized way when is needed.
#[derive(Debug, Clone)]
pub enum QueryType {
    Select, Update, Delete, Insert, Null, Create, Count
}

/// BracketType enum. It helps you to open brackets with Corresponding keyword of it's variant on sql queries.
#[derive(Debug, Clone)]
pub enum BracketType {
    Where, And, Or
}

impl std::fmt::Display for BracketType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BracketType::Where => write!(f, "WHERE"),
            BracketType::And => write!(f, "AND"),
            BracketType::Or => write!(f, "OR")
        }
    }
}

/// ValueType enum. It benefits to detect and format the value with optimized way when you have to work with exact column values. 
#[derive(Debug, Clone)]
pub enum ValueType {
    String(String), Datetime(String), Null, Boolean(bool), Int32(i32), Int16(i16), Int8(i8), Int64(i64), Int128(i128),
    Uint8(u8), Uint16(u16), Uint32(u32), Uint64(u64), Usize(usize), Float32(f32), Float64(f64),
    EpochTime(i64), JsonString(String)
}

impl std::fmt::Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::String(string) => write!(f, "'{}'", string),
            ValueType::JsonString(string) => write!(f, "\"{}\"", string),
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

/// Enum that benefits you to add json values to structs. They can be used with json functions.
/// That variants represents that kind of json values:
#[derive(Debug, Clone)]
pub enum JsonValue<'a> {
    /// 
    /// Example Value: ["hello", 21, "again"]
    /// 
    Array(&'a Vec<ValueType>), 
    
    /// 
    /// Example Value: {"name": "necdet", "message": "hello", "id": 1}
    /// 
    Object(&'a Vec<(&'a str, &'a ValueType)>), 
    
    ///
    /// example value: [{"name": "necdet", "message": "hello", "id": 1}, {"name": "kemal", "message": "hi", "id": 2}]
    /// 
    ObjectArray(&'a Vec<Vec<(&'a str, &'a ValueType)>>), 
    
    /// It's same with `ValueType` enums, just for simply passing it to that enum.
    Initial(&'a ValueType), 

    /// Mysql Json Object: It writes JSON_OBJECT() mysql function with it's synthax, such as: JSON_OBJECT('name', 'necdet', 'message', 'hello', 'id', 13). It's necessary or more accurate when working most of the json functions.
    MysqlJsonObject(&'a Vec<(&'a str, &'a ValueType)>)
}

impl <'a>std::fmt::Display for JsonValue<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonValue::Array(values) => {
                let mut json_str = "[".to_string();

                for (index, value) in values.iter().enumerate() {
                    if index == 0 {
                        json_str = format!("{}{}", json_str, value)
                    } else {
                        json_str = format!("{}, {}", json_str, value)
                    }
                }

                json_str = format!("{}]", json_str);

                write!(f, "{}", json_str)
            },
            JsonValue::Object(props) => {
                let mut json_str = "{".to_string();

                for (index, value) in props.iter().enumerate() {
                    if index == 0 {
                        json_str = format!("{}\"{}\": {}", json_str, value.0, value.1)
                    } else {
                        json_str = format!("{}, \"{}\": {}", json_str, value.0, value.1)
                    }
                }

                json_str = format!("{}}}", json_str);

                write!(f, "{}", json_str)
            },
            JsonValue::MysqlJsonObject(props) => {
                let mut json_str = "JSON_OBJECT(".to_string();

                for (index, value) in props.iter().enumerate() {
                    if index == 0 {
                        json_str = format!("{}'{}', {}", json_str, value.0, value.1)
                    } else {
                        json_str = format!("{}, '{}', {}", json_str, value.0, value.1)
                    }
                }

                json_str = format!("{})", json_str);

                write!(f, "{}", json_str)
            },
            JsonValue::ObjectArray(array) => {
                let mut json_str = "[".to_string();

                for (index1, object) in array.into_iter().enumerate() {
                    let mut object_str = "{".to_string();

                    for (index2, property) in object.into_iter().enumerate() {
                        if index2 == 0 {
                            object_str = format!("{}\"{}\": {}", object_str, property.0, property.1)
                        } else {
                            object_str = format!("{}, \"{}\": {}", object_str, property.0, property.1)
                        }
                    }

                    object_str = format!("{}}}", object_str);

                    if index1 == 0 {
                        json_str = format!("{}{}", json_str, object_str)
                    } else {
                        json_str = format!("{}, {}", json_str, object_str)
                    }
                }

                write!(f, "{}]", json_str)
            },
            JsonValue::Initial(value) => write!(f, "{}", value.to_string())
        }
    }
}

/// Timezones with Unix Timezone format, Can be used for setting timezone manually.
/// It covers european, russian, north american, south american, arabic countries. In next releases, we'll cover other african and asian timezones. 
#[derive(Debug, Clone)]
pub enum Timezone {
    System, Istanbul, Moscow, Kaliningrad, Samara, Ekaterinburg, Omsk, Krasnoyarsk, Irkutsk, Yakutsk,
    Vladivostok, Magadan, Kamchatka, Shanghai, London, Paris, Berlin, Madrid, Rome, Amsterdam, Stockholm, Oslo,
    Helsinki, Athens, NewYork, Chicago, Denver, LosAngeles, Anchorage, Honolulu, PuertoRico, Riyadh, Dubai, Qatar,
    Kuwait, Bahrain, Muscat, Aden, Baghdad, Amman, Beirut, Damascus, Gaza, Hebron, Cairo, Khartoum, Tripoli,
    Tunis, BuenosAires, LaPaz, SaoPaulo, Manaus, Recife, Cuiaba, PortoVelho, Santiago, Easter, Bogota, Guayaquil,
    Galapagos, Guyana, Asuncion, Lima, Paramaribo, Montevideo, Caracas, StJohns, Halifax, Toronto, Winnipeg,
    Edmonton, Vancouver, WhiteHorse, MexicoCity, Mazatlan, Chihuahua, Tijuana, Cancun, Belize, CostaRica, ElSalvador,
    Guatemala, Tegucigalpa, Managua, Panama, Apia, Auckland, Bougainville, Chatham, Efate, Enderbury, Fakaofo,
    Fiji, Funafuti, Gambier, Guadalcanal, Guam, Johnston, Kanton, Kiritimati, Kosrae, Kwajalein, Majuro, Marquesas,
    Midway, Nauru, Niue, Norfolk, Noumea, PagoPago, Palau, Pitcairn, Pohnpei, PortMoresby, Saipan, Rarotonga, Tahiti,
    Tarawa, Truk, Wake, Wallis, Yap, Tongatapu
}

impl std::fmt::Display for Timezone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Timezone::System => write!(f, "SYSTEM"), Timezone::Istanbul => write!(f, "Europe/Istanbul"), Timezone::Moscow => write!(f, "Europe/Moscow"),
            Timezone::Kaliningrad => write!(f, "Europe/Kaliningrad"), Timezone::Samara => write!(f, "Europe/Samara"), Timezone::Ekaterinburg => write!(f, "Asia/Yekaterinburg"),
            Timezone::Omsk => write!(f, "Asia/Omsk"), Timezone::Krasnoyarsk => write!(f, "Asia/Krasnoyarsk"), Timezone::Irkutsk => write!(f, "Asia/Irkutsk"),
            Timezone::Yakutsk => write!(f, "Asia/Yakutsk"), Timezone::Vladivostok => write!(f, "Asia/Vladivostok"), Timezone::Magadan => write!(f, "Asia/Magadan"),
            Timezone::Kamchatka => write!(f, "Asia/Kamchatka"), Timezone::Shanghai => write!(f, "Asia/Shanghai"), Timezone::London => write!(f, "Europe/London"),
            Timezone::Paris => write!(f, "Europe/Paris"), Timezone::Berlin => write!(f, "Europe/Berlin"), Timezone::Madrid => write!(f, "Europe/Madrid"),
            Timezone::Rome => write!(f, "Europe/Rome"), Timezone::Amsterdam => write!(f, "Europe/Amsterdam"), Timezone::Stockholm => write!(f, "Europe/Stockholm"),
            Timezone::Oslo => write!(f, "Europe/Oslo"), Timezone::Helsinki => write!(f, "Europe/Helsinki"), Timezone::Athens => write!(f, "Europe/Athens"),
            Timezone::NewYork => write!(f, "America/New_York"), Timezone::Chicago => write!(f, "America/Chicago"), Timezone::Denver => write!(f, "America/Denver"),
            Timezone::LosAngeles => write!(f, "America/Los_Angeles"), Timezone::Anchorage => write!(f, "America/Anchorage"), Timezone::Honolulu => write!(f, "Pacific/Honolulu"),
            Timezone::PuertoRico => write!(f, "America/Puerto_Rico"), Timezone::Riyadh => write!(f, "Asia/Riyadh"), Timezone::Dubai => write!(f, "Asia/Dubai"),
            Timezone::Qatar => write!(f, "Asia/Qatar"), Timezone::Kuwait => write!(f, "Asia/Kuwait"), Timezone::Bahrain => write!(f, "Asia/Bahrain"), Timezone::Muscat => write!(f, "Asia/Muscat"),
            Timezone::Aden => write!(f, "Asia/Aden"), Timezone::Baghdad => write!(f, "Asia/Baghdad"), Timezone::Amman => write!(f, "Asia/Amman"), Timezone::Beirut => write!(f, "Asia/Beirut"),
            Timezone::Damascus => write!(f, "Asia/Damascus"), Timezone::Gaza => write!(f, "Asia/Gaza"), Timezone::Hebron => write!(f, "Asia/Hebron"), Timezone::Cairo => write!(f, "Africa/Cairo"),
            Timezone::Khartoum => write!(f, "Africa/Khartoum"), Timezone::Tripoli => write!(f, "Africa/Tripoli"), Timezone::Tunis => write!(f, "Africa/Tunis"),
            Timezone::BuenosAires => write!(f, "America/Argentina/Buenos_Aires"), Timezone::LaPaz => write!(f, "America/La_Paz"), Timezone::SaoPaulo => write!(f, "America/Sao_Paulo"),
            Timezone::Manaus => write!(f, "America/Manaus"), Timezone::Recife => write!(f, "America/Recife"), Timezone::Cuiaba => write!(f, "America/Cuiaba"), Timezone::PortoVelho => write!(f, "America/Porto_Velho"),
            Timezone::Santiago => write!(f, "America/Santiago"), Timezone::Easter => write!(f, "Pacific/Easter"), Timezone::Bogota => write!(f, "America/Bogota"), Timezone::Guayaquil => write!(f, "America/Guayaquil"),
            Timezone::Galapagos => write!(f, "Pacific/Galapagos"), Timezone::Guyana => write!(f, "America/Guyana"), Timezone::Asuncion => write!(f, "America/Asuncion"), Timezone::Lima => write!(f, "America/Lima"),
            Timezone::Paramaribo => write!(f, "America/Paramaribo"), Timezone::Montevideo => write!(f, "America/Montevideo"), Timezone::Caracas => write!(f, "America/Caracas"),
            Timezone::StJohns => write!(f, "America/St_Johns"), Timezone::Halifax => write!(f, "America/Halifax"), Timezone::Toronto => write!(f, "America/Toronto"), Timezone::Winnipeg => write!(f, "America/Winnipeg"),
            Timezone::Edmonton => write!(f, "America/Edmonton"), Timezone::Vancouver => write!(f, "America/Vancouver"), Timezone::WhiteHorse => write!(f, "America/Whitehorse"),
            Timezone::MexicoCity => write!(f, "America/Mexico_City"), Timezone::Mazatlan => write!(f, "America/Mazatlan"), Timezone::Chihuahua => write!(f, "America/Chihuahua"),
            Timezone::Tijuana => write!(f, "America/Tijuana"), Timezone::Cancun => write!(f, "America/Cancun"), Timezone::Belize => write!(f, "America/Belize"), Timezone::CostaRica => write!(f, "America/Costa_Rica"),
            Timezone::ElSalvador => write!(f, "America/El_Salvador"), Timezone::Guatemala => write!(f, "America/Guatemala"), Timezone::Tegucigalpa => write!(f, "America/Tegucigalpa"),
            Timezone::Managua => write!(f, "America/Managua"), Timezone::Panama => write!(f, "America/Panama"), Timezone::Apia => write!(f, "Pacific/Apia"),
            Timezone::Auckland => write!(f, "Pacific/Auckland"), Timezone::Bougainville => write!(f, "Pacific/Bougainville"), Timezone::Chatham => write!(f, "Pacific/Chatham"),
            Timezone::Efate => write!(f, "Pacific/Efate"), Timezone::Enderbury => write!(f, "Pacific/Enderbury"), Timezone::Tongatapu => write!(f, "Pacific/Tongatapu"),
            Timezone::Fakaofo => write!(f, "Pacific/Fakaofo"), Timezone::Fiji => write!(f, "Pacific/Fiji"), Timezone::Funafuti => write!(f, "Pacific/Funafuti"),
            Timezone::Gambier => write!(f, "Pacific/Gambier"), Timezone::Guadalcanal => write!(f, "Pacific/Guadalcanal"), Timezone::Guam => write!(f, "Pacific/Guam"),
            Timezone::Johnston => write!(f, "Pacific/Johnston"), Timezone::Kanton => write!(f, "Pacific/Kanton"), Timezone::Kiritimati => write!(f, "Pacific/Kiritimati"),
            Timezone::Kosrae => write!(f, "Pacific/Kosrae"), Timezone::Majuro => write!(f, "Pacific/Majuro"), Timezone::Kwajalein => write!(f, "Pacific/Kwajalein"),
            Timezone::Midway => write!(f, "Pacific/Midway"), Timezone::Nauru => write!(f, "Pacific/Nauru"), Timezone::Niue => write!(f, "Pacific/Niue"),
            Timezone::Marquesas => write!(f, "Pacific/Marquesas"), Timezone::Norfolk => write!(f, "Pacific/Norfolk"), Timezone::PagoPago => write!(f, "Pacific/Pago_Pago"),
            Timezone::Noumea => write!(f, "Pacific/Noumea"), Timezone::Palau => write!(f, "Pacific/Palau"), Timezone::Pitcairn => write!(f, "Pacific/Pitcairn"),
            Timezone::Pohnpei => write!(f, "Pacific/Pohnpei"), Timezone::PortMoresby => write!(f, "Pacific/Port_Moresby"), Timezone::Rarotonga => write!(f, "Pacific/Rarotonga"),
            Timezone::Tahiti => write!(f, "Pacific/Tahiti"), Timezone::Tarawa => write!(f, "Pacific/Tarawa"), Timezone::Saipan => write!(f, "Pacific/Saipan"),
            Timezone::Truk => write!(f, "Pacific/Truk"), Timezone::Wake => write!(f, "Pacific/Wake"), Timezone::Wallis => write!(f, "Pacific/Wallis"),
            Timezone::Yap => write!(f, "Pacific/Yap")
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

        assert_eq!(test_where_not_in_custom, "SELECT name, age, id, last_login FROM users WHERE id NOT IN (1, 12, 8);");

        // test AND IN's

        let columns = ["name", "id", "last_login"].to_vec();

        let ids = [ValueType::Int32(1), ValueType::Int16(12), ValueType::Int64(8)].to_vec();

        let test_and_in = QueryBuilder::select(columns).unwrap().table("users").where_("age", ">", ValueType::Int32(35)).and_in("id", &ids).finish();

        assert_eq!(test_and_in, "SELECT name, id, last_login FROM users WHERE age > 35 AND id IN (1, 12, 8);");

        let columns = ["name", "id", "last_login"].to_vec();

        let ids = [ValueType::Int32(1), ValueType::Int16(12), ValueType::Int64(8)].to_vec();

        let test_and_not_in = QueryBuilder::select(columns).unwrap().table("users").where_("age", ">", ValueType::Int32(35)).and_not_in("id", &ids).finish();

        assert_eq!(test_and_not_in, "SELECT name, id, last_login FROM users WHERE age > 35 AND id NOT IN (1, 12, 8);");

        // test OR IN's

        let columns = ["name", "id", "last_login"].to_vec();

        let ids = [ValueType::Int32(1), ValueType::Int16(12), ValueType::Int64(8)].to_vec();

        let test_or_in = QueryBuilder::select(columns).unwrap().table("users").where_("age", ">", ValueType::Int32(35)).or_in("id", &ids).finish();

        assert_eq!(test_or_in, "SELECT name, id, last_login FROM users WHERE age > 35 OR id IN (1, 12, 8);");

        let columns = ["name", "id", "last_login"].to_vec();

        let ids = [ValueType::Int32(1), ValueType::Int16(12), ValueType::Int64(8)].to_vec();

        let test_or_not_in = QueryBuilder::select(columns).unwrap().table("users").where_("age", ">", ValueType::Int32(35)).or_not_in("id", &ids).finish();

        assert_eq!(test_or_not_in, "SELECT name, id, last_login FROM users WHERE age > 35 OR id NOT IN (1, 12, 8);")
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
        let select_query = QueryBuilder::select(["*"].to_vec()).unwrap().json_contains("pic", JsonValue::Initial(&ValueType::String("\"/files/hello.jpg\"".to_string())), Some(".path")).table("users").where_in("id", &ins).finish();

        assert_eq!(select_query, "SELECT JSON_CONTAINS(pic, '\"/files/hello.jpg\"', '$.path') FROM users WHERE id IN (1, 5, 11);".to_string());

        // test with ".where_cond()" method:

        let where_query = QueryBuilder::select(["*"].to_vec()).unwrap().table("users").where_("pic", "=", ValueType::String("".to_string())).json_contains("pic", JsonValue::Initial(&ValueType::String("\"blablabla.jpg\"".to_string())), Some(".name")).finish();

        assert_eq!(where_query, "SELECT * FROM users WHERE JSON_CONTAINS(pic, '\"blablabla.jpg\"', '$.name');".to_string());

        let and_query_1 = QueryBuilder::select(["*"].to_vec()).unwrap().table("users").where_("age", ">", ValueType::Int32(15)).and("asdfasdf", ">", ValueType::String("".to_string())).json_contains("graduation_stats", JsonValue::Initial(&ValueType::Float64(80.11)), Some(".average_point")).finish();

        assert_eq!(and_query_1, "SELECT * FROM users WHERE age > 15 AND JSON_CONTAINS(graduation_stats, 80.11, '$.average_point');".to_string());

        let and_query_2 = QueryBuilder::select(["*"].to_vec()).unwrap().table("users").where_("age", ">", ValueType::Int32(15)).and("class", "=", ValueType::String("5/c".to_string())).and("asdfasdf", ">", ValueType::String("".to_string())).json_contains("graduation_stats", JsonValue::Initial(&ValueType::Float64(80.11)), Some(".average_point")).finish();

        assert_eq!(and_query_2, "SELECT * FROM users WHERE age > 15 AND class = '5/c' AND JSON_CONTAINS(graduation_stats, 80.11, '$.average_point');".to_string());
        
        let and_query_3 = QueryBuilder::select(["*"].to_vec()).unwrap().table("users").where_("age", ">", ValueType::Int32(15)).and("class", "=", ValueType::String("5/c".to_string())).and("surname", "=", ValueType::String("etiman".to_string())).and("asdfasdf", ">", ValueType::String("".to_string())).json_contains("graduation_stats", JsonValue::Initial(&ValueType::Float64(80.11)), Some(".average_point")).finish();
    
        assert_eq!(and_query_3, "SELECT * FROM users WHERE age > 15 AND class = '5/c'  AND surname = 'etiman' AND JSON_CONTAINS(graduation_stats, 80.11, '$.average_point');".to_string());

        let and_query_4 = QueryBuilder::select(["*"].to_vec()).unwrap().table("users").where_("age", ">", ValueType::Int32(15)).and("sdfgsdfg", "=", ValueType::String("".to_string())).json_contains("parents", JsonValue::Initial(&ValueType::Int32(50)), Some(".age")).and("surname", "=", ValueType::String("etiman".to_string())).and("asdfasdf", ">", ValueType::String("".to_string())).json_contains("graduation_stats", JsonValue::Initial(&ValueType::Float32(80.11)), Some(".average_point")).finish();
    
        assert_eq!(and_query_4, "SELECT * FROM users WHERE age > 15 AND JSON_CONTAINS(parents, 50, '$.age')  AND surname = 'etiman' AND JSON_CONTAINS(graduation_stats, 80.11, '$.average_point');".to_string());

        let and_query_5 = QueryBuilder::select(["*"].to_vec()).unwrap().table("users").where_("age", ">", ValueType::Int32(15)).and("sdfgsdfg", "=", ValueType::String("".to_string())).json_contains("parents", JsonValue::Initial(&ValueType::Int32(50)), Some(".age")).and("asdfasdf", ">", ValueType::String("".to_string())).json_contains("graduation_stats", JsonValue::Initial(&ValueType::Float64(80.11)), Some(".average_point")).finish();
    
        assert_eq!(and_query_5, "SELECT * FROM users WHERE age > 15 AND JSON_CONTAINS(parents, 50, '$.age') AND JSON_CONTAINS(graduation_stats, 80.11, '$.average_point');".to_string());
        
        let or_query_1 = QueryBuilder::select(["*"].to_vec()).unwrap().table("users").where_("age", ">", ValueType::Int32(15)).or("asdfasdf", ">", ValueType::String("".to_string())).json_contains("graduation_stats", JsonValue::Initial(&ValueType::Float32(80.11)), Some(".average_point")).finish();

        assert_eq!(or_query_1, "SELECT * FROM users WHERE age > 15 OR JSON_CONTAINS(graduation_stats, 80.11, '$.average_point');".to_string());
        
        let or_query_2 = QueryBuilder::select(["*"].to_vec()).unwrap().table("users").where_("age", ">", ValueType::Int32(15)).or("class", "=", ValueType::String("5/c".to_string())).or("asdfasdf", ">", ValueType::String("".to_string())).json_contains("graduation_stats", JsonValue::Initial(&ValueType::Float64(80.11)), Some(".average_point")).finish();
        
        assert_eq!(or_query_2, "SELECT * FROM users WHERE age > 15 OR class = '5/c' OR JSON_CONTAINS(graduation_stats, 80.11, '$.average_point');".to_string());
                
        let or_query_3 = QueryBuilder::select(["*"].to_vec()).unwrap().table("users").where_("age", ">", ValueType::Int32(15)).or("class", "=", ValueType::String("5/c".to_string())).or("surname", "=", ValueType::String("etiman".to_string())).or("asdfasdf", ">", ValueType::String("".to_string())).json_contains("graduation_stats", JsonValue::Initial(&ValueType::Float64(80.11)), Some(".average_point")).finish();
            
        assert_eq!(or_query_3, "SELECT * FROM users WHERE age > 15 OR class = '5/c'  OR surname = 'etiman' OR JSON_CONTAINS(graduation_stats, 80.11, '$.average_point');".to_string());
        
        let or_query_4 = QueryBuilder::select(["*"].to_vec()).unwrap().table("users").where_("age", ">", ValueType::Int32(15)).or("sdfgsdfg", "=", ValueType::String("".to_string())).json_contains("parents", JsonValue::Initial(&ValueType::Int32(50)), Some(".age")).or("surname", "=", ValueType::String("etiman".to_string())).or("asdfasdf", ">", ValueType::String("".to_string())).json_contains("graduation_stats", JsonValue::Initial(&ValueType::Float64(80.11)), Some(".average_point")).finish();
            
        assert_eq!(or_query_4, "SELECT * FROM users WHERE age > 15 OR JSON_CONTAINS(parents, 50, '$.age')  OR surname = 'etiman' OR JSON_CONTAINS(graduation_stats, 80.11, '$.average_point');".to_string());
        
        let or_query_5 = QueryBuilder::select(["*"].to_vec()).unwrap().table("users").where_("age", ">", ValueType::Int32(15)).or("sdfgsdfg", "=", ValueType::String("".to_string())).json_contains("parents", JsonValue::Initial(&ValueType::Int64(50)), Some(".age")).or("asdfasdf", ">", ValueType::String("".to_string())).json_contains("graduation_stats", JsonValue::Initial(&ValueType::Float32(80.11)), Some(".average_point")).finish();
            
        assert_eq!(or_query_5, "SELECT * FROM users WHERE age > 15 OR JSON_CONTAINS(parents, 50, '$.age') OR JSON_CONTAINS(graduation_stats, 80.11, '$.average_point');".to_string());

        let name = ValueType::JsonString("necdet".to_string());
        let id = ValueType::Int32(1);
        let is_active = ValueType::Boolean(true);

        let object = vec![("name", &name), ("id", &id), ("isActive", &is_active)];

        let mysql_json_object = JsonValue::MysqlJsonObject(&object);

        let where_query_2 = QueryBuilder::select(["*"].to_vec()).unwrap().table("users").where_("pic", "=", ValueType::String("".to_string())).json_contains("pic", mysql_json_object, Some("")).finish();

        assert_eq!("SELECT * FROM users WHERE JSON_CONTAINS(pic, JSON_OBJECT('name', \"necdet\", 'id', 1, 'isActive', true), '$');", where_query_2)
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

    #[test]
    pub fn test_json_value(){
        let name = ValueType::JsonString("necdet".to_string());
        let age = ValueType::Int8(25);
        let id = ValueType::Int32(1);

        let values = vec![("name", &name), ("age", &age), ("id", &id)];

        let json_object = JsonValue::Object(&values);

        assert_eq!("{\"name\": \"necdet\", \"age\": 25, \"id\": 1}", json_object.to_string());

        let mysql_json_object = JsonValue::MysqlJsonObject(&values);

        assert_eq!("JSON_OBJECT('name', \"necdet\", 'age', 25, 'id', 1)", mysql_json_object.to_string());

        let name2 = ValueType::JsonString("cevdet".to_string());
        let age2 = ValueType::Int8(24);
        let id2 = ValueType::Int32(2);

        let name3 = ValueType::JsonString("serap".to_string());
        let age3 = ValueType::Int8(21);
        let id3 = ValueType::Int32(3);

        let object1 = vec![("name", &name), ("age", &age), ("id", &id)];
        let object2 = vec![("name", &name2), ("age", &age2), ("id", &id2)];
        let object3 = vec![("name", &name3), ("age", &age3), ("id", &id3)];

        let objects = vec![object1, object2, object3];
        
        let json_array = JsonValue::ObjectArray(&objects);

        assert_eq!("[{\"name\": \"necdet\", \"age\": 25, \"id\": 1}, {\"name\": \"cevdet\", \"age\": 24, \"id\": 2}, {\"name\": \"serap\", \"age\": 21, \"id\": 3}]", json_array.to_string());
    }

    #[test]
    pub fn test_json_array_append(){
        let lesson = ("lesson", &ValueType::String("math".to_string()));
        let point = ("point", &ValueType::Int32(100));

        let values = vec![lesson, point];
        
        let object = JsonValue::MysqlJsonObject(&values);

        let query = QueryBuilder::update().unwrap()
                                         .table("users")
                                         .json_array_append("points", Some(""), object.clone())
                                         .where_("id", "=", ValueType::Int8(1))
                                         .finish();

        assert_eq!("UPDATE users SET points = JSON_ARRAY_APPEND(points, '$', JSON_OBJECT('lesson', 'math', 'point', 100)) WHERE id = 1;", query);

        let query = QueryBuilder::update().unwrap()
                                         .table("users")
                                         .set("status", ValueType::String("passed".to_string()))
                                         .json_array_append("points", Some(""), object)
                                         .where_("id", "=", ValueType::Int8(1))
                                         .finish();

        assert_eq!("UPDATE users SET status = 'passed', points = JSON_ARRAY_APPEND(points, '$', JSON_OBJECT('lesson', 'math', 'point', 100)) WHERE id = 1;", query);
    }

    #[test]
    pub fn test_json_remove() {
        let query = QueryBuilder::update().unwrap()
                                         .table("blogs")
                                         .json_remove("likes", vec!["[10]"])
                                         .where_("blog_id", "=", ValueType::Int32(20))
                                         .finish();

        assert_eq!(query, "UPDATE blogs SET likes = JSON_REMOVE(likes, '$[10]') WHERE blog_id = 20;");
        
        let query = QueryBuilder::update().unwrap()
                                         .table("blogs")
                                         .set("blabla", ValueType::Int32(50))
                                         .json_remove("likes", vec!["[10]", "[11]", "[12]"])
                                         .where_("blog_id", "=", ValueType::Int32(20))
                                         .finish();

        println!("{}", query)
    }

    #[test]
    pub fn test_json_set_and_json_replace(){
        let lesson = ("lesson", &ValueType::String("math".to_string()));
        let point = ("point", &ValueType::Int32(100));

        let values = vec![lesson, point];
        
        let object = JsonValue::MysqlJsonObject(&values);

        let query = QueryBuilder::update().unwrap()
                                                        .table("users")
                                                        .json_set("points", "[0]", object)
                                                        .where_("id", "=", ValueType::Int32(1))
                                                        .finish();

        assert_eq!("UPDATE users SET points = JSON_SET(points, '$[0]', JSON_OBJECT('lesson', 'math', 'point', 100)) WHERE id = 1;", query);

        let value = ValueType::Int32(100);
        let value = JsonValue::Initial(&value);

        let query = QueryBuilder::update().unwrap()
                                         .table("users")
                                         .json_replace("points", "[0].point", value)
                                         .where_("id", "=", ValueType::Int32(1))
                                         .finish();

        assert_eq!("UPDATE users SET points = JSON_REPLACE(points, '$[0].point', 100) WHERE id = 1;", query)
    }

    #[test]
    pub fn test_json_value_initial_bugfix(){
        let file_name_val = ValueType::JsonString("chemistry".to_string());
        let file_name_val = JsonValue::Initial(&file_name_val);

        let query = QueryBuilder::select(vec!["lesson_points"]).unwrap()
                                         .json_extract("points", &format!("[{}]", 2), Some("point"))
                                         .table("students")
                                         .where_("id", "=", ValueType::Int32(5))
                                         .and("adsf", "=", ValueType::Null)
                                         .json_contains("points", file_name_val, Some(&format!("[{}].name", 0)))
                                         .finish();

        assert_eq!("SELECT JSON_EXTRACT(points, '$[2]') AS point FROM students WHERE id = 5 AND JSON_CONTAINS(points, '\"chemistry\"', '$[0].name');", query);
    }

    #[test]
    pub fn test_timezones(){
        let query = QueryBuilder::select(vec!["*"]).unwrap().table("users").time_zone(Timezone::Istanbul).finish();

        assert_eq!(query, "SET time_zone = Europe/Istanbul; SELECT * FROM users;");
        
        let query = QueryBuilder::select(vec!["*"]).unwrap()
                                         .table("users")
                                         .global_time_zone(Timezone::Amsterdam)
                                         .where_("id", "=", ValueType::Int32(3))
                                         .and("surname", "=", ValueType::String("Doe".to_string()))
                                         .finish();

        assert_eq!(query, "SET GLOBAL time_zone = Europe/Amsterdam; SELECT * FROM users WHERE id = 3 AND surname = 'Doe';");

        let query = QueryBuilder::update().unwrap().table("users").time_zone(Timezone::NewYork).set("age", ValueType::Int32(26)).set("last_online_date", ValueType::Datetime("CURRENT_TIMESTAMP".to_string())).where_("id", "=", ValueType::Int32(234)).finish();

        assert_eq!(query, "SET time_zone = America/New_York; UPDATE users SET age = 26, last_online_date = CURRENT_TIMESTAMP WHERE id = 234;");

        let query = QueryBuilder::update().unwrap().table("users").set("age", ValueType::Int32(26)).global_time_zone(Timezone::NewYork).set("last_online_date", ValueType::Datetime("CURRENT_TIMESTAMP".to_string())).where_("id", "=", ValueType::Int32(234)).finish();

        assert_eq!(query, "SET GLOBAL time_zone = America/New_York; UPDATE users SET age = 26, last_online_date = CURRENT_TIMESTAMP WHERE id = 234;");
    }

    #[test]
    pub fn test_joins(){
        let query = QueryBuilder::select(vec!["*"]).unwrap()
                                         .table("students s")
                                         .inner_join("grades g", "s.id", "=", "g.student_id")
                                         .where_("id", "=", ValueType::Int32(10))
                                         .finish();

        assert_eq!(query, "SELECT * FROM students s INNER JOIN grades g ON s.id = g.student_id WHERE id = 10;");

        let query = QueryBuilder::select(vec!["*"]).unwrap()
                                         .table("students s")
                                         .left_join("grades g", "s.id", "=", "g.student_id")
                                         .where_("id", "=", ValueType::Int32(10))
                                         .finish();

        assert_eq!(query, "SELECT * FROM students s LEFT JOIN grades g ON s.id = g.student_id WHERE id = 10;");

        let query = QueryBuilder::select(vec!["*"]).unwrap()
                                         .table("students s")
                                         .right_join("grades g", "s.id", "=", "g.student_id")
                                         .where_("id", "=", ValueType::Int32(10))
                                         .finish();

        assert_eq!(query, "SELECT * FROM students s RIGHT JOIN grades g ON s.id = g.student_id WHERE id = 10;");

        let query = QueryBuilder::select(vec!["*"]).unwrap()
                                         .table("students s")
                                         .cross_join("grades g")
                                         .where_("id", "=", ValueType::Int32(10))
                                         .finish();

        assert_eq!(query, "SELECT * FROM students s CROSS JOIN grades g WHERE id = 10;");

        let query = QueryBuilder::select(vec!["*"]).unwrap()
                                         .table("students s")
                                         .natural_join("grades g")
                                         .where_("id", "=", ValueType::Int32(10))
                                         .finish();
                                        
        assert_eq!(query, "SELECT * FROM students s NATURAL JOIN grades g WHERE id = 10;");
    }

    #[test]
    pub fn test_parentheses(){
        let query = QueryBuilder::select(vec!["*"]).unwrap()
                                                        .table("users")
                                                        .where_("grades", ">", ValueType::Int32(80))
                                                        .open_parenthesis(BracketType::And)
                                                        .and("height", ">", ValueType::Int32(170))
                                                        .or("weight", ">", ValueType::Int32(60))
                                                        .close_parenthesis()
                                                        .finish();

        assert_eq!(query, "SELECT * FROM users WHERE grades > 80 AND ( AND height > 170 OR weight > 60);");

        let query = QueryBuilder::select(vec!["*"]).unwrap()
                                                .table("users")
                                                .where_("grades", ">", ValueType::Int32(80))
                                                .open_parenthesis_with(BracketType::And, "height", ">", ValueType::Int32(170))
                                                .or("weight", ">", ValueType::Int32(60))
                                                .close_parenthesis()
                                                .finish();                                                  

        assert_eq!(query, "SELECT * FROM users WHERE grades > 80 AND (height > 170 OR weight > 60);");

        let query = QueryBuilder::select(vec!["*"]).unwrap()
                                        .table("users")
                                        .where_("grades", ">", ValueType::Int32(80))
                                        .open_parenthesis_with(BracketType::And, "height", ">", ValueType::Int32(170))
                                        .open_parenthesis_with(BracketType::Or, "weight", ">", ValueType::Int32(50))
                                        .and("weight", "<", ValueType::Int32(70))
                                        .close_parenthesis()
                                        .close_parenthesis()
                                        .finish();          

        assert_eq!(query, "SELECT * FROM users WHERE grades > 80 AND (height > 170 OR (weight > 50 AND weight < 70));");             
    }
}
