# Running Code on Cleanup with the Drop Trait

## Super Summary
```rust
//-------------- Running Code on Cleanup with the Drop Trait --------------
// In Rust, you can specify that a particular bit of code be run whenever a value goes out of scope, and the compiler will insert this code automatically, so you don’t need to be careful about placing cleanup code everywhere - and won’t leak resources!
// - Examples include file handles, sockets, and locks.
struct CustomSmartPointer {
    data: String,
}

// The Drop trait is included in the prelude, so we don’t need to bring it into scope. 
impl Drop for CustomSmartPointer {
    fn drop(&mut self) { // implementing drop trait
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created");
} // drop runs here() - at end of scope
// CustomSmartPointers created
// Dropping CustomSmartPointer with data `other stuff`!
// Dropping CustomSmartPointer with data `my stuff`!
// - Variables are dropped in the reverse order of their creation, so d was dropped before c. 

//-------------- Manually calling drop method --------------
// You might want to clean up a value early like when 
// - e.g. smart pointers that manage locks
fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created");
    c.drop();
    println!("CustomSmartPointer dropped before the end of main");
}
// error[E0040]: explicit use of destructor method
// but you can't call drop directly.

// Here we are calling std::mem::drop, which is in the prelude
fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created");
    drop(c); // Note the difference in syntax
    println!("CustomSmartPointer dropped before the end of main");
}
// CustomSmartPointer created
// Dropping CustomSmartPointer with data `some data`!
// CustomSmartPointer dropped before the end of main
```
### In Chapter - new terms


## Running Code on Cleanup with the Drop Trait

The second trait important to the smart pointer pattern is Drop, which lets you customize what happens when a value is about to go out of scope.
-  You can provide an implementation for the Drop trait on any type, and that code can be used to release resources like files or network connections.
- Drop trait is almost always used when implementing a smart pointer. 
- For example, when a Box<T> is dropped, it will deallocate the space on the heap that the box points to.
- In some languages, for some types, the programmer must call code to free memory or resources every time they finish using an instance of those types.
- Examples include file handles, sockets, and locks.
- If the programmer forgets, the system might become overloaded and crash. 
- In Rust, you can specify that a particular bit of code be run whenever a value goes out of scope, and the compiler will insert this code automatically.
- As a result, you don’t need to be careful about placing cleanup code everywhere in a program that an instance of a particular type is finished with—you still won’t leak resources!

- You specify the code to run when a value goes out of scope by implementing the Drop trait. 
- The Drop trait requires you to implement one method named drop that takes a mutable reference to self.
- To see when Rust calls drop, let’s implement drop with println! statements for now.

CustomSmartPointer struct whose only custom functionality is that it will print Dropping CustomSmartPointer! when the instance goes out of scope, to show when Rust runs the drop method.

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) { // implementing drop trait
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created");
} // drop runs here() - at end of scope
// CustomSmartPointers created
// Dropping CustomSmartPointer with data `other stuff`!
// Dropping CustomSmartPointer with data `my stuff`!
```

- The Drop trait is included in the prelude, so we don’t need to bring it into scope. 
- Rust automatically called drop for us when our instances went out of scope, calling the code we specified. 

- Unfortunately, it’s not straightforward to disable the automatic drop functionality. Disabling drop isn’t usually necessary; the whole point of the Drop trait is that it’s taken care of automatically. 
- Occasionally, however, you might want to clean up a value early. One example is when using smart pointers that manage locks: 
- You might want to force the drop method that releases the lock so that other code in the same scope can acquire the lock. 
- Rust doesn’t let you call the Drop trait’s drop method manually; instead, you have to call the std::mem::drop function provided by the standard library if you want to force a value to be dropped before the end of its scope.

```rust
// Here we are calling std::mem::drop, which is in the prelude
fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created");
    drop(c); // Note the difference in syntax
    println!("CustomSmartPointer dropped before the end of main");
}
// CustomSmartPointer created
// Dropping CustomSmartPointer with data `some data`!
// CustomSmartPointer dropped before the end of main
```

- You can use code specified in a Drop trait implementation in many ways to make cleanup convenient and safe: For instance, you could use it to create your own memory allocator! With the Drop trait and Rust’s ownership system, you don’t have to remember to clean up, because Rust does it automatically.
- You also don’t have to worry about problems resulting from accidentally cleaning up values still in use: The ownership system that makes sure references are always valid also ensures that drop gets called only once when the value is no longer being used.