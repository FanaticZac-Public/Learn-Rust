
## NetSec Courses - Rust worthy application list - post review

### COMP 7003 

This unit was more about Linux system review and most of the security stuff was OS based rather than programming. But these are noted:

- Potentially making my own christmas tree attack using Rust to create the packet permutations.
- Manually writing my own packet creator in Rust (or finding API to make them quickly)
- Using rust to run hping3/nmap/ncat commands as part of a more goal oriented pipeline (For I guess the first 2 parts of the chain in the Cyber Kill Chain)
  - It would help me get more familiar with these commands as well (but i would want it to do something the commands themselves cannot - and more useful than script - maybe just work into the botnet at the end for autonomous Reconnaissance)

### 7005


- Use rust to collect as much inode data as you can - see what you can get from internal via it's own processes file descriptor table, and/or from proc, and from open file table.
- (might just be linux review) but use rust perhaps to collect virtual file systems: proc, dev, etc.
- Experiment with FUSE (linux) never done that before.
- Find or make a Rust library that can ensure path normalization and prevent Path Traversal Attacks for all future programs
- Everything is a file (regular files, directories, symbolic links, character devices, block devices, FIFOs (named pipes), and sockets.)
  - let's dry doing a socket - or using a typical protocol to read information from other device with rust - say keyboard, and perhaps interfere with output device like monitor.
- In Rust create a program that can that can be used like FIFO pipe.
- Research and create a domain socket on same machine between 2 processes using rust - ideally domain socket should be proper Domain socket (low-overhead, High throughput, Low latency, Bidirectional).
- Attempt messing with file permission to test security for domain sockets. (attempt to influence or intercept)
- Try using authentication on the domain sockets
- Do one example of TCP, UDP, Raw socket, Sequence packet socket - if rust will le me.
- Do an example of select/poll equalities in rust
- Do an example of non-blocking without threads/processes per socket
- Complete error actions like retrying the operation, closing the socket, logging the error, or notifying the user.
- Examples from the notes
  - Building a Simple HTTP Server (done in rust book)
  - Building a Chat Client and Server (maybe)
  - Building a UDP-based Time Server (super boring)
- Should do a review on subnetting - and on address types for unicast, multicast, and anycast addresses.
- Experiment with unauthorized access, spoofing, and other attacks.
  - Firewalls, MAC filtering, encryption, and authentication are common security features.
- I like ipv4 because it's simple - but i should shift everything to ipv6 for the IPsec value
- Review and work with I/O multiplexing again
- More of a program design exercise but design a FSM and then implement in Rust (Actually this is perfect project for ESP-01 or other embedded)
- UDP - reminded me - making a peer sharing app would be cool
- Research RTP, SNMP, RIP protocols that use UDP underneath
- Implement encryption with DTLS for security with UDP
- Setup and check checksum for UDP transmission
- Might be cool to test a UDP Amplification Attack using DNS or NTP servers
- It might be worth just creating my network proxy application to test with my own reliable protocol
- Reliable protocol (with UDP) - for games - keep speed, prevent easy cheat
  - Error Detection
  - DTLS
  - Some limited form of congestion check and control with RTT
  - Research Avenue
  - Some sort of faux connection management
- For Bluetooth
  - we already have packages default to linux but I would like to use rust anyway to implement my own manager and potentially encryption and setup various protocols - or my own (get more familiar with the tool):
    - Headset Profile (HSP)
    - Advanced Audio Distribution Profile (A2DP)
    - Serial Port Profile (SPP)
  - (There's also more capabilies listed in the BlueR crate that can utilize LSCAP, RFCOMM sockets, mesh functionality, etc.)
- For secure coding practices - look below and apply

### Bigger demo ideas (that could incorporate more at once)

- HTTPS server again but expanded
- Chat Client
- Setting up my own email server
- Peer Sharing App

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