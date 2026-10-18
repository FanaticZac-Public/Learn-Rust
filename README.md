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
> Productivity Tip: Kill many birds with 1 stone
> <br>This Project = (learn rust | practice compSci | market myself | prep for Bevy)

## Hello, World!

My name is Zach, I'm a Software Developer based in Vancouver, British Columbia, Canada. I hold a Bachelor of Science degree in Applied Computer Science (Game Development set) from BCIT.

> [!NOTE]
> I took 4/5 courses from Network Security Applications set also. 💪
> <br> Built a C2 botnet as the major project for that set. 🤖🕸️

### Repository Purpose

I didn't market myself much throughout my schooling. I used several git accounts, used perforce for games courses, my major projects are top secret (shh..), I don't like dumping my previous instructors resources on git or enjoy the prospect of dumping my old assignments on git after-the-fact is... lame.

So that's where this Rust venture comes in! Since Rust has some incredible strengths as a programming language (and novel mechanisms to learn), it's a great place to also review and recreate some of my favorite projects, pursue some more ambitious objectives, and market myself in a more thoughtful way going forward.

Rust is a fascinating language conceptually with excellent characteristics for my primary hobbies and professional goals - and so this potentially monotonous process of self-marketing becomes something rather exciting. So it is in that spirit of practice, marketing, and intellectual exploration - that I begin this new project.

Advantages of Rust:

- Compiles into machine code
- High performance
  - C/C++-level performance with stronger memory and concurrency safety. (with syntactic sugar)
- Safer conduct
  - Memory safety without garbage collection
  - Pushes several run-time errors into the compile time checks
  - Ownership and borrowing
  - Thread safety

Pushes many common bugs from runtime to compile time errors:

- e.g. use-after-free, null-pointer dereferences, double frees, (many) buffer overflows

Those advantages are excellent for:

- System Programming
- Games (Bevy)
- Embedded Programming
- Web Assembly
- Socket programming

Some disadvantages:

- Steeper learning curve (bug... or a feature?)
- Longer compile time (cross that bridge when ...)

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

![Rust Scuttle](./images/rust-images/rust-scuttle.gif)

I'm all about flexible orchestration. During my degree - I often advocated for solo projects whenever possible because it gives more dynamism without bureaucracy - and I want to work on each inch of a project (not just a subset). Even if my projects had more compromises, my personal exposure to the materials was arguably more robust, and I enjoyed making project decisions more akin to industry with regard to the finity of my time. As a consequence, I also take ownership naturally.

Like Agile methodology: having a plan matters, but so does listening to signals, and pivoting to high value targets (Dijkstra comes to mind). Making decisions under a deadline, choosing compromises, pivoting to great features at the expense of others, and solving problems you didn't expect to encounter - is the humanity in software development. Flexibility and creativity is the source of the Eureka! moments that were a particular highlight in my major projects. I digress.

I will try and maintain this loose excuse for a project until such time that I'm either employed or working full-time on a private project implementation. In the meantime, I am trying to do at least 1 meaningful commit per day.

- Some exception may be given to days I focus on reading or math practice with no clear or meaningful commit here - however the point is to translate those efforts into some demonstrable value here - with Rust.

My steps are laid out below - you can consider the pending ones as flexible nodes for practice and exposures I'm interested in pursuing - a short list from the backlog of the ReadMe_Brainstorm document.

### Resource Links

Resource links for my practice, schedule, exercise/utility ideas, and so on. I'll be updating as I go. This repository is a large catch all for a bunch of things that I'm willing to share - stuff in different states of progress. This is less about producing 'a thing'. It's an educational resource I can publicize, and so inherently dynamic, flexible, amorphous.

#### My Resources
- [Rust Learning Links](./notebooks/00-rust-learing-links.md) (Noted as encountered)
- [TimeSchedule](./schedule/pomodoros-timesheet.ods) (Pomodoros based)
- [Rust - Brainstorm](./README_BRAINSTORM_.md) - Rough ideas list for this project

