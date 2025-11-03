#[derive(Clone, Debug)]
pub enum Value {
    Num(f64),
    Str(String),
    Bool(bool),
    Nil
}

pub fn print_value(constant: &Value) {
    match constant {
        Value::Num(v) => {print!("{}", v)},
        Value::Str(s) => {print!("{}", s)},
        Value::Bool(b) => {print!("{}", b)},
        Value::Nil => {print!("Nil")}
    }
}

impl From<bool> for Value {
    fn from(val: bool) -> Self {
        Value::Bool(val)
    }
}

impl From<f64> for Value {
    fn from(val: f64) -> Self {
        Value::Num(val)
    }
}

impl From<&str> for Value {
    fn from(val: &str) -> Self {
        Value::Str(val.to_string())
    }
}

impl From<Option<&str>> for Value {
    fn from(val: Option<&str>) -> Self {
        match val{
            None => {Value::Nil}
            Some(v) => {
                if v == "null" { Value::Nil }
                else {panic!()}
            }
        }
    }
}

impl Value {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Num(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<String> {
        match self {
            Value::Str(s) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn is_bool(&self) -> bool {
        match *self {
            Value::Num(_) => {false}
            Value::Str(_) => {false}
            Value::Bool(_) => {true}
            Value::Nil => {false}
        }
    }
    pub fn is_num(&self) -> bool {
        match *self {
            Value::Num(_) => {true}
            Value::Str(_) => {false}
            Value::Bool(_) => {false}
            Value::Nil => {false}
        }
    }
    pub fn is_str(&self) -> bool {
        match *self {
            Value::Num(_) => {false}
            Value::Str(_) => {true}
            Value::Bool(_) => {false}
            Value::Nil => {false}
        }
    }
    pub fn is_nil(&self) -> bool {
        match *self {
            Value::Num(_) => {false}
            Value::Str(_) => {false}
            Value::Bool(_) => {false}
            Value::Nil => {true}
        }
    }
}