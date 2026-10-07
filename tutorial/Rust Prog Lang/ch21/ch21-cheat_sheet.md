# Final Project: Building a Multithreaded Web Server

We’ll make a web server that says “Hello!” and looks like Figure 21-1 in a web browser.

Here is our plan for building the web server:

1. Learn a bit about TCP and HTTP.
2. Listen for TCP connections on a socket.
3. Parse a small number of HTTP requests.
4. Create a proper HTTP response.
5. Improve the throughput of our server with a thread pool.

First, the method we’ll use won’t be the best way to build a web server with Rust. 

Community members have published a number of production-ready crates available at crates.io that provide more complete web server and thread pool implementations than we’ll build. 

Because Rust is a systems programming language, we can choose the level of abstraction we want to work with and can go to a lower level than is possible or practical in other languages.

Second, we will not be using async and await here. Building a thread pool is a big enough challenge on its own, without adding in building an async runtime! 

However, we will note how async and await might be applicable to some of the same problems we will see in this chapter. 

Ultimately, as we noted back in Chapter 17, many async runtimes use thread pools for managing their work.

We’ll therefore write the basic HTTP server and thread pool manually so that you can learn the general ideas and techniques behind the crates you might use in the future.

Chapter 21 discusses:
- [Building a Single-Threaded Web Server](./Building_Single-Threaded_Web_Server.md)
- [From Single-Threaded to Multithreaded Server](./Single-Threaded_to_Multithreaded_Server.md)
- [Graceful Shutdown and Cleanup](./Graceful_Shutdown_and_Cleanup.md)

### We could do more here! If you want to continue enhancing this project, here are some ideas:

- Add more documentation to ThreadPool and its public methods.
- Add tests of the library’s functionality.
- Change calls to unwrap to more robust error handling.
- Use ThreadPool to perform some task other than serving web requests.
- Find a thread pool crate on crates.io and implement a similar web server using the crate instead. Then, compare its API and robustness to the thread pool we implemented.

### Summary

You’re now ready to implement your own Rust projects and help with other people’s projects. 