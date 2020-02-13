use internal::*;

#[derive(Debug, Clone)]
pub enum ParameterType {
    Map,
    List,
    Path,
    Identifier,
    Keyword,
    String,
    Character,
    Integer,
    Float,
    Boolean,
    Container,
    Key,
    Literal,
    Selector,
    Number,
    Location,
}

impl ParameterType {

    pub fn from(source: &str) -> Status<Self> {
        match source {
            "container" => return success!(ParameterType::Container),
            "key" => return success!(ParameterType::Key),
            "literal" => return success!(ParameterType::Literal),
            "selector" => return success!(ParameterType::Selector),
            "number" => return success!(ParameterType::Number),
            "location" => return success!(ParameterType::Location),
            "map" => return success!(ParameterType::Map),
            "list" => return success!(ParameterType::List),
            "path" => return success!(ParameterType::Path),
            "identifier" => return success!(ParameterType::Identifier),
            "keyword" => return success!(ParameterType::Keyword),
            "string" => return success!(ParameterType::String),
            "character" => return success!(ParameterType::Character),
            "integer" => return success!(ParameterType::Integer),
            "float" => return success!(ParameterType::Float),
            "boolean" => return success!(ParameterType::Boolean),
            _invalid => return error!(InvalidType, identifier!(str, source)),
        }
    }

    pub fn expected_list(type_filter: &Vec<ParameterType>) -> Data {
        let mut list = Vector::new();
        for filter in type_filter {
            match filter {
                ParameterType::Map => list.push(identifier!(str, "map")),
                ParameterType::List => list.push(identifier!(str, "list")),
                ParameterType::Path => list.push(identifier!(str, "path")),
                ParameterType::Identifier => list.push(identifier!(str, "identifier")),
                ParameterType::Keyword => list.push(identifier!(str, "keyword")),
                ParameterType::String => list.push(identifier!(str, "string")),
                ParameterType::Character => list.push(identifier!(str, "character")),
                ParameterType::Integer => list.push(identifier!(str, "integer")),
                ParameterType::Float => list.push(identifier!(str, "float")),
                ParameterType::Boolean => list.push(identifier!(str, "boolean")),
                ParameterType::Container => list.push(identifier!(str, "container")),
                ParameterType::Key => list.push(identifier!(str, "key")),
                ParameterType::Literal => list.push(identifier!(str, "literal")),
                ParameterType::Selector => list.push(identifier!(str, "selector")),
                ParameterType::Number => list.push(identifier!(str, "number")),
                ParameterType::Location => list.push(identifier!(str, "location")),
            }
        }
        return list!(list);
    }

    pub fn validate(parameter: &Data, number: Data, type_filter: &Vec<ParameterType>) -> Status<()> {
        for filter in type_filter {
            match filter {
                ParameterType::Map => if parameter.is_map() { return success!(()); },
                ParameterType::List => if parameter.is_list() { return success!(()); },
                ParameterType::Path => if parameter.is_path() { return success!(()); },
                ParameterType::Identifier => if parameter.is_identifier() { return success!(()); },
                ParameterType::Keyword => if parameter.is_keyword() { return success!(()); },
                ParameterType::String => if parameter.is_string() { return success!(()); },
                ParameterType::Character => if parameter.is_character() { return success!(()); },
                ParameterType::Integer => if parameter.is_integer() { return success!(()); },
                ParameterType::Float => if parameter.is_float() { return success!(()); },
                ParameterType::Boolean => if parameter.is_boolean() { return success!(()); },
                ParameterType::Container => if parameter.is_container() { return success!(()); },
                ParameterType::Key => if parameter.is_key() { return success!(()); },
                ParameterType::Literal => if parameter.is_literal() { return success!(()); },
                ParameterType::Selector => if parameter.is_selector() { return success!(()); },
                ParameterType::Number => if parameter.is_number() { return success!(()); },
                ParameterType::Location => if parameter.is_location() { return success!(()); },
            }
        }
        return error!(ExpectedParameterFound, number, Self::expected_list(type_filter), parameter.clone());
    }
}
