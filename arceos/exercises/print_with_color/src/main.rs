#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
use axstd::red_println;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    red_println!("[WithColor]: Hello, Arceos!");
}
