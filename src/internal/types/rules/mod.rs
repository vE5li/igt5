mod action;

use internal::*;

pub use self::action::Action;

#[derive(Debug, Clone)]
pub struct Rules {
    rules:      Vec<(VectorString, Action)>,
}

impl Rules {

    pub fn new() -> Self {
        Self {
            rules:      Vec::new(),
        }
    }

    pub fn has_mapping(&self, string: &str) -> bool {
        return self.rules.iter().find(|(_, action)| action.is_mapped_to(string)).is_some();
    }

    pub fn has_mapping_to(&self, source_signature: &VectorString, string: &str) -> bool {
        return self.rules.iter().find(|(signature, action)| *source_signature == *signature && action.is_mapped_to(string)).is_some();
    }

    fn contains(&self, new: &VectorString) -> bool {
        for (pattern, _rule) in self.rules.iter() {
            if pattern == new {
                return true;
            }
        }
        return false;
    }

    pub fn add(&mut self, pattern: VectorString, action: Action) -> Status<()> {
        if self.contains(&pattern) {
            return error!(DuplicateSignature, Data::String(pattern));
        }
        match self.rules.iter().position(|(current_pattern, _)| current_pattern.len() <= pattern.len()) {
            Some(index) => self.rules.insert(index, (pattern, action)),
            None => self.rules.push((pattern, action)),
        }
        return success!(());
    }

    pub fn check_stack(&self, stack: &mut CharacterStack) -> Option<(VectorString, Action)> {
        for (pattern, action) in self.rules.iter() {
            if stack.check_string(pattern) {
                 return Some((pattern.clone(), action.clone()));
            }
        }
        return None;
    }

    pub fn check_word(&self, string: &VectorString) -> Option<(VectorString, Action)> {
        for (pattern, action) in self.rules.iter() {
            if *pattern == *string {
                return Some((pattern.clone(), action.clone()));
            }
        }
        return None;
    }

    pub fn check_prefix(&self, string: &VectorString) -> Option<(VectorString, Action)> {
        for (pattern, action) in self.rules.iter() {
            if let Some(position) = string.find(pattern) {
                if position == 0 {
                    return Some((pattern.clone(), action.clone()));
                }
            }
        }
        return None;
    }
}
