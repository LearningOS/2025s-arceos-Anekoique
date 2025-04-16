mod hash_map;
mod random_state;

pub use self::hash_map::HashMap;

#[cfg(feature = "alloc")]
#[doc(no_inline)]
pub use alloc::collections::*;