//! MFRC522 register definitions

/// List of all registers for the MFRC522
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Register {
    // Reserved = 0x00,
    CommandReg = 0x01,
    ComlEnReg = 0x02,
    DivlEnReg = 0x03,
    ComIrqReg = 0x04,
    DivIrqReg = 0x05,
    ErrorReg = 0x06,
    Status1Reg = 0x07,
    Status2Reg = 0x08,
    FIFODataReg = 0x09,
    FIFOLevelReg = 0x0A,
    WaterLevelReg = 0x0B,
    ControlReg = 0x0C,
    BitFramingReg = 0x0D,
    CollReg = 0x0E,
    // Reserved = 0x0F,
    // Reserved = 0x10,
    ModeReg = 0x11,
    TxModeReg = 0x12,
    RxModeReg = 0x13,
    TxControlReg = 0x14,
    TxASKReg = 0x15,
    TxSelReg = 0x16,
    RxSelReg = 0x17,
    RxThresholdReg = 0x18,
    DemodReg = 0x19,
    // Reserved = 0x1A,
    // Reserved = 0x1B,
    MfTxReg = 0x1C,
    MfRxReg = 0x1D,
    // Reserved = 0x1E,
    SerialSpeedReg = 0x1F,
    // Reserved = 0x20,
    CRCResultRegHigh = 0x21,
    CRCResultRegLow = 0x22,
    // Reserved = 0x23,
    ModWidthReg = 0x24,
    // Reserved = 0x25,
    RFCfgReg = 0x26,
    GsNReg = 0x27,
    CWGsPReg = 0x28,
    ModGsPReg = 0x29,
    TModeReg = 0x2A,
    TPrescalerReg = 0x2B,
    TReloadRegHigh = 0x2C,
    TReloadRegLow = 0x2D,
    TCounterValRegHigh = 0x2E,
    TCounterValRegLow = 0x2F,
    // Reserved = 0x30,
    TestSel1Reg = 0x31,
    TestSel2Reg = 0x32,
    TestPinEnReg = 0x33,
    TestPinValueReg = 0x34,
    TestBusReg = 0x35,
    AutoTestReg = 0x36,
    VersionReg = 0x37,
    AnalogTestReg = 0x38,
    TestDAC1Reg = 0x39,
    TestDAC2Reg = 0x3A,
    TestADCReg = 0x3B,
    // Reserved = 0x3C-0x3F,
}

impl From<Register> for u8 {
    #[inline(always)]
    fn from(variant: Register) -> Self {
        variant as _
    }
}

impl Register {
    const R: u8 = 1 << 7;
    const W: u8 = 0 << 7;

    pub fn read_address(&self) -> u8 {
        ((*self as u8) << 1) | Self::R
    }

    pub fn write_address(&self) -> u8 {
        ((*self as u8) << 1) | Self::W
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
/// List of different commands for the MFRC522
/// with their *code* as value.
pub enum Command {
    /// No action, cancels current command.
    Idle = 0b0000,
    /// Store 25 bytes into the internal buffer.
    Mem = 0b0001,
    /// Generate a 10-byte random ID.
    GenerateRandomId = 0b0010,
    /// Activate the CRC coprocessor.
    CalcCRC = 0b0011,
    /// Transmit data from the FIFO buffer (to the antenna).
    Transmit = 0b0100,
    /// Modify other *CommandReg* bits without affecting the command.
    NoCmdChange = 0b0111,
    /// Activate the receiver circuit.
    Receive = 0b1000,
    /// Transmit data from the FIFO buffer and activate the receiver after transmission.
    Transceive = 0b1100,
    /// Perform MIFARE standard authentication as a reader.
    MFAuthent = 0b1110,
    /// Reset the MFRC522.
    SoftReset = 0b1111,
}

impl From<Command> for u8 {
    #[inline(always)]
    fn from(variant: Command) -> Self {
        variant as _
    }
}

/// CommandReg PowerDown: soft power-down mode entered
pub const POWER_DOWN: u8 = 1 << 4;

/// ComIrqReg: timer decrements the timer value in register TCounterValReg to zero
pub const TIMER_IRQ: u8 = 1 << 0;
/// ComIrqReg: an error bit in ErrorReg is set
pub const ERR_IRQ: u8 = 1 << 1;
/// ComIrqReg: TODO
pub const IDLE_IRQ: u8 = 1 << 4;
/// ComIrqReg: receiver detected the end of a valid data stream
pub const RX_IRQ: u8 = 1 << 5;

/// DivIrqReg: CalcCRC command is active and all data is processed
pub const CRC_IRQ: u8 = 1 << 2;
