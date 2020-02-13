#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Relation {
    Smaller,
    Bigger,
    Equal,
}

impl Relation {

    pub fn from_boolean(value: bool) -> Self {
        match value {
            true => return Relation::Smaller,
            false => return Relation::Bigger,
        }
    }
}

pub trait Compare {

    fn compare(&self, other: &Self) -> Relation;
}
