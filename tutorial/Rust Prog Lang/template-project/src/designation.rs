

// Trait
pub trait ProvideDesignation {
    // To be implimented by 'class' with trait
    fn summarize(&self) -> String;

    // Trait with Default action - (can be customized)
    fn summarize_id(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}



