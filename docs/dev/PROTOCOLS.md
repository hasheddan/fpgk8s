# Protocols

The following protocols are involved in programming and communicating with
FPGAs. This document gives a high-level overview of each protocol, as well as
how it is used in the `fpgk8s` project.

## Universal Serial Bus

[Universal Serial Bus (USB)](https://en.wikipedia.org/wiki/USB) is frequently
used as a friendly interface for personal computing machines to communicate with
an FPGA over other protocols. USB consists of a collection of standards that
define everything from protocols to power management. In the context of FPGAs,
an integrated circuit (IC) that can translate USB packets to other protocols
usually sits in between the personal computer and the FPGA (and any other
peripheral ICs).

### libusb

[`libusb`](https://github.com/libusb/libusb) provides a portable interface for
interacting with USB devices. It is designed as an API with pluggable backends
for various operating systems. For example, the linux backend allows for
discovering and interacting with USB devices via some combination of
[`sysfs`](https://man7.org/linux/man-pages/man5/sysfs.5.html),
[`usbfs`](http://www.linux-usb.org/USB-guide/x173.html),
[`udev`](https://wiki.debian.org/udev), and
[`netlink`](https://man7.org/linux/man-pages/man7/netlink.7.html).

After obtaining information about a USB device, we typically are interested in
reading and writing data from and to the device. This [answer on Stack
Exchange](https://unix.stackexchange.com/questions/292933/how-can-i-write-raw-data-to-a-usb-device)
provides a great overview of what communicating with a USB device entails.

`libusb` is currently consumed in `fpgk8s` via the Rust
[`rusb`](https://github.com/a1ien/rusb) crate.
