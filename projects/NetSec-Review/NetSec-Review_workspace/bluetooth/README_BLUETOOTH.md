# Bluetooth demos crate

It should be noted that Rust is just interfacing with Linux for native capabilities like this.
- the BlueR (Rust) is built off BlueZ (Linux library) as basically a wrapper. 
- However, there's still value in Rust's attention to safety and security - and practicality in building a unified operator - as well as adding value in the pipe via extended protocols. 

## Rust specific Bluetooth crates/API

- Default Linux is Bluez (bluetoothctl (preferred) and hcitool (older))
- [BlueR (Crate)](https://docs.rs/bluer/latest/bluer/)
    - [BlueR (Git)](https://github.com/bluez/bluer)
    - Built on [tokio (Crate)](https://docs.rs/tokio/1.52.3/tokio/) asynchronous runtime. 

## Demo Example Options

[Appendix A - Demo Example Options](#appendix-a---demo-example-options)

## Build notes - for testing raspberry pi
```console
// In order to build for another device you have to install cross.
// uname -m on pi was: aarch64
// podman must be installed
cross build --release --target aarch64-unknown-linux-gnu --bin bluetooth_00_client
```
I also needed to place a Cross.toml in my cargo workspace root - to install the bluetooth package required for BlueR
```toml
[target.aarch64-unknown-linux-gnu]
pre-build = [
    "dpkg --add-architecture $CROSS_DEB_ARCH",
    "apt-get update",
    "apt-get install --assume-yes libdbus-1-dev:$CROSS_DEB_ARCH",
]
```
Also note, env.rs is a file I created in place of .env to add to gitignore to hide my MAC - will probably be present in most examples but absent from git, you can just hardcode a mac if anyone actually runs these locally.

## Example 1 - Initial connection, setup and file transfer.

### Summary

Moving on from IPC, and working towards my Major Project 1, I've  setup my Raspberry Pi Zero 2 W to handing further socket programming examples P2P rather than IPC. First things first - I'm going to manage the connection and do a file transfer via the bluetooth connection. 

### Implementation Notes

- So basic example just to get initial channel working.
    - Manually paired and connected server and pi computers
    - Had to build for the pi specifically (see above)
    - env.rs is a file I created in place of .env to add to gitignore to hide my MAC
    - uses RFCOMM (which is the default modern socket type for PC to PC channel (connection based))
    - my_bluer.rs has a scan function i tested but not important for example 1 
        - It will be the library for a more comprehensive application later.


### New Rust API/Crates
- [tokio (Crate)](https://docs.rs/tokio/1.52.3/tokio/)

### Rust language feature used:

- async/await

### Bluetooth Protocol
- Bluetooth Classic RFCOMM (connection oriented byte stream)

### Source / Outcome

| Role       | Library                                               | Binary                                                       | Description                    |
|------------|-------------------------------------------------------|--------------------------------------------------------------|--------------------------------|
| Client     | [bluetooth_00/client.rs](./src/bluetooth_00/client.rs) | [bluetooth_00_client.rs](./src/bin/bluetooth_00_client.rs)   |                                |
| Server     | [bluetooth_00/server.rs](./src/bluetooth_00/server.rs) | [bluetooth_00_server.rs](./src/bin/bluetooth_00_server.rs)   |                                |
| my_bluer   | [bluetooth_00/my_bluer.rs](./src/bluetooth_00/my_bluer.rs) |                                                              | My Bluetooth utility functions |

![alt text](image.png)
_Shows "Received: Hello from the client!" on server before connection dropped purposefully_


## Appendix A - Demo Example Options

### Tasks
- MACs can be spoofed - add an authentication and restriction. 
    - Basically I want rust to handle all bluetooth control like a proxy server or middleware
        - I want one scenario where I have certs physically transferred to both devices
        - And a normal pairing system that has
            - Normal authenticated pairing (matching keyboard expectations)
                - Establishes cryptographic trust
    - Bluetooth Encryption over the connection
    - Application authentication
- As there are already libraries and various protocols built in and installed on Linux, I'll play around with a few and extend them with extra capabilities like perhaps encryption layer, and perhaps a more custom protocol. The existing protocols I know are:
    - Headset Profile (HSP)
    - Advanced Audio Distribution Profile (A2DP)
    - Serial Port Profile (SPP)
    - (There's also more capabilities listed in the BlueR crate that can utilize LSCAP, RFCOMM sockets, mesh functionality, etc.)
- I also want to emulate a bluetooth keyboard (with one handed keyboard project) - which has the bonus capability of being potentially platform independent aka work on windows/mac, not just Unix.
- Mess with lower power BTE device connection (ESP01 maybe)



Bluer Create Features
- bluetoothd: Enables all functions requiring a running Bluetooth - daemon.
- id: Enables database of assigned numbers.
- l2cap: Enables L2CAP sockets.
- rfcomm: Enables RFCOMM sockets.
- mesh: Enables Bluetooth mesh functionality.
- serde: Enables serialization and deserialization of some data types.


### More application purpose demos
- Setting up file stream between 2 devices
- Setup audio channel via bluetooth
- Setup secure transmission with SSL
- See if you can get input data from wireless headset
    - Galaxy buds
- Try a middle man attack / defense
- Could mess with some offensive security libraries.
