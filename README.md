# peer-latency-check

A command-line utility written in Rust that measures TCP connection latency to one or more peers.

The program attempts to establish a TCP connection to each provided `host:port` address and reports the time required to connect or an error if the attempt fails.

---

## Features

- Measure TCP connection latency
- Support for multiple peers in a single run
- Fixed connection timeout (3 seconds)
- Uses only the Rust standard library

---

## Requirements

- Rust (stable toolchain)
- Network access to the target hosts

---

## Installation

Clone the repository and build the binary:

```bash
git clone https://github.com/wdomagalski/peer-latency-check.git
cd peer-latency-check
cargo build --release
```

The compiled binary will be available at: 'target/release/peer-latency-check'.

---

## Usage

```bash
peer-latency-check <host:port> [host:port ...]
```

---

## Example

```bash
peer-latency-check google.com:443 8.8.8.8:53
```
