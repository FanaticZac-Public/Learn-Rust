pub fn main() {
    println!(
        "{0:?}",
        longest_common_prefix(vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string()
        ])
    );
    println!(
        "{0:?}",
        longest_common_prefix(vec![
            "dog".to_string(),
            "racecar".to_string(),
            "car".to_string()
        ])
    );
}

// My poor version - bad rust skills.. lol
//////////////////////////////////////
// pub fn longest_common_prefix(strs: Vec<String>) -> String {
//     if strs.is_empty() {
//         return String::new();
//     };
//     let mut lcp: Vec<char> = strs.first().unwrap().chars().collect();

//     for s in strs {
//         let chars = s.chars().collect();
//         if lcp.is_empty() {
//             lcp = chars;
//             continue;
//         }

//         for (i, c) in chars.iter().enumerate() {
//             if lcp.get(i) != Some(c) {
//                 if (i == 0) {return String::new()}
//                 lcp.truncate(i);
//                 break;
//             }
//         }

//         if chars.len() < lcp.len() {
//             lcp.truncate(chars.len());
//         }
//     }

//     lcp.into_iter().collect()
// }

//////////////////////////////////////
// // Duck.ai version
// pub fn longest_common_prefix(strs: Vec<String>) -> String {
//     // learned simplier iter initialization
//     let Some(first) = strs.first() else {
//         return String::new();
//     };

//     let mut lcp: Vec<char> = first.chars().collect();

//     for s in strs.iter().skip(1) {
//         let chars: Vec<char> = s.chars().collect();

//         // learned .zip() will tie 2 vectors together for easy compare of indexes
//         let common_len = lcp
//             .iter()
//             .zip(chars.iter())
//             .take_while(|(a, b)| a == b)
//             .count();

//         lcp.truncate(common_len);

//         if lcp.is_empty() {
//             break;
//         }
//     }

//     lcp.into_iter().collect()
// }

// Even simpler just using rust String functions
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut prefix = strs.first().cloned().unwrap_or_default();

    for s in &strs {
        while !s.starts_with(&prefix) {
            prefix.pop();
        }
    }

    prefix
}

// It was at this point where I made the /notebooks/api-super-summaries listings... need to memorize these functions or my code will suck in Rust.

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
