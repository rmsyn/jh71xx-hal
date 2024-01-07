use crate::{bitflag_from_u32, bitflag_is_set};

/// `CON` speed bitfield: standard.
pub const I2C_CON_SPEED_STD: u8 = 0b01;
/// `CON` speed bitfield: fast.
pub const I2C_CON_SPEED_FAST: u8 = 0b10;
/// `CON` speed bitfield: high.
pub const I2C_CON_SPEED_HIGH: u8 = 0b11;
pub const I2C_CON_SPEED_MASK: u8 = 0b11;

/// Represents the I2C `CON` register speed field.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum I2cConSpeed {
    #[default]
    Std = I2C_CON_SPEED_STD,
    Fast = I2C_CON_SPEED_FAST,
    High = I2C_CON_SPEED_HIGH,
}

impl From<u8> for I2cConSpeed {
    fn from(val: u8) -> Self {
        match val & I2C_CON_SPEED_MASK {
            I2C_CON_SPEED_STD => Self::Std,
            I2C_CON_SPEED_FAST => Self::Fast,
            I2C_CON_SPEED_HIGH => Self::High,
            // technically unreachable, but let's make the compiler happy
            _ => Self::Std,
        }
    }
}

impl From<I2cConSpeed> for u8 {
    fn from(val: I2cConSpeed) -> Self {
        val as u8
    }
}

impl From<&I2cConSpeed> for u8 {
    fn from(val: &I2cConSpeed) -> Self {
        (*val).into()
    }
}

/// Represents I2C `CON` register bitfield.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct I2cCon(u32);

bitflags! {
    impl I2cCon: u32 {
        const NONE = 0b0000_0000_0000_0000;
        const MASTER = 0b0000_0000_0000_0001;
        const SPEED_STD = 0b0000_0000_0000_0010;
        const SPEED_FAST = 0b0000_0000_0000_0100;
        const SPEED_HIGH = 0b0000_0000_0000_0110;
        const SLAVE_10BIT = 0b0000_0000_0000_1000;
        const MASTER_10BIT = 0b0000_0000_0001_0000;
        const RESTART_EN = 0b0000_0000_0010_0000;
        const SLAVE_DISABLE = 0b0000_0000_0100_0000;
        const STOP_DET_IFADDRESSED = 0b0000_0000_1000_0000;
        const TX_EMPTY_CTRL = 0b0000_0001_0000_0000;
        const RX_FIFO_FULL_HLD_CTRL = 0b0000_0010_0000_0000;
        const BUS_CLEAR_CTRL = 0b0000_1000_0000_0000;
        const MASK = 0b1011_1111_1111;
    }
}

bitflag_is_set!(I2cCon);
bitflag_from_u32!(I2cCon);

impl From<&I2cConSpeed> for I2cCon {
    fn from(val: &I2cConSpeed) -> Self {
        match val {
            I2cConSpeed::Std => Self::SPEED_STD,
            I2cConSpeed::Fast => Self::SPEED_FAST,
            I2cConSpeed::High => Self::SPEED_HIGH,
        }
    }
}

impl From<I2cConSpeed> for I2cCon {
    fn from(val: I2cConSpeed) -> Self {
        (&val).into()
    }
}

/// Represents I2C `RAW_INTR_STAT` register bitfield.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct I2cRawInterruptStatus(u32);

bitflags! {
    impl I2cRawInterruptStatus: u32 {
        const NONE = 0b0000_0000_0000_0000;
        const RX_UNDER = 0b0000_0000_0000_0001;
        const RX_OVER = 0b0000_0000_0000_0010;
        const RX_FULL = 0b0000_0000_0000_0100;
        const TX_OVER = 0b0000_0000_0000_1000;
        const TX_EMPTY = 0b0000_0000_0001_0000;
        const RD_REQ = 0b0000_0000_0010_0000;
        const TX_ABRT = 0b0000_0000_0100_0000;
        const RX_DONE = 0b0000_0000_1000_0000;
        const ACTIVITY = 0b0000_0001_0000_0000;
        const STOP_DET = 0b0000_0010_0000_0000;
        const START_DET = 0b0000_0100_0000_0000;
        const GEN_CALL = 0b0000_1000_0000_0000;
        const RESTART_DET = 0b0001_0000_0000_0000;
        const MST_ON_HOLD = 0b0010_0000_0000_0000;
        const MASK = 0b0011_1111_1111_1111;
    }
}

