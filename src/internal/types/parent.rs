
#[derive(Clone, Debug)]
pub enum Parent {
    Vector(*const Parent, *const usize, *const usize),
    Map(*const Parent, *const Node),
    None,
}

impl Parent {

    pub fn single_reference(&self) -> bool {
        match self {
            Parent::Vector(parent, list_count, block_count) => {
                unsafe {
                    match **list_count > 1 || **block_count > 1 {
                        true => return false,
                        false => return parent.as_ref().unwrap().single_reference(),
                    }
                }
            },
            Parent::None => return true,
        }
    }
}
