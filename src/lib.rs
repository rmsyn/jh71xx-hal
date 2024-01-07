#![no_std]

#[macro_use]
extern crate bitflags;

extern crate embedded_io as io;

pub extern crate jh71xx_pac as pac;

pub mod critical_section;
pub mod delay;
pub mod gpio;
pub mod i2c;
pub mod interrupt;
mod macros;
pub mod uart;
