# Network Security Applications Review/Assignments in Rust

## Project Summary and Background

Ok, so I was officially in the [Applied Computer Science (Games Development Option)](https://www.bcit.ca/programs/applied-computer-science-games-development-option-bachelor-of-science-full-time-867absc/) but because I had completed part of the degree part time, then switch to full-time to take Games courses (only offered full-time) - i had extra spots in my schedule and as a great optimizer and opportunity capitalizer - I took NetSec programming courses as well (out of pocket) from this set:

- [Applied Computer Science (Network Security Applications Development Option](https://www.bcit.ca/programs/applied-computer-science-network-security-applications-development-option-bachelor-of-science-full-time-part-time-867bbsc/#courses)

The 2 programs linked there only have about 5 specialty courses that differ between programs. The latter 3 below are all laddered (aka built on top of each other) with COMP 8505 being the top level course and involved expanding on the C2 password cracking botnet we built in COMP 8005 with more cover capabilities. Anyway here's the links to the courses in case you are curious:

- [COMP 7003 Introduction to Information and Network Security](https://www.bcit.ca/courses/introduction-to-information-and-network-security-comp-7003/)
- [COMP 7005 Computer Networks and Protocols](https://www.bcit.ca/courses/computer-networks-and-protocols-comp-7005/)
- [COMP 8005 Network and Security Applications Development](https://www.bcit.ca/courses/network-and-security-applications-development-comp-8005/)
- [COMP 8505 Special Topics in Network and Security Development](https://www.bcit.ca/courses/special-topics-in-network-and-security-development-comp-8505/)

So anyway - I'm not going to create every assignment from this program - but I do want to gain enough familiarity with using Rust while reviewing the material. I'm basically going to crash course review all the assignments in about a week and see where we're at. Many of the application we built in the courses were about reaffirming the learning outcomes - but don't have no real world practical value to me now.

For example we built a network proxy application that could mimic bad network conditions, drop packets, create duplicate packets, etc, and then had to build our own reliable protocol from scratch. Essentially we built TCP from scratch using UDP incorporating many of the techniques that enable TCP to ... reliable. I probably won't recreate this one simple because i could just use TCP if reliability is a concern. Capiche! ....

But I do want build things custom networking applications using Rust. For example, I have picked up this fancy and extremely low-profile, but still full size, Matios One-Handed keyboard and Raspberry pi Zero 2 W. I'm going to wire this baby up with battery, and make it wireless and wearable (for fun). And then setup a remote protocol to run commands remotely using wifi or bluetooth - perhaps switching between computers on the fly. But that's a future project (built off the back of this review and exercise)

| Matios One-Handed Keyboard                                                  | Raspberry Pi Zero 2 W                                                  |
| --------------------------------------------------------------------------- | ---------------------------------------------------------------------- |
| ![Matios One-Handed Keyboard](../../images/other-images/OIP-3045244786.jpg) | ![Raspberry Pi Zero 2 W](../../images/other-images/OIP-3191171976.jpg) |

## Methodology - for this unit

Ok - So I'm reviewing the learning outcomes and materials from the courses mentioned above - I'm still reluctant to put an course materials on git - and it's too verbose - but I'll drop in fundamental summaries, and write my own assignments based on course materials as I go forward.

| Checklist | Review |
|---|---|
| [x] | My Review - COMP 7003 Introduction to Information and Network Security |
| [x] | My Review - COMP 7005 Computer Networks and Protocols |
| [ ] | My Review - COMP 8005 Network and Security Applications Development |
| [ ] | My Review - COMP 8505 Special Topics in Network and Security Development |

<!-- 
- [Super Summary - COMP 7003 Introduction to Information and Network Security](./comp7003-Summary.md) -->
- [Super Summary - COMP 7005 Computer Networks and Protocols](./comp7005-Summary.md)
<!-- - [My Review - COMP 8005 Network and Security Applications Development](.)
- [My Review - COMP 8505 Special Topics in Network and Security Development](.) -->

Here's where I'm organizing the total potential demo list:
- [My (potential) Demo List](./NetSec-Review_DemoList.md)

## Default/Built-In Tool Chain

- Bluetooth
  - [My hcitool notes](./Tools-Review/hcitool-notes.md)
  - [bluetoothctl guide](https://hadess.github.io/bluez/bluetoothctl.html)



## Rust Project Demo Library

Here is the library I created for it:

- [domain-sockets](./NetSec-Review_workspace/domain-socket/src/lib.rs)
  To test the domain sockets as 2 different processes, I need to run 2 different binary applications:
- [domain-socket-server](./NetSec-Review_workspace/socket-tester/src/bin/domain-socket-server.rs)
- [domain-socket-client](./NetSec-Review_workspace/socket-tester/src/bin/domain-socket-client.rs)\

To run server:

```console
cargo run --bin domain-socket-server
```

To run client:

```console
cargo run --bin domain-socket-client
```
