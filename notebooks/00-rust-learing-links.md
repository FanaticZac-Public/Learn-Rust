# Rust Learning Links
These are links that I wanted to reference via things i encountered in the book as i went along - that have some special significance or priority. 

- [The Rust Programming Language](https://doc.rust-lang.org)
    - [The Rust Prelude - aka std library modules](https://doc.rust-lang.org/std/prelude/index.html)
    - [Appendix A: Keywords](https://doc.rust-lang.org/book/appendix-01-keywords.html)
    - [Appendix B: Operators and Symbols](https://doc.rust-lang.org/book/appendix-02-operators.html)
    - [Appendix C: Derivable Traits](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html)
    - [Appendix D: Useful Development Tools](https://doc.rust-lang.org/book/appendix-04-useful-development-tools.html)
    - [Constant Evaluation - which operations and compile time concerns](https://doc.rust-lang.org/reference/const_eval.html)
    - [Attributes in Rust](https://doc.rust-lang.org/reference/attributes.html)
- [Rust's Resource Recommendations](https://doc.rust-lang.org/stable/)
- [The Embedded Rust Book](https://doc.rust-lang.org/beta/embedded-book/)
    - [Zero Cost Abstractions](https://doc.rust-lang.org/beta/embedded-book/static-guarantees/zero-cost-abstractions.html)
- [The Cargo Book](https://doc.rust-lang.org/cargo/)
    - [Profiles - The Cargo Book](https://doc.rust-lang.org/cargo/reference/profiles.html)
- [The Rustonomicon - The Dark Arts of Unsafe Rust](https://doc.rust-lang.org/nomicon/index.html)
- [Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/)
- Macros
    - [The Little Book of Rust Macros](https://lukaswirth.dev/tlborm/)
    - [Macros by example](https://doc.rust-lang.org/reference/macros-by-example.html)
- [Rust Language API reference](https://doc.rust-lang.org/std/prelude/index.html)
    - [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
        - TODO: Need to review this (following tutorial - haven't yet).
    - [Collections](https://doc.rust-lang.org/std/collections/index.html)
        - This also has a "use a BLANK" for "BLANK" situation section - which might be helpful when building stuff from scratch in an architecturally conscience way.
        - Also has Big(O) performance chart for all (plus guide)
        - [Vectors (struct)](https://doc.rust-lang.org/std/vec/struct.Vec.html)
        - [String (struct)](https://doc.rust-lang.org/std/vec/struct.Vec.html)
            - There's some complexity and error potential with strings in Rust (re-read chapter later)
            - Be sure to check out the documentation for useful methods:
                - contains for searching in a string 
                - replace for substituting parts of a string with another string.
        - [quote (Crate)](https://docs.rs/quote/latest/quote/)
        - [syn (Crate)](https://docs.rs/syn/2.0.119/syn/index.html)
    - [Trait Termination (process)](https://doc.rust-lang.org/std/process/trait.Termination.html)
        - Is for custom main function termination exit codes with function 'report'.
    - [args (Function)](https://doc.rust-lang.org/std/env/fn.args.html)
- Package Managers
    - [Rust - The Cargo Book](https://doc.rust-lang.org/cargo/)
    - [Crates - Rust Community Crates](https://crates.io/)
        - parsing related (parse, stringify):
            - [syn (Crate)](https://crates.io/crates/syn)
            - [quote (Crate)](https://crates.io/crates/syn)
- [Bevy - Game Engine](https://bevy.org/) 
- 3rd Party Resources
    - [The Impatient Programmer's Guide to Bevy and Rust](https://aibodh.com/posts/bevy-rust-game-development-chapter-1/)
        - [Git Repo](https://github.com/jamesfebin/ImpatientProgrammerBevyRust)

## Rust - Free Exercise or tutorial links
- [Rustlings](https://rustlings.rust-lang.org/) - Recommended in parallel to reading the official Rust book 📚️
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - 

## Rust - Bevy - Free Exercise or tutorial links
- [Rustlings](https://bevy.org/learn/quick-start/introduction/) - Recommended in parallel to reading the official Rust book 📚️

## Rust - Books (Paid)
- [Rust for Rustaceans](https://rust-for-rustaceans.com/) - For developers who’ve mastered the basics

## Other Learning Links
- [Markdown (MD) Reference](https://www.markdownguide.org/cheat-sheet/)
- [SipHash - Rust hashmap default hasher to mitigate DDOS](https://en.wikipedia.org/wiki/SipHash)
    - Discussed end ot ch8 lightly
- [WebAssembly](https://webassembly.org/)
- [Linux Foundation’s Software Package Data Exchange (SPDX)](https://spdx.org/licenses/)
    - For the license field of TOML files for published crates, you need to give a license identifier value.

## Rust API Quick Reference - mostly just adding stuff i looked at for tutorial or specific use-case 

- [Enum Option (Special Enum - built in - Rust's version of null)](https://doc.rust-lang.org/std/option/enum.Option.html)
- [Enum IpAddr ](https://doc.rust-lang.org/std/net/enum.IpAddr.html)


