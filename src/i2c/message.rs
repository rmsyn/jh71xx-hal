use heapless::Vec;

use crate::bitflag_is_set;

bitflags! {
    /// Flags for supported and optional features for [I2cMessage] transfers.
    ///
    /// Documentation for flags taken from `linux/include/uapi/linux/i2c.h`.
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct I2cMsgFlag: u16 {
        /// No flags.
        const NONE = 0x0000;
        /// Read data (from slave to master). Guaranteed to be 0x0001.
        const RD = 0x0001;
        /// This is a 10-bit chip address.
        const TEN = 0x0010;
        /// Indicates whether transfers are DMA safe.
        const DMA_SAFE = 0x2000;
        /// Message length will be first received byte.
        const RECV_LEN = 0x0400;
        /// In a read message master ACK/NAK bit is skipped.
        const NO_RD_ACK = 0x0800;
        /// Treat NACK from client as ACK.
        const IGNORE_NAK = 0x1000;
        /// Toggles the Rd/Wr bit.
        const REV_DIR_ADDR = 0x2000;
        /// Skip repeated START sequence.
        const NOSTART = 0x4000;
        /// Force a STOP condition after the message.
        const STOP = 0x8000;
    }
}

bitflag_is_set!(I2cMsgFlag);

pub const I2C_MSG_MAX: usize = u16::MAX as usize;

pub type I2cBuffer = Vec<u8, I2C_MSG_MAX>;

/// An [`I2cMessage`] is the low-level representation of one segment of an I2C
/// transaction.
///
/// From `linux/include/uapi/linux/i2c.h`:
///
/// It is visible to drivers in the @i2c_transfer() procedure,
/// to userspace from i2c-dev, and to I2C adapter drivers through the
/// @i2c_adapter.@master_xfer() method.
///
/// Except when I2C "protocol mangling" is used, all I2C adapters implement
/// the standard rules for I2C transactions.  Each transaction begins with a
/// START.  That is followed by the slave address, and a bit encoding read
/// versus write.  Then follow all the data bytes, possibly including a byte
/// with SMBus PEC.  The transfer terminates with a NAK, or when all those
/// bytes have been transferred and ACKed.  If this is the last message in a
/// group, it is followed by a STOP.  Otherwise it is followed by the next
/// @i2c_msg transaction segment, beginning with a (repeated) START.
///
/// Alternatively, when the adapter supports `I2C_FUNC_PROTOCOL_MANGLING` then
/// passing certain `flags`may have changed those standard protocol behaviors.
/// Those flags are only for use with broken/nonconforming slaves, and with
/// adapters which are known to support the specific mangling options they need.
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct I2cMessage {
    addr: u16,
    flags: I2cMsgFlag,
    buf: I2cBuffer,
}
