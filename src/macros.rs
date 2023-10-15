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
