
## NetSec Courses - Rust worthy application list - post review

### COMP 7003 Introduction to Information and Network Security

This unit was more about Linux system review and most of the security stuff was OS based rather than programming. But these are noted:

- Potentially making my own christmas tree attack using Rust to create the packet permutations.
- Manually writing my own packet creator in Rust (or finding API to make them quickly)
- Using rust to run hping3/nmap/ncat commands as part of a more goal oriented pipeline (For I guess the first 2 parts of the chain in the Cyber Kill Chain)
  - It would help me get more familiar with these commands as well (but i would want it to do something the commands themselves cannot - and more useful than script - maybe just work into the botnet at the end for autonomous Reconnaissance)

### 7005

[Art of Linux Programming](http://www.catb.org/esr/writings/taoup/html/)

## Application Demos

I've created a 'cargo workspace' in `NetSec-Review_workspace` that allows me to have several library projects and binary projects together - and they share downloaded dependencies and one target folder.

Anyway - As I'm (re)creating some functional examples in Rust relating to Network Security applications, I'm going to create the various library projects, and then run them in the socket-tester binary program.

### Socket Topic board

Doing the NetSec programming course in Rust will set me up for some cool programs later, I will touch on:

- Domain Sockets aka (IPC: Inter-Process Communication)
- TCP
- UDP
- Bluetooth test
- ESP01 wireless test
- BotNet basic Example
- (There is also ic2 connection using a similar protocol - worth as lease a summary explanation that this extends beyond wifi style networking to circuitry)

### Domain Sockets

Domain sockets allow IPC (Inter-Process Communication) on my Linux system (it's 2 processes communicating bidirectionally on the same computer). It was one of the initial lessons in my NetSec courses to understand that in linux - that everything is a file (files, directories, devices, sockets), and so setting up domain sockets between processes is, from a Linux OS perspective, the same as BSD sockets, fundamentally.

Beyond being a good way to test my custom networking protocols and libraries without having to setup another computer on my network or VMs - it is also functionally useful for certain use-cases for 2 processes communicating over a channel:

- A web server and database run separately.
- A desktop app and background service communicate so the service can keep working even if the app closes.
- A privileged system service can perform a specific operation for a less-privileged app, without giving that app full system access.
- Separate programs written in different languages, or maintained by different teams, can cooperate through a defined message format.