bitflag_is_set!(I2cRawInterruptStatus);
bitflag_from_u32!(I2cRawInterruptStatus);

/// Represents I2C `INTR_STAT` register bitfield.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct I2cInterruptStatus(u32);

bitflags! {
    impl I2cInterruptStatus: u32 {
        const NONE = 0b0000_0000_0000_0000;
        const RX_UNDER = 0b0000_0000_0000_0001;
        const RX_OVER = 0b0000_0000_0000_0010;
        const RX_FULL = 0b0000_0000_0000_0100;
        const TX_OVER = 0b0000_0000_0000_1000;
        const TX_EMPTY = 0b0000_0000_0001_0000;
        const RD_REQ = 0b0000_0000_0010_0000;
        const TX_ABRT = 0b0000_0000_0100_0000;
        const RX_DONE = 0b0000_0000_1000_0000;
        const ACTIVITY = 0b0000_0001_0000_0000;
        const STOP_DET = 0b0000_0010_0000_0000;
        const START_DET = 0b0000_0100_0000_0000;
        const GEN_CALL = 0b0000_1000_0000_0000;
        const RESTART_DET = 0b0001_0000_0000_0000;
        const MST_ON_HOLD = 0b0010_0000_0000_0000;
        const MASK = 0b0011_1111_1111_1111;
    }
}

bitflag_is_set!(I2cInterruptStatus);
bitflag_from_u32!(I2cInterruptStatus);

/// Represents I2C `INTR_MASK` register bitfield.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct I2cInterruptMask(u32);

bitflags! {
    impl I2cInterruptMask: u32 {
        const NONE = 0b0000_0000_0000_0000;
        const RX_UNDER = 0b0000_0000_0000_0001;
        const RX_OVER = 0b0000_0000_0000_0010;
        const RX_FULL = 0b0000_0000_0000_0100;
        const TX_OVER = 0b0000_0000_0000_1000;
        const TX_EMPTY = 0b0000_0000_0001_0000;
        const RD_REQ = 0b0000_0000_0010_0000;
        const TX_ABRT = 0b0000_0000_0100_0000;
        const RX_DONE = 0b0000_0000_1000_0000;
        const ACTIVITY = 0b0000_0001_0000_0000;
        const STOP_DET = 0b0000_0010_0000_0000;
        const START_DET = 0b0000_0100_0000_0000;
        const GEN_CALL = 0b0000_1000_0000_0000;
        const RESTART_DET = 0b0001_0000_0000_0000;
        const MST_ON_HOLD = 0b0010_0000_0000_0000;
        const MASK = 0b0011_1111_1111_1111;
    }
}

bitflag_is_set!(I2cInterruptMask);
bitflag_from_u32!(I2cInterruptMask);

impl I2cInterruptMask {
    /// Gets the default mask for `master` operation mode.
    pub fn master() -> Self {
        Self::default() | Self::TX_EMPTY
    }

    /// Gets the default mask for `slave` operation mode.
    pub fn slave() -> Self {
        Self::default() | Self::RX_UNDER | Self::RD_REQ
    }
}

impl Default for I2cInterruptMask {
    fn default() -> Self {
        Self::RX_FULL | Self::TX_ABRT | Self::STOP_DET
    }
}

/// Represents I2C `CLR_INTR` register bitfield.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct I2cClearInterrupt(u32);

bitflags! {
    impl I2cClearInterrupt: u32 {
        const NONE = 0b0000_0000_0000_0000;
        const RX_UNDER = 0b0000_0000_0000_0001;
        const RX_OVER = 0b0000_0000_0000_0010;
        const RX_FULL = 0b0000_0000_0000_0100;
        const TX_OVER = 0b0000_0000_0000_1000;
        const TX_EMPTY = 0b0000_0000_0001_0000;
        const RD_REQ = 0b0000_0000_0010_0000;
        const TX_ABRT = 0b0000_0000_0100_0000;
        const RX_DONE = 0b0000_0000_1000_0000;
        const ACTIVITY = 0b0000_0001_0000_0000;
        const STOP_DET = 0b0000_0010_0000_0000;
        const START_DET = 0b0000_0100_0000_0000;
        const GEN_CALL = 0b0000_1000_0000_0000;
        const RESTART_DET = 0b0001_0000_0000_0000;
        const MST_ON_HOLD = 0b0010_0000_0000_0000;
        const MASK = 0b0011_1111_1111_1111;
    }
}

