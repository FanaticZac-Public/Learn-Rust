<div align="center">
  <img src="./images/rust-images/rust-panics.svg" width="30%">
  <img src="./images/rust-images/rust-does_not_compile.svg" width="30%">
  <img src="./images/rust-images/rust-panics.svg" width="30%">
</div>

# Learning Rust

A professional learning record of Rust - which will evolve into applications focused on practicing physics, algorithms, game development, socket programming, and beyond - in Rust.

## Goals (Dynamic)

- Learn Rust fundamentals
- Create subprojects for post-degree practice
  - Physics (with Bevy)
  - Algorithms
  - Game Development (with Bevy)
  - Socket Programming (NetSec + Multiplayer Games)
  - Professional Projects
  - Review all course material from degree (and work it in where possible)
- For this git project to be my reference and career demonstration

> [!TIP]
>
> The 'Kill many birds with 1 stone throw' philosophy maximizes productivity  
> - (1 ACTION FOR 2 OR MORE RESULTS)
> - This Project = (learn rust | practice compSci | market for the jobs I want | prelude to Bevy game)

<table cellpadding="0" cellspacing="0"  bgcolor="#090909"   border="1"
  frame="box"   rules="none">
  <tr>
    <td width="66" valign="middle">
      <img
        src="./images/ed-images/ed_hack_1.png"
        width="40"
        height="40"
        alt="Author Git Logo"
        align="center"
      >
    </td>
    <td>
   <strong>"What an arm!"
</strong> ... they'll say. 
    </td>
  </tr>
</table>

## Hello, World ...?

My name is Zach, I'm a Software Developer based in British Columbia, Canada. I hold a Bachelor of Science degree in Applied Computer Science (Game Development specialty) from BCIT.

- I completed specialty courses in Network Security Applications also. 💪

### Repository Purpose

I didn't marketing myself much throughout my schooling. I used several git accounts, used perforce for games courses, my major projects are top secret (shh..), and frankly, I don't like dumping my previous instructors resources on git, nor like the prospect of dumping my old assignments on git after-the-fact is... lame lol.

Frankly - that's where this Rust venture is an excellent avenue. Since Rust has some incredible strengths as a programming language (and novel mechanisms to learn), it's a great place to also review and recreate some of my favorite projects, pursue some more ambitious objectives, and market myself in a more thoughtful way - going forward.

So it is in that spirit of practice, marketing, and intellectual exploration - that I have created this project repository. Rust (an actually new-ish language conceptually, with excellent characteristics for my primary hobbies and professional goals) makes this potentially monotonous process of self-marketing something rather exciting instead.

Advantages of Rust:
- Compiles into machine code
- High performance 
  - C/C++-level performance with stronger memory and concurrency safety.
  - (But still with that syntactic sugar (so sweet))
- Safer conduct 
  - Memory safety without garbage collection
  - Pushes several run-time errors into the compile time checks
  - Ownership and borrowing 
  - Thread safety

Pushes many common bugs from runtime to compile time errors: 
- e.g. Prevents these bugs: use-after-free, null-pointer dereferences, double frees, (many) buffer overflows

Those advantages are excelling for:
  - System Programming
  - Games (Bevy)
  - Embedded Programming
  - Web Assembly
  - Socket programming

Some disadvantages:
  - Steeper learning curve 
    - (Oh baby)
  - Longer compile time 
    - (Sure, but pairs well with espresso breaks)

In short, it offers: 
  - C/C++-level performance with stronger memory and concurrency safety. 
  - And a plethora of practical use-cases for my interests

<table cellpadding="0" cellspacing="0"  bgcolor="#090909"   border="1"
  frame="box"   rules="none">
  <tr>
    <td width="66" valign="middle">
      <img
        src="./images/ed-images/ed_hack_1.png"
        width="40"
        height="40"
        alt="Author Git Logo"
        align="center"
      >
    </td>
    <td>
   <strong>Rustacean in training 
</strong>
    </td>
  </tr>
</table>

## Project Orchestration (Flexible)

### Step 1 - The Rust Programming Language - Book Read Through

<table cellpadding="0" cellspacing="0"  bgcolor="#090909"   border="1"
  frame="box"   rules="none">
  <tr>
    <td width="66"  valign="middle"  cellpadding="0"
  cellspacing="0">
      <img
        src="./images/rust-images/rust-does_not_compile.svg"
        width="40"
        height="40"
        alt="Rust - Does note compile crab"
        align="center"
      >
    </td>
    <td>
      <strong>Currently in progress</strong>
    </td>
  </tr>
</table>

My initial objective is to just read through the entire book and do all the primary exercises to become familiar with the language and it's core mechanisms that have given Rust it's reputation for safety and performance with higher language syntactic sugar.

I'm most interested in getting pass this part quickly so i can build - real things, especially with Bevy, but also other applications. Ill add resource links below to my practice, schedule, exercise ideas, and so on. I'll be updating as I go - so this repository is a large catch all for a bunch of things that I'm willing to share - and is less about producing a thing. It's an educational resource I can publicize, and so inherently dynamic.

#### Personal Resources

Quick references that are relevant to me and this leg of the project.

- [Rust Learning Links](./notebooks/00-rust-learing-links.md) (Noted as encountered)
- [TimeSchedule](./schedule/pomodoros-timesheet.ods) (Pomodoros based)

