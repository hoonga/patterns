static mut GLOBAL_COUNTER: u32 = 0;

union Color8 {
    val: u32,
    rgba: [u8;4]
}

pub unsafe fn notsafe() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let color = Color8 {
        val: 0xFF00FF00
    };

    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
    *r2 = 123;
    println!("r1 is: {}", *r1);
    GLOBAL_COUNTER += 1;
    println!("{:?}", color.rgba);
}

pub fn still_not_safe() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        *r2 = 123;
        println!("r1 is: {}", *r1);
        GLOBAL_COUNTER += 1;
        println!("GLOBAL VALUE: {}", GLOBAL_COUNTER);
    }
}