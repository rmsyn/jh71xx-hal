use embedded_hal::delay::DelayNs;
use riscv::register::{cycle, mcycle};

/// Clock rate of the U74 core (in Hertz): 1,500 MHz
pub const U74_CLOCK_HZ: u64 = 1_500_000_000;

/// Machine mode cycle counter (`mcycle`) as a delay provider
#[derive(Clone, Copy)]
pub struct McycleDelay {
    ticks_second: u64,
}

impl McycleDelay {
    /// Create a new [McycleDelay] from the provided parameter.
    ///
    /// `ticks_second`: clock cycle rate (in Hertz).
    pub const fn new(ticks_second: u64) -> Self {
        Self { ticks_second }
    }
}

impl DelayNs for McycleDelay {
    #[inline]
    fn delay_ns(&mut self, ns: u32) {
        let t0 = mcycle::read64();
        let ns_64 = u64::from(ns);
        let clock = ns_64
            .saturating_mul(self.ticks_second)
            .saturating_div(1_000_000_000u64);
        while mcycle::read64().wrapping_sub(t0) <= clock {}
    }
}

/// User mode cycle counter (`cycle`) as a delay provider
#[derive(Clone, Copy)]
pub struct UcycleDelay {
    ticks_second: u64,
}

impl UcycleDelay {
    /// Create a new [UcycleDelay] from the provided parameter.
    ///
    /// `ticks_second`: clock cycle rate (in Hertz).
    pub const fn new(ticks_second: u64) -> Self {
        Self { ticks_second }
    }
}

impl DelayNs for UcycleDelay {
    #[inline]
    fn delay_ns(&mut self, ns: u32) {
        let t0 = cycle::read64();
        let ns_64 = u64::from(ns);
        let clock = ns_64
            .saturating_mul(self.ticks_second)
            .saturating_div(1_000_000_000u64);
        while cycle::read64().wrapping_sub(t0) <= clock {}
    }
}

/// Convenience function to get a [McycleDelay] for the `U74` riscv core.
pub fn u74_mdelay() -> McycleDelay {
    McycleDelay::new(U74_CLOCK_HZ)
}

/// Convenience function to get a [UcycleDelay] for the `U74` riscv core.
pub fn u74_udelay() -> UcycleDelay {
    UcycleDelay::new(U74_CLOCK_HZ)
}
