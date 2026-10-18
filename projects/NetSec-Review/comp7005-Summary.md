# My Review - COMP 7005 Computer Networks and Protocols

## Super Summary (or summary of stuff I'd like to deep dive again)

- Look up researching Validating, sanitizing, and normalizing paths with Rust (Im sure there's crates or standards)
- Reading this (particularly file descriptors section) makes me think it might be fun to try and mess around with trying to break file descriptor or open tables file - or at least reading these resources from a Rust application to see them visually.
- Looking at Rust fs api, looks like locks and shared locks are available for the same file access. (worth a deep dive when doing some fs work - to reread file descriptors section here )
- FYI - each processes file descriptor table is located in /proc/PID/fd/
- 'lsof' can be used to check system-wide file-handle counts
- Domain sockets are utilized in Rust through the [Unix Stream Stuct](https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html) - needs more research though.

### Rust related adaptation potentials

- Research and create a domain socket on same machine between 2 processes using rust - ideally domain socket should be proper Domain socket (low-overhead, High throughput, Low latency, Bidirectional).

## Comp7005 - Course Review

Ok - so this course was the first real specialty course for NetSec, where we built things, and also delved into actually programming network channels.

Again - I'm just reviewing my course materials without sharing here - to review and get some ideas for programs to build with Rust.

This was also a much more detailed look into Linux operating systems, processes/services, and low level network communications. This will be much more relevant to using Rust.

[Art of Linux Programming](http://www.catb.org/esr/writings/taoup/html/)

I'm basically going to skim all the documents from the course and just note anything I forgot that needs review or reference or anything that is worthy of looking at specifically within a Rust application

Topics:

### File System (Unix)

- Inodes - data structure in Unix storing metadata about files/directories, excluding name and content
  - Has a unique inode number, size, ownership, timestamp, location of the data on storage
  - Using inodes allows the file system to access and manipulate files without searching the entire directory structure

Basically this is mostly just Linux basics, filesystems nd commands

- FUSE (Filesystem in Userspace):
  - FUSE enables users to create custom file systems without modifying the kernel.
  - FUSE-based file systems can be mounted by regular users, offering flexibility for various use cases.
- Virtual File Systems:
  - Virtual file systems provide interfaces to access system information or resources.
  - Examples include /proc (process information), /sys (kernel parameters), and /dev (device files).
- Previous Directory Shortcut (-)
  - forgot about this 'cd -"

### Path Manipulations

Path Normalization and Canonicalization

- Converting symbolic links into their ultimate target paths ensures the unique identification of every file and directory.
  - This might be relevant to some future app
- Converting relative paths to absolute paths, based on the current working directory, helps universal identification.

Validate User Input

- Basically checking user paths, usiing regex, and checking length limits, etc.

Path Traversal Attacks

- Path traversal attacks occur when attackers exploit insufficiently secured user input to navigate out of the intended directory, often to access restricted files.
  - Sanitizing:
  - Use an allowlist of acceptable inputs and block everything else.
  - Block or escape special characters like "..", "/", or "\" that can manipulate paths.
- Normalize and Sanitize Input
  - Use proper encoding techniques to handle special characters.
  - Transform paths to their simplest, standard form to prevent obfuscated attacks.

This is worth just researching Validating, sanitizing, and normalizing paths with Rust

### File Descriptors

Ok - now some good review stuff.

- Programs establish file descriptors when requiring file access; these descriptors are closed upon completion.
- Standard input, output, and error streams have corresponding file descriptors.
- Each program receives its own set of file descriptors, ensuring isolation and preventing interference.
- Proper closure of file descriptors is crucial to prevent resource leaks and ensure efficient resource utilization.

Process File Descriptor Table

- Each process maintains a unique file descriptor table, preventing sharing file descriptors between processes.
- It keeps track of the open file descriptors specific to that process and associated information, such as file status flags, file position, and I/O buffer pointers.
- This individual file descriptor table allows each process to maintain its state and modify its file descriptors independently.

Operating System Open File Table

- The operating system maintains the open file table.
- The open file table is a system-wide data structure managed by the operating system.
- Each entry in the open file table represents an open file or I/O resource, regardless of which process has opened it.
- Entries in the open file table store metadata such as file status flags, current file position, and other relevant details.

File Descriptors and Open File Table

- The process's file descriptor table references the open file table
- When a process opens a file, the operating system creates an entry in the open file table representing the opened file.
- The file descriptor acts as an index or handle to access the associated entry in the open file table.

File Descriptors and Process Isolation

- File descriptors are specific to each process and not shared between different processes.
- When the operating system creates a new process, it assigns a unique file descriptor table independent of other processes.

stdin, stdout, and stderr

- The file descriptor numbers 0, 1, and 2 are associated with stdin, stdout, and stderr, respectively.

Entries in the file descriptor table and open file table

- The process can read or write data, change the file position, modify file status flags, and perform other operations using the file descriptor and its associated entry in the open file table.

Separate copies of the file descriptor table

- While multiple processes can have the same file open and share the same entry in the open file table, they have separate file descriptors pointing to that shared entry.
- Each process maintains its state within its file descriptor table, including input/output buffers, file positions, and file status flags.
- The input/output buffers associated with file descriptors actively hold data that the process has read from or written to the file but has yet to consume or flush, respectively.
- Modifications to the state of file descriptors in one process, such as reading data or updating the file position, do not affect the state of file descriptors in other processes.
- This isolation of state ensures that processes can work independently on the same file without interfering with each other's operations or compromising data integrity.

Concurrent Access

- Each file descriptor maintains its individual file offset.
- Therefore, different offsets can exist for the same file if multiple processes each have their descriptors and access the same file.
- If multiple descriptors correspond to the same file, which may occur due to functions like dup() or fork(), they share the same file offset.
- Consequently, any read/write operation by one process might impact the file offset viewed by another process sharing the same descriptor.

Best Practices and Considerations

- Much of the concerns have to do with C usage. However I'll be using APIs for file handling, not interfacing with file descriptors directly

Security Considerations When Working with File Descriptors

- Be mindful of race conditions when working with files. Use file-locking mechanisms where appropriate.
- Check the return values of all file operations to detect and handle failures that could be caused by attacks (e.g., filling up the disk to cause write() to fail).

### Everything is a File

- Unix treats regular files, directories, devices, sockets, and more as files.
- Unix maintains a file descriptor table, with each entry representing an open file or I/O resource.
- Special files like FIFOs (named pipes) and Unix domain sockets are treated as files, facilitating inter-process communication through standard file I/O.
- The "everything is a file" approach simplifies design and programming by providing a consistent abstraction for I/O operations.

Inter-Process Communication (IPC)

- Unix provides mechanisms like named pipes (FIFOs) and Unix domain sockets for IPC.
- These special files allow processes to communicate with each other using standard file I/O operations, providing a convenient and consistent approach for inter-process communication.

### Domain Sockets

- Domain sockets, also known as Unix domain sockets, are an inter-process communication (IPC) mechanism used in Unix-like operating systems.
  - I effectively used IPC when doing process messaging in Rust Book
- With domain sockets, communication occurs through a local file system path, utilizing file-like abstractions for data exchange.
- The path can represent a file in the file system or an abstract namespace in memory.
- Domain sockets employ a file-like interface for communication between processes.
- Processes can utilize familiar file I/O operations, such as reading and writing, to exchange data through the socket.
- Processes can leverage existing file system permissions and access control mechanisms by treating the socket as a file.

Low Overhead

- Domain sockets have minimal processing and memory overhead compared to other IPC mechanisms.
- They transmit data directly between processes without involving network stacks or additional layers.
- This efficiency is achieved by transferring data within the same machine, eliminating the need for network protocols.

High Throughput

- Domain sockets offer fast data transfer between processes, providing a high-bandwidth communication channel.
- They are well-suited for scenarios that involve the exchange of large amounts of data or require real-time communication.

Low Latency

- Communication via domain sockets exhibits low latency, enabling quick and responsive process interactions.
- The absence of network protocols and the local nature of the communication contribute to reduced latency.
- This feature is particularly advantageous in real-time systems or high-performance applications where timely communication is crucial.

Bidirectional Communication

- Unlike pipes, which typically support unidirectional communication and are often limited to a parent-child relationship, domain sockets allow bidirectional communication.
- Processes can send and receive data to/from each other using the same socket, facilitating more versatile and flexible communication patterns.
- This bidirectional nature allows for interactive and collaborative communication between processes.

#### Connection Establishment

Passive Connection Waiting

- After the domain socket listens and accepts incoming connections, it passively waits for new connection requests.
- The listening socket remains open and ready to accept new connections while the accepted connections operate independently.
- The process is in a passive connection, waiting until a new connection request arrives.
  Bidirectional Communication
- The server and the client can send and receive data through their respective sockets, enabling data exchange and interaction.

_it talks about cleanup and shutdown - but this is one the strengths of rust - happens automatically_

#### Use Cases and Best Practices

Local Client-Server Applications

- Client and server processes running on the same machine commonly use domain sockets for communication.
- They provide a fast and efficient means of exchanging data and coordinating activities between client and server components of an application.
- Domain sockets offer a convenient and reliable IPC mechanism for inter-process communication, allowing client and server processes to interact seamlessly.

Security Implications

- When using domain sockets, security considerations revolve around protecting the file system resources associated with the sockets.
- Setting proper file permissions is essential to control access to the socket files, ensuring that only authorized processes can communicate through the sockets.
- Access control mechanisms, such as user and group permissions, can be employed to restrict access to sensitive socket resources.
- It is essential to follow security best practices and ensure that the processes communicating through domain sockets are trustworthy and properly authenticated.

### BSD Sockets

### Portable Types

### Endianness

### sockaddr

### Network Interfaces

### Networking Addressing

### I/O Multiplexing

### Finite State Machines

### libdc_fsm

### UDP

### Reliable Protocol

### Routing

### SSL

### Secure Coding
