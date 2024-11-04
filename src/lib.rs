#[derive(Debug, Clone)]
pub struct QueryBuilder {
    pub query: String,
    pub table: String,
    pub qtype: QueryType,
    pub list: Vec<KeywordList>
}

impl QueryBuilder {
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

    pub fn delete() -> std::result::Result<Self, std::io::Error> {
        return Ok(QueryBuilder {
            query: "DELETE FROM".to_string(),
            table: "".to_string(),
            qtype: QueryType::Delete,
            list: vec![KeywordList::Delete]
        })
    }

    pub fn update() -> std::result::Result<Self, std::io::Error> {
        return Ok(QueryBuilder {
            query: "UPDATE".to_string(),
            table: "".to_string(),
            qtype: QueryType::Update,
            list: vec![KeywordList::Update]
        })
    }

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
            QueryType::Null => panic!("You cannot add a table before you start a query"),
            QueryType::Create => panic!("You cannot use create keyword with a QueryBuilder instance")
        }

        self.list.push(KeywordList::Table);

        Self {
            query: self.query.clone(),
            table: self.table.clone(),
            qtype: self.qtype.clone(),
            list: self.list.clone()
        }
    }

    pub fn where_cond(&mut self, column: &str, mark: &str, value: (&str, ValueType)) -> Self {
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
                    }
                }

                self.list.push(KeywordList::Where);
                return Self {
                    query: self.query.clone(),
                    table: self.table.clone(),
                    qtype: self.qtype.clone(),
                    list: self.list.clone()
                }
            },
            Err(error) => {
                println!("An Error Occured when executing where method: {}", error);

                self.list.push(KeywordList::Where);
                return Self {
                    query: self.query.clone(),
                    table: self.table.clone(),
                    qtype: self.qtype.clone(),
                    list: self.list.clone()
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

                self.list.push(KeywordList::Or);

                return Self {
                    query: self.query.clone(),
                    table: self.table.clone(),
                    qtype: self.qtype.clone(),
                    list: self.list.clone()
                }
            },
            Err(error) => {
                println!("An Error Occured when executing or method: {}", error);

                self.list.push(KeywordList::Or);
                return Self {
                    query: self.query.clone(),
                    table: self.table.clone(),
                    qtype: self.qtype.clone(),
                    list: self.list.clone()
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

                self.list.push(KeywordList::Set);
                return Self {
                    query: self.query.clone(),
                    table: self.table.clone(),
                    qtype: self.qtype.clone(),
                    list: self.list.clone()
                }
            },
            Err(error) => {
                println!("An Error Occured when executing or method: {}", error);

                self.list.push(KeywordList::Set);
                return Self {
                    query: self.query.clone(),
                    table: self.table.clone(),
                    qtype: self.qtype.clone(),
                    list: self.list.clone()
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

                self.list.push(KeywordList::And);
                return Self {
                    query: self.query.clone(),
                    table: self.table.clone(),
                    qtype: self.qtype.clone(),
                    list: self.list.clone()
                }
            },
            Err(error) => {
                println!("An Error Occured when executing and method: {}", error);

                self.list.push(KeywordList::And);
                return Self {
                    query: self.query.clone(),
                    table: self.table.clone(),
                    qtype: self.qtype.clone(),
                    list: self.list.clone()
                }
            }
        }
    }

    pub fn offset(&mut self, offset: i32) -> Self {
        self.query = format!("{} OFFSET {}", self.query, offset);

        self.list.push(KeywordList::Offset);
        Self {
            query: self.query.clone(),
            table: self.table.clone(),
            qtype: self.qtype.clone(),
            list: self.list.clone()
        }
    }

    pub fn limit(&mut self, limit: i32) -> Self {
        self.query = format!("{} LIMIT {}", self.query, limit);

        self.list.push(KeywordList::Limit);
        Self {
            query: self.query.clone(),
            table: self.table.clone(),
            qtype: self.qtype.clone(),
            list: self.list.clone()
        }
    }

    pub fn like(&mut self, columns: Vec<&str>, operand: &str) -> Self {
        match QueryBuilder::sanitize(columns.clone()) {
            Ok(_) => {
                match QueryBuilder::sanitize(vec![operand]){
                    Ok(_) => (),
                    Err(error) => {
                        println!("That Error Occured in like method: {}", error);
        
                        self.list.push(KeywordList::Like);
                        return Self {
                            query: self.query.clone(),
                            table: self.table.clone(),
                            qtype: self.qtype.clone(),
                            list: self.list.clone()
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

                self.list.push(KeywordList::Like);
                Self {
                    query: self.query.clone(),
                    table: self.table.clone(),
                    qtype: self.qtype.clone(),
                    list: self.list.clone()
                }
            },
            Err(error) => {
                println!("That Error Occured in like method: {}", error);

                self.list.push(KeywordList::Like);
                Self {
                    query: self.query.clone(),
                    table: self.table.clone(),
                    qtype: self.qtype.clone(),
                    list: self.list.clone()
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

                self.list.push(KeywordList::OrderBy);
                return Self {
                    query: self.query.clone(),
                    table: self.table.clone(),
                    qtype: self.qtype.clone(),
                    list: self.list.clone()
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
        self.list.push(KeywordList::OrderBy);
        Self {
            query: self.query.clone(),
            table: self.table.clone(),
            qtype: self.qtype.clone(),
            list: self.list.clone()
        }
    }

    pub fn order_random(&mut self) -> Self {
        if self.query.contains("ORDER BY") {
            panic!("Error in order_random method: you cannot add ordering option twice on a query.");
        }

        self.query = format!("{} ORDER BY RAND()", self.query);
        self.list.push(KeywordList::OrderBy);
        Self {
            query: self.query.clone(),
            table: self.table.clone(),
            qtype: self.qtype.clone(),
            list: self.list.clone()
        }
    }

    pub fn finish(&mut self) -> String {
        self.list.push(KeywordList::Finish);

        return format!("{};", self.query);
    }

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

#[derive(Debug, Clone)]
pub struct SchemaBuilder {
    pub query: String,
    pub schema: String,
    pub list: Vec<KeywordList>
}

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

    pub fn if_not_exists(&mut self) -> Self {
        match self.list[0] {
            KeywordList::Create => (),
            KeywordList::Table => (),
            _ => panic!("if_not_exists method cannot be used without Create or Table queries")
        }

        let split_the_query =  self.query.split(" DATABASE ").collect::<Vec<&str>>();
        self.query = format!("{} DATABASE IF NOT EXISTS {}", split_the_query[0], split_the_query[1]);

        self.list.insert(0, KeywordList::IfNotExist);
        Self {
            query: self.query.clone(),
            schema: self.schema.clone(),
            list: self.list.clone()
        }
    }

    pub fn use_schema(&mut self, name: Option<&str>) -> Self {
        match name {
            Some(schema_name) => {
                self.query = format!("USE {}", schema_name)
            },
            None => {
                self.query = format!("USE {}", self.schema);
            }
        }

        Self {
            query: self.query.clone(),
            schema: self.schema.clone(),
            list: self.list.clone()
        }
    }

    pub fn finish(&self) -> String {
        return format!("{};", self.query)
    }
}

#[derive(Debug, Clone)]
pub struct TableBuilder {
    pub query: String,
    pub name: String,
    pub schema: String,
    pub all: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ForeignKey {
    pub first: ForeignKeyItem,
    pub second: ForeignKeyItem,
    pub on_delete: Option<ForeignKeyActions>,
    pub on_update: Option<ForeignKeyActions>,
    pub constraint: Option<String>
}

#[derive(Debug, Clone)]
pub struct ForeignKeyItem {
    pub table: String,
    pub column: String
}

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

#[derive(Debug, Clone)]
pub enum KeywordList {
    Select, Update, Delete, Insert, Table, Where, Or, And, Set, Finish, OrderBy, Like, Limit, Offset, IfNotExist, Create, Use 
}

#[derive(Debug, Clone)]
pub enum QueryType {
    Select, Update, Delete, Insert, Null, Create
}

#[derive(Debug, Clone)]
pub enum ValueType {
    String, Boolean, Integer, Float 
}

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

        let mut select_query = select.table("blogs");
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
}
