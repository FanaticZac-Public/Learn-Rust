// All right - So this project is just a place for me to put together all lessons examples (more or less) in one place together to be mindful of the big picture, properly use mods, lib, main together, interfaces, etc.. .etc..

// Rust changes the hyphen to an underscore for crate imports
use template_project::{Agent, ProvideDesignation, User};

fn main() {
    let user = User::new("Alice");
    let agent = Agent::new("Helper");

    user.greet();
    agent.greet();

    println!("{}", user.summarize());
    println!("{}", agent.summarize());
}