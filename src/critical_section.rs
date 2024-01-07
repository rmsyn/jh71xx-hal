use critical_section::{set_impl, Impl, RawRestoreState};

use crate::interrupt;

struct PlicCriticalSection;
set_impl!(PlicCriticalSection);

unsafe impl Impl for PlicCriticalSection {
    unsafe fn acquire() -> RawRestoreState {
        interrupt::disable()
    }

    unsafe fn release(was_active: RawRestoreState) {
        interrupt::enable(was_active)
    }
}
