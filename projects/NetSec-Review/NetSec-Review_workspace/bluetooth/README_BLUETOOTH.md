# Bluetooth demos crate

It should be noted that in linux most of the packages for bluetooth are built on top of OS system packages
- like for example BlueR is built off BlueZ as basically a wrapper. 
- However, there's still custom protocol and security stuff to demonstrate. 

## Demo Example Options

- we already have packages default to linux but I would like to use rust anyway to implement my own manager and potentially encryption and setup various protocols - or my own (get more familiar with the tool):
    - Headset Profile (HSP)
    - Advanced Audio Distribution Profile (A2DP)
    - Serial Port Profile (SPP)
    - (There's also more capabilities listed in the BlueR crate that can utilize LSCAP, RFCOMM sockets, mesh functionality, etc.)
- I believe i can also emulate a bluetooth keyboard (with one handed keyboard project) - which would work on any system (windows/mac), not just Unix. And would be easier to connect to PCs I don't own (could experiment)

## More application purpose demos
- Setting up file stream between 2 devices
- Setup audio channel via bluetooth
- Setup secure transmission with SSL
- See if you can get input data from wireless headset
    - Galaxy buds
- Try a middle man attack / defense

## Rust specific Bluetooth crates/API

- Default Linux is Bluez (bluetoothctl (preferred) and hcitool (older))
- [BlueR (Crate)](https://docs.rs/bluer/latest/bluer/)
- [BlueR (Git)](https://github.com/bluez/bluer)
