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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum XcpErrorCode {
    /// Command processor synchronization.
    ErrCmdSynch = 0x00,

    /// Command was not executed.
    ErrCmdBusy = 0x10,

    /// Command rejected because DAQ is running.
    ErrDaqActive = 0x11,

    /// Command rejected because PGM is running.
    ErrPgmActive = 0x12,

    /// Unknown command or not implemented optional command.
    ErrCmdUnknown = 0x20,

    /// Command syntax invalid.
    ErrCmdSyntax = 0x21,

    /// Command syntax valid but command parameter(s) out of range.
    ErrOutOfRange = 0x22,

    /// The memory location is write protected.
    ErrWriteProtected = 0x23,

    /// The memory location is not accessible.
    ErrAccessDenied = 0x24,

    /// Access denied, Seed & Key is required.
    ErrAccessLocked = 0x25,

    /// Selected page not available.
    ErrPageNotValid = 0x26,

    /// Selected mode not available.
    ErrModeNotValid = 0x27,

    /// Selected segment not valid.
    ErrSegmentNotValid = 0x28,

    /// Sequence error.
    ErrSequence = 0x29,

    /// DAQ configuration not valid.
    ErrDaqConfig = 0x2A,

    /// Memory overflow error.
    ErrMemoryOverflow = 0x30,

    /// Generic error.
    ErrGeneric = 0x31,

    /// The slave internal program verify routine detects an error.
    ErrVerify = 0x32,

    /// Access to the requested resource is temporary not possible.
    ErrResourceTemporaryNotAccessible = 0x33,

    /// Unknown sub command or not implemented optional sub command.
    ErrSubcmdUnknown = 0x34,

    /// Fake error code representing an unknown error
    #[default]
    ErrUnknown = 0xFF
}


impl XcpErrorCode {
    /// Converts a u8 code to an XcpError.
    pub fn from_code(code: u8) -> Self {
        match code {
            0x00 => XcpErrorCode::ErrCmdSynch,
            0x10 => XcpErrorCode::ErrCmdBusy,
            0x11 => XcpErrorCode::ErrDaqActive,
            0x12 => XcpErrorCode::ErrPgmActive,
            0x20 => XcpErrorCode::ErrCmdUnknown,
            0x21 => XcpErrorCode::ErrCmdSyntax,
            0x22 => XcpErrorCode::ErrOutOfRange,
            0x23 => XcpErrorCode::ErrWriteProtected,
            0x24 => XcpErrorCode::ErrAccessDenied,
            0x25 => XcpErrorCode::ErrAccessLocked,
            0x26 => XcpErrorCode::ErrPageNotValid,
            0x27 => XcpErrorCode::ErrModeNotValid,
            0x28 => XcpErrorCode::ErrSegmentNotValid,
            0x29 => XcpErrorCode::ErrSequence,
            0x2A => XcpErrorCode::ErrDaqConfig,
            0x30 => XcpErrorCode::ErrMemoryOverflow,
            0x31 => XcpErrorCode::ErrGeneric,
            0x32 => XcpErrorCode::ErrVerify,
            0x33 => XcpErrorCode::ErrResourceTemporaryNotAccessible,
            0x34 => XcpErrorCode::ErrSubcmdUnknown,
            _ => XcpErrorCode::ErrUnknown
        }
    }

    pub fn to_code(&self) -> u8 {
        *self as u8
    }
}


/// Trait for XCP commands, providing a method to encode commands into CAN frames.
pub trait XcpCommand {
    fn to_can_frame(&self) -> Vec<u8>;
    fn get_code(&self) -> XcpCommandCode;
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
            XcpResponseFrame { data: T::from_can_frame(frame) }
    }
}

