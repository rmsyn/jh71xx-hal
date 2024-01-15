use critical_section::RawRestoreState;

use crate::pac::PLIC;

/// Disables all interrupts on all cores.
///
/// # Safety
///
/// Users should not need to call this function directly. Instead, used the [free] function.
///
/// This function must **NOT** be called within an interrupt-free context (e.g. in an interrupt
/// handler).
#[inline]
pub unsafe fn disable() -> RawRestoreState {
    let ie = riscv::register::mstatus::read().mie() as u64;
    let plic = &*PLIC::ptr();

    let restore = [
        (plic.enable_1_0().read().bits() as u64) << 32 | (plic.enable_0_0().read().bits() as u64),
        (plic.enable_3_0().read().bits() as u64) << 32 | (plic.enable_2_0().read().bits() as u64),
        (plic.enable_0_1().read().bits() as u64) << 32 | (plic.enable_4_0().read().bits() as u64),
        (plic.enable_2_1().read().bits() as u64) << 32 | (plic.enable_1_1().read().bits() as u64),
        (plic.enable_4_1().read().bits() as u64) << 32 | (plic.enable_3_1().read().bits() as u64),
        (plic.enable_1_2().read().bits() as u64) << 32 | (plic.enable_0_2().read().bits() as u64),
        (plic.enable_3_2().read().bits() as u64) << 32 | (plic.enable_2_2().read().bits() as u64),
        (plic.enable_0_3().read().bits() as u64) << 32 | (plic.enable_4_2().read().bits() as u64),
        (plic.enable_2_3().read().bits() as u64) << 32 | (plic.enable_1_3().read().bits() as u64),
        (plic.enable_4_3().read().bits() as u64) << 32 | (plic.enable_3_3().read().bits() as u64),
        (plic.enable_1_4().read().bits() as u64) << 32 | (plic.enable_0_4().read().bits() as u64),
        (plic.enable_3_4().read().bits() as u64) << 32 | (plic.enable_2_4().read().bits() as u64),
        ie << 32 | (plic.enable_4_4().read().bits() as u64),
        0,
        0,
        0,
    ];

    riscv::register::mstatus::clear_mie();

    plic.enable_0_0().reset();
    plic.enable_1_0().reset();
    plic.enable_2_0().reset();
    plic.enable_3_0().reset();
    plic.enable_4_0().reset();

    plic.enable_0_1().reset();
    plic.enable_1_1().reset();
    plic.enable_2_1().reset();
    plic.enable_3_1().reset();
    plic.enable_4_1().reset();

    plic.enable_0_2().reset();
    plic.enable_1_2().reset();
    plic.enable_2_2().reset();
    plic.enable_3_2().reset();
    plic.enable_4_2().reset();

    plic.enable_0_3().reset();
    plic.enable_1_3().reset();
    plic.enable_2_3().reset();
    plic.enable_3_3().reset();
    plic.enable_4_3().reset();

    plic.enable_0_4().reset();
    plic.enable_1_4().reset();
    plic.enable_2_4().reset();
    plic.enable_3_4().reset();
    plic.enable_4_4().reset();

    restore
}

/// Enables previously enabled interrupts on all HARTs.
///
/// # Safety
///
/// - Do not call this function inside a critical section.
#[inline]
pub unsafe fn enable(restore: RawRestoreState) {
    // check if MIE was set in `acquire`
    if restore[12] & (1u64 << 32) != 0 {
        let enable_0_0 = restore[0] as u32;
        let enable_1_0 = (restore[0] >> 32) as u32;
        let enable_2_0 = restore[1] as u32;
        let enable_3_0 = (restore[1] >> 32) as u32;
        let enable_4_0 = restore[2] as u32;
        let enable_0_1 = (restore[2] >> 32) as u32;
        let enable_1_1 = restore[3] as u32;
        let enable_2_1 = (restore[3] >> 32) as u32;
        let enable_3_1 = restore[4] as u32;
        let enable_4_1 = (restore[4] >> 32) as u32;
        let enable_0_2 = restore[5] as u32;
        let enable_1_2 = (restore[5] >> 32) as u32;
        let enable_2_2 = restore[6] as u32;
        let enable_3_2 = (restore[6] >> 32) as u32;
        let enable_4_2 = restore[7] as u32;
        let enable_0_3 = (restore[7] >> 32) as u32;
        let enable_1_3 = restore[8] as u32;
        let enable_2_3 = (restore[8] >> 32) as u32;
        let enable_3_3 = restore[9] as u32;
        let enable_4_3 = (restore[9] >> 32) as u32;
        let enable_0_4 = restore[10] as u32;
        let enable_1_4 = (restore[10] >> 32) as u32;
        let enable_2_4 = restore[11] as u32;
        let enable_3_4 = (restore[11] >> 32) as u32;
        let enable_4_4 = restore[12] as u32;

        riscv::register::mstatus::set_mie();

        let plic = &*PLIC::ptr();

        plic.enable_0_0()
            .modify(|_, w| w.enable().variant(enable_0_0));
        plic.enable_1_0()
            .modify(|_, w| w.enable().variant(enable_1_0));
        plic.enable_2_0()
            .modify(|_, w| w.enable().variant(enable_2_0));
        plic.enable_3_0()
            .modify(|_, w| w.enable().variant(enable_3_0));
        plic.enable_4_0()
            .modify(|_, w| w.enable().variant(enable_4_0));

        plic.enable_0_1()
            .modify(|_, w| w.enable().variant(enable_0_1));
        plic.enable_1_1()
            .modify(|_, w| w.enable().variant(enable_1_1));
        plic.enable_2_1()
            .modify(|_, w| w.enable().variant(enable_2_1));
        plic.enable_3_1()
            .modify(|_, w| w.enable().variant(enable_3_1));
        plic.enable_4_1()
            .modify(|_, w| w.enable().variant(enable_4_1));

        plic.enable_0_2()
            .modify(|_, w| w.enable().variant(enable_0_2));
        plic.enable_1_2()
            .modify(|_, w| w.enable().variant(enable_1_2));
        plic.enable_2_2()
            .modify(|_, w| w.enable().variant(enable_2_2));
        plic.enable_3_2()
            .modify(|_, w| w.enable().variant(enable_3_2));
        plic.enable_4_2()
            .modify(|_, w| w.enable().variant(enable_4_2));

        plic.enable_0_3()
            .modify(|_, w| w.enable().variant(enable_0_3));
        plic.enable_1_3()
            .modify(|_, w| w.enable().variant(enable_1_3));
        plic.enable_2_3()
            .modify(|_, w| w.enable().variant(enable_2_3));
        plic.enable_3_3()
            .modify(|_, w| w.enable().variant(enable_3_3));
        plic.enable_4_3()
            .modify(|_, w| w.enable().variant(enable_4_3));

        plic.enable_0_4()
            .modify(|_, w| w.enable().variant(enable_0_4));
        plic.enable_1_4()
            .modify(|_, w| w.enable().variant(enable_1_4));
        plic.enable_2_4()
            .modify(|_, w| w.enable().variant(enable_2_4));
        plic.enable_3_4()
            .modify(|_, w| w.enable().variant(enable_3_4));
        plic.enable_4_4()
            .modify(|_, w| w.enable().variant(enable_4_4));
    }
}

/// Execute closure `f` with interrupts disabled in the current hart.
///
/// This halts interrupts on all cores, making it suitable for the multicore JH71XX SoCs.
#[inline]
pub fn free<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let restore = unsafe { disable() };

    let r = f();

    unsafe {
        enable(restore);
    }

    r
}
