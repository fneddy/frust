use std::{borrow::Cow, fmt::Display};

/// value on the stack
/// - `String`: owned string
/// - `Int`: 64 bit signed integer
/// TODO: document
/// TODO: test
#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub enum Variable {
    String(String),
    Int(i64),
}
impl From<i64> for Variable {
    fn from(value: i64) -> Self {
        Variable::Int(value)
    }
}
impl From<Cow<'_, str>> for Variable {
    fn from(value: Cow<str>) -> Self {
        Variable::String(value.into_owned())
    }
}
impl From<&Variable> for Variable {
    fn from(value: &Variable) -> Self {
        value.clone()
    }
}
impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(v) => write!(f, "{}", v), // TODO evaluate value mode
            Self::String(v) => write!(f, "{}", v),
        }
    }
}
impl std::ops::Add for Variable {
    type Output = Variable;

    fn add(self, rhs: Variable) -> Self::Output {
        return match (self, rhs) {
            (Variable::Int(a), Variable::Int(b)) => Variable::Int(a + b),
            _ => Variable::String("NAN".into()),
        };
    }
}
impl std::ops::Sub for Variable {
    type Output = Variable;

    fn sub(self, rhs: Variable) -> Self::Output {
        return match (self, rhs) {
            (Variable::Int(a), Variable::Int(b)) => Variable::Int(a - b),
            _ => Variable::String("NAN".into()),
        };
    }
}
impl std::ops::Mul for Variable {
    type Output = Variable;

    fn mul(self, rhs: Variable) -> Self::Output {
        return match (self, rhs) {
            (Variable::Int(a), Variable::Int(b)) => Variable::Int(a * b),
            _ => Variable::String("NAN".into()),
        };
    }
}
impl std::ops::Div for Variable {
    type Output = Variable;

    fn div(self, rhs: Variable) -> Self::Output {
        return match (self, rhs) {
            (Variable::Int(a), Variable::Int(b)) => Variable::Int(a / b),
            _ => Variable::String("NAN".into()),
        };
    }
}
impl std::ops::Rem for Variable {
    type Output = Variable;

    fn rem(self, rhs: Variable) -> Self::Output {
        return match (self, rhs) {
            (Variable::Int(a), Variable::Int(b)) => Variable::Int(a % b),
            _ => Variable::String("NAN".into()),
        };
    }
}
