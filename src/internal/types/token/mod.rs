use internal::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Comment(VectorString, Vec<Position>),
    Keyword(VectorString, Vec<Position>),
    Operator(VectorString, Vec<Position>),
    Identifier(VectorString, Vec<Position>),
    TypeIdentifier(VectorString, Vec<Position>),
    String(VectorString, Vec<Position>),
    Character(Character, Vec<Position>),
    Integer(i64, Vec<Position>),
    Float(f64, Vec<Position>),
}

impl Token {

    pub fn parsable(&self) -> bool {
        match self {
            Token::Comment(..) => false,
            _ => true,
        }
    }

    pub fn to_location(&self) -> Data {
        match self {
            Token::Comment(..) => panic!(),
            Token::Operator(operator, _position) => return Data::Identifier(format_vector!("operator:{}", operator)),
            Token::Keyword(keyword, _position) => return Data::Identifier(format_vector!("keyword:{}", keyword)),
            Token::Identifier(..) => return identifier!(str, "identifier"),
            Token::TypeIdentifier(..) => return identifier!(str, "type_identifier"),
            Token::Character(..) => return identifier!(str, "character"),
            Token::String(..) => return identifier!(str, "string"),
            Token::Integer(..) => return identifier!(str, "integer"),
            Token::Float(..) => return identifier!(str, "float"),
        }
    }
}
