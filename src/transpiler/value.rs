#[derive(Clone, Debug)]
pub enum Value {
    Num(f64),
    Obj(Obj),
    Bool(bool),
    Nil,

}
#[derive(Clone, Debug)]
pub enum Obj {
    Str(String),

}

pub fn print_value(constant: &Value) {
    match constant {
        Value::Num(v) => {print!("{}", v)},
        Value::Obj(obj) => {
            match obj {
                Obj::Str(str) => {
                    print!("{}", str)
                }
            }
        },
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
        Value::Obj(Obj::Str(val.to_string()))
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

    pub fn equal_by_type(a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::Num(x), Value::Num(y)) => true,
            (Value::Obj(x), Value::Obj(y)) => {
                match (x,y) {
                    (Obj::Str(a),Obj::Str(b))=> true
                }
            },
            (Value::Bool(x), Value::Bool(y)) => true,
            (Value::Nil, Value::Nil) => true,
            _ => false,
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Num(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_obj(&self) -> Option<Obj> {
        match self {
            Value::Obj(n) => Some(n.clone()),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<String> {
        match self {
            Value::Obj(s) => {
                match s {
                    Obj::Str(ss) => Some(ss.clone())
                }
            },
            _ => None,
        }
    }

    pub fn is_bool(&self) -> bool {
        match *self {
            Value::Num(_) => {false}
            Value::Obj(_) => {false}
            Value::Bool(_) => {true}
            Value::Nil => {false}
        }
    }
    pub fn is_num(&self) -> bool {
        match *self {
            Value::Num(_) => {true}
            Value::Obj(_) => {false}
            Value::Bool(_) => {false}
            Value::Nil => {false}
        }
    }
    pub fn is_str(&self) -> bool {
        match self {
            Value::Obj(Obj::Str(_)) => true,
            _ => false,
        }
    }
    pub fn is_nil(&self) -> bool {
        match *self {
            Value::Num(_) => {false}
            Value::Obj(_) => {false}
            Value::Bool(_) => {false}
            Value::Nil => {true}
        }
    }
    pub fn is_obj(&self) -> bool {
        match *self {
            Value::Num(_) => {false}
            Value::Obj(_) => {true}
            Value::Bool(_) => {false}
            Value::Nil => {false}
        }
    }

    pub fn is_obj_type(&self, obj_type: Obj) -> bool {
        match self {
            Value::Obj(obj_type) => true,
            _ => false,
        }
    }
}