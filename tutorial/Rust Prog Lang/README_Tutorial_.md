# The Rust Programming Language - Notes/Guide

_Ok - I'm actually creating this document at chapter 8_

So far I've followed the book closely, completed the exercises, and made a document for notes in each folder for each chapter.

- However, on first read - I started copying too much of the book into my notes (especially when tired lol)
- I also wish now that I had more specific notes about Rust specific knowledge. Since I'm already familiar with other languages, I mostly just noted Rust specific stuff, but I also kept way too much just for posterity sake as i went along.
- Another thing - is there are sections of the text, like in chapter 7 that don't expressly ask you create side projects to test out it's learning objectives, just code examples.

As much as I find this kind of tedious and would rather just start on bevy or other projects - I do want to gain a mastery level knowledge of Rust, and I want to spend a bit more time digging down on the special interest concepts. So I will:

- Create a much smaller cheat-sheet document for each chapter - that i can refer to (for things like ownership, commands, etc.)
- I will go back for chapters that don't have full exercises and create my won lesson to help memorize certain topics
- I will jot down ideas for putting this all together in simple test bed application that explores these topic together.
- And hopefully this will allow me dig down on Rust fundamentals most important to programmers who are not new to programming but maybe new to rust (and perhaps i can do a short video related at the end of this section)

So I will go back for chapters 1-8 for each tasks, I will still do a first read-through for quick notes for each chapter but do all potential exercises to get my head around the concepts if new, and then later i will come back and make cheat sheets for that on second pass (is better for memorization). But I will note Rust specific mentions to make cheat sheets easier later.

## Next Step - Queue - Descending
1. Make my own game engine in Rust - New Repository worthy.
2. ... 

## Personal Chapter exercise competency  
For chapter 9 and beyond - going to ensure perfect practice first time. 
For 1-8 - going to go back and review with cheat-sheet creation and assess. Will mark below whether I'm confident with the learning outcomes - remaining requires review or extra practice.


## Exercise Options
- Create you own version of the walkthroughs in ch07 (Packages, Crates, and Modules)
    - Most of it was common, except it didn't explore the 2 crate types enough, their typical
- I need to write (or find) an acceptable benchmark utility
- Test out different async runtimes 
    - for you server
    - for your pi.
- LeetCode has rust options - you should do 1 a day and add to practice section. 
- Find some Rust language related quizzes online.

## Specific Topics / Notes to review - in context of Rust
- Ownership (elaborate later as going back to do the cheat sheets)
- TDD principles
- Shadowing
- [Zero Cost Abstractions](https://doc.rust-lang.org/beta/embedded-book/static-guarantees/zero-cost-abstractions.html)


## Topics of the book that would be blog worthy (or just worth doing a write up to contextualize myself)
- Rusts version of Polymorphism (bounded parametric polymorphism) using trait bounds.



## App/Objective/Project backlog for Rust learning (more serious then exercises)

Just brainstorming here. 

Methodologies:
    - Pick favorite textbooks to dig into (Algorithms)
    - Test Rust Paradigms (Bevy, WebAssembly, embedded) 
        - At least touch on each briefly
    - Pick at platform: web, micro-controller (raspberry, esp), gpu
    - Portfolio booster

### Portfolio Website - With demos, videos - in progress
- Build backend server in Rust for my Unity WebGL multiplayer demo (for portfolio) 

### Bevy
- Bevy - Create simulations using bevy for every chapter of the Physics 11 (upgrading course) from BCIT
    - preferably before I take the (12 one) which is offered free in winter semester
- Do initial Bevy tutorial
    - "The Impatient Programmer's Guide to Bevy and Rust" looks good but it's also only free up to chapter 7. 

### Physics 11 Review - with Bevy Simulations
- One chapter review per week

### Algorithms BCIT courses
- Go over algorithm courses material
    - Redo assignments in Rust

### NetSec / Networking / Socket Programming
- Recreate every assignment from NetSec specialty courses (but in Rust)
- Remote keyboard control of raspberry pi 1 handed keyboard project 
    - multiple computers need to switch between them with wearable keyboard (and Raspberry Pi Zero 2)
- Major Project was C2 BotNet - I'd like to redo this project especially.

### Web or General
- I really want to make a logical statement deconstructing
    - For running text book through:
        - Applying propositional logic analysis
        - Pulling out statements and marking logical strength
        - Have option for reordering statements into questions and answers (for flashcards later)

### Linux OS - Util or App
- Recreate encryption util from NetSec courses
- Create custom app as deep-dive for printing /proc folder updates.

### CompSci Review Topics
- Processes and threads breakdown - 
    - Make Rust program to find boundaries and limits

### WebAssembly 
- [WebAssembly](https://webassembly.org/) aims to execute at native speed by taking advantage of common hardware capabilities available on a wide range of platforms.
    - And one of Rust's strengths is that it can build to this. 
    - Test early because i feel there's some good app ideas to come out of this.


### Mobile App
- Setup starter app for cross compatible app
- Setup starter app for cross compatible game
    - I want to know how easy this is to configure while i plan my game project 
    - be able to estimate the effort and ways around the 30% platform markups.

### Embedded
- I have ESPs (I usually just still with raspberry pi) but could use one for testing my keyboard socket program.
- Overlaps with [The Embedded Rust Book](https://doc.rust-lang.org/beta/embedded-book/)

### Books worth doing a Rust related walkthrough
- [The Embedded Rust Book](https://doc.rust-lang.org/beta/embedded-book/)
    - My keyboard project above for raspberry pi Zero 2 would pair well with this. 

### Raspberry Pi kit - Central Repository Kit
- Make cargo crate for added GPIO support