bitflag_is_set!(I2cClearInterrupt);
bitflag_from_u32!(I2cClearInterrupt);

/// Represents the I2C `TAR` register bitfield.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct I2cTar(u32);

bitflags! {
    impl I2cTar: u32 {
        const NONE = 0b0000_0000_0000_0000;
        const ADDR_MASK_7BIT = 0b0000_0000_0111_1111;
        const ADDR_MASK_10BIT = 0b0000_0011_1111_1111;
        const MODE_10BIT = 0b0001_0000_0000_0000;
        const MASK = 0b0001_0011_1111_1111;
    }
}

bitflag_is_set!(I2cTar);
bitflag_from_u32!(I2cTar);

impl I2cTar {
    /// Gets the target address for 7-bit mode.
    pub fn address_7bit(&self) -> u8 {
        (*self & Self::ADDR_MASK_7BIT).bits() as u8
    }

    /// Gets the target address for 10-bit mode.
    pub fn address_10bit(&self) -> u16 {
        (*self & Self::ADDR_MASK_10BIT).bits() as u16
    }
}

/// Represents the I2C `SAR` register bitfield.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct I2cSar(u32);

bitflags! {
    impl I2cSar: u32 {
        const NONE = 0b0000_0000_0000;
        const ADDR_MASK_7BIT = 0b0000_0111_1111;
        const ADDR_MASK_10BIT = 0b0011_1111_1111;
        const MASK = 0b0011_1111_1111;
    }
}

bitflag_is_set!(I2cSar);
bitflag_from_u32!(I2cSar);

impl I2cSar {
    /// Gets the target address for 7-bit mode.
    pub fn address_7bit(&self) -> u8 {
        (*self & Self::ADDR_MASK_7BIT).bits() as u8
    }

    /// Gets the target address for 10-bit mode.
    pub fn address_10bit(&self) -> u16 {
        (*self & Self::ADDR_MASK_10BIT).bits() as u16
    }
}

/// Represents I2C `ENABLE` register bitfield.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct I2cEnable(u32);

bitflags! {
    impl I2cEnable: u32 {
        const NONE = 0b0000;
        const ENABLE = 0b0001;
        const ABORT = 0b0010;
        const MASK = 0b0011;
    }
}

bitflag_is_set!(I2cEnable);
bitflag_from_u32!(I2cEnable);

/// Represents I2C `ENABLE` register bitfield.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct I2cEnableStatus(u32);

bitflags! {
    impl I2cEnableStatus: u32 {
        const NONE = 0b0000_0000;
        const ACTIVITY = 0b0000_0001;
        const TFE = 0b0000_0100;
        const RFNE = 0b0000_1000;
        const MASTER_ACTIVITY = 0b0010_0000;
        const SLAVE_ACTIVITY = 0b0100_0000;
        const MASK = 0b0111_1101;
    }
}

bitflag_is_set!(I2cEnableStatus);
bitflag_from_u32!(I2cEnableStatus);

/// Represents I2C functionality bitfield.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct I2cFunc(u32);

bitflags! {
    impl I2cFunc: u32 {
        const NONE = 0x0000_0000;
        const I2C = 0x0000_0001;
        const ADDRESS_10BIT = 0x0000_0002;
        const PROTOCOL_MANGLING = 0x0000_0004;
        const SMBUS_PEC = 0x0000_0008;
        const NOSTART = 0x0000_0010;
        const SLAVE = 0x0000_0020;
        const SMBUS_BLOCK_PROC_CALL = 0x0000_8000;
        const SMBUS_QUICK = 0x0001_0000;
        const SMBUS_READ_BYTE = 0x0002_0000;
        const SMBUS_WRITE_BYTE = 0x0004_0000;
        const SMBUS_READ_BYTE_DATA = 0x0008_0000;
        const SMBUS_WRITE_BYTE_DATA = 0x0010_0000;
        const SMBUS_READ_WORD_DATA = 0x0020_0000;
        const SMBUS_WRITE_WORD_DATA = 0x0040_0000;
        const SMBUS_PROC_CALL = 0x0080_0000;
        const SMBUS_READ_BLOCK_DATA = 0x0100_0000;
        const SMBUS_WRITE_BLOCK_DATA = 0x0200_0000;
        const SMBUS_READ_I2C_BLOCK = 0x0400_0000;
        const SMBUS_WRITE_I2C_BLOCK = 0x0800_0000;
        const SMBUS_HOST_NOTIFY = 0x1000_0000;
        const SMBUS_BYTE = 0x0002_0000 | 0x0004_0000;
        const SMBUS_BYTE_DATA = 0x0008_0000 | 0x0010_0000;
        const SMBUS_WORD_DATA = 0x0020_0000 | 0x0040_0000;
        const SMBUS_BLOCK_DATA = 0x0100_0000 | 0x0200_0000;
        const SMBUS_I2C_BLOCK = 0x0400_0000 | 0x0800_0000;
    }
}

