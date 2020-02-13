use internal::*;
use super::{ Templates, Decision, Piece, Token };

macro_rules! find {
    ($type:ident, $internal:ident, $self:expr) => ({
        if let Decision::Filter(..) = $self.decision_stream[$self.decision_index] {
            $self.decision_index += 1;
        }
        while let Token::Comment(..) = &$self.token_stream[$self.token_index] {
            $self.token_index += 1;
        }
        if let Token::$type(data, positions) = &$self.token_stream[$self.token_index] {
            $self.token_index += 1;
            (Data::$internal(data.clone()), positions.clone())
        } else {
            panic!()
        }
    });
}

pub struct TemplateBuilder<'t> {
    pub token_stream:   &'t Vec<Token>,
    decision_stream:    &'t Vector<Decision>,
    templates:          &'t Templates,
    decision_index:     usize,
    pub token_index:    usize,
}

impl<'t> TemplateBuilder<'t> {

    pub fn new(token_stream: &'t Vec<Token>, decision_stream: &'t Vector<Decision>, templates: &'t Templates) -> Self {
        let vec: Vec<Decision> = decision_stream.iter().cloned().collect();

        Self {
            token_stream:       token_stream,
            decision_stream:    decision_stream,
            templates:          templates,
            decision_index:     0,
            token_index:        0,
        }
    }

    pub fn build(&mut self, add_passes: bool) -> Status<(Data, Vec<Position>)> {
        let mut map = confirm!(map!().insert(&keyword!(str, "position"), map!()));

        if let Decision::Filter(..) = self.decision_stream[self.decision_index] {
            self.decision_index += 1;
        }

        let template = match self.decision_stream[self.decision_index] {
            Decision::Template(ref template) => self.templates.get(template).unwrap(), // TODO
            _ => panic!("decision expected template"),
        };

        let flavor = match self.decision_stream[self.decision_index + 1] {
            Decision::Flavor(flavor) => flavor,
            _ => panic!("decision expected flavor"),
        };

        self.decision_index += 2;
        if add_passes {
            if let Some(passes) = &template.passes {
                map = confirm!(map.insert(&keyword!(str, "pass"), passes.clone()));
            }
        }

        let mut template_positions = Vec::new();

        for piece in template.flavors[flavor].pieces.iter() {
            let (key, (data, mut positions)) = confirm!(self.build_piece(piece));

            if let Some(key) = key {
                map = confirm!(map.insert(&key, data));
                let serialized_positions = Position::serialize_positions(&positions);
                let entry = confirm!(map.index(&keyword!(str, "position"))).unwrap();
                let new_entry = confirm!(entry.insert(&key, serialized_positions));
                map = confirm!(map.overwrite(&keyword!(str, "position"), new_entry));
            } else if let Piece::Merge(..) = piece {
                map = confirm!(map.merge(&data));
            }

            template_positions.append(&mut positions);
        }

        return success!((map, Position::range(template_positions, true)));
    }

    fn collect_comment(&mut self) -> (Data, Vec<Position>) {
        let mut comment = AsciiString::new();
        let mut comment_positions = Vec::new();
        while let Token::Comment(data, positions) = &self.token_stream[self.token_index] {
            comment_positions.extend_from_slice(&positions[..]); // MAKE THIS BETTER AND FASTER
            self.token_index += 1;
            comment.push_str(data);
        }
        return (Data::String(comment), Position::range(comment_positions, true));
    }

    fn build_list(&mut self, part: &Piece, seperator: &Option<Piece>) -> Status<(Data, Vec<Position>)> {
        let mut items = Vector::new();
        let mut list_positions = Vec::new();

        if let Decision::List = &self.decision_stream[self.decision_index] {
            self.decision_index += 1;
        } else {
            panic!("expected list decision");
        }

        loop {
            let (_, (part_data, part_positions)) = confirm!(self.build_piece(part));
            let mut data_map = DataMap::new();
            let mut positions_map = DataMap::new();

            let serialized_part_position = Position::serialize_positions(&part_positions);
            list_positions.extend_from_slice(&part_positions[..]); // MAKE THIS BETTER AND FASTER
            data_map.insert(identifier!(str, "part"), part_data);
            positions_map.insert(identifier!(str, "part"), serialized_part_position);

            if let Decision::End = &self.decision_stream[self.decision_index] {
                self.decision_index += 1;
                data_map.insert(identifier!(str, "position"), map!(positions_map));
                items.push(map!(data_map));
                break;
            }

            if let Decision::Next = &self.decision_stream[self.decision_index] {
                self.decision_index += 1;
                if let Some(ref seperator) = *seperator {
                    let (_, (seperator_data, seperator_positions)) = confirm!(self.build_piece(seperator));
                    let serialized_seperator_positions = Position::serialize_positions(&seperator_positions);
                    list_positions.extend_from_slice(&seperator_positions[..]); // MAKE THIS BETTER AND FASTER
                    data_map.insert(identifier!(str, "seperator"), seperator_data);
                    positions_map.insert(identifier!(str, "seperator"), serialized_seperator_positions);
                }
                data_map.insert(identifier!(str, "position"), map!(positions_map));
                items.push(map!(data_map));
            }
        }

        return success!((list!(items), Position::range(list_positions, true)));
    }

    fn build_piece(&mut self, piece: &Piece) -> Status<(Option<Data>, (Data, Vec<Position>))> {
        match piece {
            Piece::Merge(_) => return success!((None, confirm!(self.build(false)))),
            Piece::Template(key, _) => return success!((key.clone(), confirm!(self.build(true)))),
            Piece::Comment(key) => return success!((Some(key.clone()), self.collect_comment())),
            Piece::Data(key, data) => return success!((Some(key.clone()), (data.clone(), Vec::new()))),
            Piece::List(key, part, seperator) => return success!((key.clone(), confirm!(self.build_list(part, seperator)))),
            Piece::Confirmed(key, part, seperator) => return success!((key.clone(), confirm!(self.build_list(part, seperator)))),
            Piece::Keyword(key, _) => return success!((key.clone(), find!(Keyword, Identifier, self))),
            Piece::Operator(key, _) => return success!((key.clone(), find!(Operator, Identifier, self))),
            Piece::Identifier(key, _) => return success!((key.clone(), find!(Identifier, Identifier, self))),
            Piece::TypeIdentifier(key, _) => return success!((key.clone(), find!(TypeIdentifier, Identifier, self))),
            Piece::String(key, _) => return success!((key.clone(), find!(String, String, self))),
            Piece::Character(key, _) => return success!((key.clone(), find!(Character, Character, self))),
            Piece::Integer(key, _) => return success!((key.clone(), find!(Integer, Integer, self))),
            Piece::Float(key, _) => return success!((key.clone(), find!(Float, Float, self))),
        }
    }
}
