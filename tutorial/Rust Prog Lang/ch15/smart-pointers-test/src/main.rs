// USAGE OF - RefCell<T> and the Interior Mutability Pattern

pub trait Messenger {
    // Takes immutable reference to self, and msg
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    // We also want to test the behavior of the set_value method on the LimitTracker
    // We can change what we pass in for the value parameter, but set_value doesn’t return anything for us to make assertions on.
    // We want to be able to say that if we create a LimitTracker with something that implements the Messenger trait and a particular value for max, the messenger is told to send the appropriate messages when we pass different numbers for value.
    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>, // now of type RefCell<Vec<String>>
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]), // around the new vector
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // We call borrow_mut on the RefCell<Vec<String>> in self.sent_messages to get a mutable reference to the value inside the RefCell<Vec<String>>, which is the vector.
            // Then, we can call push on the mutable reference to the vector to keep track of the messages sent during the test.
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        // we call borrow on the RefCell<Vec<String>> to get an immutable reference to the vector.
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