impl Default for I2cFunc {
    fn default() -> Self {
        I2cFunc::I2C
            | I2cFunc::SMBUS_BYTE
            | I2cFunc::SMBUS_BYTE_DATA
            | I2cFunc::SMBUS_WORD_DATA
            | I2cFunc::SMBUS_BLOCK_DATA
            | I2cFunc::SMBUS_I2C_BLOCK
    }
}

/// Represents the I2C `DATA_CMD` register bitfield.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct I2cDataCmd(u32);

bitflags! {
    impl I2cDataCmd: u32 {
        const NONE = 0b0000_0000_0000_0000;
        const DATA_MASK = 0b0000_0000_1111_1111;
        const READ = 0b0000_0001_0000_0000;
        const STOP = 0b0000_0010_0000_0000;
        const RESTART = 0b0000_0100_0000_0000;
        const FIRST_DATA_BYTE = 0b0000_1000_0000_0000;
        const MASK = 0b0000_1111_1111_1111;
    }
}

impl I2cDataCmd {
    /// Creates a new [I2cDataCmd].
    pub const fn new() -> Self {
        Self(0)
    }

    /// Gets the data byte.
    pub fn data(&self) -> u8 {
        (*self & Self::DATA_MASK).bits() as u8
    }

    /// Sets the data byte.
    pub fn set_data(&mut self, val: u8) {
        *self &= !Self::DATA_MASK;
        *self |= val.into();
    }

    /// Builder function that sets the data byte.
    pub fn with_data(mut self, val: u8) -> Self {
        self.set_data(val);
        self
    }
}

impl From<u8> for I2cDataCmd {
    fn from(val: u8) -> Self {
        Self(val as u32)
    }
}

impl From<&u8> for I2cDataCmd {
    fn from(val: &u8) -> Self {
        (*val).into()
    }
}

bitflag_is_set!(I2cDataCmd);
bitflag_from_u32!(I2cDataCmd);

/// Represents the I2C `TX_ABRT_SOURCE` register bitfield.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct I2cTxAbortSource(u32);

bitflags! {
    impl I2cTxAbortSource: u32 {
        const NONE = 0b0000_0000_0000_0000;
        const B7_ADDR_NOACK = 0b0000_0000_0000_0001;
        const B10_ADDR1_NOACK = 0b0000_0000_0000_0010;
        const B10_ADDR2_NOACK = 0b0000_0000_0000_0100;
        const TXDATA_NOACK = 0b0000_0000_0000_1000;
        const GCALL_NOACK = 0b0000_0000_0001_0000;
        const GCALL_READ = 0b0000_0000_0010_0000;
        const SBYTE_ACKDET = 0b0000_0000_1000_0000;
        const SBYTE_NORSTRT = 0b0000_0010_0000_0000;
        const B10_RD_NORSTRT = 0b0000_0100_0000_0000;
        const MASTER_DIS = 0b0000_1000_0000_0000;
        const ARB_LOST = 0b0001_0000_0000_0000;
        const SLAVE_FLUSH_TXFIFO = 0b0010_0000_0000_0000;
        const SLAVE_ARB_LOST = 0b0100_0000_0000_0000;
        const SLAVE_RD_INTX = 0b1000_0000_0000_0000;
        const MASK = 0b1111_1110_1011_1111;
    }
}

bitflag_is_set!(I2cTxAbortSource);
bitflag_from_u32!(I2cTxAbortSource);
