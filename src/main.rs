pub mod patterns;
pub mod iterators;
pub mod disambig;
pub mod sized;
pub mod fp;
pub mod rcbox;
pub mod notsafe;
fn main() {
    patterns::all_sorts_of_patterns();
    iterators::iterators();
    disambig::run();
    sized::sized();
    fp::curried();
    rcbox::is_rcboxed();
    unsafe {
        notsafe::notsafe();
    }
    notsafe::still_not_safe();
}
