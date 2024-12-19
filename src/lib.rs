//! Main module for XCP communication using CAN bus.
//!
//! This module sets up and manages XCP communication, including
//! connecting to the XCP server and retrieving seed data.

extern crate socketcan;
mod xcp;

use socketcan::{CanSocket, Socket};
use xcp::xcp_command::{ConnectMode, XcpResourceFlags};


/// CAN bus ID for XCP request frames.
const XCP_REQUEST_ID: u32 = 0x00;
/// CAN bus ID for XCP response frames.
const XCP_RESPONSE_ID: u32 = 0x00;




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_connect() {
        let iface = "can0";
        let mut sock: CanSocket = CanSocket::open(&iface).expect("Failed to open socket on interface");

        let mut master = xcp::master::XcpMaster {
            tx_id: XCP_REQUEST_ID,
            rx_id: XCP_RESPONSE_ID,
            socket: &mut sock
        };


        master.connect(ConnectMode::Normal);
    }

    #[test]
    fn test_get_full_seed() {
        let iface = "can0";
        let mut sock: CanSocket = CanSocket::open(&iface).expect("Failed to open socket on interface");

        let mut master = xcp::master::XcpMaster {
            tx_id: XCP_REQUEST_ID,
            rx_id: XCP_RESPONSE_ID,
            socket: &mut sock
        };

        master.connect(ConnectMode::Normal);
        let seed = master.get_seed(XcpResourceFlags(1));
        println!("{:x?}", seed);
    }
}
