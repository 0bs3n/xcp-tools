use std::fmt::Debug;
use crate::xcp::xcp_command::{
    ConnectCommand, ConnectResponse, ConnectMode, 
    GetSeedCommand, GetSeedResponse, GetSeedMode,
    UnlockCommand, UnlockResponse,
    NegativeResponse,
    XcpResourceFlags
};
use crate::xcp::frame::{XcpCommandFrame, XcpCommand, XcpResponseFrame, XcpResponse, XcpResponseCode };
use socketcan::{CanSocket, CanFrame, Socket, EmbeddedFrame, StandardId, BlockingCan, Id};

pub struct XcpMaster<'a> {
    pub tx_id: u32,
    pub rx_id: u32,
    pub max_cto: usize,
    pub max_dto: usize,
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
    pub fn connect(&mut self, mode: ConnectMode) -> Result<XcpResponseFrame<ConnectResponse>, XcpResponseFrame<NegativeResponse>> {
        let mut connect_req = XcpCommandFrame {
            data: ConnectCommand { mode },
        };

        let connect_resp = self.send_recv_one_blocking(&mut connect_req, |frame| {
            XcpResponseFrame::<ConnectResponse>::from_can_frame(&frame.data())
        });

        let connect_resp = match connect_resp {
            Ok(resp) => resp,
            Err(err_resp) => {
                println!("{:#?}", err_resp);
                return Err(err_resp);
            }
        };

        println!("{:#?}", connect_resp);
        Ok(connect_resp)
    }

    /// Retrieves the full seed from the XCP server.
    ///
    /// # Arguments
    /// * `resource` - XcpResourceFlags indicating the resource to be unlocked by the seed/key exchange
    ///
    /// # Returns
    /// A vector containing the full seed data.
    pub fn get_seed(&mut self, resource: XcpResourceFlags) -> Result<Vec<u8>, XcpResponseFrame<NegativeResponse>> {
        println!("{:#?}", resource);
        println!("{:#?}", u8::from(resource));
        let mut seed = Vec::<u8>::new();
        let mode = GetSeedMode::StartSeed;

        let mut getseed_req = XcpCommandFrame {
            data: GetSeedCommand { mode, resource },
        };

        'seed_loop: loop {
            let getseed_resp = self.send_recv_one_blocking(&mut getseed_req, |frame| {
                XcpResponseFrame::<GetSeedResponse>::from_can_frame(frame.data())
            });

            let getseed_resp = match getseed_resp {
                Ok(resp) => resp,
                Err(err_resp) => {
                    println!("{:#?}", err_resp);
                    return Err(err_resp);
                }
            };

            println!("{:#?}", getseed_resp);

            seed.append(&mut getseed_resp.data.seed_data.clone());

            if getseed_resp.data.remaining_length as usize == getseed_resp.data.seed_data.len() {
                break 'seed_loop;
            }

            getseed_req.data.mode = GetSeedMode::ContinueSeed;
        }

        Ok(seed)
    }

    /// Sends the XCP Unlock command with a key generated based on 
    /// the provided seed and key algorithm
    ///
    /// # Arguments
    /// * `seed` - The seed retrieved from a call to XcpMaster::get_seed
    /// * `key_algo` - a closure which transforms the seed into the associated
    ///                key based on the specification for a particular XCP slave
    ///
    pub fn unlock<F: Fn(&[u8]) -> Vec<u8>>(&mut self, seed: &[u8], key_algo: F) 
        -> Result<XcpResponseFrame<UnlockResponse>, XcpResponseFrame<NegativeResponse>> {
        let key = key_algo(seed);
        println!("seed: {:x?}, key: {:x?}", seed, key);

        let mut curr_key_idx = 0;
        let mut remaining_len = key.len();

        loop {
            let key_data_capacity = usize::min(self.max_cto - 2, remaining_len);
            let last_idx = curr_key_idx + key_data_capacity;
            let keyslice_range = curr_key_idx..last_idx;
            println!("{:x?}", key[curr_key_idx..last_idx].to_vec());

            let mut unlock_req = XcpCommandFrame {
                data: UnlockCommand { 
                    remaining_length: remaining_len as u8,
                    key_data: key[keyslice_range].to_vec()
                }
            };
            
            let unlock_resp = self.send_recv_one_blocking(&mut unlock_req, |frame| {
                XcpResponseFrame::<UnlockResponse>::from_can_frame(frame.data())
            });

            match unlock_resp {
                Ok(_) => {},
                Err(e) => return Err(e)
            }

            remaining_len -= key_data_capacity;
            curr_key_idx += key_data_capacity;
            println!("{:#x?}", unlock_req);
            if remaining_len == 0 { 
                return unlock_resp;
            }
        };
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
    ) -> Result<XcpResponseFrame<R>, XcpResponseFrame<NegativeResponse>> {
        println!("{:#?}", command);
        let frame_data = command.to_can_frame();
        let id = StandardId::new(self.tx_id as u16).unwrap();
        let frame = CanFrame::new(id, &frame_data).unwrap();
        println!("{:x?}", frame);

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
                if id == self.rx_id { 
                    match XcpResponseCode::from_code(frame.data()[0]) {
                        XcpResponseCode::PositiveResponse => { return Ok(handler(frame)) }
                        XcpResponseCode::NegativeResponse => { 
                            return Err(
                                XcpResponseFrame::<NegativeResponse> { 
                                    data: NegativeResponse::from_can_frame(frame.data()) 
                                }
                            )
                        }
                        XcpResponseCode::UnknownResponse => panic!("unknown XcpResponseCode")
                    }
                }
                else { continue }
            } else {
                panic!("Could not read from the bus");
            }
        }
    }
}
