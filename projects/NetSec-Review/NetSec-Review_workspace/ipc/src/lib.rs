// Basic Example
pub mod ipc_00;

// Open input message sending
pub mod ipc_01;

// Multiple clients, threads, and connection data
pub mod ipc_02;

// Server commands to clients, clients await reconnect, logging
pub mod ipc_03;

// Robust connection handling, multicast, data-transfer, 
pub mod ipc_04;

// Simple UnixDatagram example
pub mod ipc_05;

// Using uds (crate) for abstract addr, fd-passing, SOCK_SEQPACKET
pub mod ipc_06;