#### Rust Resources (3rd Party guides)
- TODO: I want API method lists (this isn't in the book) 

### Step 1 - The Rust Programming Language - Book Read Through

<table cellpadding="0" cellspacing="0"  bgcolor="#2d961f" color="#333"  border="1"
  frame="box"   rules="none">
  <tr>
    <td width="66"  valign="middle"  cellpadding="0"
  cellspacing="0">
      <img
        src="./images/ed-images/ed_hack_11.png"
        width="40"
        height="40"
        alt="Rust - Does note compile crab"
        align="right"
      >
    </td>
    <td>
    <font color="#111">
    <strong>
  Done
<br>
Still cleaning/refining retroactively alongside next steps
</strong>
</font>
    </td>
  </tr>
</table>



<!-- <details open>  -->

My initial objective is to read through the ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) book and do all the primary exercises to become familiar with the language and it's core novel mechanisms that have give Rust it's reputation for safety and performance.

<!-- #### Personal Resources -->
<!-- Here's something special. Unlike the cheat sheet below which is more exhaustive to overall syntax related to Rust, I wanted to create a document that digs down specifically on the advantages of the Rust languages, in a shorter form document (for my mindfulness and as reference for others).

- [Rust - Primary Advantages](./notebooks/01-rust-primary-advantages) -->

#### Cheat Sheets - Quick Reference

