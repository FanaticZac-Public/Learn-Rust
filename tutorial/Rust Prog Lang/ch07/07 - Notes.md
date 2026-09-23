# Chapter 07 - Packages, Crates, and Modules

_Note: this is mostly just copied from the chapter as reading - because most of it is simply rust related. I did break it out into lines though_

- By grouping related functionality and separating code with distinct features, you’ll clarify where to find code that implements a particular feature and where to go to change how a feature works.
- A package can contain multiple binary crates and optionally one library crate.
- As a package grows, you can extract parts into separate crates that become external dependencies.
- For very large projects comprising a set of interrelated packages that evolve together, Cargo provides workspaces (ch 14)
- We’ll also discuss encapsulating implementation details, which lets you reuse code at a higher level
- Once you’ve implemented an operation, other code can call your code via its public interface without having to know how the implementation works.
  - The way you write code defines which parts are public for other code to use and which parts are private implementation details that you reserve the right to change.
- Will discuss scope and how to resolve conflicts
- Rust has a number of features that allow you to manage your code’s organization, including which details are exposed, which details are private, and what names are in each scope in your programs.
  These features, sometimes collectively referred to as the module system, include:
- Packages: A Cargo feature that lets you build, test, and share crates
- Crates: A tree of modules that produces a library or executable
- Modules and use: Let you control the organization, scope, and privacy of paths
- Paths: A way of naming an item, such as a struct, function, or module

## 7.1 Packages and Crates

- A crate is the smallest amount of code that the Rust compiler considers at a time.
- Even if you run rustc rather than cargo and pass a single source code file the compiler considers that file to be a crate.
- Crates can contain modules, and the modules may be defined in other files that get compiled with the crate
- A crate can come in one of two forms: a binary crate or a library crate.
  - Binary crates are programs you can compile to an executable that you can run, such as a command line program or a server.
    - Each must have a function called main that defines what happens when the executable runs. All the crates we’ve created so far have been binary crates.
  - Library crates don’t have a main function, and they don’t compile to an executable. Instead, they define functionality intended to be shared with multiple projects.
    - For example, the rand crate we used in Chapter 2 provides functionality that generates random numbers.
    - Most of the time when Rustaceans say “crate,” they mean library crate, and they use “crate” interchangeably with the general programming concept of a “library.”
- The 'crate root' is a source file that the Rust compiler starts from and makes up the root module of your crate
- A package is a bundle of one or more crates that provides a set of functionality
  - A package contains a Cargo.toml file that describes how to build those crates.
  - Cargo is actually a package that contains the binary crate for the command line tool you’ve been using to build your code. - The Cargo package also contains a library crate that the binary crate depends on.
    Other projects can depend on the Cargo library crate to use the same logic the Cargo command line tool uses.
- A package can contain as many binary crates as you like, but at most only one library crate. A package must contain at least one crate, whether that’s a library or binary crate.

Let’s walk through what happens when we create a package.

```rust
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```

- In the my-project directory, there’s a Cargo.toml file, giving us a package.
- There’s also a src directory that contains main.rs. Open Cargo.toml in your text editor and note that there’s no mention of src/main.rs.
- Cargo follows a convention that src/main.rs is the crate root of a binary crate with the same name as the package.
- Likewise, Cargo knows that if the package directory contains src/lib.rs, the package contains a library crate with the same name as the package, and src/lib.rs is its crate root.
- Cargo passes the crate root files to rustc to build the library or binary.
- If a package contains src/main.rs and src/lib.rs, it has two crates: a binary and a library, both with the same name as the package.
  - A package can have multiple binary crates by placing files in the src/bin directory: Each file will be a separate binary crate.

## 7.2 Control Scope and Privacy with Modules

we’ll talk about modules and other parts of the module system, namely paths, which allow you to name items; the use keyword that brings a path into scope; and the pub keyword to make items public. We’ll also discuss the as keyword, external packages, and the glob operator.

