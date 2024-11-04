use std::any::type_name_of_val;

macro_rules! rcbox {
    ($x:expr) => (std::rc::Rc::new(Box::new($x)));
}

pub fn is_rcboxed() {
    println!("{:?}", type_name_of_val(&rcbox!(String::from("asdf"))));
    println!("{:?}", type_name_of_val(&rcbox!("asdf")));
}