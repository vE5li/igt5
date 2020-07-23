use kami::*;

pub struct Checked<T> {
    name: &'static str,
    inner: Option<T>,
    set: bool,
}

impl<T> Checked<T> {

    pub fn some(name: &'static str, inner: T) -> Self {
        Self {
            name: name,
            inner: Some(inner),
            set: false,
        }
    }

    pub fn none(name: &'static str) -> Self {
        Self {
            name: name,
            inner: None,
            set: false,
        }
    }

    pub fn into_inner(self) -> Option<T> {
        self.inner
    }

    pub fn changed(self) -> Option<T> {
        match self.set {
            true => self.inner,
            false => None,
        }
    }

    pub fn update(&mut self, parameters: &mut Vec<T>) -> Status<()> {
        ensure!(!self.set, Message, string!("{} may not be set twice", self.name));
        ensure!(!parameters.is_empty(), Message, string!("parameters expected {}", self.name));
        self.inner = Some(parameters.remove(0));
        self.set = true;
        return success!(());
    }
}
