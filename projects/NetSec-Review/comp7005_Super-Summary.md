# My Super Summary - COMP 7005 Computer Networks and Protocols

Post Review - went though all my notes, programs, and lessons. Won't share the notes - but the outcomes and Rust related bits I'll move here. And basically collected as many ideas for programs to build with NetSec, networking, and OS - in Rust

## Rust related demo potentials

Moved to [DemoList](./NetSec-Review_DemoList.md)

### Book Recommendations:

- Computer Networking: A Top-Down Approach
  - Note: There's links to slides and videos for this in the Reliable Protocol document related.
- [Art of Linux Programming](http://www.catb.org/esr/writings/taoup/html/)

### Rust related libraries to core course concepts

- For FUSE (Filesystem in Userspace) for Rust
  - [fuser (Crate)](https://crates.io/crates/fuser)
- Managing Virtual file system with Rust
  - [vfs](https://crates.io/crates/vfs) — Virtual filesystem abstractions for Rust.
- Domain sockets are utilized in Rust through:
  - [Unix Stream](https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html)
  - [Unix Listener](https://doc.rust-lang.org/std/os/unix/net/struct.UnixListener.html)
  - [UnixDatagram](https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html) - More like UDP domain socket
- BSD sockets in Rust is just:
  - [TcpListener](https://doc.rust-lang.org/std/net/struct.TcpListener.html)
  - [TcpStream](https://doc.rust-lang.org/std/net/struct.TcpStream.html)
  - [UdpSocket](https://doc.rust-lang.org/std/net/struct.UdpSocket.html)
  - [tokio UdpSocket](https://docs.rs/tokio/latest/tokio/net/struct.UdpSocket.html) - Tokio’s asynchronous UDP socket to handle network I/O without blocking a thread.
  - [socket2 API](https://docs.rs/socket2/latest/socket2/) socket using advanced configuration options
- For preventing path traversal attacks
  - [Path](https://doc.rust-lang.org/std/path/struct.Path.html)
    - Just std library - but there are methods on the Component struct like (::Normal and ::CurDir) to help validate.
    - There are other crates that had help with sanitization
    - more research required
- Using rust to check it's own process attributes?
  - It will still be stored as /proc/self/fd
  - There is a tracing crate - that can allow you to record the input, decision, and result.
  - Otherwise it's normal linux system environment commands and OS.
  - [rustix (Crate)](https://docs.rs/rustix/latest/rustix/) is a memory-safe, I/O safe wrapper for system API calls
  - [cap-std](https://crates.io/crates/cap-std) — Capability-based filesystem APIs for directory-scoped file access.
- To make rust work like FIFO pipe:
  - Use normal std::io - and pull in stdin.
- For multiplexing IO operations
  - [Tokio](https://docs.rs/tokio/latest/tokio/)
  - [Mio](https://docs.rs/mio/latest/mio/struct.Poll.html)
- Finite State Machines - in Rust
  - There is no direct equivalent to libdc_fsm in C, but some to check out maybe
  - [statig (crate)](https://crates.io/crates/statig) - works with no-std
  - [sfsm crate](https://crates.io/crates/sfsm) - works with no-std
  - Note this also more practical for Embedded Rust where you can't load the std library - and need to function on lower level using no-std
  - [The Embedded Rust Book](https://docs.rust-embedded.org/book/intro/no-std.html)
- BlueTooth with Bluez on linux
  - Default Linux is Bluez (bluetoothctl(preferred) and hcitool)
  - [BlueR (Crate)](https://docs.rs/bluer/latest/bluer/)
  - [BlueR (Git)](https://github.com/bluez/bluer)
- SSL API
  - [rustls (Cratee)](https://crates.io/crates/rustls) - modern TLS library for Rust
  - [openssl (crate)](https://crates.io/crates/openssl) - OpenSSL bindings (has DTLS for UDP)
  - [tokio-rustls](https://github.com/rustls/tokio-rustls) - for Asynchronous TLS/SSL streams for Tokio using Rustls.
  - [dimpl (Crate)](https://docs.rs/dimpl/latest/dimpl/) - DTLS handshake and record layer used by WebRTC.
  - [webrtc.rs](https://github.com/webrtc-rs/webrtc/) - async-friendly WebRTC
- Secure Coding Practices (Rust and for top of mind)
  - **Dependency vulnerability checks:** [cargo-audit](https://github.com/rustsec/rustsec) — checks dependencies against the RustSec advisory database.
  - **Dependency policy checks:** [cargo-deny](https://github.com/EmbarkStudios/cargo-deny) — checks advisories, licenses, duplicate crate versions, and dependency sources.
  - **Static linting:** [Clippy](https://doc.rust-lang.org/clippy/) — catches common mistakes and suspicious patterns.
  - **TLS:** [rustls](https://github.com/rustls/rustls) — a Rust implementation of TLS.
  - **Password hashing:** [argon2](https://docs.rs/argon2/latest/argon2/) — use Argon2id for password storage rather than a general-purpose hash.
  - **Secret handling:** [secrecy](https://docs.rs/secrecy/latest/secrecy/) — helps avoid accidentally exposing secrets; [zeroize](https://docs.rs/zeroize/latest/zeroize/) — clears sensitive values from memory on drop where possible.
  - **Cryptographic randomness:** [getrandom](https://docs.rs/getrandom/latest/getrandom/) — obtains randomness from the operating system.
  - **Authenticated encryption:** [aes-gcm](https://docs.rs/aes-gcm/latest/aes_gcm/) or [chacha20poly1305](https://docs.rs/chacha20poly1305/latest/chacha20poly1305/).
  - **Constant-time operations:** [subtle](https://docs.rs/subtle/latest/subtle/) — provides constant-time comparisons for supported values.

## Super Note Summary - mostly things actionable rather than informative

### File system

Mostly just tips for myself going forward - best practices, things to be mindful of without rereading the notes. It's not verbose - it's only stuff i don't know or might forget from the other notes i made.

- Path protection
  - validating, sanitizing, normalizing
  - Prevent path traversal attacks
- Files (File Descriptors)
  - Be mindful of race conditions when working with files. Use file-locking mechanisms where appropriate.
  - Check the return values of all file operations to detect and handle failures that could be caused by attacks (e.g., filling up the disk to fail).
- Everything is a File (just a reminder)
  - Unix treats regular files, directories, devices, sockets, and more as files.
- IPC can add abstraction between different programs with
  - Low Overhead
  - High Throughput
  - Low Latency
  - Bidirectional Communication
- Domain Sockets - Security Implications
  - protecting the file system resources associated with the sockets (aka file).
  - Setting proper file permissions is essential to ensuring that only authorized processes
  - Access control mechanisms, such as user and group permissions to restrict access to sensitive socket resources.
- BSD sockets to implement protocols like TCP/IP and UDP/IP.
  - Stream sockets provide a reliable, connection-oriented communication mechanism.
  - Datagram sockets provide an unreliable, connectionless communication mechanism.
  - Raw sockets provide a low-level interface that allows direct access to the network layer.
  - Sequence-packet sockets provide reliable, connection-oriented communication with preserved message boundaries.
  - ServerSide
    1. Create socket
    2. Bind the socket to address and port
    3. Listen to set listening state
    4. Accept connection
    5. Handle the client connections (theads or forks)
    6. Communicate (using socket discriptor - each thead)
    7. Close the client socket and cleanup
  - ClientSide
    1. Create Socket (address family, socket type, protocol)
    2. Connect (server addr and port)
    3. Send/Revr data (or read/write/etc)
    4. Close connection
  - Mutexes can protect shared resources
  - non-blocking sockets introduce challenges
    - Multiplexing can monitor multiple sockets for events without separating threads or processes per socket.
  - Errors: connection failures, timeouts, resource limitations, and protocol-specific errors.
  - Error handling: checking the return values of socket functions for error, taking actions based on the error.
    - Actions e.g.: retrying the operation, closing the socket, logging the error, or notifying the user.
- Portable types (sort of built into rust)
- Endianness
  - Big-Endian (also known as Network Byte Order)
  - rust example is: `let bytes = value.to_be_bytes();`
- Address Families and Specific sockaddr structs
  - AF_UNIX and the sockaddr_un: Unix domain socket using
    file system paths. (e.g., /tmp/socket)
  - AF_INET and the sockaddr_in: IPv4 socket addresses using four octets separated by dots.
  - AF_INET6 and the sockaddr_in6: IPv6 socket addresses using the format of hexadecimal numbers separated by colons.
- Network
  - Security Considerations:
    - Somebody must apply security measures to network interfaces to protect against unauthorized access, spoofing, and other attacks.
    - Firewalls, MAC filtering, encryption, and authentication are common security features.
- IPv6 - I should only be using IPv6 for irl
  - In IPv6, IPsec is a fundamental part of the protocol
  - introduces different addresses: unicast, multicast, and anycast.
  - optional "extension header" can be utilized in IPv6 if needed.
  - Fragmentation
    - In IPv4, routers can fragment packets.
    - In IPv6, the source device manages fragmentation, while the responsibility of reassembling the fragmented packets lies with the destination device.
  - While both protocols support QoS, IPv6 includes a Traffic Class field in the header, making identifying and providing QoS easier.
- DNS
  - DNS uses both TCP and UDP on port 53.
  - Some common record types include:
    - A - Maps a hostname to an IPv4 address
    - AAAA - Maps a hostname to an IPv6 address
    - CNAME - Creates an alias (canonical name) for a hostname
    - MX - Specifies the mail exchange servers for a domain
    - NS - Identifies the authoritative nameservers for a domain
    - PTR - Resolves an IP address to a hostname (reverse DNS lookup)
  - Domain Name System Security Extensions (DNSSEC) add security to DNS responses to prevent common attacks, such as DNS spoofing.
  - DNSSEC enables DNS responses to be signed digitally to ensure their authenticity and integrity.
- I/O Multiplexing - single thread monitor multiple I/O streams
  - Two common modes are edge-triggered and level-triggered
  - epoll - was introduced in Linux to overcome the limitations of select and poll.
  - kqueue - similar to epoll, provides a highly scalable and efficient event notification interface.
- Finite State Machines
  - mealy machines - generate outputs based on input and current state
  - Moore Machines: solely generating the outputs based on the current state.
  - Basically it's good to list the above elements as sets.
    - states for your program
    - inputs/events for your program
    - State transition
    - Initial state
    - Final/Accepting state
  - Then represent all as a State transition table
    - rows as states
    - column as input/even
    - column header as input/outputs
  - Deterministic will have unique next state/input-combo
  - Non-deterministic will have potentially same next states/input-combo

| Current State          | Input                       | Next State             |
| ---------------------- | --------------------------- | ---------------------- |
| Created                | Enough Memory               | Ready to Run in Memory |
| Created                | Not Enough Memory           | Ready to Run Swapped   |
| Ready to Run in Memory | Reschedule Process          | Kernel Running         |
| Ready to Run in Memory | Swap Out                    | Ready to Run Swapped   |
| Ready to Run Swapped   | Swap In                     | Ready to Run in Memory |
| Kernel Running         | Interrupt, Interrupt Return | Kernel Running         |
| Kernel Running         | Exit                        | Zombie                 |
| Kernel Running         | Sleep                       | Asleep in Memory       |
| Kernel Running         | Return                      | User Running           |
| User Running           | Syscall, Interrupt          | Kernel Running         |
| Asleep in Memory       | Wakeup                      | Ready to Run in Memory |
| Asleep in Memory       | Swap Out                    | Sleep, Swapped         |
| Sleep, Swapped         | Wakeup                      | Ready to Run Swapped   |
| Preempted              | Return to User              | User Running           |

- UDP
  - lack of built-in congestion control mechanisms can lead to network congestion.
  - additional protocols (like DTLS) or application-level for security
  - Security Considerations - UDP flood is a Denial of Service (DoS) attack server replies ICMP - sometimes leveraging a network of DNS or NTP servers to overwhelm a target system.
    Strategies for Securing UDP Traffic - Control the rate of traffic to prevent your server from being overwhelmed. - Use intrusion detection systems (IDS) to identify abnormal traffic patterns and take appropriate action. - Configure your firewall to block incoming UDP traffic from unknown sources or on unused ports. - Though UDP does not provide encryption, developers can combine it with protocols like DTLS to ensure secure data transmission. - Use more secure protocols for sensitive data transmission, like DNS over HTTPS (DoH) or DNS over TLS (DoT) for DNS queries.
- Reliable protocol - Communication Channel
  - Imperfect:
    - Data Corruption
    - Data Loss
    - Latency
    - Limited Bandwidth
    - Jitter (variation of delay)
    - Seq numbers
  - Mitigation
    - error detection and correction
      - checksums, CRC (Cyclic Redundancy Check), and parity bits to detect errors during transmission.
      - sometimes error correction algorithms can fix the error without retransmitting.
    - Traffic shaping and congestion control
    - Quality of Service (QoS) mechanisms
    - Retransmissions (Ack packets no received or timeout)
    - Sequence numbers to reassemble order
    - Flow control can manage a slow receiver (buffer overflow)
    - Congestion control algorithms adjust the rate of packet transmission based on network conditions to prevent or manager
    - TCP (Transmission Control Protocol) combine these techniques to provide a reliable communication channel
    - Timeouts (for ACK - before resend)
      - Dynamic Timeout Adjustment - based on analyzing network conditions and adjusting based on RTT(RoundTripTime)
    - Pipelining - when round-trip time (RTT) is long especially
      - techniques
        - go-back-N
        - selective repeat (TCP uses this)
    - carefully manage the reuse of sequence numbers to avoid "wrap-around problem"
      - Helps packet re-ordering, duplicate detection, loss detection.
    - Cumulative acknowledgments (ACKs) are used in networking protocols, such as TCP, to acknowledge the receipt of multiple packets using a single ACK message.
      - reduces the need for sending many ACKs
      - TCP also includes "selective acknowledgments" (SACKs)
    - Flow Control
      - Use "sliding window", depending on the receiver's buffer capacity to increase throughput
- Bluetooth
  - A piconet is a network of devices connected via Bluetooth technology, typically consisting of a single master device and up to seven active slave devices.
  - Multiple piconets can be linked to form a scatternet
  - Frequency Hopping Spread Spectrum (FHSS)
    - involves rapidly switching frequencies within the ISM radio band.
    - minimizes interference and increases the security of the data transmission, as it's more difficult to intercept or disrupt.
  - Common protocols:
    - Headset Profile (HSP) for telephone calls through headsets
    - Advanced Audio Distribution Profile (A2DP) for streaming high-quality audio.
    - Serial Port Profile (SPP) for serial data transfer
  - Classes
    - Class 1 (up to 100 meters)
    - Class 2 (around 10 meters)
    - Class 3 (less than 10 meters).
  - Backwards compatible - but reduced to limits of older versions (security/performance)
  - The authentication usually involves a PIN or a passkey.
    - Newer versions, process is simplified with a confirmation of a displayed number or an automatic process for devices with better security features.
    - For devices without an interface (like headsets), a method called "Just Works" is used, where pairing occurs automatically with minimal user interaction. Usually Bluetooth Low-Energy (BLE) devices.
  - Out of Band (OOB) - using NFC or other technique to authenticate locally
  - After pairing, the devices remember each other's Bluetooth addresses and stored keys
  - BlueZ is the official Linux Bluetooth protocol stack.
    - hcitool is a command-line tool that is part of the BlueZ package.
    - bluetoothctl is another command-line utility for interacting with Bluetooth
      - bluetoothctl is generally the preferred tool in newer distributions.
- Secure Coding (honestly just kept whole chapter to bake into my brain)
  - Check inputs (scrutinize the revolving door)
    - Use input validation techniques to ensure the incoming data has the right length, format, and type.
    - Be extra vigilant with special characters that could be used for SQL injection or other types of attacks.
    - client-side and server-side validation
  - Limit output data to minimum necessary
  - Plan Security from the Start
    - it's an ongoing commitment.
    - Security should be at the forefront when you start sketching your software.
    - Conduct threat modelling to identify possible vulnerabilities and develop mitigation strategies.
    - Embed security checkpoints throughout your development and deployment process.
    - Before declaring your application "done," rigorously test its security features to ensure they withstand various attack scenarios.
  - Authentication and Authorization
    - always hash passwords securely before storing them.
    - Multi-Factor Authentication (MFA)
    - Role-Based Access Control (RBAC)
    - Secure Session Management
        - After a user has gained access, the security checks shouldn't stop.
        - Treat each user session as a temporary, revocable pass to the secure building.
        - Implement session timeout policies and always re-authenticate users before they access sensitive features.
        - Use secure, random session identifiers to ensure that session data is encrypted in transit and at rest.
  - Data Handling and Storage
    - Data Encryption
    - Ensure the use of strong encryption algorithms
    - Whether data is at rest in a database or transit over the network, encryption protects against unauthorized access.
  - Secure File Handling
    - needs to read or write files, handle this action as securely as you would any other sensitive operation.
    - Use secure methods to open, read, and write files.
    - Check the file type, its source, and any associated permissions.
    - avoids vulnerabilities like directory traversal attacks.
    - Always validate that incoming files conform to expected formats
  - Secure Data Storage
    - well-audited storage solutions, whether on-premises or in the cloud.
    - strong access control mechanisms on your databases, and avoid storing sensitive data unless absolutely necessary.
    - If you must store sensitive information, like passwords or personal user data, ensure it's hashed or encrypted and that keys are managed securely.
  - Output Validation
    - ensure that it doesn't contain malicious content or reveal sensitive information.
    - Implement output encoding to protect against vulnerabilities like Cross-Site Scripting (XSS) and always sanitize data displayed to the end user.
  - Network and Communication Security
    - employ encryption protocols like TLS for the data in transit.
    - Always enforce HTTPS over HTTP, and consider technologies like VPNs or secure
    tunnels for particularly sensitive communication.
  - API
    - Strong authentication and authorization mechanisms specific to API usage.
    - Make use of API keys and tokens, but safeguard them diligently.
    - Rate-limit API requests to protect against abuse and meticulously validate all incoming and outgoing data to prevent injection attacks or data leakage.
  - Network Security
    - Use firewalls to control incoming and outgoing network traffic based on an organization's previously determined security policies.
    - Employ intrusion detection systems to identify and combat malicious activities.
    - Segment your network logically to restrict access to parts of your system that don't need to be publicly accessible, and regularly scan for vulnerabilities.
  - Error and Exception Handling
    - Shows generic error messages to the user 
    - log detailed errors securely for internal review
    - secure logging
    - encrypted and securely stored.
    - Logs often contain sensitive information
    - Ensure that only authorized individuals can access these logs
    - always monitor for any unusual activity which could indicate a security issue.
    - Implement real-time monitoring tools to scrutinize your logs and trigger alerts for suspicious activity.
    - This should be both automated and manual. Automated tools can flag known patterns of malicious activity, while manual review can identify new, previously unrecognized
    threats.
    - Ensure that your monitoring tools are configured to focus on your application's most critical assets and vulnerabilities.
  - Memory and Resource Management
    - Buffer Overflows
    -  can cause the program to behave unpredictably or even allowing malicious code execution.
    - To prevent buffer overflows, always validate the data length before processing it.
    - Utilize boundary-aware functions (language based functions) and perform boundary checks to ensure you're not exceeding the buffer size.
    - Memory Management
    - This actually is much less a concern in Rust (but not flawless)
    - Failure to manage memory can result in memory leaks or use-after-free vulnerabilities, compromising system stability or security.
  - Advanced Security Measures
    - Security Headers and Configuration
    - headers, like Content Security Policy (CSP) or Strict-Transport-Security, act as an additional layer of security that instructs the browser on how to behave when interacting with your site.
    - Properly configured, they can prevent clickjacking or data injection attacks.
    - Regularly audit and update these settings to match the current best practices in web security.
  - Rate Limiting
    - you don't want your application's resources overwhelmed by too many requests.
    - Implement rate limiting to restrict the number of API calls or user actions within a specific time frame.
    - This preserves your application's resources and provides a defense against brute-force and denial-of-service attacks.
  - Operational Practices: 
    - Patch Management
    - Data Backup and Recovery
    - Incident Response Plan
    - Social Engineering Awareness
    - Secure Defaults
    - Third-Party Dependencies
    - Regulatory Compliance
    - Education and Training
    - Testing and QA
    - Comments and Documentation

## Deep Dive - Refresh Potentials

- Validating, sanitizing, and normalizing paths with Rust
