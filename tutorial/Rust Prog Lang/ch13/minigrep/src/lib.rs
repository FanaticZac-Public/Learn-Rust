// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     let mut results = Vec::new();

//     // returns iterator
//     for line in contents.lines() {
//         // contains() is method of iterator
//         if line.contains(query) {
//             results.push(line);
//         }
//     }

//     results
//     // Test passes now
// }


pub fn search<'a>(query: &str, contents: &'a str) -> impl Iterator<Item = &'a str> {
    contents
        .lines()
        .filter(move |line| line.contains(&query))
}

// pub fn search_case_insensitive<'a>(
//     query: &str,
//     contents: &'a str,
// ) -> Vec<&'a str> {
//     let query = query.to_lowercase();
//     let mut results = Vec::new();

//     for line in contents.lines() {
//         if line.to_lowercase().contains(&query) {
//             results.push(line);
//         }
//     }

//     results
// }

// pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     contents
//         .lines()
//         .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
//         .collect()
// }

// now with iterator adapter
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> impl Iterator<Item = &'a str> {
    contents
        .lines()
        .filter(move |line| line.to_lowercase().contains(&query.to_lowercase()))

}


// Our TDD example function for the mini-grep program

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape."; // ADDED

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    // Added test case for section:
    // Working with Environment Variables
    // To check where user wants case sensitivity
    // First step - writing a failed test
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
