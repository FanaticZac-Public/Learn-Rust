// Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.

// Symbol       Value
// I             1
// V             5
// X             10
// L             50
// C             100
// D             500
// M             1000

// pub fn main() {
//     println!("{0:?}", roman_to_int("III".to_string()));
//     println!("{0:?}", roman_to_int("LVIII".to_string()));
//     println!("{0:?}", roman_to_int("MCMXCIV".to_string()));
// }

use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
    let romans = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);

    let mut tally = 0;
    let mut chars = s.chars().peekable();

    while let Some(current) = chars.next() {
        let currentValue = romans.get(&current).unwrap();

        if let Some(next) = chars.peek() {
            if currentValue < romans.get(&next).unwrap() {
                tally -= currentValue;
            } else {
                tally += currentValue;
            }
        } else {
             tally += currentValue;
        }
    }
    tally
}

#[cfg(test)]
mod tests {
    use super::roman_to_int;

    #[test]
    fn roman_to_int_tests() {
        assert_eq!(roman_to_int("III".to_string()), 3);
        assert_eq!(roman_to_int("LVIII".to_string()), 58);
        assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
