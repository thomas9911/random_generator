#![no_std]

mod macros;
pub use macros::*;

#[no_mangle]
pub extern "C" fn _DllMainCRTStartup() -> ! {
    loop {}
}

// use core::panic::PanicInfo;

// mod traits;
// pub use traits::*;

// #[panic_handler]
// fn panic(_panic: &PanicInfo<'_>) -> ! {
//     loop {}
// }

#[cfg(not(any(test, feature = "benchmarking")))]
use core::panic::PanicInfo;

// #[cfg(not(test))]
#[cfg(not(any(test, feature = "benchmarking")))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
