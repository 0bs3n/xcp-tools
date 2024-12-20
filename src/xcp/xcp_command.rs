//! Module containing XCP commands and related data structures.
//!
//! This module defines the XCP protocol commands, response handling, and associated enums
//! for interacting with XCP over CAN bus. It provides traits for encoding commands to CAN
//! frames and decoding responses from CAN frames.

use crate::xcp::frame::{ XcpCommand, XcpCommandCode, XcpResponse, XcpResponseCode, XcpErrorCode, XcpResponseFrame };

use bitfield::bitfield;

/// XCP "Connect" command structure.
#[derive(Debug)]
pub struct ConnectCommand {
    pub mode: ConnectMode,
}

impl XcpCommand for ConnectCommand {
    fn to_can_frame(&self) -> Vec<u8> {
        let mut frame_data = Vec::<u8>::new();
        frame_data.push(self.get_code().to_code());
        frame_data.push(self.mode as u8);
        frame_data
    }

    fn get_code(&self) -> XcpCommandCode { XcpCommandCode::Connect }
}

/// XCP "Connect" response structure.
#[derive(Debug)]
pub struct ConnectResponse {
    pub resource: XcpResourceFlags,
    pub comm_mode_basic: XcpCommModeBasic,
    pub max_cto: u8,
    pub max_dto: u8,
    pub protocol_version: u8,
    pub transport_version: u8,
}

impl XcpResponse for ConnectResponse {
    fn from_can_frame(can_frame: &[u8]) -> ConnectResponse {
        ConnectResponse {
            resource: XcpResourceFlags::from(can_frame[1]),
            comm_mode_basic: XcpCommModeBasic(can_frame[2]),
            max_cto: can_frame[3],
            max_dto: can_frame[4],
            protocol_version: can_frame[5],
            transport_version: can_frame[5],
        }
    }
}

/// XCP "Get Seed" command structure.
#[derive(Debug, Copy, Clone)]
pub struct GetSeedCommand {
    pub mode: GetSeedMode,
    pub resource: XcpResourceFlags,
}

impl XcpCommand for GetSeedCommand {
    fn to_can_frame(&self) -> Vec<u8> {
        let mut frame_data = Vec::<u8>::new();
        frame_data.push(self.get_code().to_code());
        frame_data.push(self.mode as u8);
        frame_data.push(u8::from(self.resource));
        frame_data
    }

    fn get_code(&self) -> XcpCommandCode { XcpCommandCode::GetSeed }
}

/// XCP "Get Seed" response structure.
#[derive(Debug, Clone)]
pub struct GetSeedResponse {
    pub requested_resource_is_protected: bool,
    pub remaining_length: u8,
    pub seed_data: Vec<u8>,
}

impl XcpResponse for GetSeedResponse {
    fn from_can_frame(can_frame: &[u8]) -> GetSeedResponse {
        GetSeedResponse {
            requested_resource_is_protected: can_frame[1] != 0,
            remaining_length: can_frame[1],
            seed_data: can_frame[2..].into(),
        }
    }
}

/// XCP "Unlock" command structure
#[derive(Debug, Clone)]
pub struct UnlockCommand {
    pub remaining_length: u8,
    pub key_data: Vec<u8>
}

impl XcpCommand for UnlockCommand {
    fn to_can_frame(&self) -> Vec<u8> {
        let mut frame_data = Vec::<u8>::new();
        frame_data.push(self.get_code().to_code());
        frame_data.push(self.remaining_length);
        frame_data.append(&mut self.key_data.clone());
        frame_data
    }

    fn get_code(&self) -> XcpCommandCode { XcpCommandCode::Unlock }
}

#[derive(Debug, Clone)]
pub struct UnlockResponse {
    pub resource: XcpResourceFlags
}

impl XcpResponse for UnlockResponse {
    fn from_can_frame(frame: &[u8]) -> UnlockResponse {
        UnlockResponse { resource: XcpResourceFlags::from(frame[1]) }
    }
}

/// Enumeration for XCP connection modes.
#[derive(Copy, Clone, Debug)]
pub enum ConnectMode {
    Normal,
    UserDefined,
}

/// Enumeration for XCP seed modes.
#[derive(Copy, Clone, Debug)]
pub enum GetSeedMode {
    StartSeed,
    ContinueSeed,
}

pub enum XcpResource {
    CalPage,
    Daq,
    Stim,
    Pgm
}

#[derive(Default, Copy, Clone, Debug)]
pub struct XcpResourceFlags {
    pub cal_page: bool,
    pub daq: bool,
    pub stim: bool,
    pub pgm: bool
}


impl XcpResourceFlags {
}

impl From<XcpResource> for XcpResourceFlags {
    fn from(resource: XcpResource) -> XcpResourceFlags {
        XcpResourceFlags {
            cal_page: match resource { XcpResource::CalPage => true, _ => false },
            daq:      match resource { XcpResource::Daq => true, _ => false },
            stim:     match resource { XcpResource::Stim => true, _ => false },
            pgm:      match resource { XcpResource::Pgm => true, _ => false }
        }
    }
}

impl From<u8> for XcpResourceFlags {
    fn from(val: u8) -> XcpResourceFlags {
        let mut flags = XcpResourceFlags::default();
        if (val & 0x01) != 0 { flags.cal_page = true }
        if (val & 0x04) != 0 { flags.daq = true }
        if (val & 0x08) != 0 { flags.stim = true }
        if (val & 0x10) != 0 { flags.pgm = true }
        flags
    }
}

impl From<XcpResourceFlags> for u8 {
    fn from(flags: XcpResourceFlags) -> u8 {
        (if flags.pgm {0x10} else {0x00}) |
        (if flags.stim {0x08} else {0x00}) |
        (if flags.daq {0x04} else {0x00}) |
        (if flags.cal_page {0x01} else {0x00})
    }
}

bitfield! {
    /// Flags representing XCP resources.
    #[derive(Copy, Clone)]
    pub struct _XcpResourceFlags(u8);
    impl Debug;

    pub cal_page, set_cal_page: 0;
    pub daq, set_daq: 2;
    pub stim, set_stim: 3;
    pub pgm, set_pgm: 4;
}

bitfield! {
    /// Flags representing XCP communication mode.
    #[derive(Copy, Clone)]
    pub struct XcpCommModeBasic(u8);
    impl Debug;

    pub byte_order, set_byte_order: 0;
    pub address_granularity, set_address_granularity: 2, 1;
    pub slave_block_mode, set_slave_block_mode: 6;
    pub optional, set_optional: 7;
}

// impl From<XcpResourceFlags> for u8 {
    // fn from(v: XcpResourceFlags) -> u8 {
        // v.0 as u8
    // }
// }

impl From<XcpCommModeBasic> for u8 {
    fn from(v: XcpCommModeBasic) -> u8 {
        v.0 as u8
    }
}

#[derive(Debug)]
pub struct NegativeResponse {
    pub error_code: XcpErrorCode
}

impl XcpResponse for NegativeResponse {
    fn from_can_frame(frame: &[u8]) -> Self {
        NegativeResponse {
            error_code: XcpErrorCode::from_code(frame[1])
        }
    }
}

pub enum XcpResponseType {
    PositiveConnectResponse(XcpResponseFrame<ConnectResponse>),
    PositiveGetSeedResponse(XcpResponseFrame<GetSeedResponse>),
    PositiveUnlockResponse(XcpResponseFrame<UnlockResponse>),
    NegativeResponse(XcpResponseFrame<NegativeResponse>)
}
