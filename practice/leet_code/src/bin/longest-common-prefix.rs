// Write a function to find the longest common prefix string amongst an array of strings.

// If there is no common prefix, return an empty string "".

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

#[cfg(test)]
mod tests {
    use super::longest_common_prefix;

    #[test]
    fn finds_common_prefix() {
        assert_eq!(
            longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string(),
            ]),
            "fl"
        );
    }

    #[test]
    fn returns_empty_string_when_there_is_no_common_prefix() {
        assert_eq!(
            longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string(),
            ]),
            ""
        );
    }
}