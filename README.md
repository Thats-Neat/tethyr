<h1 align="center">
    <a style"font-size:large;">Tethyr</a>
</h1>

<div align="center">
  <a href="https://opensource.org/license/mit/">
    <img src="https://img.shields.io/badge/Licence-MIT-blue" alt="continuous integration" style="height: 20px;">
  </a>
  <a>
    <img src="https://img.shields.io/badge/Version-v0.1.0-orange" alt="continuous integration" style="height: 20px;">
  </a>
</div>
<br/>

A modern and dead simple way to connect your Raspberry Pi to a mobile device. With bluetooth tethering you can create a PAN (Personal Area Network) allowing your mobile device to access websites or services locally hosted on the Pi without cables or a router.

## Features
- Simple bluetooth communication using the D-Bus, achieved with the [zbus](https://docs.rs/zbus/latest/zbus/) Rust crate
- Code display to facilitate secure connection
- Setup of pan0 bridge on Raspberry Pi for communication between Pi and mobile device

## Installation
- Download the latest [release](https://github.com/Thats-Neat/tethyr/releases) onto a Raspberry Pi

## Usage
1. Run the pan bridge setup (if needed):
```bash
./setup_pan.sh
```
4. Run the executable:
```bash
./tethyr
```
3. Connect your mobile device to the `tethyr-device` bluetooth device
4. Ensure the code displayed corresponds to the code presented on the Raspberry Pi
5. Set a manual ip on your mobile device to the same subnet as the Pi (ex. 192.168.7.10) with a subnet mask of 255.255.255.0
    - In the default `setup_pan.sh` file the Pi is setup as 192.168.7.1, this is the IP you will use to access hosted websites or services

*In this current implementation WiFi should be turned off on the mobile device*
