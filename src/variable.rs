use std::fmt::Display;

/// value on the stack
/// - `String`: owned string
/// - `Int`: 64 bit signed integer
/// TODO: document
/// TODO: test
#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub enum Variable {
    Array(Vec<Variable>),
    String(String),
    Int(i64),
}
impl From<i64> for Variable {
    fn from(value: i64) -> Self {
        Variable::Int(value)
    }
}
impl From<&str> for Variable {
    fn from(value: &str) -> Self {
        Variable::String(value.to_owned())
    }
}
impl From<&Variable> for Variable {
    fn from(value: &Variable) -> Self {
        value.clone()
    }
}
impl From<Vec<Variable>> for Variable {
    fn from(value: Vec<Variable>) -> Self {
        Variable::Array(value)
    }
}
impl From<Vec<String>> for Variable {
    fn from(values: Vec<String>) -> Self {
        let collection: Vec<Variable> = values
            .iter()
            .map(|value| Variable::from(value.as_str()))
            .collect();
        Variable::Array(collection)
    }
}
impl From<Vec<i64>> for Variable {
    fn from(values: Vec<i64>) -> Self {
        let collection: Vec<Variable> = values.iter().map(|value| Variable::from(*value)).collect();
        Variable::Array(collection)
    }
}
impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(v) => write!(f, "{}", v), // TODO evaluate value mode
            Self::String(v) => write!(f, "{}", v),
            Self::Array(values) => values.iter().map(|v| write!(f, "{}", v)).collect(),
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
