fn operate(operand: i32, f: Box<dyn Fn(i32) -> Box<dyn Fn(i32) -> i32>>) -> Box<dyn Fn(i32) -> i32> {
    Box::new((*f)(operand))
}

fn add(operand: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |rhs| {operand + rhs})
}

pub fn curried() {
    println!("curried: {}", operate(1, Box::new(add))(1));
}