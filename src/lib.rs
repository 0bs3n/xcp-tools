//! Main module for XCP communication using CAN bus.
//!
//! This module sets up and manages XCP communication, including
//! connecting to the XCP server and retrieving seed data.

extern crate socketcan;
mod xcp;

use socketcan::{CanSocket, Socket};
use xcp::xcp_command::{ConnectMode, XcpResource, XcpResourceFlags};

use serial_test::serial;

#[cfg(test)]
mod tests {
    /// CAN bus ID for XCP request frames.
    const XCP_REQUEST_ID: u32 = 0x000;
    /// CAN bus ID for XCP response frames.
    const XCP_RESPONSE_ID: u32 = 0x000;
    use super::*;

    #[test]
    fn unlock() {
        let iface = "can0";
        let mut sock: CanSocket = CanSocket::open(&iface).expect("Failed to open socket on interface");

        let mut master = xcp::master::XcpMaster {
            tx_id: XCP_REQUEST_ID,
            rx_id: XCP_RESPONSE_ID,
            max_cto: 8, // @todo this can be acquired dynamically, better to do it that way
            max_dto: 8,
            socket: &mut sock
        };

        let _ = master.connect(ConnectMode::Normal); // @todo handle negative response to this
        let seed = master.get_seed(XcpResource::Pgm.into());
        /*
        match seed {
            Ok(s) => { 
                println!("got seed: {:x?}", s); 
                let unlock_resp = master.unlock(s.as_slice(), |sa| {sa.to_vec()});
                match unlock_resp {
                    Ok(res) => println!("{:#x?}", res),
                    Err(e) => println!("{:#x?}", e)
                }
            }
            Err(res) => { println!("Response error:\n{:#x?}", res); assert!(false) }
        }
        */
    }

    #[test]
    #[serial]
    fn connect() {
        let iface = "can0";
        let mut sock: CanSocket = CanSocket::open(&iface).expect("Failed to open socket on interface");

        let mut master = xcp::master::XcpMaster {
            tx_id: XCP_REQUEST_ID,
            rx_id: XCP_RESPONSE_ID,
            max_cto: 8,
            max_dto: 8,
            socket: &mut sock
        };

        master.connect(ConnectMode::Normal);
    }

    #[test]
    #[serial]
    fn get_seed() {
        let iface = "can0";
        let mut sock: CanSocket = CanSocket::open(&iface).expect("Failed to open socket on interface");

        let mut master = xcp::master::XcpMaster {
            tx_id: XCP_REQUEST_ID,
            rx_id: XCP_RESPONSE_ID,
            max_cto: 8,
            max_dto: 8,
            socket: &mut sock
        };

        master.connect(ConnectMode::Normal);
        let seed = master.get_seed(XcpResource::Pgm.into());
        match seed {
            Ok(seed) => { println!("got seed: {:x?}", seed); assert!(true) }
            Err(res) => { println!("Response error:\n{:#x?}", res); assert!(false) }
        }
    }

    #[test]
    #[serial]
    fn negative_response() {
        let iface = "can0";
        let mut sock: CanSocket = CanSocket::open(&iface).expect("Failed to open socket on interface");

        let mut master = xcp::master::XcpMaster {
            tx_id: XCP_REQUEST_ID,
            rx_id: XCP_RESPONSE_ID,
            max_cto: 8,
            max_dto: 8,
            socket: &mut sock
        };

        master.connect(ConnectMode::Normal);
        let seed = master.get_seed(XcpResourceFlags::from(0xff));
        match seed {
            Ok(seed) => { println!("got seed: {:x?}", seed); assert!(false) }
            Err(res) => { println!("Response error:\n{:#x?}", res); assert!(true) }
        }
    }
}
