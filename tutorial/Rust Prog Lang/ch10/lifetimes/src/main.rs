// Example 1 - dangling reference  See Error
// -----------
// fn main() {
//     let r;

//     {
//         let x = 5;
//         r = &x;
//     }

//     println!("r: {r}");

//     //error[E0597]: `x` does not live long enough
// }

// Example 2 - Function dangling reference - See Error
// -----------
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() { x } else { y }
// }

// fn main() {
//     let string1 = String::from("abcd");
//     let string2 = "xyz";

//     let result = longest(string1.as_str(), string2);
//     println!("The longest string is {result}");

//     // Result:
//     // error[E0106]: missing lifetime specifier

//     // The Rust compiler has a borrow checker that compares scopes to determine whether all borrows are valid.
//     // Borrow checker needs to know where the lifetimes of references are and ensure they will remain valid
//     // This error occurs because Rust doesn't know which will be returned, x or y. Therefore compiler can't compare the lifetimes or validity.
// }

// Example 3 - In Function Signatures to handle dangling reference success
// -----------
// <`a> and `a syntax specifying that all the references in the signature must have the same lifetime 'a
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() { x } else { y }
// }

// fn main() {
//     let string1 = String::from("long string is long");

//     {
//         let string2 = String::from("xyz");
//         let result = longest(string1.as_str(), string2.as_str());
//         println!("The longest string is {result}");
//     }

//     // Result:
//     // The longest string is long string is long

//     // In this case it works because Rust is considering the lifetime equal to the shortest of both lifetimes together - where the string1 lifetime is to the end of it's scope - at least 8 (these comments move the curly brace down but it's to that), and string2 is 3 to the end of that scope - so now they share a lifetime of 3 (as far as that function is concerned) - and because result and usage of result in print is in that same block - then it's successful.
//     // - Might seem unimportant but pushing this logic into a mechanism that the compiler can check helps prevent danging reference on much bigger code - because it causes bug.
// }

// Example 4 - In Function Signatures to handle dangling reference failure
// -----------
// <`a> and `a syntax specifying that all the references in the signature must have the same lifetime 'a
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
// fn main() {
//     let string1 = String::from("long string is long");
//     let result;
//     {
//         let string2 = String::from("xyz");
//         result = longest(string1.as_str(), string2.as_str());
//     }
//     println!("The longest string is {result}");

//     // Result:
//     // error[E0597]: `string2` does not live long enough

//     // In this case the lifetimes are shared by the parameter to e shorest, same as last time, however the usage is out of scope, beyond that shortest lifeline and so the error is triggered.
//     // If we put print in the curley braces it would work.
// }

// Example 5 - In Function Signatures adjusted for needs
// -----------

// Adjusted signature here - parameters don't have lifetime syntax, just marker and return value.
// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()

// }

// fn main() {
//     let string1 = String::from("long string is long");
//     let result;
//     {
//         let string2 = String::from("xyz");
//         result = longest(string1.as_str(), string2.as_str());
//     }
//     println!("The longest string is {result}");

//     // Result
//     //error[E0515]: cannot return value referencing local variable `result`

//     // The problem is that result goes out of scope and gets cleaned up at the end of the longest function.
//     // In this case it doesn't matter the lifetime syntax because we don't want to borrow from an inner scope function anyway and should just pass owned value.
// }

// Example 6 - In Struct Definitions for lifetime
// -----------
// struct ImportantExcerpt<'a> { // mark lifetime
//     part: &'a str,  // must mark each child
// }

// fn main() {
//     let novel = String::from("Call me Ishmael. Some years ago...");
//     let first_sentence = novel.split('.').next().unwrap();
//     let i = ImportantExcerpt {
//         part: first_sentence,
//     };
//     println!("{0}", i.part);
//     // Call me Ishmael

//     // This annotation means an instance of ImportantExcerpt can’t outlive the reference it holds in its part field.
//     // novel doesn't go out of scope before i does - so it's fine.
//     // Point is we're using a reference and the borrow checker knows what to look for.
//     // error[E0106]: missing lifetime specifier occurs if i remove the lifetime specifier.
// }

// Example 7 - Lifetime Elision
// -----------
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");

    let first = first_word(&novel.as_str());
    println!("{0}", first);

    // This one worked but why? doesn't first_word need lifetime syntax?
    // The reason this function compiles without lifetime annotations is historical: In early versions (pre-1.0) of Rust, this code wouldn’t have compiled, because every reference needed an explicit lifetime. 
    // fn first_word<'a>(s: &'a str) -> &'a str {

    // However - there are some patterns where this because irrelevant and tedious for programmers - the function is determinist anyway. 
    // lifetime elision rules are these exceptions to the rule.
    // so in some cases the lifetimes are inferred rather than explicit.
    // The elision rules don’t provide full inference. If there is still ambiguity about what lifetimes the references have after Rust applies the rules, the compiler will give you error.

    // See more on this in lifetimes section - there's no code to run - it's purely understanding the compiler stuff. But ill put the 3 rules - compiler will follow all 3 and if it's still unsure about the lifeline it will error. 

    // 1. The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference. 
    // 2. The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
    // 3. The third rule is that, if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters. 
        // -  This third rule makes methods much nicer to read and write because fewer symbols are necessary.
}