### Modules Cheat Sheet

- Start from the crate root: When compiling a crate, the compiler first looks in the crate root file (usually src/lib.rs for a library crate and src/main.rs for a binary crate) for code to compile.
- Declaring modules: In the crate root file, you can declare new modules; say you declare a “garden” module with mod garden;. The compiler will look for the module’s code in these places:
  - Inline, within curly brackets that replace the semicolon following mod garden
  - In the file src/garden.rs
  - In the file src/garden/mod.rs
- Declaring submodules: In any file other than the crate root, you can declare submodules. For example, you might declare mod vegetables; in src/garden.rs. The compiler will look for the submodule’s code within the directory named for the parent module in these places:
  - Inline, directly following mod vegetables, within curly brackets instead of the semicolon
  - In the file src/garden/vegetables.rs
  - In the file src/garden/vegetables/mod.rs
- Paths to code in modules: Once a module is part of your crate, you can refer to code in that module from anywhere else in that same crate, as long as the privacy rules allow, using the path to the code. For example, an Asparagus type in the garden vegetables module would be found at crate::garden::vegetables::Asparagus.
- Private vs. public: Code within a module is private from its parent modules by default. To make a module public, declare it with pub mod instead of mod. To make items within a public module public as well, use pub before their declarations.
- The use keyword: Within a scope, the use keyword creates shortcuts to items to reduce repetition of long paths. In any scope that can refer to crate::garden::vegetables::Asparagus, you can create a shortcut with use crate::garden::vegetables::Asparagus;, and from then on you only need to write Asparagus to make use of that type in the scope.

Here, we create a binary crate named backyard that illustrates these rules. The crate’s directory, also named backyard, contains these files and directories:

```
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

The crate root file in this case is src/main.rs, and it contains:

- Filename: src/main.rs

```rust
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}
```

Here, pub mod vegetables; means the code in src/garden/vegetables.rs is included too. That code is:

```rust
#[derive(Debug)]
pub struct Asparagus {}
```

### Grouping Related Code in Modules

- Modules let us organize code within a crate for readability and easy reuse.
- **Modules also allow us to control the privacy of items because code within a module is private by default.**
- Private items are internal implementation details not available for outside use.
- We can choose to make modules and the items within them public, which exposes them to allow external code to use and depend on them.
  -As an example, let’s write a library crate that provides the functionality of a restaurant. We’ll define the signatures of functions but leave their bodies empty to concentrate on the organization of the code rather than the implementation of a restaurant.
- In the restaurant industry, some parts of a restaurant are referred to as front of house and others as back of house. Front of house is where customers are; this encompasses where the hosts seat customers, servers take orders and payment, and bartenders make drinks. Back of house is where the chefs and cooks work in the kitchen, dishwashers clean up, and managers do administrative work.
- To structure our crate in this way, we can organize its functions into nested modules. Create a new library named restaurant by running cargo new restaurant --lib.
  Filename: src/lib.rs

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

- We define a module with the mod keyword followed by the name of the module (in this case, front_of_house). The body of the module then goes inside curly brackets.
  - Inside modules, we can place other modules, as in this case with the modules hosting and serving. Modules can also hold definitions for other items, such as structs, enums, constants, traits, and as in Listing 7-1, functions.
- Programmers using this code can navigate the code based on the groups rather than having to read through all the definitions, making it easier to find the definitions relevant to them.
  Earlier, we mentioned that src/main.rs and src/lib.rs are called crate roots. The reason for their name is that the contents of either of these two files form a module named crate at the root of the crate’s module structure, known as the module tree.

```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

## 7.3 Paths for Referring to an Item in the Module Tree

To show Rust where to find an item in a module tree, we use a path in the same way we use a path when navigating a filesystem. To call a function, we need to know its path.

- A path can take two forms: - An absolute path is the full path starting from a crate root; for code from an external crate, the absolute path begins with the crate name, and for code from the current crate, it starts with the literal crate. - A relative path starts from the current module and uses self, super, or an identifier in the current module.
  -Both absolute and relative paths are followed by one or more identifiers separated by double colons (::).

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

