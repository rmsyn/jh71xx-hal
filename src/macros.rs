/// Implements a function to determine if a `bitflag` has a set flag.
#[macro_export]
macro_rules! bitflag_is_set {
    ($ty:ident) => {
        impl $ty {
            /// Determines whether `oth` flag is set.
            pub fn is_set(self, oth: Self) -> bool {
                self & oth != Self::NONE
            }
        }
    };
}

/// Implements the [`From`](core::convert::From) trait for bitflag types.
///
/// **NOTE**: bitflag type must be a tuple struct around a [`u32`].
/// Also, the bitflag type must have a member `MASK` that represents the bitmask.
///
/// Example:
///
/// ```no_build,no_run
/// pub struct SomeFlag(u32);
///
/// bitflags! {
///     impl SomeFlag: u32 {
///         const SOME_FIELD: 0b01;
///         const SOME_OTH_FIELD: 0b11;
///         const MASK: 0b11;
///     }
/// }
///
/// bitflag_from_u32!(SomeFlag);
///
/// assert_eq!(SomeFlag::from(1u32), SomeFlag::SOME_FIELD);
/// ```
#[macro_export]
macro_rules! bitflag_from_u32 {
    ($flag:ident) => {
        impl From<u32> for $flag {
            fn from(val: u32) -> Self {
                Self(val & Self::MASK.bits())
            }
        }
    };
}
