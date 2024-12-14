/// Struct that benefits to build queries for interactions with rdbms's.
#[derive(Debug, Clone)]
pub struct QueryBuilder {
    pub query: String,
    pub table: String,
    pub qtype: QueryType,
    pub list: Vec<KeywordList>
}

/// Implementations For QueryBuilder.
impl QueryBuilder {
    /// Select constructor. Use it if you want to build a Select Query.
    pub fn select(fields: Vec<&str>) -> std::result::Result<Self, std::io::Error> {
        match QueryBuilder::sanitize(fields.clone()) {
            Ok(_) => {
                if fields[0] == "*" {
                    let query = "SELECT * FROM".to_string();
    
                    return Ok(QueryBuilder {
                        query,
                        table: "".to_string(),
                        qtype: QueryType::Select,
                        list: vec![KeywordList::Select]
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
                        list: vec![KeywordList::Select]
                    })
                }
            },
            Err(_) => {
                return Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "query cannot build because inserted arbitrary query."))
            }
        }
    }

    /// Delete constructor. Use it if you want to build a Delete Query.
    pub fn delete() -> std::result::Result<Self, std::io::Error> {
        return Ok(QueryBuilder {
            query: "DELETE FROM".to_string(),
            table: "".to_string(),
            qtype: QueryType::Delete,
            list: vec![KeywordList::Delete]
        })
    }

    /// Update constructor. Use it if you want to build a Update Query.
    pub fn update() -> std::result::Result<Self, std::io::Error> {
        return Ok(QueryBuilder {
            query: "UPDATE".to_string(),
            table: "".to_string(),
            qtype: QueryType::Update,
            list: vec![KeywordList::Update]
        })
    }

    /// Insert constructor. Use it if you want to build a Insert Query.
    pub fn insert(columns: Vec<&str>, values: Vec<(&str, ValueType)>) -> std::result::Result<Self, std::io::Error> {
        let mut query = "INSERT INTO".to_string();

        match QueryBuilder::sanitize(columns.clone()) {
            Ok(_) => (),
            Err(_) => {
                return Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "query cannot build on insert constructor: because inserted arbitrary query on columns parameter."))
            }
        }

        let get_actual_values = values.clone().iter().map(|(s, _)| *s).collect::<Vec<&str>>();

        match QueryBuilder::sanitize(get_actual_values.clone()) {
            Ok(_) => (),
            Err(_) => {
                return Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "query cannot build on insert constructor: because inserted arbitrary query in values parameter."))

            }
        }

        let mut columns_string = "(".to_string();
        let mut values_string = "(".to_string();
        let length_of_columns = columns.clone().len();
        let length_of_values = get_actual_values.len();

        if length_of_columns != length_of_values {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "columns and values has to be same length, error on building query on insert constructor."))
        }

        for (i, column) in columns.clone().into_iter().enumerate() {
            for (p, (value, vtype)) in values.clone().into_iter().enumerate() {
                match i == p {
                    true => {
                        if (i + 1) != length_of_columns {
                            columns_string = format!("{}{}, ", columns_string, column);

                            match vtype {
                                ValueType::String => {
                                    if value.contains('"') {
                                        values_string = format!("{}'{}', ", values_string, value)
                                    } else if value.contains("'") {
                                        values_string = format!("{}'{}', ", values_string, value)
                                    } else {
                                        values_string = format!("{}'{}', ", values_string, value)
                                    }
                                },
                                ValueType::Integer => values_string = format!("{}{}, ", values_string, value),
                                ValueType::Float => values_string = format!("{}{}, ", values_string, value),
                                ValueType::Boolean => values_string = format!("{}{}, ", values_string, value),
                                ValueType::Time => match value {
                                    "UNIX_TIMESTAMP" => values_string = format!("{}{}, ", values_string, value),
                                    "CURRENT_TIMESTAMP" => values_string = format!("{}{}, ", values_string, value),
                                    "CURRENT_DATE" => values_string = format!("{}{}, ", values_string, value),
                                    "CURRENT_TIME" => values_string = format!("{}{}, ", values_string, value),
                                    "NOW()" => values_string = format!("{}{}, ", values_string, value),
                                    "CURDATE()" => values_string = format!("{}{}, ", values_string, value),
                                    "CURTIME()" => values_string = format!("{}{}, ", values_string, value),
                                    _ => {
                                        if value.split("").collect::<Vec<&str>>().into_iter().all(|char| char == "" ||  char.parse::<i32>().is_ok()) {
                                            values_string = format!("{}FROM_UNIXTIME({}), ", values_string, value.trim())
                                        } else {
                                            values_string = format!("{}'{}', ", values_string, value)
                                        }  
                                    },
                                }
                            }
                        } else {
                            columns_string = format!("{}{})", columns_string, column);

                            match vtype {
                                ValueType::String => {
                                    if value.contains('"') {
                                        values_string = format!("{}'{}')", values_string, value)
                                    } else if value.contains("'") {
                                        values_string = format!("{}'{}')", values_string, value)
                                    } else {
                                        values_string = format!("{}'{}')", values_string, value)
                                    }
                                },
                                ValueType::Integer => values_string = format!("{}{})", values_string, value),
                                ValueType::Float => values_string = format!("{}{})", values_string, value),
                                ValueType::Boolean => values_string = format!("{}{})", values_string, value),
                                ValueType::Time => match value {
                                    "UNIX_TIMESTAMP" => values_string = format!("{}{})", values_string, value),
                                    "CURRENT_TIMESTAMP" => values_string = format!("{}{})", values_string, value),
                                    "CURRENT_DATE" => values_string = format!("{}{})", values_string, value),
                                    "CURRENT_TIME" => values_string = format!("{}{})", values_string, value),
                                    "NOW()" => values_string = format!("{}{})", values_string, value),
                                    "CURDATE()" => values_string = format!("{}{})", values_string, value),
                                    "CURTIME()" => values_string = format!("{}{})", values_string, value),
                                    _ => {
                                        if value.split("").collect::<Vec<&str>>().into_iter().all(|char|  char == "" || char.parse::<i32>().is_ok()) {
                                            values_string = format!("{}FROM_UNIXTIME({}))", values_string, value.trim())
                                        } else {
                                            values_string = format!("{}'{}')", values_string, value)
                                        }  
                                    },
                                }
                            }
                        }
                    },
                    false => continue
                }
            }
        }

        query = format!("{} {} VALUES {}", query, columns_string, values_string);
        

        return Ok(Self {
            query,
            table: "".to_string(),
            qtype: QueryType::Insert,
            list: vec![KeywordList::Insert]
        })
    }

    /// Count constructor. Use it if you want to learn to length of a table.
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
            list: vec![KeywordList::Count]
        }
    }

    /// define the table. It should came after the constructors.
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

    /// add the "WHERE" keyword with it's synthax.
    pub fn where_cond(&mut self, column: &str, mark: &str, value: (&str, ValueType)) -> &mut Self {
        match QueryBuilder::sanitize(vec![column, mark, value.0]) {
            Ok(_) => {
                match value.1 {
                    ValueType::String => {
                        if value.0.contains('"') {
                            self.query = format!("{} WHERE {} {} '{}'", self.query, column, mark, value.0)
                        } else if value.0.contains("'") {

                        } else {
                            self.query = format!("{} WHERE {} {} '{}'", self.query, column, mark, value.0)
                        }
                    }
                    ValueType::Integer => self.query = format!("{} WHERE {} {} {}", self.query, column, mark, value.0),
                    ValueType::Float => self.query = format!("{} WHERE {} {} {}", self.query, column, mark, value.0),
                    ValueType::Boolean => {
                        if value.0 == "true" || value.0 == "TRUE" {
                            self.query = format!("{} WHERE {} {} {}", self.query, column, mark, true);
                        }

                        if value.0 == "false" || value.0 == "FALSE" {
                            self.query = format!("{} WHERE {} {} {}", self.query, column, mark, false);
                        }

                        if value.0 != "0" {
                            self.query = format!("{} WHERE {} {} 1", self.query, column, mark);
                        } else {
                            self.query = format!("{} WHERE {} {} 0", self.query, column, mark);
                        }
                    },
                    ValueType::Time => { 
                        if value.0.split("").collect::<Vec<&str>>().into_iter().all(|char| char == "" ||  char.parse::<i32>().is_ok()) {
                            self.query = format!("{} WHERE {} {} FROM_UNIXTIME({})", self.query, column, mark, value.0.trim());
                        } else {
                            self.query = format!("{} WHERE {} {} '{}'", self.query, column, mark, value.0);
                        }  
                    }
                }

                self.list.push(KeywordList::Where);

                self
            },
            Err(error) => {
                println!("An Error Occured when executing where method: {}", error);

                self.list.push(KeywordList::Where);

                self
            }
        }
    }

    /// It adds the "IN" keyword with it's synthax. Don't use ".where_cond()" method if you use it.
    pub fn where_in(&mut self, column: &str, ins: Vec<(&str, ValueType)>) -> &mut Self {
        self.query = format!("{} WHERE {} IN (", self.query, column);

        let length_of_ins = ins.len();

        for (index, value) in ins.into_iter().enumerate() {
            match value.1 {
                ValueType::String => {
                    if index + 1 == length_of_ins {
                        self.query = format!("{}'{}')", self.query, value.0);
                    
                        continue;
                    }

                    self.query = format!("{}'{}', ", self.query, value.0);
                },
                ValueType::Integer => {
                    if index + 1 == length_of_ins {
                        self.query = format!("{}{})", self.query, value.0);
                    
                        continue;
                    }

                    self.query = format!("{}{}, ", self.query, value.0);
                },
                ValueType::Float => {
                    if index + 1 == length_of_ins {
                        self.query = format!("{}{})", self.query, value.0);
                    
                        continue;
                    }

                    self.query = format!("{}{}, ", self.query, value.0);
                },
                ValueType::Boolean => {
                    if length_of_ins > 1 {
                        panic!("Error: if your value type is boolean and you cannot pass more than one IN parameter.");
                    }
    
                    self.query = format!("{}{})", self.query, value.0);
                },
                ValueType::Time => panic!("Not supported for now. If you want to search time types with IN operator use '.where_in_custom()' instead.")
            }
        }

        self.list.push(KeywordList::In);
        self
    }

    /// It adds the "NOT IN" keyword with it's synthax. Don't use ".where_cond()" method if you use it.
    pub fn where_not_in(&mut self, column: &str, ins: Vec<(&str, ValueType)>) -> &mut Self {
        self.query = format!("{} WHERE {} NOT IN (", self.query, column);

        let length_of_ins = ins.len();

        for (index, value) in ins.into_iter().enumerate() {
            match value.1 {
                ValueType::String => {
                    if index + 1 == length_of_ins {
                        self.query = format!("{}'{}')", self.query, value.0);
                    
                        continue;
                    }

                    self.query = format!("{}'{}', ", self.query, value.0);
                },
                ValueType::Integer => {
                    if index + 1 == length_of_ins {
                        self.query = format!("{}{})", self.query, value.0);
                    
                        continue;
                    }

                    self.query = format!("{}{}, ", self.query, value.0);
                },
                ValueType::Float => {
                    if index + 1 == length_of_ins {
                        self.query = format!("{}{})", self.query, value.0);
                    
                        continue;
                    }

                    self.query = format!("{}{}, ", self.query, value.0);
                },
                ValueType::Boolean => {
                    if length_of_ins > 1 {
                        panic!("Error: if your value type is boolean and you cannot pass more than one IN parameter.");
                    }
    
                    self.query = format!("{}{})", self.query, value.0);
                },
                ValueType::Time => panic!("Not supported for now. If you want to search time types with IN operator use '.where_in_custom()' instead.")
            }
        }

        self.list.push(KeywordList::NotIn);
        self
    }

    /// It adds the "IN" keyword with it's synthax and an empty condition, use it if you want to give more complex condition to "IN" keyword. Don't use ".where_cond()" with it.
    pub fn where_in_custom(&mut self, column: &str, query: &str) -> &mut Self {
        self.query = format!("{} WHERE {} IN ({})", self.query, column, query);

        self.list.push(KeywordList::In);
        self
    }

    /// It adds the "NOT IN" keyword with it's synthax and an empty condition, use it if you want to give more complex condition to "NOT IN" keyword. Don't use ".where_cond()" with it.
    pub fn where_not_in_custom(&mut self, column: &str, query: &str) -> &mut Self {
        self.query = format!("{} WHERE {} NOT IN ({})", self.query, column, query);

        self.list.push(KeywordList::NotIn);

        self
    }

    /// It adds the "OR" keyword with it's synthax. Warning: It's not ready yet to chaining "AND" and "OR" keywords, for now, applying that kind of complex query use ".append_custom()" method instead.
    pub fn or(&mut self, column: &str, mark: &str, value: (&str, ValueType)) -> &mut Self {
        match QueryBuilder::sanitize(vec![column, mark, value.0]) {
            Ok(_) => {
                match value.1 {
                    ValueType::String => self.query = format!("{} OR {} {} '{}'", self.query, column, mark, value.0),
                    ValueType::Integer => self.query = format!("{} OR {} {} {}", self.query, column, mark, value.0),
                    ValueType::Float => self.query = format!("{} OR {} {} {}", self.query, column, mark, value.0),
                    ValueType::Boolean => {
                        if value.0 == "true" || value.0 == "TRUE" {
                            self.query = format!("{} OR {} {} {}", self.query, column, mark, true);
                        }

                        if value.0 == "false" || value.0 == "FALSE" {
                            self.query = format!("{} OR {} {} {}", self.query, column, mark, false);
                        }
                    },
                    ValueType::Time => {
                        if value.0.split("").collect::<Vec<&str>>().into_iter().all(|char| char == "" ||  char.parse::<i32>().is_ok()) {
                            self.query = format!("{} OR {} {} FROM_UNIXTIME({})", self.query, column, mark, value.0.trim());
                        } else {
                            self.query = format!("{} OR {} {} '{}'", self.query, column, mark, value.0);
                        }  
                    }
                }

                self.list.push(KeywordList::Or);

                self
            },
            Err(error) => {
                println!("An Error Occured when executing or method: {}", error);

                self.list.push(KeywordList::Or);

                self
            }
        }
    }

    /// It adds the "SET" keyword with it's synthax.
    pub fn set(&mut self, column: &str, value: (&str, ValueType)) -> &mut Self {
        match QueryBuilder::sanitize(vec![column, value.0]) {
            Ok(_) => {
                match value.1 {
                    ValueType::String => {
                        if self.query.contains("SET") {
                            self.query = format!("{}, {} = '{}'", self.query, column, value.0)
                        } else {
                            self.query = format!("{} SET {} = '{}'", self.query, column, value.0)
                        }
                    },
                    ValueType::Integer => {
                        if self.query.contains("SET") {
                            self.query = format!("{}, {} = {}", self.query, column, value.0)
                        } else {
                            self.query = format!("{} SET {} = {}", self.query, column, value.0);
                        }
                    }
                    ValueType::Float => {
                        if self.query.contains("SET") {
                            self.query = format!("{}, {} = {}", self.query, column, value.0)
                        } else {
                            self.query = format!("{} SET {} = {}", self.query, column, value.0);
                        }
                    }
                    ValueType::Boolean => {
                        if value.0 == "true" || value.0 == "TRUE" {
                            if self.query.contains("SET") {
                                self.query = format!("{}, {} = {}", self.query, column, true);
                            } else {
                                self.query = format!("{} SET {} = {}", self.query, column, true);
                            }
                        }

                        if value.0 == "false" || value.0 == "FALSE" {
                            if self.query.contains("SET") {
                                self.query = format!("{}, {} = {}", self.query, column, false);
                            } else {
                                self.query = format!("{} SET {} = {}", self.query, column, false);
                            }
                        }
                    },
                    ValueType::Time => {
                        if self.query.contains("SET") {
                            match value.0 {
                                "UNIX_TIMESTAMP" => self.query = format!("{}, {} = {}", self.query, column, value.0),
                                "CURRENT_TIMESTAMP" => self.query = format!("{}, {} = {}", self.query, column, value.0),
                                "CURRENT_DATE" => self.query = format!("{}, {} = {}", self.query, column, value.0),
                                "CURRENT_TIME" => self.query = format!("{}, {} = {}", self.query, column, value.0),
                                "NOW()" => self.query = format!("{}, {} = {}", self.query, column, value.0),
                                "CURDATE()" => self.query = format!("{}, {} = {}", self.query, column, value.0),
                                "CURTIME()" => self.query = format!("{}, {} = {}", self.query, column, value.0),
                                _ => {
                                    self.query = format!("{}, {} = '{}'", self.query, column, value.0);

                                    if value.0.split("").collect::<Vec<&str>>().into_iter().all(|char| char == "" ||  char.parse::<i32>().is_ok()) {
                                        self.query = format!("{}, {} = FROM_UNIXTIME({})", self.query, column, value.0.trim());
                                    } else {
                                        self.query = format!("{}, {} = '{}'", self.query, column, value.0);
                                    }  
                                }
                            }
                        } else {
                            match value.0 {
                                "UNIX_TIMESTAMP" => self.query = format!("{} SET {} = {}", self.query, column, value.0),
                                "CURRENT_TIMESTAMP" => self.query = format!("{} SET {} = {}", self.query, column, value.0),
                                "CURRENT_DATE" => self.query = format!("{} SET {} = {}", self.query, column, value.0),
                                "CURRENT_TIME" => self.query = format!("{} SET {} = {}", self.query, column, value.0),
                                "NOW()" => self.query = format!("{} SET {} = {}", self.query, column, value.0),
                                "CURDATE()" => self.query = format!("{} SET {} = {}", self.query, column, value.0),
                                "CURTIME()" => self.query = format!("{} SET {} = {}", self.query, column, value.0),
                                _ => {
                                    if value.0.split("").collect::<Vec<&str>>().into_iter().all(|char| char == "" ||  char.parse::<i32>().is_ok()) {
                                        self.query = format!("{} SET {} = FROM_UNIXTIME({})", self.query, column, value.0.trim());
                                    } else {
                                        self.query = format!("{} SET {} = '{}'", self.query, column, value.0);
                                    }  
                                }
                            }
                        }
                    }
                }

                self.list.push(KeywordList::Set);

                self
            },
            Err(error) => {
                println!("An Error Occured when executing or method: {}", error);

                self.list.push(KeywordList::Set);

                self
            }
        }
    }

    /// It adds the "AND" keyword with it's synthax. Warning: It's not ready yet to chaining "OR" and "AND" keywords, for now, applying that kind of complex query use ".append_custom()" method instead.
    pub fn and(&mut self, column: &str, mark: &str, value: (&str, ValueType)) -> &mut Self {
        match QueryBuilder::sanitize(vec![column, mark, value.0]) {
            Ok(_) => {
                match value.1 {
                    ValueType::String => self.query = format!("{} AND {} {} '{}'", self.query, column, mark, value.0),
                    ValueType::Integer => self.query = format!("{} AND {} {} {}", self.query, column, mark, value.0),
                    ValueType::Float => self.query = format!("{} AND {} {} {}", self.query, column, mark, value.0),
                    ValueType::Boolean => {
                        if value.0 == "true" || value.0 == "TRUE" {
                            self.query = format!("{} AND {} {} {}", self.query, column, mark, true);
                        }

                        if value.0 == "false" || value.0 == "FALSE" {
                            self.query = format!("{} AND {} {} {}", self.query, column, mark, false);
                        }
                    },
                    ValueType::Time => {
                        if value.0.split("").collect::<Vec<&str>>().into_iter().all(|char| char == "" ||  char.parse::<i32>().is_ok()) {
                            self.query = format!("{} AND {} {} FROM_UNIXTIME({})", self.query, column, mark, value.0.trim());
                        } else {
                            self.query = format!("{} AND {} {} '{}'", self.query, column, mark, value.0);
                        }  
                    }
                }

                self.list.push(KeywordList::And);

                self
            },
            Err(error) => {
                println!("An Error Occured when executing and method: {}", error);

                self.list.push(KeywordList::And);

                self
            }
        }
    }

    /// It adds the "OFFSET" keyword with it's synthax. Be careful about it's alignment with "LIMIT" keyword.
    pub fn offset(&mut self, offset: i32) -> &mut Self {
        self.query = format!("{} OFFSET {}", self.query, offset);

        self.list.push(KeywordList::Offset);

        self
    }

    /// It adds the "LIMIT" keyword with it's synthax.
    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.query = format!("{} LIMIT {}", self.query, limit);

        self.list.push(KeywordList::Limit);

        self
    }

    /// It adds the "LIKE" keyword with it's synthax.
    pub fn like(&mut self, columns: Vec<&str>, operand: &str) -> &mut Self {
        match QueryBuilder::sanitize(columns.clone()) {
            Ok(_) => {
                match QueryBuilder::sanitize(vec![operand]){
                    Ok(_) => (),
                    Err(error) => {
                        println!("That Error Occured in like method: {}", error);
        
                        self.list.push(KeywordList::Like);

                        return self
                    }
                }

                for (i, column) in columns.into_iter().enumerate() {
                    if i == 0 {
                        self.query = format!("{} WHERE {} LIKE '%{}%'", self.query, column, operand);
                    } else {
                        self.query = format!("{} OR {} LIKE '%{}%'", self.query, column, operand);
                    }
                }

                self.list.push(KeywordList::Like);

                self
            },
            Err(error) => {
                println!("That Error Occured in like method: {}", error);

                self.list.push(KeywordList::Like);

                self
            }
        }
    }

    /// It adds the "ORDER BY" keyword with it's synthax. It only accepts "ASC", "DESC", "asc", "desc" values.
    pub fn order_by(&mut self, column: &str, mut ordering: &str) -> &mut Self {
        if self.query.contains("ORDER BY") {
            panic!("Error in order_by method: you cannot add ordering option twice on a query.");
        }

        match QueryBuilder::sanitize(vec![column]) {
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

        self.query = format!("{} ORDER BY {} {}", self.query, column, ordering);
        self.list.push(KeywordList::OrderBy);

        self
    }

    /// A practical method that adds a query for shuffling the lines.
    pub fn order_random(&mut self) -> &mut Self {
        if self.query.contains("ORDER BY") {
            panic!("Error in order_random method: you cannot add ordering option twice on a query.");
        }

        self.query = format!("{} ORDER BY RAND()", self.query);
        self.list.push(KeywordList::OrderBy);

        self
    }

    /// It adds the "GROUP BY" keyword with it's Synthax.
    pub fn group_by(&mut self, column: &str) -> &mut Self {
        self.query = format!("{} GROUP BY {}", self.query, column);

        self.list.push(KeywordList::GroupBy);

        self
    }

    /// A wildcard method that gives you the chance to write a part of your query. Warning, it does not add any keyword to builder, i'll encourage to add proper keyword to it with `.append_keyword()` method for your custom query, otherwise you should continue building your query by yourself with that function, or you've to be prepared to encounter bugs.  
    pub fn append_custom(&mut self, query: &str) -> &mut Self {
        self.query = format!("{} {}", self.query, query);

        self
    }

    /// A wildcard method that benefits you to append a keyword to the keyword list, so the QueryBuilder can build your queries properly, later than you appended your custom string to your query. It should be used with `.append_custom()` method. 
    pub fn append_keyword(&mut self, keyword: KeywordList) -> &mut Self {
        self.list.push(keyword);

        self
    }
    
    /// It applies "JSON_EXTRACT()" mysql function with it's Synthax. If you encounter any syntactic bugs or deficiencies about that function, please report it via opening an issue.
    pub fn json_extract(&mut self, column: &str, field: &str, _as: Option<&str>) -> &mut Self {
        match self.list.last() {
            Some(keyword) => {
                match keyword {
                    KeywordList::Where => {
                        if _as.is_some() {
                            println!("Warning: You've gave _as value to some variant and used it later than 'WHERE' keyword on .json_extract() method. In that usage, that value has no effect, you should gave it none value.");
                        }

                        match self.query.contains(column) {
                            false => panic!("Your query should include the same column name with column parameter if you use '.json_extract()' method later than '.where_cond()' method."),
                            true => ()
                        }

                        match self.table.as_str() == column {
                            true => {
                                let mut split_the_query = self.query.split(column);
                                let string_for_replace = format!("JSON_EXTRACT({}, '$.{}')", column, field);

                                self.query = format!("SELECT{}{}{}", self.table, string_for_replace, split_the_query.nth(2).unwrap()) 
                            },
                            false => {
                                let string_for_replace = format!("JSON_EXTRACT({}, '$.{}')", column, field);

                                self.query = self.query.replace(column,&string_for_replace)
                            }
                        }
                    },
                    KeywordList::And => {
                        if _as.is_some() {
                            println!("Warning: You've gave _as value to some variant and used it later than 'AND' keyword on .json_extract() method. In that usage, that value has no effect, you should gave it none value.");
                        }

                        let query_to_comp = format!("AND {}", column);

                        match self.query.contains(&query_to_comp) {
                            false => panic!("Your query should include the same column name with column parameter if you use '.json_extract()' method later than '.and()' method."),
                            true => ()
                        }

                        match self.table.as_str() == column {
                            true => {
                                let mut split_the_query = self.query.split(&query_to_comp);
                                let string_for_replace = format!("JSON_EXTRACT({}, '$.{}')", column, field);

                                self.query = format!("{}AND {}{}", split_the_query.nth(0).unwrap(), string_for_replace, split_the_query.nth(0).unwrap()) 
                            },
                            false => {
                                match self.query.matches(&query_to_comp).count() {
                                    0 => (),
                                    1 => {
                                        let string_for_replace = format!("AND JSON_EXTRACT({}, '$.{}')", column, field);

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

                                        let string_for_replace = format!("AND JSON_EXTRACT({}, '$.{}')", column, field);

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

                        let query_to_comp = format!("OR {}", column);

                        match self.query.contains(&query_to_comp) {
                            false => panic!("Your query should include the same column name with column parameter if you use '.json_extract()' method later than '.or()' method."),
                            true => ()
                        }

                        match self.table.as_str() == column {
                            true => {
                                let mut split_the_query = self.query.split(&query_to_comp);
                                let string_for_replace = format!("JSON_EXTRACT({}, '$.{}')", column, field);

                                self.query = format!("{}OR {}{}", split_the_query.nth(0).unwrap(), string_for_replace, split_the_query.nth(0).unwrap()) 
                            },
                            false => {
                                match self.query.matches(&query_to_comp).count() {
                                    0 => (),
                                    1 => {
                                        let string_for_replace = format!("OR JSON_EXTRACT({}, '$.{}')", column, field);

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

                                        let string_for_replace = format!("OR JSON_EXTRACT({}, '$.{}')", column, field);

                                        self.query = format!("{} {} {}", new_chunk, string_for_replace, last_chunk)
                                    }
                                }
                            }
                        }
                    },
                    KeywordList::Select => {
                        let string_for_put = format!("JSON_EXTRACT({}, '$.{}')", column, field);

                        match _as {
                            Some(_as) => self.query = format!("SELECT {} AS {} FROM", string_for_put, _as),
                            None => self.query = format!("SELECT {} FROM", string_for_put),
                        }
                    },
                    KeywordList::Table => {
                        let string_for_put = format!("JSON_EXTRACT({}, '$.{}')", column, field);

                        match _as {
                            Some(_as) => self.query = format!("SELECT {} AS {} FROM {}", string_for_put, _as, self.table),
                            None => self.query = format!("SELECT {} FROM {}", string_for_put, self.table),
                        }
                    },
                    KeywordList::OrderBy => {
                        if _as.is_some() {
                            println!("Warning: You've gave _as value to some variant and used it later than 'ORDER BY' operator on .json_extract() method. In that usage, that value has no effect, you should gave it none value.");
                        }

                        match self.query.contains(&column) {
                            false => panic!("Your query should include the same column name with column parameter if you use '.json_extract()' method later than '.order_by()' method."),
                            true => ()
                        }

                        match self.query.matches(" ORDER BY ").count() {
                            0 => (),
                            1 => {
                                let split_the_query = self.query.clone();
                                let mut split_the_query = split_the_query.split(" ORDER BY ");

                                let string_for_put = format!("ORDER BY JSON_EXTRACT({}, '$.{}')", column, field);
        
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
                            Some(_as) => format!("JSON_EXTRACT({}, '$.{}') AS {}", column, field, _as),
                            None => format!("JSON_EXTRACT({}, '$.{}')", column, field)
                        };

                        self.query = format!("SELECT {}, COUNT{}", string_for_put, split_the_query.nth(1).unwrap())
                    },
                    _ => ()
                }
            },
            None => ()
        }
        
        self.list.push(KeywordList::JsonExtract);
        self
    }

    /// finishes the query and returns the result as string.
    pub fn finish(&self) -> String {
        return format!("{};", self.query);
    }

    /// checks the inputs for potential sql injection patterns and throws error if they exist.
    fn sanitize(fields: Vec<&str>) -> std::result::Result<String, std::io::Error> {
        if fields.len() == 1 && fields[0] == "" {
            return Ok("".to_string());
        };

        for field in fields.into_iter() {
            if (field.to_lowercase().contains(";") && (field.len() < 40)) ||
                field.to_lowercase().contains("; drop") ||
                field.to_lowercase().contains("admin' #") ||
                field.to_lowercase().contains("admin'/*") ||
                field.to_lowercase().contains("; union") ||
                field.to_lowercase().contains("or 1 = 1") ||
                field.to_lowercase().contains("or 1 = 1#") ||
                field.to_lowercase().contains("or 1 = 1/*") ||
                field.to_lowercase().contains("or true = true") ||
                field.to_lowercase().contains("or false = false") ||
                field.to_lowercase().contains("or '1' = '1'") ||
                field.to_lowercase().contains("or '1' = '1'#") ||
                field.to_lowercase().contains("or '1' = '1'/*") ||
                field.to_lowercase().contains("; sleep(") ||
                field.to_lowercase().contains("--") ||
                field.to_lowercase().contains("drop table") ||
                field.to_lowercase().contains("drop schema") ||
                field.to_lowercase().contains("select if") ||
                field.to_lowercase().contains("union select") ||
                field.to_lowercase().contains("union all") ||
                field.to_lowercase().contains("exec") ||
                field.to_lowercase().contains("master..") ||
                field.to_lowercase().contains("masters..") ||
                field.to_lowercase().contains("information_schema") ||
                field.to_lowercase().contains("load_file") ||
                field.to_lowercase().contains("alter user") {
                    return Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "You cannot run arbitrary queries"))
            }
        }

        return Ok("".to_string())
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

    pub fn default(&mut self, value: (&str, ValueType)) -> &mut Self {
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
            match value.1 {
                ValueType::Integer => self.query = format!("{} DEFAULT {}", self.query, value.0),
                _ => panic!("Error: If your column has the of the types of INT, TINYINT, SMALLINT, MEDIUMINT, BIGINT, BIT or SERIAL, it has to be an integer.")
            }
        }

        if last_query.contains("BOOL") || 
           last_query.contains("BOOLEAN") {
            match value.1 {
                ValueType::Boolean => self.query = format!("{} DEFAULT {}", self.query, value.0),
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
            match value.1 {
                ValueType::String => self.query = format!("{} DEFAULT '{}'", self.query, value.0),
                _ => panic!("Error: if your column type is one of the types of CHAR, VARCHAR, TEXT, TINYTEXT, MEDIUMTEXT, LONGTEXT, BINARY or VARBINARY, your value type has to be String.")
            }
        }

        if last_query.contains("DATETIME") ||
           last_query.contains("TIMESTAMP") {
            match value.1 {
                ValueType::String => self.query = format!("{} DEFAULT {}", self.query, value.0),
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

    pub fn default_on_null(&mut self, value: (&str, ValueType)) -> &mut Self {
        match value.1 {
            ValueType::String => self.query = format!("{} DEFAULT '{}' ON NULL", self.query, value.0),
            _ => self.query = format!("{} DEFAULT {} ON NULL", self.query, value.0),
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
#[derive(Debug, Clone)]
pub enum KeywordList {
    Select, Update, Delete, Insert, Count, Table, Where, Or, And, Set, 
    Finish, OrderBy, GroupBy, Like, Limit, Offset, IfNotExist, Create, Use, In, 
    NotIn, JsonExtract,
}

/// QueryType enum. It helps to detect the type of a query with more optimized way when is needed.
#[derive(Debug, Clone)]
pub enum QueryType {
    Select, Update, Delete, Insert, Null, Create, Count
}

/// ValueType enum. It benefits to detect and format the value with optimized way when you have to work with exact column values. 
#[derive(Debug, Clone)]
pub enum ValueType {
    String, Boolean, Integer, Float, Time 
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
        let values = vec![("What's Up?", ValueType::String), ("John Doe", ValueType::String), ("Lorem ipsum dolor sit amet, consectetur adipiscing elit.", ValueType::String)];
    
        let insert_query = QueryBuilder::insert(columns, values).unwrap().table("blogs").finish();

        println!("{}", insert_query);
        assert_eq!("INSERT INTO blogs (title, author, description) VALUES ('What's Up?', 'John Doe', 'Lorem ipsum dolor sit amet, consectetur adipiscing elit.');".to_string(), 
                    insert_query);
    }

    #[test]
    pub fn test_update_query(){
        let update_query = QueryBuilder::update().unwrap().table("blogs").set("title", ("Hello Rust!", ValueType::String)).set("author", ("Necdet", ValueType::String)).finish();

        assert_eq!("UPDATE blogs SET title = 'Hello Rust!', author = 'Necdet';", update_query);
    }

    #[test]
    pub fn test_delete_query(){
        let delete_query = QueryBuilder::delete().unwrap().table("blogs").where_cond("id", "=", ("1", ValueType::String)).finish();

        assert_eq!("DELETE FROM blogs WHERE id = '1';", delete_query);
    }

    #[test]
    pub fn test_select_query_declarative(){
        let mut select = QueryBuilder::select(["id", "title", "description", "point"].to_vec()).unwrap();

        let select_query = select.table("blogs")
                                    .where_cond("id", "=", ("10", ValueType::Integer))
                                    .and("point", ">", ("90", ValueType::Integer))
                                    .or("id", "=", ("20", ValueType::Integer))
                                    .finish();

        assert_eq!("SELECT id, title, description, point FROM blogs WHERE id = 10 AND point > 90 OR id = 20;", select_query)
    }

    #[test]
    pub fn test_select_query_imperative(){
        let mut select = QueryBuilder::select(["*"].to_vec()).unwrap();

        let select_query = select.table("blogs");
        select_query.where_cond("id", "=", ("5", ValueType::Integer));
        select_query.or("id", "=", ("25", ValueType::Integer));

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
        let values = [("necoo33", ValueType::String), ("123456", ValueType::String), ("CURRENT_TIMESTAMP", ValueType::Time)].to_vec();
    
        let time_insert_test = QueryBuilder::insert(columns, values).unwrap().table("users").finish();

        assert_eq!(time_insert_test, "INSERT INTO users (name, password, last_login) VALUES ('necoo33', '123456', CURRENT_TIMESTAMP);");

        let time_update_test = QueryBuilder::update().unwrap().table("users").set("last_login", ("CURRENT_TIMESTAMP", ValueType::Time)).where_cond("name", "=", ("necoo33", ValueType::String)).finish();

        assert_eq!(time_update_test, "UPDATE users SET last_login = CURRENT_TIMESTAMP WHERE name = 'necoo33';")
    }

    #[test]
    pub fn test_unix_epoch_times(){
        let columns = ["name", "password", "last_login"].to_vec();
        let values = [("necoo33", ValueType::String), ("123456", ValueType::String), ("134523452", ValueType::Time)].to_vec();
    
        let time_insert_with_unix_epoch_times_test = QueryBuilder::insert(columns, values).unwrap().table("users").finish();
        assert_eq!(time_insert_with_unix_epoch_times_test, "INSERT INTO users (name, password, last_login) VALUES ('necoo33', '123456', FROM_UNIXTIME(134523452));");
    
        let time_update_with_unix_epoch_times_test = QueryBuilder::update().unwrap().table("users").set("last_login", ("3456436", ValueType::Time)).where_cond("name", "=", ("necoo33", ValueType::String)).finish();

        assert_eq!(time_update_with_unix_epoch_times_test, "UPDATE users SET last_login = FROM_UNIXTIME(3456436) WHERE name = 'necoo33';");

        let columns = ["name", "password", "last_login", "created_at"].to_vec();

        let unix_epoch_times_test_3 = QueryBuilder::select(columns).unwrap().table("users").where_cond("created_at", ">", ("3234534", ValueType::Time)).or("last_login", ">=", ("2134432", ValueType::Time)).offset(0).limit(20).finish();

        assert_eq!(unix_epoch_times_test_3, "SELECT name, password, last_login, created_at FROM users WHERE created_at > FROM_UNIXTIME(3234534) OR last_login >= FROM_UNIXTIME(2134432) OFFSET 0 LIMIT 20;")
    }

    #[test]
    pub fn test_where_ins(){
        let columns = ["name", "age", "id", "last_login"].to_vec();

        let ids = [("1", ValueType::Integer), ("12", ValueType::Integer), ("8", ValueType::Integer)].to_vec();

        let test_where_in = QueryBuilder::select(columns).unwrap().table("users").where_in("id", ids).finish();

        assert_eq!(test_where_in, "SELECT name, age, id, last_login FROM users WHERE id IN (1, 12, 8);");

        let columns = ["name", "age", "id", "last_login"].to_vec();

        let ids = [("1", ValueType::Integer), ("12", ValueType::Integer), ("8", ValueType::Integer)].to_vec();

        let test_where_not_in = QueryBuilder::select(columns).unwrap().table("users").where_not_in("id", ids).finish();

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
        let count_of_users = QueryBuilder::count("*", None).table("users").where_cond("age", ">", ("25", ValueType::Integer)).finish();

        assert_eq!(count_of_users, "SELECT COUNT(*) FROM users WHERE age > 25;".to_string());

        let count_of_users_as_length = QueryBuilder::count("*", Some("length")).table("users").finish();

        assert_eq!(count_of_users_as_length, "SELECT COUNT(*) AS length FROM users;".to_string());
    }

    #[test]
    pub fn test_json_extract(){
        // tests with "select()" constructor

        let select_query_1 = QueryBuilder::select(["*"].to_vec()).unwrap().json_extract("data", "age", Some("student_age")).table("students").finish();

        assert_eq!(select_query_1, "SELECT JSON_EXTRACT(data, '$.age') AS student_age FROM students;".to_string());
        
        let select_query_2 = QueryBuilder::select(["*"].to_vec()).unwrap().json_extract("data", "age", Some("student_age")).table("students").where_cond("successfull", "=", ("1", ValueType::Boolean)).finish();

        assert_eq!(select_query_2, "SELECT JSON_EXTRACT(data, '$.age') AS student_age FROM students WHERE successfull = 1;".to_string());
        
        let select_query_3 = QueryBuilder::select(["*"].to_vec()).unwrap().json_extract("data", "age", Some("student_age")).table("students").where_cond("points", ">", ("85", ValueType::Integer)).json_extract("points", "name", None).finish();

        assert_eq!(select_query_3, "SELECT JSON_EXTRACT(data, '$.age') AS student_age FROM students WHERE JSON_EXTRACT(points, '$.name') > 85;".to_string());

        // tests with ".where_cond()" method

        let with_where = QueryBuilder::delete().unwrap().table("users").where_cond("id", ">", ("200", ValueType::Integer)).json_extract("id", "user_id", None).finish();

        assert_eq!(with_where, "DELETE FROM users WHERE JSON_EXTRACT(id, '$.user_id') > 200;".to_string());

        // tests with ".table()" method
        
        let fields = ["name", "age"].to_vec();
        
        let with_table = QueryBuilder::select(fields).unwrap().table("users").json_extract("id", "user_id", None).finish();

        assert_eq!(with_table, "SELECT JSON_EXTRACT(id, '$.user_id') FROM users;".to_string());

        // tests with ".and()" method

        let fields = ["name", "age"].to_vec();

        let with_and_1 = QueryBuilder::select(fields).unwrap().table("height").where_cond("weight", ">", ("60", ValueType::Integer)).and("height", ">", ("1.70", ValueType::Float)).json_extract("height", "student_height", None).finish();

        assert_eq!(with_and_1, "SELECT name, age FROM height WHERE weight > 60 AND JSON_EXTRACT(height, '$.student_height') > 1.70;".to_string());
    
        let with_and_2 = QueryBuilder::select(["*"].to_vec()).unwrap().table("students").where_cond("weight", ">", ("60", ValueType::Integer)).and("height", ">", ("1.70", ValueType::Float)).json_extract("height", "student_height", None).finish();

        assert_eq!(with_and_2, "SELECT * FROM students WHERE weight > 60 AND JSON_EXTRACT(height, '$.student_height') > 1.70;".to_string());

        // tests with ".or()" method

        let fields = ["name", "age"].to_vec();

        let with_or_1 = QueryBuilder::select(fields).unwrap().table("height").where_cond("weight", ">", ("60", ValueType::Integer)).or("height", ">", ("1.70", ValueType::Float)).json_extract("height", "student_height", None).finish();

        assert_eq!(with_or_1, "SELECT name, age FROM height WHERE weight > 60 OR JSON_EXTRACT(height, '$.student_height') > 1.70;".to_string());
    
        let with_or_2 = QueryBuilder::select(["*"].to_vec()).unwrap().table("students").where_cond("weight", ">", ("60", ValueType::Integer)).or("height", ">", ("1.70", ValueType::Float)).json_extract("height", "student_height", None).finish();

        assert_eq!(with_or_2, "SELECT * FROM students WHERE weight > 60 OR JSON_EXTRACT(height, '$.student_height') > 1.70;".to_string());

        // tests with "count()" constructor

        let count_query_1 = QueryBuilder::count("*", None).json_extract("age", "student_age", Some("value")).table("students").group_by("points").finish();

        assert_eq!(count_query_1, "SELECT JSON_EXTRACT(age, '$.student_age') AS value, COUNT(*) FROM students GROUP BY points;".to_string());

        // tests with ".order_by()" method
        
        let fields = ["title", "desc", "created_at", "updated_at", "keywords", "pics", "likes"].to_vec();

        let order_by_query_1 = QueryBuilder::select(fields).unwrap().table("contents").where_cond("published", "=", ("1", ValueType::Boolean)).order_by("likes", "ASC").json_extract("likes", "name", None).finish();

        assert_eq!(order_by_query_1, "SELECT title, desc, created_at, updated_at, keywords, pics, likes FROM contents WHERE published = 1 ORDER BY JSON_EXTRACT(likes, '$.name') ASC;".to_string());
    }
}
