//! Main module for XCP communication using CAN bus.
//!
//! This module sets up and manages XCP communication, including
//! connecting to the XCP server and retrieving seed data.

extern crate socketcan;

use socketcan::{CanSocket, CanFrame, Socket, EmbeddedFrame, StandardId, BlockingCan, Id};
use std::{env, fmt::Debug};

mod xcp;
use xcp::xcp_commands::*;

/// CAN bus ID for XCP request frames.
const XCP_REQUEST_ID: u32 = 0x000;
/// CAN bus ID for XCP response frames.
const XCP_RESPONSE_ID: u32 = 0x000;

/// Establishes a connection with the XCP server.
///
/// # Arguments
/// * `sock` - A mutable reference to the CAN socket.
/// * `mode` - The connection mode.
///
/// This function sends a connection request and processes the response.
fn xcp_connect(mut sock: &mut CanSocket, mode: ConnectMode) -> XcpResponseFrame<ConnectResponse> {
    let mut connect_req = XcpCommandFrame {
        data: XcpConnectCommand { mode },
    };

    let connect_resp = send_and_receive_one(&mut connect_req, &mut sock, |frame| {
        XcpResponseFrame::<ConnectResponse>::from_can_frame(&frame.data())
    });
    connect_resp
}

/// Retrieves the full seed from the XCP server.
///
/// # Arguments
/// * `sock` - A mutable reference to the CAN socket.
///
/// # Returns
/// A vector containing the full seed data.
fn xcp_get_full_seed(mut sock: &mut CanSocket) -> Vec<u8> {
    let mut seed = Vec::<u8>::new();
    let mode = GetSeedMode::StartSeed;
    let mut resource = XcpResourceFlags(0);
    resource.set_pgm(true);

    let mut getseed_req = XcpCommandFrame {
        data: XcpGetSeedCommand { mode, resource },
    };

    'seed_loop: loop {
        let getseed_resp = send_and_receive_one(&mut getseed_req, &mut sock, |frame| {
            XcpResponseFrame::<XcpGetSeedResponse>::from_can_frame(frame.data())
        });

        getseed_req.data.mode = GetSeedMode::ContinueSeed;
        seed.append(&mut getseed_resp.data.seed_data.clone());
        if getseed_resp.data.remaining_length as usize == getseed_resp.data.seed_data.len() {
            break 'seed_loop;
        }
    }

    seed
}

/// Sends a single XCP command and waits for a response.
///
/// # Arguments
/// * `command` - The command to send.
/// * `sock` - A mutable reference to the CAN socket.
/// * `handler` - A closure that handles the received response frame.
///
/// # Returns
/// The parsed response frame.
fn send_and_receive_one<
    F: Fn(CanFrame) -> XcpResponseFrame<R>,
    C: XcpCommand + Debug,
    R: XcpResponse,
>(
    command: &mut XcpCommandFrame<C>,
    sock: &mut CanSocket,
    handler: F,
) -> XcpResponseFrame<R> {
    println!("{:#?}", command);
    let frame_data = command.to_can_frame();
    let id = StandardId::new(XCP_REQUEST_ID as u16).unwrap();
    let frame = CanFrame::new(id, &frame_data).unwrap();

    match sock.transmit(&frame) {
        Ok(_) => (),
        Err(e) => println!("Failed to transmit frame! Error: {}", e),
    };

    loop {
        if let Ok(frame) = sock.read_frame() {
            let id = match frame.id() {
                Id::Standard(id) => id.as_raw() as u32,
                Id::Extended(id) => id.as_raw(),
            };
            match id {
                XCP_RESPONSE_ID => return handler(frame),
                _ => (),
            }
        } else {
            panic!("Could not read from the bus");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_connect() {
        let iface = "can0";
        let mut sock: CanSocket = CanSocket::open(&iface).expect("Failed to open socket on interface");

        xcp_connect(&mut sock, ConnectMode::Normal);
    }

    #[test]
    fn test_get_full_seed() {
        let iface = "can0";
        let mut sock: CanSocket = CanSocket::open(&iface).expect("Failed to open socket on interface");

        xcp_connect(&mut sock, ConnectMode::Normal);
        let seed = xcp_get_full_seed(&mut sock);
        println!("{:x?}", seed);
    }
}