- say we want to call the add_to_waitlist function. This is the same as asking: What’s the path of the add_to_waitlist function?
- The eat_at_restaurant function is part of our library crate’s public API, so we mark it with the pub keyword.

_I already understand paths, the example is enough._

- Choosing whether to use a relative or absolute path is a decision you’ll make based on your project, and it depends on whether you’re more likely to move item definition code separately from or together with the code that uses the item.
- Rust preference in general is to specify absolute paths because it’s more likely we’ll want to move code definitions and item calls independently of each other.

- This will still get error[E0603]: module `hosting` is private
- we have the correct paths for the hosting module and the add_to_waitlist function, but Rust won’t let us use them because it doesn’t have access to the private sections.
- In **Rust**, all items (functions, methods, structs, enums, modules, and constants) are private to parent modules by default. If you want to make an item like a function or struct private, you put it in a module.
- Items in a parent module can’t use the private items inside child modules, but items in child modules can use the items in their ancestor modules.

### Exposing Paths with the pub Keyword

Add pub to mod:

```rust
mod front_of_house {
    pub mod hosting { //added
        fn add_to_waitlist() {}
    }
}
```

- still there is error[E0603]: function `add_to_waitlist` is private
  - And so pub must be added to that function too

```rust
mod front_of_house {
    pub mod hosting { //added
        pub fn add_to_waitlist() {}
    }
}
```

- While front_of_house isn’t public, because the eat_at_restaurant function is defined in the same module as front_of_house (that is, eat_at_restaurant and front_of_house are siblings), we can refer to front_of_house from eat_at_restaurant.
- In the relative path, the logic is the same as the absolute path except for the first step: Rather than starting from the crate root, the path starts from front_of_house. The front_of_house module is defined within the same module as eat_at_restaurant, so the relative path starting from the module in which eat_at_restaurant is defined works.
- your public API is your contract with users of your crate that determines how they can interact with your code.
- There are many considerations around managing changes to your public API to make it easier for people to depend on your crate.
  - [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

### Best Practices for Packages with a Binary and a Library

- **We mentioned that a package can contain both a src/main.rs binary crate root as well as a src/lib.rs library crate root, and both crates will have the package name by default. Typically, packages with this pattern of containing both a library and a binary crate will have just enough code in the binary crate to start an executable that calls code defined in the library crate. This lets other projects benefit from the most functionality that the package provides because the library crate’s code can be shared.**

- **The module tree should be defined in src/lib.rs. Then, any public items can be used in the binary crate by starting paths with the name of the package. The binary crate becomes a user of the library crate just like a completely external crate would use the library crate: It can only use the public API. This helps you design a good API; not only are you the author, but you’re also a client!**

### Starting Relative Paths with super

- We can construct relative paths that begin in the parent module, rather than the current module or the crate root, by using super at the start of the path.
  - This is like starting a filesystem path with the .. syntax that means to go to the parent directory.
- Using super allows us to reference an item that we know is in the parent module, which can make rearranging the module tree easier when the module is closely related to the parent but the parent might be moved elsewhere in the module tree someday.

```rust
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
```

- super here is like ../ of a relative path to parent of current module (in this case crate)
- Because think the back_of_house module and the deliver_order function are likely to stay in the same relationship to each other and get moved together should we decide to reorganize the crate’s module tree. Then we used super so that we’ll have fewer places to update code in the future if this code gets moved to a different module.

### Making Structs and Enums Public

- If we use pub before a struct definition, we make the struct public, but the struct’s fields will still be private.
  - We can make each field public or not on a case-by-case basis.

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast.
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal.
    // meal.seasonal_fruit = String::from("blueberries");
}
```

- Ok - So we can only access the toast in struct directly - we can initialize a Breakfast with the summer function - and it can acess seasonal_fruit but eat_at_resaurant can't, but it can change the toast
  In contrast, if we make an enum public, all of its variants are then public. We only need the pub before the enum keyword, as shown in Listing 7-10.

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

- Because we made the Appetizer enum public, we can use the Soup and Salad variants in eat_at_restaurant.
- Enums aren’t very useful unless their variants are public; it would be annoying to have to annotate all enum variants with pub in every case, so the default for enum variants is to be public.
- Structs are often useful without their fields being public, so struct fields follow the general rule of everything being private by default unless annotated with pub.

## 7.4 Bringing Paths into Scope with the use Keyword

Pretty standard.

- use crate::front_of_house::hosting;
- the only need to call:
  - hosting::add_to_waitlist();
- Similar to symbolic links in filesystem - it says

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
```

