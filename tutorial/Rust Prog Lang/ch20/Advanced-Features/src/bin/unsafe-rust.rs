// You can take five actions in unsafe Rust that you can’t in safe Rust, which we call unsafe superpowers:
// 1. Dereference a raw pointer.
// 2. Call an unsafe function or method.
// 3. Access or modify a mutable static variable.
// 4. Implement an unsafe trait.
// 5. Access fields of unions.

// Example 1 - Dereferencing raw pointers within an unsafe block
// fn main() {
//     let mut num = 5;

//     let r1 = &raw const num;
//     let r2 = &raw mut num;

//     unsafe {
//         println!("r1 is: {}", *r1);
//         println!("r2 is: {}", *r2);
//     }
// }

// Example 2 - Calling an Unsafe Function or Method 
fn main() {
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
    // If we try to call dangerous without the unsafe block, we’ll get an error:
    // error[E0133]: call to unsafe function `dangerous` is unsafe and requires unsafe block
}

// Example 3 - Using unsafe code to split an 



