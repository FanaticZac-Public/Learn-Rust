// // Example 1 - Attempting Traditional Object-Oriented Style
// pub struct Post {
//     state: Option<Box<dyn State>>,
//     content: String,
// }

// impl Post {
//     pub fn new() -> Post {
//         Post {
//             state: Some(Box::new(Draft {})),
//             content: String::new(),
//         }
//     }
//     pub fn add_text(&mut self, text: &str) {
//         self.content.push_str(text);
//     }
//     pub fn content(&self) -> &str {
//         self.state.as_ref().unwrap().content(self)
//     }
//     pub fn request_review(&mut self) {
//         if let Some(s) = self.state.take() {
//             self.state = Some(s.request_review())
//         }
//     }
//     pub fn approve(&mut self) {
//         if let Some(s) = self.state.take() {
//             self.state = Some(s.approve())
//         }
//     }
// }

// trait State {
//     fn request_review(self: Box<Self>) -> Box<dyn State>;
//     fn approve(self: Box<Self>) -> Box<dyn State>;
//     fn content<'a>(&self, post: &'a Post) -> &'a str {
//         ""
//     }
// }

// struct Draft {}

// impl State for Draft {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         Box::new(PendingReview {})
//     }
//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
//     // fn content - has default trait behavior
// }

// struct PendingReview {}

// impl State for PendingReview {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         Box::new(Published {})
//     }
//     // fn content - has default trait behavior
// }

// struct Published {}

// impl State for Published {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         self
//     }

//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
//     fn content<'a>(&self, post: &'a Post) -> &'a str {
//         &post.content
//     }
// }

// Example 2 - Encoding States and Behavior as Types - Rust strengths

// Rather than encapsulating the states and transitions completely so that outside code has no knowledge of them, we’ll encode the states into different types.
// Consequently, Rust’s type-checking system will prevent attempts to use draft posts where only published posts are allowed by issuing a compiler error.

// main.rs
fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}

// lib.rs
pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
