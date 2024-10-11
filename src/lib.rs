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
}