Here's something special. Unlike the cheat sheet below which is more exhaustive to overall syntax related to Rust, I wanted to create a document that digs down specifically on the advantages of the Rust languages, in a shorter form document (for my mindfulness and as reference for others).

- [Rust - Primary Advantages](./notebooks/01-rust-primary-advantages)

#### Cheat Sheets - Quick Reference

These mostly contain factual statements or code samples from the book without the fluff - for quick reference by me - but I also added more where I had extra questions, wanted more samples, or ran tangential experiments. 

If you have already read the rust book (or have professional coding experience) - might be a faster way to explore Rust (or least skim it's benefits - like safety enforcement - big time). At any rate - it's a record of my learning progress

This is a more rough brain stormy project plan (at this point I don't have a scrum board)

- [README_Tutorial](./tutorial/Rust%20Prog%20Lang/README_Tutorial_.md)

<!-- This is the template project for working on big picture organization (package, modules, structure), understand lib.rs, main.rs, bin/, and a larger projects shape and standards throughout.

- [Template-Project](./tutorial/Rust%20Prog%20Lang/template-project/) -->

These are my cheat sheets for each chapter

1. [My Cheat Sheet - ch01 - Getting Started](./tutorial/Rust%20Prog%20Lang/ch01/ch01-cheat_sheet.md) - DONE
2. [My Cheat Sheet - ch02 - Programming a Guessing Game](./tutorial/Rust%20Prog%20Lang/ch02/ch02-cheat_sheet.md) - DONE - but fix writing later.
3. [My Cheat Sheet - ch03 - Common Programming Concepts](./tutorial/Rust%20Prog%20Lang/ch03/ch03-cheat_sheet.md) - DONE
4. [My Cheat Sheet - ch04 - Understanding Ownership](./tutorial/Rust%20Prog%20Lang/ch04/ch04-cheat_sheet.md) - Done
5. [My Cheat Sheet - ch05 - Using Structs to Structure Related Data](./tutorial/Rust%20Prog%20Lang/ch05/ch05-cheat_sheet.md) - DONE
6. [My Cheat Sheet - ch06 - Enums and Pattern Matching](./tutorial/Rust%20Prog%20Lang/ch06/ch06-cheat_sheet.md) - Done
7. [My Cheat Sheet - ch07 - Packages, Crates, and Modules](./tutorial/Rust%20Prog%20Lang/ch07/ch07-cheat_sheet) - Needs special attention
8. [My Cheat Sheet - ch08 - Common Collections](./tutorial/Rust%20Prog%20Lang/ch08/ch08-cheat_sheet.md) - Done
9. [My Cheat Sheet - ch09 - Error Handling](./tutorial/Rust%20Prog%20Lang/ch09/ch9-cheat_sheet.md)
10. [My Cheat Sheet - ch10 - Generic Types, Traits, and Lifelines](./tutorial/Rust%20Prog%20Lang/ch10/ch10-cheat_sheet.md)
11. [My Cheat Sheet - ch11 - Writing Tests](./tutorial/Rust%20Prog%20Lang/ch11/ch11-cheat-sheet.md) - DONE
12. [My Cheat Sheet - ch12 - An I/O Project: Command Line Program](./tutorial/Rust%20Prog%20Lang/ch12/ch12-cheat_sheet.md)
13. [My Cheat Sheet - ch13 - Functional Language Features: Iterators and Closures](./tutorial/Rust%20Prog%20Lang/ch13/ch13-cheat_sheet.md) - DONE - but fix writing later / review
14. [My Cheat Sheet - ch14 - More About Cargo and Crates.io](./tutorial/Rust%20Prog%20Lang/ch14/ch14-cheat_sheet.md) - Done, but needs more work for final cheat-sheet

##### Final Checklist for all chapters
- [ ] Check - CODE EXAMPLES (add more if wanting)
- [ ] Check - Writing makes sense
- [ ] Check - Formatting
- [ ] Check - Create links for 'by subject'
- [ ] Check - For my primary advantages guide
- [ ] Check - That all summaries are added to the cheat sheet main page
- [ ] Finally - Can probably remove the original notes.md after each chapter is FINAL CHECKED.

---

### Step 2 - Bevy Tutorial

<table cellpadding="0" cellspacing="0"  bgcolor="#090909"   border="1"
  frame="box"   rules="none">
  <tr>
    <td width="66"  valign="middle"  cellpadding="0"
  cellspacing="0">
      <img
          src="./images/rust-images/rust-panics.svg"
        width="40"
        height="40"
        alt="Rust - Panic crab"
        align="center"
      >
    </td>
    <td>
      <strong>Blocked / Pending</strong>
    </td>
  </tr>
</table>

---

### Step 3 - Physics Simulations with Bevy - Grade 11 Physics Review

<table cellpadding="0" cellspacing="0"  bgcolor="#090909"   border="1"
  frame="box"   rules="none">
  <tr>
    <td width="66"  valign="middle"  >
      <img
          src="./images/rust-images/rust-panics.svg"
        width="40"
        height="40"
        alt="Rust - Panic crab"
        align="center"
      >
    </td>
    <td>
      <strong>Blocked / Pending</strong>
    </td>
  </tr>
</table>
