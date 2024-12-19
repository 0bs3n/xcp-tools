//! Module containing XCP commands and related data structures.
//!
//! This module defines the XCP protocol commands, response handling, and associated enums
//! for interacting with XCP over CAN bus. It provides traits for encoding commands to CAN
//! frames and decoding responses from CAN frames.

use bitfield::bitfield;

/// Enumeration of XCP command codes based on the XCP Protocol specification.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
#[allow(dead_code)]
pub enum XcpCommandCode {
    Connect = 0xFF,
    Disconnect = 0xFE,
    GetStatus = 0xFD,
    Synch = 0xFC,
    GetCommModeInfo = 0xFB,
    GetId = 0xFA,
    SetRequest = 0xF9,
    GetSeed = 0xF8,
    Unlock = 0xF7,
    SetMta = 0xF6,
    Upload = 0xF5,
    ShortUpload = 0xF4,
    BuildChecksum = 0xF3,
    TransportLayerCmd = 0xF2,
    UserCmd = 0xF1,
    Download = 0xF0,
    DownloadNext = 0xEF,
    DownloadMax = 0xEE,
    ShortDownload = 0xED,
    ModifyBits = 0xEC,
    SetCalPage = 0xEB,
    GetCalPage = 0xEA,
    GetPagProcessorInfo = 0xE9,
    GetSegmentInfo = 0xE8,
    GetPageInfo = 0xE7,
    SetSegmentMode = 0xE6,
    GetSegmentMode = 0xE5,
    CopyCalPage = 0xE4,
    ClearDaqList = 0xE3,
    SetDaqPtr = 0xE2,
    WriteDaq = 0xE1,
    SetDaqListMode = 0xE0,
    GetDaqListMode = 0xDF,
    StartStopDaqList = 0xDE,
    StartStopSynch = 0xDD,
    GetDaqClock = 0xDC,
    ReadDaq = 0xDB,
    GetDaqProcessorInfo = 0xDA,
    GetDaqResolutionInfo = 0xD9,
    GetDaqListInfo = 0xD8,
    GetDaqEventInfo = 0xD7,
    FreeDaq = 0xD6,
    AllocDaq = 0xD5,
    AllocOdt = 0xD4,
    AllocOdtEntry = 0xD3,
    ProgramStart = 0xD2,
    ProgramClear = 0xD1,
    Program = 0xD0,
    ProgramReset = 0xCF,
    GetPgmProcessorInfo = 0xCE,
    GetSectorInfo = 0xCD,
    ProgramPrepare = 0xCC,
    ProgramFormat = 0xCB,
    ProgramNext = 0xCA,
    ProgramMax = 0xC9,
    ProgramVerify = 0xC8,

    #[default]
    Unknown = 0x00,
}

impl XcpCommandCode {
    /// Convert a raw command code to an `XcpCommandCode` enum variant.
    pub fn from_code(code: u8) -> XcpCommandCode {
        match code {
            0xFE => XcpCommandCode::Disconnect,
            0xFD => XcpCommandCode::GetStatus,
            0xFC => XcpCommandCode::Synch,
            0xFB => XcpCommandCode::GetCommModeInfo,
            0xFA => XcpCommandCode::GetId,
            0xF9 => XcpCommandCode::SetRequest,
            0xF8 => XcpCommandCode::GetSeed,
            0xF7 => XcpCommandCode::Unlock,
            0xF6 => XcpCommandCode::SetMta,
            0xF5 => XcpCommandCode::Upload,
            0xF4 => XcpCommandCode::ShortUpload,
            0xF3 => XcpCommandCode::BuildChecksum,
            0xF2 => XcpCommandCode::TransportLayerCmd,
            0xF1 => XcpCommandCode::UserCmd,
            0xF0 => XcpCommandCode::Download,
            0xEF => XcpCommandCode::DownloadNext,
            0xEE => XcpCommandCode::DownloadMax,
            0xED => XcpCommandCode::ShortDownload,
            0xEC => XcpCommandCode::ModifyBits,
            0xEB => XcpCommandCode::SetCalPage,
            0xEA => XcpCommandCode::GetCalPage,
            0xE9 => XcpCommandCode::GetPagProcessorInfo,
            0xE8 => XcpCommandCode::GetSegmentInfo,
            0xE7 => XcpCommandCode::GetPageInfo,
            0xE6 => XcpCommandCode::SetSegmentMode,
            0xE5 => XcpCommandCode::GetSegmentMode,
            0xE4 => XcpCommandCode::CopyCalPage,
            0xE3 => XcpCommandCode::ClearDaqList,
            0xE2 => XcpCommandCode::SetDaqPtr,
            0xE1 => XcpCommandCode::WriteDaq,
            0xE0 => XcpCommandCode::SetDaqListMode,
            0xDF => XcpCommandCode::GetDaqListMode,
            0xDE => XcpCommandCode::StartStopDaqList,
            0xDD => XcpCommandCode::StartStopSynch,
            0xDC => XcpCommandCode::GetDaqClock,
            0xDB => XcpCommandCode::ReadDaq,
            0xDA => XcpCommandCode::GetDaqProcessorInfo,
            0xD9 => XcpCommandCode::GetDaqResolutionInfo,
            0xD8 => XcpCommandCode::GetDaqListInfo,
            0xD7 => XcpCommandCode::GetDaqEventInfo,
            0xD6 => XcpCommandCode::FreeDaq,
            0xD5 => XcpCommandCode::AllocDaq,
            0xD4 => XcpCommandCode::AllocOdt,
            0xD3 => XcpCommandCode::AllocOdtEntry,
            0xD2 => XcpCommandCode::ProgramStart,
            0xD1 => XcpCommandCode::ProgramClear,
            0xD0 => XcpCommandCode::Program,
            0xCF => XcpCommandCode::ProgramReset,
            0xCE => XcpCommandCode::GetPgmProcessorInfo,
            0xCD => XcpCommandCode::GetSectorInfo,
            0xCC => XcpCommandCode::ProgramPrepare,
            0xCB => XcpCommandCode::ProgramFormat,
            0xCA => XcpCommandCode::ProgramNext,
            0xC9 => XcpCommandCode::ProgramMax,
            0xC8 => XcpCommandCode::ProgramVerify,
            _ => XcpCommandCode::Unknown,
        }
    }

