// Example modularizing 
use crate::designation::ProvideDesignation;

// User Object
pub struct User {
    name: String,
}

// User Methods
impl User {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn greet(&self) {
        println!("Hello, {}!", self.name);
    }
}

// User Traits
impl ProvideDesignation for User {
    fn summarize(&self) -> String {
        format!("User: {}", self.name)
    }
}

