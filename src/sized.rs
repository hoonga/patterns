fn only_sized<T: Sized>(_: T) {
    println!("size: {}", size_of::<T>())
}

fn not_only_sized<T: ?Sized>(t: &T) {
    println!("size: {}", size_of_val(t))
}

pub fn sized() {
    only_sized(vec![1,2,3]);
    only_sized(vec![1]);
    only_sized([1,2,3]);
    only_sized([1]);
    only_sized("asdf");
    // only_sized(*"asdfasdf");
    not_only_sized(&[1,2,3]);
    not_only_sized("asdf");
    not_only_sized(&vec![1,2,3]);
}