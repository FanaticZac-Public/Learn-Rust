// An input string is valid if:

// Open brackets must be closed by the same type of brackets.
// Open brackets must be closed in the correct order.
// Every close bracket has a corresponding open bracket of the same type.

// pub fn main() {
//     let s: String = "()".to_string();
//     let s2: String = "()[]{}".to_string();
//     let s3: String = "(]".to_string();
//     let s4: String = "([])".to_string();
//     let s5: String = "([)]".to_string();

//     println!("{0:?}", is_valid(s));
//     println!("{0:?}", is_valid(s2));
//     println!("{0:?}", is_valid(s3));
//     println!("{0:?}", is_valid(s4));
//     println!("{0:?}", is_valid(s5));
// }

// My poor version - bad rust skills.. lol
//////////////////////////////////////
pub fn is_valid(s: String) -> bool {
    // println!("{0:?}", s.chars());
    let chars = s.chars();

    let mut stack: Vec<char> = Vec::new();

    for c in chars {
        if stack.is_empty() {
            stack.push(c);
            continue;
        }
        match c {
            '(' => stack.push(c),
            '{' => stack.push(c),
            '[' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            _ => {}
        }
    }

    return true;
}

// Let's see what duckduckgo things can make this concise
//////////////////////////////////////
/// FanaticZac:
/// Uses or symbols to make to scenarios, another switch, not that much better.
// pub fn is_valid(s: String) -> bool {
//     let mut stack = Vec::new();

//     for c in s.chars() {
//         match c {
//             '(' | '[' | '{' => stack.push(c),
//             ')' | ']' | '}' => {
//                 let expected = match c {
//                     ')' => '(',
//                     ']' => '[',
//                     _ => '{',
//                 };
//                 if stack.pop() != Some(expected) {
//                     return false;
//                 }
//             }
//             _ => return false,
//         }
//     }

//     stack.is_empty()
// }

// For posterity sake - I should do the tests first going forward.
#[cfg(test)]
mod tests {
    use super::is_valid;

    #[test]
    fn valid_parentheses_tests() {
        assert_eq!(is_valid("()".to_string()), true);
        assert_eq!(is_valid("()[]{}".to_string()), true);
        assert_eq!(is_valid("(]".to_string()), false);
        assert_eq!(is_valid("([])".to_string()), true);
        assert_eq!(is_valid("([)]".to_string()), false);
    }
}
