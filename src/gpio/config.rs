use crate::pac::sys_pinctrl::*;

/// Configuration options for the GPIO drive-strength (in milliamps).
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DriveStrength {
    /// 2 mA rated output drive strength. 
    #[default]
    Two = 0b00,
    /// 4 mA rated output drive strength. 
    Four = 0b01,
    /// 8 mA rated output drive strength. 
    Eight = 0b10,
    /// 12 mA rated output drive strength. 
    Twelve = 0b11,
}

impl From<u8> for DriveStrength {
    fn from(val: u8) -> Self {
        match val & 0b11 {
            0b00 => Self::Two,
            0b01 => Self::Four,
            0b10 => Self::Eight,
            0b11 => Self::Twelve,
            // technically unreachable, but let's make the compiler happy without panicking
            _ => Self::Two,
        }
    }
}

impl From<DriveStrength> for u8 {
    fn from(val: DriveStrength) -> Self {
        val as u8
    }
}

/// Configuration options for the GPIO slew rate control.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Slew {
    /// Slow (half-frequency) slew rate control.
    #[default]
    Slow = 0,
    /// Fast slew rate control.
    Fast = 1,
}

impl From<bool> for Slew {
    fn from(val: bool) -> Self {
        match val {
            false => Self::Slow,
            true => Self::Fast,
        }
    }
}

impl From<u8> for Slew {
    fn from(val: u8) -> Self {
        (val != 0).into()
    }
}

impl From<Slew> for bool {
    fn from(val: Slew) -> Self {
        val as u8 != 0
    }
}

impl From<Slew> for u8 {
    fn from(val: Slew) -> Self {
        val as u8
    }
}

/// Configuration options for the GPIO Schmitt trigger hysteresis.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SchmittTrigger {
    /// No hysteresis
    Disable = 0,
    /// Enables the Schmitt Trigger hysteresis
    Enable = 1,
}

impl From<bool> for SchmittTrigger {
    fn from(val: bool) -> Self {
        match val {
            false => Self::Disable,
            true => Self::Enable,
        }
    }
}

impl From<u8> for SchmittTrigger {
    fn from(val: u8) -> Self {
        (val != 0).into()
    }
}

impl From<SchmittTrigger> for bool {
    fn from(val: SchmittTrigger) -> Self {
        val as u8 != 0
    }
}

impl From<SchmittTrigger> for u8 {
    fn from(val: SchmittTrigger) -> Self {
        val as u8
    }
}

/// Configuration options for the GPIO Power-on-Start feature.
pub enum PowerOnStart {
    /// No active pull-down on loss of core power.
    Disable = 0,
    /// Enables active pull-down for loss of core power.
    Enable = 1,
}

impl From<bool> for PowerOnStart {
    fn from(val: bool) -> Self {
        match val {
            false => Self::Disable,
            true => Self::Enable,
        }
    }
}

impl From<u8> for PowerOnStart {
    fn from(val: u8) -> Self {
        (val != 0).into()
    }
}

impl From<PowerOnStart> for bool {
    fn from(val: PowerOnStart) -> Self {
        val as u8 != 0
    }
}

impl From<PowerOnStart> for u8 {
    fn from(val: PowerOnStart) -> Self {
        val as u8
    }
}

/// Configuration trait for GPIO peripheral registers.
pub trait GpioCfg {
    /// Gets the pad number for the GPIO.
    fn pad() -> u32;

    /// Gets whether the GPIO is configured as an input.
    fn is_input_enabled(&self) -> bool;
    /// Enables the GPIO as an input.
    fn input_enable(&self, enable: bool);

    /// Gets the drive-strength (in milliamps) of the GPIO output.
    fn drive_strength(&self) -> DriveStrength;
    /// Sets the drive-strength (in milliamps) of the GPIO output.
    fn set_drive_strength(&self, drive_strength: DriveStrength);

    /// Gets whether the GPIO is configured as a high-impedance (HiZ) input.
    fn is_high_z(&self) -> bool;
    /// Gets whether the GPIO is configured as a Pull-Up input.
    fn is_pull_up(&self) -> bool;
    /// Gets whether the GPIO is configured as a Pull-Down input.
    fn is_pull_down(&self) -> bool;

