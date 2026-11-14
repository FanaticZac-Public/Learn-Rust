// Given an integer x, return true if x is a palindrome, and false otherwise.

// could improve memory - but it's so elegant... 
pub fn is_palindrome(x: i32) -> bool {
    let s = x.to_string();
    s.chars().eq(s.chars().rev())
}

#[cfg(test)]
mod tests {
    use super::is_palindrome;

    #[test]
    fn palindrome_tests() {
        let examples = [
            (101, true),
            (-121, false),
            (10, false),
        ];

        for (input, expected) in examples {
            let actual = is_palindrome(input);
            println!("{input} → {actual}");
            assert_eq!(actual, expected, "failed for input {input}");
        }
    }
}
