// Example 1 - Setting up the signature for import before testing
// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     unimplemented!();
// }

// Example 2 - Adjusting the function after setting up test
// - Then returning empty vector instead of unimplimented!() to stop panic! (TDD principle) to test basic failure condition.
// Check test - it fails but doesn't panic - as expected.
// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     vec![]
// }

// Example 3 & 4- Implementation of the search function
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    // returns iterator
    for line in contents.lines() {
        // contains() is method of iterator
        if line.contains(query) {
            results.push(line);
        }
    }

    results
    // Test passes now
}

// Example 4 - Testing for case sensitivity or insensitivity

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}




// Our TDD example function for the mini-grep program

#[cfg(test)]
mod tests {
    use super::*;

    // Example 3 Test to match Example 3
//     #[test]
//     fn one_result() {
//         let query = "duct";
//         // backslash tells rust not to place '\n'
//         let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.";

//         // We assert returned like is the one that has 'duct' inside
//         assert_eq!(vec!["safe, fast, productive."], search(query, contents));
//     }



    // Example 4 section - adjusted different name and added duct tape line that should trip test
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