    /// Configures the GPIO as a high-impedance input.
    fn set_high_z(&self);
    /// Configures the GPIO as a Pull-Down input.
    fn set_pull_down(&self);
    /// Configures the GPIO as a Pull-Up input.
    fn set_pull_up(&self);

    /// Gets the slew control rate of the GPIO.
    fn slew(&self) -> Slew;
    /// Sets the slew control rate of the GPIO.
    fn set_slew(&self, slew: Slew);

    /// Gets the Schmitt Trigger configuration of the GPIO.
    fn schmitt_trigger(&self) -> SchmittTrigger;
    /// Sets the Schmitt Trigger configuration of the GPIO.
    fn set_schmitt_trigger(&self, trigger: SchmittTrigger);

    /// Gets the Power-on-Start configuration of the GPIO.
    fn power_on_start(&self) -> PowerOnStart;
    /// Sets the Power-on-Start configuration of the GPIO.
    fn set_power_on_start(&self, pos: PowerOnStart);
}

macro_rules! gpio_cfg {
    ($gpio:ident, $pad:expr) => {
        impl $crate::gpio::GpioCfg for $gpio {
            fn pad() -> u32 {
                $pad
            }

            fn is_input_enabled(&self) -> bool {
                self.read().ie().bit_is_set()
            }

            fn input_enable(&self, enable: bool) {
                self.modify(|_, w| if enable {
                    w.ie().set_bit()
                } else {
                    w.ie().clear_bit()
                })
            }

            fn drive_strength(&self) -> $crate::gpio::DriveStrength {
                self.read().ds().bits().into()
            }

            fn set_drive_strength(&self, drive_strength: $crate::gpio::DriveStrength) {
                self.modify(|_, w| w.ds().variant(drive_strength.into()));
            }

            fn is_high_z(&self) -> bool {
                let r = self.read();
                r.pu().bit_is_clear() && r.pd().bit_is_clear()
            }

            fn is_pull_up(&self) -> bool {
                self.read().pu().bit_is_set()
            }

            fn is_pull_down(&self) -> bool {
                self.read().pd().bit_is_set()
            }

            fn set_high_z(&self) {
                self.modify(|_, w| {
                    w.pu().clear_bit();
                    w.pd().clear_bit()
                });
            }

            fn set_pull_up(&self) {
                self.modify(|_, w| {
                    w.pu().set_bit();
                    w.pd().clear_bit()
                });
            }

            fn set_pull_down(&self) {
                self.modify(|_, w| {
                    w.pu().clear_bit();
                    w.pd().set_bit()
                });
            }

            fn slew(&self) -> $crate::gpio::Slew {
                self.read().slew().bit_is_set().into()
            }

            fn set_slew(&self, slew: $crate::gpio::Slew) {
                self.modify(|_, w| match slew {
                    $crate::gpio::Slew::Slow => w.slew().clear_bit(),
                    $crate::gpio::Slew::Fast => w.slew().set_bit(),
                });
            }

            fn schmitt_trigger(&self) -> $crate::gpio::SchmittTrigger {
                self.read().smt().bit_is_set().into()
            }

            fn set_schmitt_trigger(&self, trigger: $crate::gpio::SchmittTrigger) {
                self.modify(|_, w| match trigger {
                    $crate::gpio::SchmittTrigger::Disable => w.smt().clear_bit(),
                    $crate::gpio::SchmittTrigger::Enable => w.smt().set_bit(),
                });
            }

            fn power_on_start(&self) -> $crate::gpio::PowerOnStart {
                self.read().pos().bit_is_set().into()
            }

            fn set_power_on_start(&self, pos: $crate::gpio::PowerOnStart) {
                self.modify(|_, w| match pos {
                    $crate::gpio::PowerOnStart::Disable => w.pos().clear_bit(),
                    $crate::gpio::PowerOnStart::Enable => w.pos().set_bit(),
                });
            }
        }
    }
}

