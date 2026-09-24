// A package can contain as many binary crates as you like,
// - but at most only one library crate.

use crate::garden::vegetables::Asparagus;

pub mod garden;

// The compiler will look for the module’s code in these places:
// Inline, within curly brackets that replace the semicolon following mod garden
// In the file src/garden.rs
// In the file src/garden/mod.rs

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}