    /// Convert an `XcpCommandCode` enum variant to its raw command code.
    pub fn to_code(&self) -> u8 {
        *self as u8
    }
}

/// Enumeration of XCP response codes.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
#[allow(dead_code)]
pub enum XcpResponseCode {
    PositiveResponse = 0xFF,
    NegativeResponse = 0xFE,
    #[default]
    UnknownResponse = 0x00,
}

impl XcpResponseCode {
    /// Convert a raw response code to an `XcpResponseCode` enum variant.
    pub fn from_code(code: u8) -> XcpResponseCode {
        match code {
            0xFF => XcpResponseCode::PositiveResponse,
            0xFE => XcpResponseCode::NegativeResponse,
            _ => XcpResponseCode::UnknownResponse,
        }
    }

    /// Convert an `XcpResponseCode` enum variant to its raw response code.
    pub fn to_code(&self) -> u8 {
        *self as u8
    }
}

/// Trait for XCP commands, providing a method to encode commands into CAN frames.
pub trait XcpCommand {
    fn to_can_frame(&self) -> Vec<u8>;
}

/// Generic structure representing an XCP command frame.
#[derive(Debug)]
pub struct XcpCommandFrame<T: XcpCommand> {
    pub data: T,
}

impl<T: XcpCommand> XcpCommandFrame<T> {
    /// Convert the command frame into a CAN frame data vector.
    pub fn to_can_frame(&self) -> Vec<u8> {
        self.data.to_can_frame()
    }
}

/// XCP "Connect" command structure.
#[derive(Debug)]
pub struct XcpConnectCommand {
    pub mode: ConnectMode,
}

impl XcpCommand for XcpConnectCommand {
    fn to_can_frame(&self) -> Vec<u8> {
        let mut frame_data = Vec::<u8>::new();
        frame_data.push(XcpCommandCode::Connect.to_code());
        frame_data.push(self.mode as u8);
        frame_data
    }
}

/// XCP "Get Seed" command structure.
#[derive(Debug, Copy, Clone)]
pub struct XcpGetSeedCommand {
    pub mode: GetSeedMode,
    pub resource: XcpResourceFlags,
}

impl XcpCommand for XcpGetSeedCommand {
    fn to_can_frame(&self) -> Vec<u8> {
        let mut frame_data = Vec::<u8>::new();
        frame_data.push(XcpCommandCode::GetSeed.to_code());
        frame_data.push(self.mode as u8);
        frame_data.push(self.resource.into());
        frame_data
    }
}

/// Trait for XCP responses, providing a method to decode responses from CAN frames.
pub trait XcpResponse {
    fn from_can_frame(frame: &[u8]) -> Self;
}

/// Generic structure representing an XCP response frame.
#[derive(Debug)]
pub struct XcpResponseFrame<T: XcpResponse> {
    pub data: T,
}

impl<T: XcpResponse> XcpResponseFrame<T> {
    /// Decode a CAN frame into an XCP response frame.
    pub fn from_can_frame(frame: &[u8]) -> Self {
        XcpResponseFrame {
            data: T::from_can_frame(frame),
        }
    }
}

/// XCP "Get Seed" response structure.
#[derive(Debug, Clone)]
pub struct XcpGetSeedResponse {
    pub requested_resource_is_protected: bool,
    pub remaining_length: u8,
    pub seed_data: Vec<u8>,
}

impl XcpResponse for XcpGetSeedResponse {
    fn from_can_frame(can_frame: &[u8]) -> XcpGetSeedResponse {
        XcpGetSeedResponse {
            requested_resource_is_protected: can_frame[1] != 0,
            remaining_length: can_frame[1],
            seed_data: can_frame[2..].into(),
        }
    }
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
            resource: XcpResourceFlags(can_frame[1]),
            comm_mode_basic: XcpCommModeBasic(can_frame[2]),
            max_cto: can_frame[3],
            max_dto: can_frame[4],
            protocol_version: can_frame[5],
            transport_version: can_frame[5],
        }
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

bitfield! {
    /// Flags representing XCP resources.
    #[derive(Copy, Clone)]
    pub struct XcpResourceFlags(u8);
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

impl From<XcpResourceFlags> for u8 {
    fn from(v: XcpResourceFlags) -> u8 {
        v.0 as u8
    }
}

impl From<XcpCommModeBasic> for u8 {
    fn from(v: XcpCommModeBasic) -> u8 {
        v.0 as u8
    }
}
