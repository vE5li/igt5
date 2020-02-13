use internal::*;

pub fn serialize_float(value: f64) -> AsciiString {
    let mut string = value.to_string();
    if !string.contains(".") {
        string.push_str(".0");
    }
    return AsciiString::from(&string);
}

pub fn serialize_literal(source: &AsciiString, delimiter: char) -> AsciiString {
    return format_ascii!("{}{}{}", delimiter, source.serialize(), delimiter);
}

pub fn serialize_map(source: &DataMap) -> AsciiString {
    let mut string = AsciiString::from("{");

    for (key, value) in source.iter() {
        string.push(Character::from_char(' '));
        string.push_str(&key.serialize());
        string.push(Character::from_char(' '));
        string.push_str(&value.serialize());
    }
    string.push(Character::from_char(' '));
    string.push(Character::from_char('}'));

    return string;
}

pub fn serialize_list(source: &Vector<Data>) -> AsciiString {
    let mut string = AsciiString::from("[");

    for item in source.iter() {
        string.push(Character::from_char(' '));
        string.push_str(&item.serialize());
    }
    string.push(Character::from_char(' '));
    string.push(Character::from_char(']'));

    return string;
}

pub fn serialize_path(source: &Vector<Data>) -> AsciiString {
    let mut string = AsciiString::new();

    for (index, step) in source.iter().enumerate() {
        string.push_str(&step.serialize());
        if index != source.len() - 1 {
            string.push(Character::from_char(':'));
        }
    }

    return string;
}