- A use statement only applies in the scope it’s in.
  - error[E0433]: failed to resolve: use of unresolved module or unlinked crate `hosting`
- To fix, move the use within the customer module too, or reference the shortcut in the parent module with super::hosting within the child customer module.

### Creating Idiomatic use Paths

- We can use 'use' for a particular function but we just want to bring the parent of the function into scope, call that parent and it's method as it's informative.
- On the other hand, when bringing in structs, enums, and other items with use, it’s idiomatic to specify the full path.
- Hashmap bonus! shows the idiomatic way to bring the standard library’s HashMap struct into the scope of a binary crate.

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

- The exception to this idiom is if we’re bringing two items with the same name into scope with use statements, because **Rust** doesn’t allow that.
  Example of conflict:

```rust
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}
```

- As you can see, using the parent modules distinguishes the two Result types.

### Providing New Names with the as Keyword

Use of the 'as' keyboard

- There’s another solution to the problem of bringing two types of the same name into the same scope with use: After the path, we can specify as and a new local name, or alias, for the type.

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

- Basically giving one an alias
- _Both methods, using 'as' or only using 'use' up to nonconflicting parent are idiomatic to Rust_

### Re-exporting Names with pub use

When we bring a name into scope with the use keyword, the name is private to the scope into which we imported it.

- To enable code outside that scope to refer to that name as if it had been defined in that scope, we can combine pub and use.
- This technique is called re-exporting

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

- Before this change, external code would have to call the add_to_waitlist function by using the path restaurant::front_of_house::hosting::add_to_waitlist(), which also would have required the front_of_house module to be marked as pub. - **Now that this pub use has re-exported the hosting module from the root module, external code can use the path restaurant::hosting::add_to_waitlist() instead.**

- Re-exporting is useful when the internal structure of your code is different from how programmers calling your code would think about the domain.
- With pub use, we can write our code with one structure but expose a different structure. Doing so makes our library well organized for programmers working on the library and programmers calling the library.

### Using External Packages

- To use rand in our project, we added this line to Cargo.toml:

```
rand = "0.8.5"
```

- Adding rand as a dependency in Cargo.toml tells Cargo to download the rand package and any dependencies from crates.io and make rand available to our project.
- Then, to bring rand definitions into the scope of our package, we added a use line starting with the name of the crate, rand, and listed the items we wanted to bring into scope.
- Recall that in “Generating a Random Number” in Chapter 2, we brought the Rng trait into scope and called the rand::thread_rng function:

```rust
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
```

- Members of the Rust community have made many packages available at crates.io, and pulling any of them into your package involves these same steps: listing them in your package’s Cargo.toml file and using use to bring items from their crates into scope.
- Note that the standard std library is also a crate that’s external to our package.
- Because the standard library is shipped with the Rust language, we don’t need to change Cargo.toml to include std.
- But we do need to refer to it with use to bring items from there into our package’s scope. For example, with HashMap we would use this line:

```rust
use std::collections::HashMap;
```

- This is an absolute path starting with std, the name of the standard library crate.

### Using Nested Paths to Clean Up use Lists

