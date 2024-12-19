# XCP CAN Communication Library

This Rust library provides tools for communicating with an XCP (Universal Measurement and Calibration Protocol) server over the CAN (Controller Area Network) bus. It implements core functionalities for establishing a connection, sending commands, and processing responses, as defined by the XCP protocol specification.

## Features

- **XCP Protocol Implementation**: Core commands and responses for interacting with XCP servers.
- **CAN Bus Communication**: Use `socketcan` for reading and writing CAN frames.
- **Seed and Key Retrieval**: Support for fetching full seed data from the XCP server.
- **Customizable Commands**: Define and extend XCP commands for specialized use cases.

## Getting Started

### Prerequisites

- **Rust**: Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed.
- **SocketCAN**: Ensure your system supports and is configured for `socketcan` (Linux-based).

### Installation

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
xcp_can = "0.1.0"
```

### Example Usage

Here is an example of connecting to an XCP server and retrieving the seed data:

```rust
use xcp_can::{xcp_connect, xcp_get_full_seed, ConnectMode};
use socketcan::CanSocket;

fn main() {
    let iface = "can0";
    let mut sock: CanSocket = CanSocket::open(&iface).expect("Failed to open socket on interface");

    xcp_connect(&mut sock, ConnectMode::Normal);
    let seed = xcp_get_full_seed(&mut sock);
    println!("Retrieved seed data: {:x?}", seed);
}
```

## Modules

### `lib.rs`

The main module that sets up the library. It provides:

- Connection management with `xcp_connect`.
- Seed retrieval with `xcp_get_full_seed`.
- A utility function for sending and receiving single XCP commands.

### `xcp_commands.rs`

Contains the core definitions for XCP commands, responses, and related data structures:

- Command codes (`XcpCommandCode`) and response codes (`XcpResponseCode`).
- Traits for encoding commands and decoding responses.
- Specialized command and response structures for `Connect`, `GetSeed`, and more.

### `mod.rs`

Serves as the module entry point and re-exports relevant functionality.

## Testing

The library includes unit tests that can be run with:

```bash
cargo test
```

Example tests include:

- Connecting to an XCP server (`test_connect`).
- Retrieving full seed data (`test_get_full_seed`).

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any feature suggestions or bug fixes.

---

Start building robust XCP over CAN communication applications today with this library!


