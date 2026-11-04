# Bluetooth demos crate

It should be noted that Rust is just interfacing with Linux for native capabilities like this.

- the BlueR (Rust) is built off BlueZ (Linux library) as basically a wrapper.
- However, there's still value in Rust's attention to safety and security - and practicality in building a unified operator - as well as adding value in the pipe via extended protocols.

## Rust specific Bluetooth crates/API

- Default Linux is Bluez (bluetoothctl (preferred) and hcitool (older))
- [BlueR (Crate)](https://docs.rs/bluer/latest/bluer/)
  - [BlueR (Git)](https://github.com/bluez/bluer)
  - Built on [tokio (Crate)](https://docs.rs/tokio/1.52.3/tokio/) asynchronous runtime.

## Appendices (Just to keep top example focused)

- [Appendix A - Demo Example Options](#appendix-a---demo-example-options)
- [Appendix B - Bluetooth Resources](#appendix-b---bluetooth-resources)

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

> [!Note] env.rs is a file I created in place of .env to add to gitignore to hide my MAC - will probably be present in most examples but absent from git, you can just hardcode a mac if anyone actually runs these locally.

> [!Important] One thing that makes bluetooth connections different than wifi - is that after pairing with the network - you don't maintain that connection - it only become live whenever a service requests it. On the Pi with graphical interface it stays up (once computers are connected) - but in headless mode - these examples below - will only connect to run the service and if the client process ends, so will the connection.

## Example 1 - Initial connection, setup and message transfer.

### Summary

Moving on from IPC, and working towards my Major Project 1, I've setup my Raspberry Pi Zero 2 W to handing further socket programming examples P2P rather than IPC. First things first - I'm going to manage the connection and do a message transfer via the bluetooth connection.

### Implementation Notes

- So basic example just to get initial channel working.
  - Manually paired and connected server and pi computers
  - Had to build for the pi specifically (see above)
  - env.rs is a file I created in place of .env to add to gitignore to hide my MAC
  - uses RFCOMM (which is the default modern socket type for PC to PC channel (connection based))
  - my_bluer.rs has a scan function I tested but not important for example 1
    - It will be the library for a more comprehensive application later.

### New Rust API/Crates

- [tokio (Crate)](https://docs.rs/tokio/1.52.3/tokio/)

### Rust language feature used:

- async/await

### Bluetooth Protocol

- Bluetooth Classic RFCOMM (connection oriented byte stream)

### Source / Outcome

| Role     | Library                                                    | Binary                                                     | Description                    |
| -------- | ---------------------------------------------------------- | ---------------------------------------------------------- | ------------------------------ |
| Client   | [bluetooth_00/client.rs](./src/bluetooth_00/client.rs)     | [bluetooth_00_client.rs](./src/bin/bluetooth_00_client.rs) |                                |
| Server   | [bluetooth_00/server.rs](./src/bluetooth_00/server.rs)     | [bluetooth_00_server.rs](./src/bin/bluetooth_00_server.rs) |                                |
| my_bluer | [bluetooth_00/my_bluer.rs](./src/bluetooth_00/my_bluer.rs) |                                                            | My Bluetooth utility functions |

![alt text](image.png)
_Shows "Received: Hello from the client!" on server before connection dropped purposefully_

![alt text](image-1.png)

_Shows data in transit via wireshark_

### Addendum

In the wireshark readout - there are 14 l2cap protocol source/desc setting up the connection, then 17 rfcomm protocol including the data transfer, and then 4 l2cap for the disconnect (send/rev) requests. There are also controller/host packet including "Encryption Enable: Link Level Encryption is ON (0x01)". Anyway - it indicates we have link level encryption, but as we can see the message in plaintext in wireshark - we know that message is being decrypted after transmission in the blueZ stack.

```console
$ btmgmt info
hci0:   Primary controller
        (shows mac - I removed)
        supported settings: powered connectable fast-connectable discoverable bondable link-security ssp br/edr le advertising secure-conn debug-keys privacy configuration static-addr phy-configuration
        current settings: powered ssp br/edr le secure-conn
hci0:   Configuration options
        supported options: public-address

// ssp: Secure Simple Pairing is enabled for BR/EDR.
// br/edr: Bluetooth Classic is enabled.
// le: Bluetooth Low Energy is enabled.
// secure-conn: Secure Connections is enabled on the controller.
```

It's important to note that the pi and my server are negotiating security based on what each can (or willing) to provide at pairing time. You can see the encryption for each option above here [Understanding Bluetooth Security](https://unam.re/blog/understanding-bluetooth-security-i)

Anyway - Doing a little investigation, my Link Key type for this pairing was showing 4 indicating SSP connection which means unauthenticated (because i didn't want to hook up a keyboard on the pi :P) - Encryption algorithm: P-192 elliptic-curve cryptography. - Authentication: No MITM protection was established during pairing. - Effectively if someone was spoofing my MAC - this could have established a middle man connection during pair possibly (whoops - you can have security or laziness - not both)

Final futureZac notes:

- I'll setup a proper secure connection that will give me P-256 using OOB (Out-Of-Bound) pairing since i have physical access to both devices.
- I'll add some application layer encryption - will ensure my data on the wire would be obfuscated from both a MITM attacker and/or the HCI on either system.
- I will look at forbidding lesser connection types on my keyboard controller for MP1 (or set some restrictions for future use of lesser pairing key types) to harden my final project.

## Example 2 - Protect the pipe!

Alright - so now I just want to add some encryption to the application layer to protect my application from any awesome hackers that may have MITM my poorly paired bluetooth connection (from the last example). Those funny ruffians.

### Implementation Notes

- I'm going to start with an AES256_GCM encryption from the same named package on Rust. 
- I've added new library my_cryptor for the encryption and decryption package. 
- I'm just using nonsense key using Rust's range syntax "[7u8; 32]" for filling 7s as unsigned 8-bit, 32 element array [7, 7, 7, 7, ...] etc, 32 times.
  - This should be a random generated key for real use.

### New Rust API/Crates

- [aes_gcm (Crate)](https://docs.rs/aes-gcm/latest/aes_gcm/)
  - I also looked at this [AES Encryption in Rust] (https://www.rustfaq.org/en/how-to-use-aes-encryption-in-rust/) for added context

### Bluetooth Protocol

| Role       | Library                                                        | Binary                                                     | Description |
| ---------- | -------------------------------------------------------------- | ---------------------------------------------------------- | ----------- |
| Client     | [bluetooth_01/client.rs](./src/bluetooth_01/client.rs)         | [bluetooth_01_client.rs](./src/bin/bluetooth_01_client.rs) |             |
| Server     | [bluetooth_01/server.rs](./src/bluetooth_01/server.rs)         | [bluetooth_01_server.rs](./src/bin/bluetooth_01_server.rs) |             |
| my_bluer   | [bluetooth_01/my_bluer.rs](./src/bluetooth_01/my_bluer.rs)     |                                                            |             |
| my_cryptor | [bluetooth_01/my_cryptor.rs](./src/bluetooth_01/my_cryptor.rs) |                                                            | AES-GSM encrypt/decrypt    |
![alt text](image-4.png)

![alt text](image-2.png)

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
- Let's see if we can setup an automated pairing scheme.

### Notes

- BlueZ already handles Bluetooth pairing and link encryption.
- L2CAP sockets are low-level byte stream sockets
- rfcomm are most common packet style sockets
- mesh: Enables Bluetooth mesh functionality.
  - [Bluetooth - Mesh Protocol](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/MshPRT_v1.1/out/en/index-en.html)
    - Cover's different model types for mesh networks
  - This would be an awesome area of interests for dealing with graph topology of a local phone based multiplayer game.
    - I always think of gameboy p2p links back in the day was so cool, but to be honest in a wireless world, the local networking paradigm is sort of a limitation over LTE, not a feature, in the eyes of average player.

- To beef up security between 2 trusted PCs
  - Establish device identity using independently generated cryptographic certificates.
  - Exchange public keys physically.
  - Authenticate each computer using those keys. (should do that for SSL too)
  - Establish an application-level encrypted channel using modern cryptography.

## Appendix B - Bluetooth Resources

- Bluez - Default/Built-In Tool Chain
  - [bluetoothctl guide](https://hadess.github.io/bluez/bluetoothctl.html) - is generally the preferred tool in newer distributions.
  - [My hcitool notes](./Tools-Review/hcitool-notes.md) - legacy

  - [btmon - Bluetooth monitor](https://hadess.github.io/bluez/btmon.html) - debugging discovery and connection
  - [rctest - RFCOMM testing](https://hadess.github.io/bluez/rctest.html)
  - [l2ping - Send L2CAP echo request and receive answer](https://hadess.github.io/bluez/l2ping.html)
  - [btattach - Attach serial devices to BlueZ stack](https://hadess.github.io/bluez/btattach.html) - Bringing up a board’s Bluetooth module wired over a UART.
  - [isotest - test Isochronous (CIS/BIS) communications on BlueZ](https://hadess.github.io/bluez/isotest.html) - Development or diagnostics for Bluetooth audio transport.
  - [btmgmt](https://hadess.github.io/bluez/btmgmt.html) - Inspecting controller capabilities or changing low-level settings.
    - Not installed by default
      - [ hid2hci - Bluetooth HID to HCI mode switching utility](https://hadess.github.io/bluez/hid2hci.html) - not installed by default
      - [bdaddr - changing the Bluetooth device address](https://hadess.github.io/bluez/bdaddr.html)

- [Bluetooth - Mesh Protocol](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/MshPRT_v1.1/out/en/index-en.html)
