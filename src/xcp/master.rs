use std::fmt::Debug;
use crate::xcp::xcp_command::{
    ConnectCommand, ConnectResponse, ConnectMode, 
    GetSeedCommand, GetSeedResponse, GetSeedMode,
    XcpResourceFlags
};
use crate::xcp::frame::{XcpCommandFrame, XcpCommand, XcpResponseFrame, XcpResponse };
use socketcan::{CanSocket, CanFrame, Socket, EmbeddedFrame, StandardId, BlockingCan, Id};

pub struct XcpMaster<'a> {
    pub tx_id: u32,
    pub rx_id: u32,
    pub socket: &'a mut CanSocket
}

impl<'a> XcpMaster<'a> {
    /// Establishes a connection with the XCP server.
    ///
    /// # Arguments
    /// * `sock` - A mutable reference to the CAN socket.
    /// * `mode` - The connection mode.
    ///
    /// This function sends a connection request and processes the response.
    pub fn connect(&mut self, mode: ConnectMode) -> XcpResponseFrame<ConnectResponse> {
        let mut connect_req = XcpCommandFrame {
            data: ConnectCommand { mode },
        };

        let connect_resp = self.send_recv_one_blocking(&mut connect_req, |frame| {
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
    pub fn get_seed(&mut self, resource: XcpResourceFlags) -> Vec<u8> {
        let mut seed = Vec::<u8>::new();
        let mode = GetSeedMode::StartSeed;

        let mut getseed_req = XcpCommandFrame {
            data: GetSeedCommand { mode, resource },
        };

        'seed_loop: loop {
            let getseed_resp = self.send_recv_one_blocking(&mut getseed_req, |frame| {
                XcpResponseFrame::<GetSeedResponse>::from_can_frame(frame.data())
            });

            seed.append(&mut getseed_resp.data.seed_data.clone());

            if getseed_resp.data.remaining_length as usize == getseed_resp.data.seed_data.len() {
                break 'seed_loop;
            }

            getseed_req.data.mode = GetSeedMode::ContinueSeed;
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
    pub fn send_recv_one_blocking<
        F: Fn(CanFrame) -> XcpResponseFrame<R>,
        C: XcpCommand + Debug,
        R: XcpResponse,
    >(
        &mut self,
        command: &mut XcpCommandFrame<C>,
        handler: F,
    ) -> XcpResponseFrame<R> {
        println!("{:#?}", command);
        let frame_data = command.to_can_frame();
        let id = StandardId::new(self.tx_id as u16).unwrap();
        let frame = CanFrame::new(id, &frame_data).unwrap();

        match self.socket.transmit(&frame) {
            Ok(_) => (),
            Err(e) => println!("Failed to transmit frame! Error: {}", e),
        };

        loop {
            if let Ok(frame) = self.socket.read_frame() {
                let id = match frame.id() {
                    Id::Standard(id) => id.as_raw() as u32,
                    Id::Extended(id) => id.as_raw(),
                };
                if id == self.rx_id { return handler(frame)}
                else { continue }
            } else {
                panic!("Could not read from the bus");
            }
        }
    }
}
