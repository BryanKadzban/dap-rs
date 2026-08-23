use num_enum::{IntoPrimitive, TryFromPrimitive};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, TryFromPrimitive, IntoPrimitive, PartialEq, Debug)]
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum StandardCommand {
    // General Commands
    DAP_Info = 0x00,
    DAP_HostStatus = 0x01,
    DAP_Connect = 0x02,
    DAP_Disconnect = 0x03,
    DAP_WriteABORT = 0x08,
    DAP_Delay = 0x09,
    DAP_ResetTarget = 0x0A,

    // Common SWD/JTAG Commands
    DAP_SWJ_Pins = 0x10,
    DAP_SWJ_Clock = 0x11,
    DAP_SWJ_Sequence = 0x12,

    // SWD Commands
    DAP_SWD_Configure = 0x13,
    DAP_SWD_Sequence = 0x1D,

    // SWO Commands
    DAP_SWO_Transport = 0x17,
    DAP_SWO_Mode = 0x18,
    DAP_SWO_Baudrate = 0x19,
    DAP_SWO_Control = 0x1A,
    DAP_SWO_Status = 0x1B,
    DAP_SWO_ExtendedStatus = 0x1E,
    DAP_SWO_Data = 0x1C,

    // JTAG Commands
    DAP_JTAG_Sequence = 0x14,
    DAP_JTAG_Configure = 0x15,
    DAP_JTAG_IDCODE = 0x16,

    // Transfer Commands
    DAP_TransferConfigure = 0x04,
    DAP_Transfer = 0x05,
    DAP_TransferBlock = 0x06,
    DAP_TransferAbort = 0x07,

    // Atomic Commands
    DAP_ExecuteCommands = 0x7F,
    DAP_QueueCommands = 0x7E,

    // Unimplemented Command Response
    Unimplemented = 0xFF,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Command {
    Standard(StandardCommand),
    Vendor(u8),
}

impl TryFrom<u8> for Command {
    type Error = <StandardCommand as TryFrom<u8>>::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value >= 0x80 && value <= 0x9f {
            Ok(Self::Vendor(value))
        } else {
            StandardCommand::try_from(value).map(|sc| Self::Standard(sc))
        }
    }
}

impl From<Command> for u8 {
    fn from(value: Command) -> u8 {
        match value {
            Command::Standard(cmd) => cmd.into(),
            Command::Vendor(id) => id + 0x80,
        }
    }
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, IntoPrimitive, Debug)]
#[repr(u8)]
pub enum ResponseStatus {
    DapOk = 0x00,
    DapError = 0xFF,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, TryFromPrimitive, Debug)]
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum DapInfoID {
    VendorID = 0x01,
    ProductID = 0x02,
    SerialNumber = 0x03,
    FirmwareVersion = 0x04,
    TargetVendor = 0x05,
    TargetName = 0x06,
    Capabilities = 0xF0,
    TestDomainTimer = 0xF1,
    SWOTraceBufferSize = 0xFD,
    MaxPacketCount = 0xFE,
    MaxPacketSize = 0xFF,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, TryFromPrimitive, Debug)]
#[repr(u8)]
pub enum HostStatusType {
    Connect = 0,
    Running = 1,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
pub enum HostStatus {
    Connected(bool),
    Running(bool),
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, TryFromPrimitive, Debug)]
#[repr(u8)]
pub enum ConnectPort {
    Default = 0,
    SWD = 1,
    JTAG = 2,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
#[repr(u8)]
pub enum ConnectPortResponse {
    Failed = 0,
    SWD = 1,
    JTAG = 2,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
pub enum DapMode {
    SWD,
    JTAG,
}
