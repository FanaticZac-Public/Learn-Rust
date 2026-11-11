pub fn main() {
    println!("{0:?}", roman_to_int("III".to_string()));
    println!("{0:?}", roman_to_int("LVIII".to_string()));
    println!("{0:?}", roman_to_int("MCMXCIV".to_string()));
}

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

// #[cfg(test)]
// mod tests {
//     use super::two_sum;

//     #[test]
//     fn example_cases() {
//         assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
//         assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
//         assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
//     }
// }
