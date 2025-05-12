use crate::{Context, Result, Variable};
use std::{collections::VecDeque, rc::Rc};

// This is the worst part till now
// really need to go over it and rewrite
// TODO: rewrite
// TODO: document
// TODO: test

#[derive(Debug, PartialEq, Clone)]
pub enum Data {
    Variable(Variable),
    Constant(Variable),
    Array(Vec<Data>),
}
type WordFunction = fn(&mut Context, &mut VecDeque<&str>) -> Result<()>;
#[derive(Debug, PartialEq, Clone)]
pub enum Code {
    Native(WordFunction),
    Dynamic(Vec<Code>),
    Variable,
    Constant,
}

#[derive(Debug, PartialEq)]
pub struct DictionaryValue {
    pub name: String,
    pub code: Code,
    pub data: Option<Data>,
}

pub type DictionaryLink = Option<Rc<Box<DictionaryEntry>>>;

#[derive(Debug, PartialEq)]
pub struct DictionaryEntry {
    pub value: DictionaryValue,
    pub next: DictionaryLink,
}

#[derive(Debug, PartialEq)]
pub struct Dictionary {
    pub head: DictionaryLink,
}

impl Dictionary {
    pub fn new() -> Dictionary {
        Dictionary { head: None }
    }

    pub fn add_data(&mut self, name: &str, value: Data) -> DictionaryLink {
        let alloc_val = Rc::new(Box::new(DictionaryEntry {
            value: DictionaryValue {
                name: name.to_owned(),
                code: Code::Variable,
                data: Some(value),
            },
            next: self.head.take(),
        }));
        self.head = Some(alloc_val);
        self.head.clone()
    }

    pub fn add_code(&mut self, name: &str, code: Code) -> DictionaryLink {
        let alloc_code = Rc::new(Box::new(DictionaryEntry {
            value: DictionaryValue {
                name: name.to_owned(),
                code,
                data: None,
            },
            next: self.head.take(),
        }));
        self.head = Some(alloc_code);
        self.head.clone()
    }

    pub fn get(&self, name: &str) -> DictionaryLink {
        let mut i = self.head.clone();
        while let Some(link) = i {
            if link.value.name == name {
                return Some(link);
            }
            i = link.next.clone();
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dictionary() {
        let mut d = Dictionary::new();
        d.add_data("value", Data::Variable(23i64.into()));
        d.add_data("value1", Data::Variable(32i64.into()));
        d.add_data("value33", Data::Variable(42i64.into()));

        assert_eq!(d.get("foo"), None);

        if let Some(value) = d.get("value") {
            assert_eq!(value.value.name, "value");
            assert_eq!(value.value.data, Some(Data::Variable(23i64.into())));
        } else {
            assert!(false);
        }

        if let Some(value) = d.get("value1") {
            assert_eq!(value.value.name, "value1");
            assert_eq!(value.value.data, Some(Data::Variable(32i64.into())));
        } else {
            assert!(false);
        }
    }
}
