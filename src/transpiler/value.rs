#[derive(Clone)]
pub enum Types {
    Val(f64),
    Str(String)
}

pub fn print_value(constant: &Types) {
    match constant {
        Types::Val(v) =>  print!("{}", v),
        Types::Str(s) => {print!("{}", s)}
    }
}