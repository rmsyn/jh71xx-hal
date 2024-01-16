use critical_section::RawRestoreState;

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
    #[cfg(not(any(feature = "rt", feature = "rts")))]
    let ie = false;

    #[cfg(all(feature = "rt", not(feature = "rts")))]
    let ie = riscv::register::mstatus::read().mie();
    #[cfg(feature = "rts")]
    let ie = riscv::register::sstatus::read().sie();

    #[cfg(all(feature = "rt", not(feature = "rts")))]
    riscv::register::mstatus::clear_mie();
    #[cfg(feature = "rts")]
    riscv::register::sstatus::clear_sie();

    ie
}

/// Enables previously enabled interrupts on all HARTs.
///
/// # Safety
///
/// - Do not call this function inside a critical section.
#[inline]
pub unsafe fn enable(restore: RawRestoreState) {
    if restore {
        #[cfg(all(feature = "rt", not(feature = "rts")))]
        riscv::register::mstatus::set_mie();
        #[cfg(feature = "rts")]
        riscv::register::sstatus::set_sie();
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
