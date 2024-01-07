#![no_std]

#[macro_use]
extern crate bitflags;

extern crate embedded_io as io;

#[cfg(any(
    feature = "visionfive2-12a",
    feature = "visionfive2-12a-rt",
    feature = "visionfive2-12a-rts"
))]
pub use jh71xx_pac::jh7110_vf2_12a_pac as pac;
#[cfg(any(
    feature = "visionfive2-13b",
    feature = "visionfive2-13b-rt",
    feature = "visionfive2-13b-rts"
))]
pub use jh71xx_pac::jh7110_vf2_13b_pac as pac;

pub mod critical_section;
pub mod delay;
pub mod gpio;
pub mod i2c;
pub mod interrupt;
mod macros;
pub mod uart;
