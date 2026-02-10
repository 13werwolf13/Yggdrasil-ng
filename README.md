# Yggdrasil-ng

A Rust rewrite of the [Yggdrasil Network](https://yggdrasil-network.github.io/) — an early-stage implementation of a fully end-to-end encrypted IPv6 networking protocol.
This project aims to provide a lightweight, self-arranging, and secure mesh network alternative to the original Go implementation.

## Features

- **End-to-end encryption** for all network traffic
- **Self-arranging mesh topology** — nodes automatically discover optimal paths
- **IPv6 native** — provides every node with a unique, cryptographically bound IPv6 address
- **Cross-platform** support (Linux, macOS, Windows, Android, iOS)
- **Lightweight** — minimal resource footprint suitable for embedded devices
- **Rust implementation** — memory safety, performance, and modern tooling

## Building from Source

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable version recommended)
- Cargo (included with Rust)
- C compiler toolchain (for some dependencies)
- `libssl-dev` or equivalent (on Linux, for cryptographic operations)

### Clone the Repository

```bash
git clone https://github.com/Revertron/Yggdrasil-ng.git
cd Yggdrasil-ng
```

### Building the Binaries

#### Build Both Binaries (Release Mode)

```bash
cargo build --release
```

This will produce two binaries in `./target/release/`:
- `yggdrasil` — The main network daemon
- `yggdrasilctl` — Administrative control utility

#### Build Individual Binaries

**Build only the daemon:**
```bash
cargo build --release --bin yggdrasil
```

**Build only the control utility:**
```bash
cargo build --release --bin yggdrasilctl
```

#### Development/Debug Builds

For development purposes with faster compile times (but slower runtime performance):

```bash
cargo build
```

Binaries will be located in `./target/debug/`.

### Cross-Compilation

To build for a different target, use the `--target` flag. For example, for Linux ARM64:

```bash
cargo build --release --target aarch64-unknown-linux-gnu
```

## Installation

After building, you can install the binaries system-wide:

```bash
# Copy binaries to system PATH
sudo cp target/release/yggdrasil /usr/local/bin/
sudo cp target/release/yggdrasilctl /usr/local/bin/

# Or use cargo install for local user installation
cargo install --path .
```

## Usage

### Starting Yggdrasil

Generate a default configuration file:

```bash
yggdrasil --genconf > /etc/yggdrasil.conf
```

Edit the configuration to add peers, then start the daemon:

```bash
sudo yggdrasil -useconffile /etc/yggdrasil.conf
```

Or run with auto-configuration:

```bash
sudo yggdrasil --autoconf
```

### Using yggdrasilctl

The `yggdrasilctl` utility connects to the running daemon's admin socket:

```bash
# Get your node's info
yggdrasilctl getSelf

# List connected peers
yggdrasilctl getPeers

# Show DHT (Distributed Hash Table) entries
yggdrasilctl getDHT

# View session information
yggdrasilctl getSessions

# Check the routing table
yggdrasilctl getSwitchPeers
```

## Configuration

Yggdrasil uses a JSON configuration file. Key settings include:

- **Peers**: Static peers to connect to (TCP or TLS)
- **Listen**: Address to listen for incoming connections
- **AdminSocket**: Path to the admin socket for `yggdrasilctl`
- **NodeInfo**: Optional metadata about your node

Example minimal configuration:

```json
{
  "Peers": [
    "tcp://192.0.2.1:443",
    "tcp://[2001:db8::1]:12345"
  ],
  "Listen": [
    "tcp://[::]:1234"
  ]
}
```

## Development

### Running Tests

```bash
cargo test
```

## Contributing

Contributions are not very welcome! Please don't feel free to submit issues or pull requests.
Ensure your code follows the project's own style guidelines and passes all tests.

## License

This project is licensed under the same terms as the original Yggdrasil project (typically LGPLv3 or similar). See the repository for the full license text.

## Links

- [Yggdrasil Network Official Site](https://yggdrasil-network.github.io/)
- [Original Yggdrasil (Go implementation)](https://github.com/yggdrasil-network/yggdrasil-go)
- [Project Wiki](https://github.com/Revertron/Yggdrasil-ng/wiki)

---

**Note**: This is an experimental implementation. Network protocols and APIs may change. Not recommended for production-critical deployments without thorough testing.