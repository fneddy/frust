use crate::{Error, Result, Variable};

// This is just a simple stack
// should i just have used Vec or VecDeque?
// TODO: investigate Vec or VecDeque
// TODO: document and doctest

#[derive(Debug)]
pub struct Stack {
    val: Vec<Variable>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack { val: vec![] }
    }
    pub fn push<T>(&mut self, value: T) where T: Into<Variable> {
        self.val.push(value.into());
    }

    pub fn pop(&mut self) -> Result<Variable> {
        if self.val.len() > 0 {
            return Ok(self.val.pop().unwrap());
        }
        Err(Error::Stack)
    }

    pub fn at(&self, pos: usize) -> Result<&Variable> {
        if self.val.len() > 0 {
            return Ok(self.val.get(pos).unwrap());
        }
        Err(Error::Stack)
    }

    pub fn at_mut(&mut self, pos: usize) -> Result<&mut Variable> {
        if self.val.len() > 0 {
            return Ok(self.val.get_mut(pos).unwrap());
        }
        Err(Error::Stack)
    }
    pub fn len(&self) -> usize {
        self.val.len()
    }

    pub fn iter(&self) -> StackIterator {
        StackIterator {
            stack: self,
            index: 0,
        }
    }
}

pub struct StackIterator<'a> {
    stack: &'a Stack,
    index: usize,
}

impl<'a> Iterator for StackIterator<'a> {
    type Item = &'a Variable;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.stack.len() {
            let value = Some(&self.stack.val[self.index]);
            self.index += 1;
            return value;
        }
        None
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_stack() {
        let mut stack = Stack::new();
        let val: Variable = 1.into();

        stack.push(val);
        {
            let r = stack.at(0).expect("ref not working");
            assert_eq!(*r, 1i64.into());
        }

        let ret = stack.pop().expect("pop not working");
        assert_eq!(ret, 1i64.into());

        let ret = stack.pop();
        assert_eq!(ret, Err(Error::Stack));
    }
}