If you have already read the rust book (or have professional coding experience) - might be a faster way to explore Rust (or least skim it's benefits - like safety enforcement - big time). At any rate - it's a record of my learning progress.

<!-- This is the template project for working on big picture organization (package, modules, structure), understand lib.rs, main.rs, bin/, and a larger projects shape and standards throughout.

- [Template-Project](./tutorial/Rust%20Prog%20Lang/template-project/) -->

#### Read-through/Cheat-Sheet Strategy

My implementation has been to read the (sub)chapters, copying in the quality text from the book line by line as bullet points (easier to analyze later) as I'm reading - and I also added more where I had extra questions, wanted more samples, or ran tangential experiments.

Then coming back I reread important chapters while doing Rustlings (next step) or other work, completed the super-summaries aka code only section (with comments), added links, and sometimes a 1-2 paragraph chapter summary.

- Note: Not always consistent but 
I just want quick references to look at - and to avoid pedantically wasting too much time for the sake of others.

#### These are my cheat sheets for each of the 21 chapters:

Most have a **Super Summary** section that is mostly the code with annotations - as concise as possible - for quick reference. (Work in Progress)

1. [Cheat Sheet - ch01 - Getting Started](./tutorial/Rust%20Prog%20Lang/ch01/ch01-cheat_sheet.md)
2. [Cheat Sheet - ch02 - Programming a Guessing Game](./tutorial/Rust%20Prog%20Lang/ch02/ch02-cheat_sheet.md) <!-- - DONE - but fix writing later. -->
3. [Cheat Sheet - ch03 - Common Programming Concepts](./tutorial/Rust%20Prog%20Lang/ch03/ch03-cheat_sheet.md) <!-- - DONE -->
4. [Cheat Sheet - ch04 - Understanding Ownership](./tutorial/Rust%20Prog%20Lang/ch04/ch04-cheat_sheet.md) <!-- - Done -->
5. [Cheat Sheet - ch05 - Using Structs to Structure Related Data](./tutorial/Rust%20Prog%20Lang/ch05/ch05-cheat_sheet.md) <!-- - DONE -->
6. [Cheat Sheet - ch06 - Enums and Pattern Matching](./tutorial/Rust%20Prog%20Lang/ch06/ch06-cheat_sheet.md) <!-- - Done -->
7. [Cheat Sheet - ch07 - Packages, Crates, and Modules](./tutorial/Rust%20Prog%20Lang/ch07/ch07-cheat_sheet) <!-- - Needs special attention -->
8. [Cheat Sheet - ch08 - Common Collections](./tutorial/Rust%20Prog%20Lang/ch08/ch08-cheat_sheet.md) <!-- - Done -->
9. [Cheat Sheet - ch09 - Error Handling](./tutorial/Rust%20Prog%20Lang/ch09/ch9-cheat_sheet.md) <!-- - DONE -->
10. [Cheat Sheet - ch10 - Generic Types, Traits, and Lifelines](./tutorial/Rust%20Prog%20Lang/ch10/ch10-cheat_sheet.md)
11. [Cheat Sheet - ch11 - Writing Tests](./tutorial/Rust%20Prog%20Lang/ch11/ch11-cheat-sheet.md) <!-- - DONE -->
12. [Cheat Sheet - ch12 - An I/O Project: Command Line Program](./tutorial/Rust%20Prog%20Lang/ch12/ch12-cheat_sheet.md)
13. [Cheat Sheet - ch13 - Functional Language Features: Iterators and Closures](./tutorial/Rust%20Prog%20Lang/ch13/ch13-cheat_sheet.md) <!-- - DONE - but fix writing later revie -->
14. [Cheat Sheet - ch14 - More About Cargo and Crates.io](./tutorial/Rust%20Prog%20Lang/ch14/ch14-cheat_sheet.md) <!-- - Done, but needs more work for final cheat-sheet -->
15. [Cheat Sheet - ch15 - Smart Pointers](./tutorial/Rust%20Prog%20Lang/ch15/ch15-cheat_sheet.md) <!-- - Done, but worth a re-read for sure (particularly RefCell) -->
16. [Cheat Sheet - ch16 - Fearless Concurrency](./tutorial/Rust%20Prog%20Lang/ch16/ch16-cheat_sheet.md) <!-- - Done -->
17. [Cheat Sheet - ch17 - Fundamentals of Asynchronous Programming](./tutorial/Rust%20Prog%20Lang/ch17/ch17-cheat_sheet.md) <!-- - Done (worth re-read) -->
18. [Cheat Sheet - ch18 - Object Oriented Programming Features](./tutorial/Rust%20Prog%20Lang/ch18/ch18-cheat_sheet.md) <!-- - Done (worth re-read) -->
19. [Cheat Sheet - ch19 - Patterns and Matching](./tutorial/Rust%20Prog%20Lang/ch19/ch19-cheat_sheet.md) <!-- - Done -->
20. [Cheat Sheet - ch20 - Advanced Features](./tutorial/Rust%20Prog%20Lang/ch20/ch20-cheat_sheet.md)
21. [Cheat Sheet - ch21 - Final Project: Building a Multithreaded Web Server](./tutorial/Rust%20Prog%20Lang/ch21/ch21-cheat_sheet.md)

<!-- #### Final Checklist for all chapters
- [ ] Check - CODE EXAMPLES (add more if wanting)
- [ ] Check - Writing makes sense (each section with a header - has a summary that equates to understanding in the code or other section of 'super summary)
- [ ] Check - Formatting
- [ ] Check - Create links for 'by subject'
- [ ] Check - Added section to my primary advantages guide (if worthy)
- [ ] Check - That all summaries are added to the cheat sheet main
- [ ] Check - Super Summary added to each chapter.
- [ ] Check - Should move definitions in sub-chapter to chapter main page.

- [ ] Do a special interest sheet.
  - Collect the borrowing rules (ch04)
  - Basically 1 paragraph of each sub-chapter (no code)
    - To add to the Super Summary section. -->

<!-- </details> -->

#### UPDATE: FINISHED
- But will likely refine more in the future, we'll see.
---

### Side Quest: Rustlings - Exercises/Quizzes

<table cellpadding="0" cellspacing="0"  bgcolor="#2d961f"   border="1"
  frame="box"   rules="none">
  <tr>
    <td width="66"  valign="middle"  cellpadding="0"
  cellspacing="0">
      <img
        src="./images/ed-images/ed_hack_11.png"
        width="40"
        height="40"
        alt="Rust - Does note compile crab"
        align="right"
      >
    </td>
    <td>
    <font color="#111">
    <strong>
  Done
<br>
Still cleaning/refining retroactively alongside next steps
</strong>
</font>
    </td>
  </tr>
</table>


<!-- <details><summary><strong>Rustlings - Details</strong></summary> -->

<!-- <details><summary><strong>Details</strong></summary> -->

To reinforce the book's learning outcomes while fresh of mind - I'm committing to some further practicing of the basics - drawing from the book (while I also refine the cheat sheets).

Rustlings is an interactive program that has 94 exercises where you have to fix the code to beat the lesson. It's intended as a work book alongside reading the official Rust book, but I wasn't aware of it at first. Doing tests after completing the entire book rather than when having just read the relevant section is better for testing memory recall anyway.

🦀 (Used my cheat sheets or official API reference only) 🦀

The official website for Rustlings is here:

- [Official Rustlings Page](https://rustlings.rust-lang.org/)
- [Rustlings Git](https://github.com/rust-lang/rustlings)

#### UPDATE: FINISHED
![Finished Screenshot](./images/rust-images/Screenshot%20from%202026-10-16%2021-02-17.png)
<!-- </details> -->

---

### Step 2 - Network Security Applications Review/Assignments in Rust

<table cellpadding="0" cellspacing="0"  bgcolor="#e9a700"   border="1"
  frame="box"   rules="none">
  <tr>
    <td width="66"  valign="middle"  cellpadding="0"
  cellspacing="0">
      <img
        src="./images/rust-images/rust-panics.svg"
        width="40"
        height="40"
        alt="Rust - Does note compile crab"
        align="right"
      >
    </td>
    <td>
        <font color="#111">
    <strong>
  Currently in progress
</strong>
</font>
    </td>
  </tr>
</table>

Ok - so I'm choosing this as next step for a few reasons:
- The game demos I'm wanting to make in the future require interesting multiplayer dynamics - so networking in Rust is paramount to the planning process later
- I have a personal private project which includes socket programming (I'll discuss more in the ReadMe for this project)
- Socket programming is a low-level programming practice with good use of advance techniques around concurrency, parallelism, distributed computing concerns - and so if often done in C in the wild (but Rust is better and is a direct competitor).
- This skill and these demos expand into games, embedded programming (such as some electronics I'd like to communicate with with), and other useful Rust avenues for the future.

Essential this leg of the project will constitute me reviewing all the documentation, and completing all my favorite assignments from the Network Security Applications specialty courses as part of the BScACS Degree at BCIT - but in Rust!

And so this will be my first 'project' or application set with Rust after familiarizing myself with the language (and reviewing all my NetSet Course materials).

For this project, I have created a specific project ReadMe here:
- [NetSec Review Project](./projects/NetSec-Review/NetSec-Review_ReadMe.md)

And project commenceth!

---

### Flexible Side Quests: 

<table cellpadding="0" cellspacing="0"  bgcolor="#090909"   border="1"
  frame="box"   rules="none">
  <tr>
    <td width="66"  valign="middle"  cellpadding="0"
  cellspacing="0">
      <img
        src="./images/rust-images/rust-not_desired_behavior.svg"
        width="40"
        height="40"
        alt="Rust - Does note compile crab"
        align="right"
      >
    </td>
    <td>
  Clickity-clack, take a quest from this stack!
    </td>
  </tr>
</table>

<details>
<summary>
Side Quests - Details Dropdown
</summary>

> [!Caution]
> BRAINSTORM AHEAD:

#### Side Quest: Test out different targets with Rust

<!-- <table cellpadding="0" cellspacing="0"  bgcolor="#090909"   border="1"
  frame="box"   rules="none">
  <tr>
    <td width="66"  valign="middle"  cellpadding="0"
  cellspacing="0">
      <img
        src="./images/rust-images/rust-not_desired_behavior.svg"
        width="40"
        height="40"
        alt="Rust - Does note compile crab"
        align="right"
      >
    </td>
    <td>
      <strong>A flexible aside</strong>
    </td>
  </tr>
</table> -->

<!-- <details> <summary><strong>Details</strong></summary> -->

Before Starting Bevy - I want to play around (while i practice) to see how difficult it is to build applications on different targets, platforms, and few different frameworks.

- Familiarize using Rust on different targets and frameworks (to better inform future projects)
  - WebAssembly
  - Mobile
  - Embedded - I have a few ESP01 hanging around
  - Desktop (UI) - (There several crates for this)
  - Web

<!-- - Git will be problematic for devising some applications - since I'm a code hoarder at heart and I'll need to focus on orienting much of my actual project work as  demos (or working utils) and need to investigate via these endpoints how I'd like to do that. -->

Incorporate:

- Rust basics and/or saved exercises
- E.g. Perhaps 1 mini application for each target using:
  - at least 1 (or more) sections of the book as priority inspiration.
  - 1 review/assignment/task (algorithm/application/demonstration)
  - (Maybe) - Personal video demonstration (marketing 🙄)

<!-- </details> -->

---

#### Side Quest: Build a series of mini-applications

<table cellpadding="0" cellspacing="0"  bgcolor="#090909"   border="1"
  frame="box"   rules="none">
  <tr>
    <td width="66"  valign="middle"  cellpadding="0"
  cellspacing="0">
      <img
        src="./images/rust-images/rust-not_desired_behavior.svg"
        width="40"
        height="40"
        alt="Rust - Does note compile crab"
        align="right"
      >
    </td>
    <td>
      <strong>A flexible aside</strong>
    </td>
  </tr>
</table>

<!-- <details><summary><strong>See Details (Closed by Default)</strong></summary> -->

Ok - I'm thinking I'm dilly-dallying too much with the textbook and syntax - although I'm getting better I want to build some interesting stuff now. My unfiltered ideas are still in the README_BRAINSTORM, but I'm going to start picking winners and losers and placing them here - this side quest can be a living list if mini-applications and experiments to apply Rust knowledge and build a usable thing or at least something demonstrable. Each is pursued requires at least a 1 page project plan (and I might even do those while judging them):

<!-- Ensure these are links when created - in applications folder -->

- [ ] Build robust multi-thread application template with data passing to mimic a botnet (distributed computing)
- [ ] Build backend webserver for my Unity WebGL multiplayer game demo using UTP-2.0
- [ ] Textual Logic Analyzer for parsing and evaluating writing snippets (using propositional logic)
  - Start with just text parsing as a good Rust project
  - Can identify statements and reverse into correct Q&A formatting for:
- [ ] FlashCards backend - I just want to a system to run through memorization tasks
  - This could likely have overlap with Textual Logic Analyzer

<!-- </details> -->

---

#### Side Quest: Short-Queue of Exercises


<table cellpadding="0" cellspacing="0"  bgcolor="#090909"   border="1"
  frame="box"   rules="none">
  <tr>
    <td width="66"  valign="middle"  cellpadding="0"
  cellspacing="0">
      <img
        src="./images/rust-images/rust-not_desired_behavior.svg"
        width="40"
        height="40"
        alt="Rust - Does note compile crab"
        align="right"
      >
    </td>
    <td>
      <strong>A flexible aside</strong>
    </td>
  </tr>
</table>

<!-- <details><summary><strong>See Details (Closed by Default)</strong></summary> -->

This is a priority (desc) queue of exercises pulled from the brainstorm page that I think are worth doing but not necessarily having a useful deliverable (exercises will be moved out of backlog in Brainstorm readme and placed here):

> [!Tip]
> These might make good blog posts

<!-- Ensure these are links when created - in practice folder -->

- [ ] Write (or find) an acceptable benchmark utility
- [ ] investigate keyword 'union' (It's rarely discussed in the text)
- [ ] Read through [Clippy](https://doc.rust-lang.org/clippy/) - particularly Strict coding styles section
  - It informs generally about safety and idiomatic Rust
- [ ] Test out using Cargo workspaces for my practice or project sections
- [ ] Investigate other common smart pointers

<!-- </details> -->

---

#### Side Quest: Reinforce core Rust API knowledge

<!-- <details><summary><strong>See Details (Closed by Default)</strong></summary> -->

While doing the iterators section of the Rustlings exercises, one of my solutions used the flat_map() method to flatten an array of hashmaps such that I could count the values in all of them across hashmaps in a single concise command. I added it to my notes in [cheat sheet ch13 - iterators](./tutorial/Rust%20Prog%20Lang/ch13/Iterators.md) at end of Super Summary even through the book didn't touch on it because i wanted to example stored for reference.

I may want to similarly extend other Super Summaries with more method investigation of primary APIs. Here's a priority (desc) queue of potential priority areas:

<!-- Ensure docs and links when created - in the notebooks folder -->

- [Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html)

</details>

---

### Step 3 - Physics Simulations with Bevy via 11 Physics course material aka review

<table cellpadding="0" cellspacing="0"  bgcolor="#090909"   border="1"
  frame="box"   rules="none">
  <tr>
    <td width="66"  valign="middle"  >
      <img
          src="./images/rust-images/rust-does_not_compile.svg"
        width="40"
        height="40"
        alt="Rust - Panic crab"
        align="right"
      >
    </td>
    <td>
  Blocked / Pending
    </td>
  </tr>
</table>


To be implemented in new repository (linked from here)

<!-- <details open>
TODO
</details> -->

---

### Step 2 - (Bevy Tutorial OR (My own) Game Engine project?...)

<table cellpadding="0" cellspacing="0"  bgcolor="#090909"   border="1"
  frame="box"   rules="none">
  <tr>
    <td width="66"  valign="middle"  >
      <img
          src="./images/rust-images/rust-does_not_compile.svg"
        width="40"
        height="40"
        alt="Rust - Panic crab"
        align="right"
      >
    </td>
    <td>
  Blocked / Pending
    </td>
  </tr>
</table>


To be implemented in new repository (linked from here)

This would be recreating the work I did in the 4 courses for Network Security Applications, including basics of low-level socket communication, custom reliable protocols with proxy (to mimic poor network communication), and recreating a C2 botnet (with some additions for covert communications - to make it fun).

<!-- <details open>
TODO
</details> -->

---
