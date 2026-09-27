// Example 1 - Default template from running cargo new adder --lib

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

// Example 2 - Custom Test - 1 pass * 1 fail

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn exploration() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }

//     #[test]
//     fn another() {
//         panic!("Make this test fail");
//     }
// }

// Example 3 - Previous code example - but with test added

// // Real code example
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//         // mess with < and > to see how code mistake later could trip tests
//     }
// }

// // Test for code example
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn larger_can_hold_smaller() {
//         let larger = Rectangle {
//             width: 8,
//             height: 7,
//         };
//         let smaller = Rectangle {
//             width: 5,
//             height: 1,
//         };

//         assert!(larger.can_hold(&smaller));
//     }

//     #[test]
//     fn smaller_cannot_hold_larger() {
//         let larger = Rectangle {
//             width: 8,
//             height: 7,
//         };
//         let smaller = Rectangle {
//             width: 5,
//             height: 1,
//         };

//         assert!(!smaller.can_hold(&larger)); // checking that is ! (aka not) true
//     }
// }

// Example 4 - Testing Equality with assert_eq! and assert_ne!

// pub fn add_two(a: u64) -> u64 {
//     a + 2
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_adds_two() {
//         let result = add_two(2);
//         assert_eq!(result, 4);
//         assert!(result = 4);
//         or use the assert_ne!() // ne (not equal)
//         assert_ne!(3)
//         // assert_eq and assert_ne are more descriptive in output
//     }
// }

// Example 5 -  test that the name we pass into the function appears in the output

// // correct
// pub fn greeting(name: &str) -> String {
//     format!("Hello {name}!")
// }

// introduce bug
// pub fn greeting(name: &str) -> String {
//     String::from("Hello!")
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn greeting_contains_name() {
//         let result = greeting("Carol");
//         // assert!(result.contains("Carol"));

//         // custom error
//         assert!(
//             result.contains("Carol"),
//             "Greeting did not contain name, value was `{result}`"
//         );

//         // OUTPUT:
//         // failures:
//         // ---- tests::greeting_contains_name stdout ----
//     }
// }

// Example 5 -  Testing that something should_panic and also with expected error msg

// pub struct Guess {
//     value: i32,
// }

// // first test
// // impl Guess {
// //     pub fn new(value: i32) -> Guess {
// //         if value < 1 || value > 100 {
// //             panic!("Guess value must be between 1 and 100, got {value}.");
// //         }

// //         Guess { value }
// //     }
// //      // Suceeded in test
// // }

// // but test
// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         // if value < 1 {
//         //     panic!("Guess value must be between 1 and 100, got {value}.");
//         // }

//         // changed to check expected should_panic
//         if value < 1 {
//             panic!("Guess value must be less than or equal to 100, got {value}.");
//         } else if value > 100 {
//             panic!("Guess value must be greater than or equal to 1, got {value}.");
//         }

//         Guess { value }
//     }
//     // Failed in test
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     // #[should_panic] // Should panic
//     #[should_panic(expected = "less than or equal to 100")] // should panic and match specific error
//     fn greater_than_100() {
//         Guess::new(200); // Should panic
//     }

//     // should_panic (with expected) output:
//     // note: panic did not contain expected string
//     // panic message: "Guess value must be greater than or equal to 1, got 200."
//     // expected substring: "less than or equal to 100"
// }

// Example 6 - Using Result<T, E> in Tests

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() -> Result<(), String> {
//         let result = add(2, 2);

//         if result == 4 {
//             Ok(())
//         } else {
//             Err(String::from("two plus two does not equal four"))
//         }
//     }
// }


// Example - Mine - Unified Result for cheat sheet

// Example Code 1 - Example of function you want to test 
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// Example Test 1
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);

        // variations of handling result - choose 1 or the 4
        // Note - second param from custom error (result = 4, "custom msg")

        // Regular assert - Less descriptive message on failure
        assert!(result = 4); // or e.g. result.contains("Carol"), any bool
        
        // Assert equal
        assert_eq!(result, 4);

        // Assert not equal
        Assert_ne!(3)

        // Using Result<T, E> 
        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    // ---------------------------------------------------------

    // Example Code 1 - Example of function you want to test 

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 {
                panic!(
                    "Guess value must be greater than or equal to 1, got {value}."
                );
            } else if value > 100 {
                panic!(
                    "Guess value must be less than or equal to 100, got {value}."
                );
            }

            Guess { value }
        }
    }

    // Example Test 2 - should_panic and expect syntax
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        // Choose 1 of 2
        #[should_panic]         // Test passes if panics - but could panic for multi-reasons
        #[should_panic(expected = "less than or equal to 100")]         // Pass with expected error text
        fn greater_than_100() {
            Guess::new(200);
        }
    }
}