gpio_cfg!(GPIO_0, 0);
gpio_cfg!(GPIO_1, 1);
gpio_cfg!(GPIO_2, 2);
gpio_cfg!(GPIO_3, 3);
gpio_cfg!(GPIO_4, 4);
gpio_cfg!(GPIO_5, 5);
gpio_cfg!(GPIO_6, 6);
gpio_cfg!(GPIO_7, 7);
gpio_cfg!(GPIO_8, 8);
gpio_cfg!(GPIO_9, 9);
gpio_cfg!(GPIO_10, 10);
gpio_cfg!(GPIO_11, 11);
gpio_cfg!(GPIO_12, 12);
gpio_cfg!(GPIO_13, 13);
gpio_cfg!(GPIO_14, 14);
gpio_cfg!(GPIO_15, 15);
gpio_cfg!(GPIO_16, 16);
gpio_cfg!(GPIO_17, 17);
gpio_cfg!(GPIO_18, 18);
gpio_cfg!(GPIO_19, 19);
gpio_cfg!(GPIO_20, 20);
gpio_cfg!(GPIO_21, 21);
gpio_cfg!(GPIO_22, 22);
gpio_cfg!(GPIO_23, 23);
gpio_cfg!(GPIO_24, 24);
gpio_cfg!(GPIO_25, 25);
gpio_cfg!(GPIO_26, 26);
gpio_cfg!(GPIO_27, 27);
gpio_cfg!(GPIO_28, 28);
gpio_cfg!(GPIO_29, 29);
gpio_cfg!(GPIO_30, 30);
gpio_cfg!(GPIO_31, 31);
gpio_cfg!(GPIO_32, 32);
gpio_cfg!(GPIO_33, 33);
gpio_cfg!(GPIO_34, 34);
gpio_cfg!(GPIO_35, 35);
gpio_cfg!(GPIO_36, 36);
gpio_cfg!(GPIO_37, 37);
gpio_cfg!(GPIO_38, 38);
gpio_cfg!(GPIO_39, 39);
gpio_cfg!(GPIO_40, 40);
gpio_cfg!(GPIO_41, 41);
gpio_cfg!(GPIO_42, 42);
gpio_cfg!(GPIO_43, 43);
gpio_cfg!(GPIO_44, 44);
gpio_cfg!(GPIO_45, 45);
gpio_cfg!(GPIO_46, 46);
gpio_cfg!(GPIO_47, 47);
gpio_cfg!(GPIO_48, 48);
gpio_cfg!(GPIO_49, 49);
gpio_cfg!(GPIO_50, 50);
gpio_cfg!(GPIO_51, 51);
gpio_cfg!(GPIO_52, 52);
gpio_cfg!(GPIO_53, 53);
gpio_cfg!(GPIO_54, 54);
gpio_cfg!(GPIO_55, 55);
gpio_cfg!(GPIO_56, 56);
gpio_cfg!(GPIO_57, 57);
gpio_cfg!(GPIO_58, 58);
gpio_cfg!(GPIO_59, 59);
gpio_cfg!(GPIO_60, 60);
gpio_cfg!(GPIO_61, 61);
gpio_cfg!(GPIO_62, 62);
gpio_cfg!(GPIO_63, 63);
gpio_cfg!(SD0_CLK, 64);
gpio_cfg!(SD0_CMD, 65);
gpio_cfg!(SD0_DATA_0, 66);
gpio_cfg!(SD0_DATA_1, 67);
gpio_cfg!(SD0_DATA_2, 68);
gpio_cfg!(SD0_DATA_3, 69);
gpio_cfg!(SD0_DATA_4, 70);
gpio_cfg!(SD0_DATA_5, 71);
gpio_cfg!(SD0_DATA_6, 72);
gpio_cfg!(SD0_DATA_7, 73);
gpio_cfg!(SD0_STRB, 74);
gpio_cfg!(QSPI_SCLK, 89);
gpio_cfg!(QSPI_CSN_0, 90);
gpio_cfg!(QSPI_DATA_0, 91);
gpio_cfg!(QSPI_DATA_1, 92);
gpio_cfg!(QSPI_DATA_2, 93);
gpio_cfg!(QSPI_DATA_3, 94);
