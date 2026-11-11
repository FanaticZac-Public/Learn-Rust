pub fn main() {
    println!("{0}", is_palindrome(101))
}

// cargo test --bin palindrome-number -- --nocapture 

// could improve memory - but it's so elegant... 
pub fn is_palindrome(x: i32) -> bool {
    let s = x.to_string();
    s.chars().eq(s.chars().rev())
}

#[cfg(test)]
mod tests {
    use super::is_palindrome;

    #[test]
    fn palindrome_cases() {
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
