// So the library should be considered the public interface file
// Traits should have their own files
// "Objects/Entities" should have their own files


// Ok - I understand - so lib.rs is primary for organizing my public sharing of other modules and methods
// These can be used by a binary crate in the same project or not.
// its sort of like header files in c++ but easier and less redundant in terms of signature.

// These refer to the file names
mod agent;
mod designation;
mod user;

// We then call the mods :: their 'class' aka structure and their impl methods and traits
pub use agent::Agent;
pub use designation::ProvideDesignation;
pub use user::User;



