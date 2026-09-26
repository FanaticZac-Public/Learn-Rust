use crate::designation::ProvideDesignation;

pub struct Agent {
    name: String,
}

impl Agent {
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

impl ProvideDesignation for Agent {
    fn summarize(&self) -> String {
        format!("Agent: {}", self.name)
    }
}
