use super::Command;

pub struct Request<'a> {
    /// The CMSIS-DAP command.
    pub command: Command,

    /// The request payload.
    pub data: &'a [u8],
}

impl<'a> Request<'a> {
    /// Returns None if the report is empty
    pub fn from_report(report: &'a [u8]) -> Option<Self> {
        let (command, data) = report.split_first()?;

        let command = (*command).try_into().unwrap_or(Command::Unimplemented);

        Some(Request { command, data })
    }

    fn command_len(&self) -> Option<usize> {
        match self.command {
            Command::DAP_Info => Some(1),
            Command::DAP_HostStatus => Some(2),
            Command::DAP_Connect => Some(1),
            Command::DAP_Disconnect => Some(0),
            Command::DAP_WriteABORT => Some(5),
            Command::DAP_Delay => Some(2),
            Command::DAP_ResetTarget => Some(0),
            Command::DAP_SWJ_Pins => Some(6),
            Command::DAP_SWJ_Clock => Some(4),
            Command::DAP_SWJ_Sequence => {
                let bit_count = usize::from(*(self.data.get(0)?));
                let byte_count = if bit_count == 0 {
                    32
                } else {
                    (bit_count + 7) >> 3
                };
                Some(byte_count + 1)
            }
            Command::DAP_SWD_Configure => Some(1),
            Command::DAP_SWD_Sequence => {
                let sequences = *(self.data.get(0)?);
                let mut data = self.data.get(1..)?;
                let mut total_len = 1;
                for _ in 0..sequences {
                    let info = usize::from(*(data.get(0)?) & 0x3f);
                    let clocks = if (info & 0x3f) == 0 { 64 } else { info & 0x3f };
                    total_len += 1;
                    data = data.get(1..)?;
                    if info & 0x80 == 0 {
                        let bytes = (clocks + 7) >> 3;
                        total_len += bytes;
                        data = data.get(bytes..)?;
                    }
                }
                Some(total_len)
            }
            Command::DAP_SWO_Transport => Some(1),
            Command::DAP_SWO_Mode => Some(1),
            Command::DAP_SWO_Baudrate => Some(4),
            Command::DAP_SWO_Control => Some(1),
            Command::DAP_SWO_Status => Some(0),
            Command::DAP_SWO_ExtendedStatus => Some(1),
            Command::DAP_SWO_Data => Some(2),
            Command::DAP_JTAG_Sequence => {
                let sequences = *(self.data.get(0)?);
                let mut data = self.data.get(1..)?;
                let mut total_len = 1;
                for _ in 0..sequences {
                    let info = usize::from(*(data.get(0)?) & 0x3f);
                    let clocks = if (info & 0x3f) == 0 { 64 } else { info & 0x3f };
                    total_len += 1;
                    data = data.get(1..)?;
                    let bytes = (clocks + 7) >> 3;
                    total_len += bytes;
                    data = data.get(bytes..)?;
                }
                Some(total_len)
            }
            Command::DAP_JTAG_Configure => Some((*(self.data.get(0)?) + 1).into()),
            Command::DAP_JTAG_IDCODE => Some(1),
            Command::DAP_TransferConfigure => Some(5),
            Command::DAP_Transfer => {
                let transfers = *(self.data.get(1)?);
                let mut data = self.data.get(2..)?;
                let mut total_len = 2;
                for _ in 0..transfers {
                    let transfer = data.get(0)?;
                    let is_write = (transfer & 0x02) == 0;
                    let is_match = (transfer & 0x10) != 0;
                    total_len += 1;
                    data = data.get(1..)?;
                    if is_write || is_match {
                        total_len += 4;
                        data = data.get(4..)?;
                    }
                }
                Some(total_len)
            }
            Command::DAP_TransferBlock => {
                let transfers =
                    usize::from(u16::from_le_bytes(self.data.get(1..3)?.try_into().unwrap()));
                let transfer = *(self.data.get(3)?);
                let is_write = (transfer & 0x2) == 0;
                let mut byte_count = 4;
                if is_write {
                    byte_count += 4 * transfers;
                }
                Some(byte_count)
            }
            Command::DAP_TransferAbort => Some(0),
            Command::DAP_ExecuteCommands => Some(self.data.len()),
            Command::DAP_QueueCommands => Some(self.data.len()),
            Command::Unimplemented => Some(self.data.len()),
        }
    }

    /// Consumes the next sub_request worth of bytes and returns the sub Request, if any
    pub fn next_sub_request(&mut self) -> Option<Self> {
        if let Command::DAP_ExecuteCommands = self.command {
            let next = Self::from_report(self.data)?;
            // Add one byte for the command byte that from_report above didn't consume
            self.data = self.data.get((next.command_len()? + 1)..)?;
            Some(next)
        } else {
            None
        }
    }

    /// Consumes the next byte and returns it as a `u8` value.
    pub fn next_u8(&mut self) -> u8 {
        let value = self.data[0];
        self.data = &self.data[1..];
        value
    }

    /// Consumes the next two bytes and returns them as a `u16` value.
    pub fn next_u16(&mut self) -> u16 {
        let value = u16::from_le_bytes(self.data[0..2].try_into().unwrap());
        self.data = &self.data[2..];
        value
    }

    /// Consumes the next four bytes and returns them as a `u32` value.
    pub fn next_u32(&mut self) -> u32 {
        let value = u32::from_le_bytes(self.data[0..4].try_into().unwrap());
        self.data = &self.data[4..];
        value
    }

    /// Consumes the first `count` bytes of the data.
    pub fn consume(&mut self, count: usize) {
        self.data = &self.data[count..];
    }

    /// Returns the remaining data.
    pub fn rest(self) -> &'a [u8] {
        &self.data
    }
}
