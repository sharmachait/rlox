pub enum Types {
    Val(f64)
}

pub fn print_value(constant: &Types) {
    match constant {
        Types::Val(v) =>  print!("{}", v)
    }
}