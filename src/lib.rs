#[derive(Debug, Clone)]
pub struct QueryBuilder {
    pub query: String,
    pub table: String,
    pub qtype: QueryType
}

impl QueryBuilder {
    pub fn select(fields: Vec<&str>) -> std::result::Result<Self, std::io::Error> {
        match QueryBuilder::sanitize(fields.clone()) {
            Ok(_) => {
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
                    qtype: QueryType::Select
                })
            },
            Err(_) => {
                return Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "query cannot build because inserted arbitrary query."))
            }
        }
    }

    pub fn delete() -> std::result::Result<Self, std::io::Error> {
        return Ok(QueryBuilder {
            query: "DELETE FROM".to_string(),
            table: "".to_string(),
            qtype: QueryType::Delete,
        })
    }

    pub fn update() -> std::result::Result<Self, std::io::Error> {
        return Ok(QueryBuilder {
            query: "UPDATE".to_string(),
            table: "".to_string(),
            qtype: QueryType::Delete,
        })
    }

    pub fn insert(columns: Vec<&str>, values: Vec<(&str, ValueType)>) -> std::result::Result<Self, std::io::Error> {
        let mut query = "INSERT INTO".to_string();

        match QueryBuilder::sanitize(columns.clone()) {
            Ok(_) => (),
            Err(_) => {
                return Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "query cannot build on update constructor: because inserted arbitrary query on columns parameter."))
            }
        }

        let get_actual_values = values.clone().iter().map(|(s, _)| *s).collect::<Vec<&str>>();

        match QueryBuilder::sanitize(get_actual_values.clone()) {
            Ok(_) => (),
            Err(_) => {
                return Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "query cannot build on update constructor: because inserted arbitrary query in values parameter."))

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
                                ValueType::String => values_string = format!("{}'{}', ", values_string, value),
                                ValueType::Integer => values_string = format!("{}{}, ", values_string, value),
                                ValueType::Float => values_string = format!("{}{}, ", values_string, value),
                                ValueType::Boolean => values_string = format!("{}{}, ", values_string, value),
                            }
                        } else {
                            columns_string = format!("{}{})", columns_string, column);

                            match vtype {
                                ValueType::String => values_string = format!("{}'{}')", values_string, value),
                                ValueType::Integer => values_string = format!("{}{})", values_string, value),
                                ValueType::Float => values_string = format!("{}{})", values_string, value),
                                ValueType::Boolean => values_string = format!("{}{})", values_string, value),
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
            qtype: QueryType::Insert
        })
    }

    pub fn table(&mut self, table: &str) -> Self {
        match self.qtype {
            QueryType::Select => {
                self.query = format!("{} {}", self.query, table);
                self.table = table.to_string()
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
            QueryType::Null => panic!("You cannot add a table before you start a query")
        }

        Self {
            query: self.query.clone(),
            table: self.table.clone(),
            qtype: self.qtype.clone(),
        }
    }

    pub fn where_cond(&mut self, column: &str, mark: &str, value: (&str, ValueType)) -> Self {
        match QueryBuilder::sanitize(vec![column, mark, value.0]) {
            Ok(_) => {
                match value.1 {
                    ValueType::String => self.query = format!("{} WHERE {} {} '{}'", self.query, column, mark, value.0),
                    ValueType::Integer => self.query = format!("{} WHERE {} {} {}", self.query, column, mark, value.0),
                    ValueType::Float => self.query = format!("{} WHERE {} {} {}", self.query, column, mark, value.0),
                    ValueType::Boolean => {
                        if value.0 == "true" || value.0 == "TRUE" {
                            self.query = format!("{} WHERE {} {} {}", self.query, column, mark, true);
                        }

                        if value.0 == "false" || value.0 == "FALSE" {
                            self.query = format!("{} WHERE {} {} {}", self.query, column, mark, false);
                        }
                    }
                }

                return Self {
                    query: self.query.clone(),
                    table: self.table.clone(),
                    qtype: self.qtype.clone(),
                }
            },
            Err(error) => {
                println!("An Error Occured when executing where method: {}", error);

                return Self {
                    query: self.query.clone(),
                    table: self.table.clone(),
                    qtype: self.qtype.clone(),
                }
            }
        }
    }

    pub fn or(&mut self, column: &str, mark: &str, value: (&str, ValueType)) -> Self {
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
                    }
                }

                return Self {
                    query: self.query.clone(),
                    table: self.table.clone(),
                    qtype: self.qtype.clone(),
                }
            },
            Err(error) => {
                println!("An Error Occured when executing or method: {}", error);

                return Self {
                    query: self.query.clone(),
                    table: self.table.clone(),
                    qtype: self.qtype.clone()
                }
            }
        }
    }

    pub fn set(&mut self, column: &str, value: (&str, ValueType)) -> Self {
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
                    }
                }

                return Self {
                    query: self.query.clone(),
                    table: self.table.clone(),
                    qtype: self.qtype.clone()
                }
            },
            Err(error) => {
                println!("An Error Occured when executing or method: {}", error);

                return Self {
                    query: self.query.clone(),
                    table: self.table.clone(),
                    qtype: self.qtype.clone()
                }
            }
        }
    }

    pub fn and(&mut self, column: &str, mark: &str, value: (&str, ValueType)) -> Self {
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
                    }
                }

                return Self {
                    query: self.query.clone(),
                    table: self.table.clone(),
                    qtype: self.qtype.clone()
                }
            },
            Err(error) => {
                println!("An Error Occured when executing and method: {}", error);

                return Self {
                    query: self.query.clone(),
                    table: self.table.clone(),
                    qtype: self.qtype.clone()
                }
            }
        }
    }

    pub fn offset(&mut self, offset: i32) -> Self {
        self.query = format!("{} OFFSET {}", self.query, offset);

        Self {
            query: self.query.clone(),
            table: self.table.clone(),
            qtype: self.qtype.clone()
        }
    }

    pub fn limit(&mut self, limit: i32) -> Self {
        self.query = format!("{} LIMIT {}", self.query, limit);

        Self {
            query: self.query.clone(),
            table: self.table.clone(),
            qtype: self.qtype.clone()
        }
    }

    pub fn like(&mut self, columns: Vec<&str>, operand: &str) -> Self {
        match QueryBuilder::sanitize(columns.clone()) {
            Ok(_) => {
                match QueryBuilder::sanitize(vec![operand]){
                    Ok(_) => (),
                    Err(error) => {
                        println!("That Error Occured in like method: {}", error);
        
                        return Self {
                            query: self.query.clone(),
                            table: self.table.clone(),
                            qtype: self.qtype.clone()
                        }
                    }
                }

                for (i, column) in columns.into_iter().enumerate() {
                    if i == 0 {
                        self.query = format!("{} WHERE {} LIKE '%{}%'", self.query, column, operand);
                    } else {
                        self.query = format!("{} OR {} LIKE '%{}%'", self.query, column, operand);
                    }
                }

                Self {
                    query: self.query.clone(),
                    table: self.table.clone(),
                    qtype: self.qtype.clone()
                }
            },
            Err(error) => {
                println!("That Error Occured in like method: {}", error);

                Self {
                    query: self.query.clone(),
                    table: self.table.clone(),
                    qtype: self.qtype.clone()
                }
            }
        }
    }

    pub fn order_by(&mut self, column: &str, mut ordering: &str) -> Self {
        if self.query.contains("ORDER BY") {
            panic!("Error in order_by method: you cannot add ordering option twice on a query.");
        }

        match QueryBuilder::sanitize(vec![column]) {
            Ok(_) => (),
            Err(error) => {
                println!("{}", error);

                return Self {
                    query: self.query.clone(),
                    table: self.table.clone(),
                    qtype: self.qtype.clone()
                }
            }
        }

        match ordering {
            "asc" => ordering = "ASC",
            "desc" => ordering = "DESC",
            "ASC" => (),
            "DESC" => (),
            &_ => panic!("Panicking in order_by method: There is no other ordering options than ASC or DESC.")
        }

        self.query = format!("{} ORDER BY {} {}", self.query, column, ordering);

        Self {
            query: self.query.clone(),
            table: self.table.clone(),
            qtype: self.qtype.clone()
        }
    }

    pub fn order_random(&mut self) -> Self {
        if self.query.contains("ORDER BY") {
            panic!("Error in order_random method: you cannot add ordering option twice on a query.");
        }

        self.query = format!("{} ORDER BY RAND()", self.query);

        Self {
            query: self.query.clone(),
            table: self.table.clone(),
            qtype: self.qtype.clone()
        }
    }

    pub fn finish(&mut self) -> String {
        return format!("{};", self.query);
    }

    fn sanitize(fields: Vec<&str>) -> std::result::Result<String, std::io::Error> {
        if fields.len() == 1 && fields[0] == "" {
            return Ok("".to_string());
        };

        for field in fields.into_iter() {
            if field.contains("'") ||
               field.contains('"') ||
               field.contains(";") ||
               field.contains("OR 1 = 1") ||
               field.contains("-") ||
               field.contains("DROP TABLE") ||
               field.contains("DROP SCHEMA") ||
               field.contains("UNION SELECT") {
                return Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "You cannot run arbitrary queries"))
               }
        }

        return Ok("".to_string())
    }
}

#[derive(Debug, Clone)]
pub enum QueryType {
    Select, Update, Delete, Insert, Null
}

#[derive(Debug, Clone)]
pub enum ValueType {
    String, Boolean, Integer, Float 
}
