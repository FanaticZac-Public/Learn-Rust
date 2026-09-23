# Learning Rust

A professional learning record of Rust - which will evolve into applications focused on practicing physics, algorithms, game development, socket programming, and beyond.

- Note: I have a Bsc.CompSci Degree so I may skip over some stuff I find boring and standard.
    - However I will probably still go over all sections of the text just for posterity sake.
- This is all me btw - no AI - at least for the book tutorials
    - For larger projects - or professional indy stuff... time is money. 
    - I don't publish what i don't understand at any rate.

## Goals

- Learn Rust fundamentals
- Create subprojects for post-degree practice
    - Physics (with Bevy)
    - Algorithms
    - Game Development (with Bevy)
    - Socket Programming (NetSec + Multiplayer Games)
    - Professional Projects
    - Review all course material from degree (and work it in where possible)
- For this git project to be my reference and career demonstration

    ### Note
    - Kill 2 birds with 1 stone is one of my fundamental personal/professional philosophies for maximizing productivity 
    - 1 action - 2 (or more) results
    - This Project = (learn rust | practice compSci | market for the jobs I want | prelude to Bevy game)

## Plan
- Going to go over entire "The Rust Programming Language" text. 
- Will learn Bevy Quick


## Rust Learning Links
- [The Rust Programming Language](https://doc.rust-lang.org)
    - [Appendix A: Keywords](https://doc.rust-lang.org/book/appendix-01-keywords.html)
    - [Appendix B: Operators and Symbols](https://doc.rust-lang.org/book/appendix-02-operators.html)
    - [Appendix C: Derivable Traits](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html)
    - [Appendix D: Useful Development Tools](https://doc.rust-lang.org/book/appendix-04-useful-development-tools.html)
    - [Constant Evaluation - which operations and compile time concerns](https://doc.rust-lang.org/reference/const_eval.html)

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
- Package Managers
    - [Rust - The Cargo Book](https://doc.rust-lang.org/cargo/)
    - [Crates - Rust Community Crates](https://crates.io/)
- [Bevy - Game Engine](https://bevy.org/) 
- 3rd Party Resources
    - [The Impatient Programmer's Guide to Bevy and Rust](https://aibodh.com/posts/bevy-rust-game-development-chapter-1/)
        - [Git Repo](https://github.com/jamesfebin/ImpatientProgrammerBevyRust)

## Other Learning Links
- [Markdown (MD) Reference](https://www.markdownguide.org/cheat-sheet/)
- [SipHash - Rust hashmap default hasher to mitigate DDOS](https://en.wikipedia.org/wiki/SipHash)
    - Discussed end ot ch8 lightly
## Rust API Quick Reference - mostly just adding stuff i looked at for tutorial or specific use-case 
- [Enum Option (Special Enum - built in - Rust's version of null)](https://doc.rust-lang.org/std/option/enum.Option.html)
- [Enum IpAddr ](https://doc.rust-lang.org/std/net/enum.IpAddr.html)