If we’re using multiple items defined in the same crate or same module, listing each item on its own line can take up a lot of vertical space in our files. For example, these two use statements we had in the guessing game in Listing 2-4 bring items from std into scope:

```rust
// --snip--
use std::cmp::Ordering;
use std::io;
// --snip--
```

Instead, we can use nested paths to bring the same items into scope in one line.

```rust
use std::{cmp::Ordering, io};
```

We can use a nested path at any level in a path, which is useful when combining two use statements that share a subpath.

```rust
use std::io;
use std::io::Write;
```

or we can do this similarly with self

```rust
use std::io::{self, Write};
```

- This line brings std::io and std::io::Write into scope.

### Importing Items with the Glob Operator

If we want to bring all public items defined in a path into scope, we can specify that path followed by the \* glob operator:

```rust
use std::collections::*;
```

- This use statement brings all public items defined in std::collections into the current scope.
- Be careful when using the glob operator! Glob can make it harder to tell what names are in scope and where a name used in your program was defined.
  - Additionally, if the dependency changes its definitions, what you’ve imported changes as well, which may lead to compiler errors when you upgrade the dependency if the dependency adds a definition with the same name as a definition of yours in the same scope, for example.
- The glob operator is often used when testing to bring everything under test into the tests module

## 7.5 Separating Modules into Different Files

_skipping a lot of the example stuff here for notes - im familiar_

So far, all the examples in this chapter defined multiple modules in one file. When modules get large, you might want to move their definitions to a separate file to make the code easier to navigate.

- First, we’ll extract the front_of_house module to its own file. Remove the code inside the curly brackets for the front_of_house module, leaving only the mod front_of_house; declaration, so that src/lib.rs contains the code shown

```rust
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

- Note that this won’t compile until we create the src/front_of_house.rs file
  Filename: src/lib.rs

```rust
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

- Note that you only need to load a file using a mod declaration once in your module tree.
- Once the compiler knows the file is part of the project (and knows where in the module tree the code resides because of where you’ve put the mod statement), other files in your project should refer to the loaded file’s code using a path to where it was declared
  Next, we’ll extract the hosting module to its own file. The process is a bit different because hosting is a child module of front_of_house, not of the root module. We’ll place the file for hosting in a new directory that will be named for its ancestors in the module tree, in this case src/front_of_house.
- To start moving hosting, we change src/front_of_house.rs to contain only the declaration of the hosting module:
  Filename: src/front_of_house.rs

```rust
pub mod hosting;
```

- Then, we create a src/front_of_house directory and a hosting.rs file to contain the definitions made in the hosting module:

Filename: src/front_of_house/hosting.rs

```rust
pub fn add_to_waitlist() {}
```

- It still needs to be child in the actual path
  - The compiler’s rules for which files to check for which modules’ code mean the directories and files more closely match the module tree.

### Alternate File Paths

Rust also supports an older style of file path. For a module named front_of_house declared in the crate root, the compiler will look for the module’s code in:
 - src/front_of_house.rs (what we covered)
- src/front_of_house/mod.rs (older style, still supported path)

For a module named hosting that is a submodule of front_of_house, the compiler will look for the module’s code in:

- src/front_of_house/hosting.rs (what we covered)
- src/front_of_house/hosting/mod.rs (older style, still supported path)

If you use both styles for the same module, you’ll get a compiler error.
- Using a mix of both styles for different modules in the same project is allowed but might be confusing for people navigating your project.
The main downside to the style that uses files named mod.rs is that your project can end up with many files named mod.rs, which can get confusing when you have them open in your editor at the same time.
- *In short - Avoid using the mod.rs files but understand what they are as you might encounter*

## Summary
Rust lets you split a package into multiple crates and a crate into modules so that you can refer to items defined in one module from another module. You can do this by specifying absolute or relative paths. These paths can be brought into scope with a use statement so that you can use a shorter path for multiple uses of the item in that scope. Module code is private by default, but you can make definitions public by adding the pub keyword.