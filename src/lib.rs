#![no_std]
#![feature(
    core_intrinsics,
    lang_items,
    alloc_error_handler,
    strict_provenance,
    panic_info_message
)]

pub mod system;
use core::fmt::Write;
use core::intrinsics::abort;
use core::panic::PanicInfo;

extern crate alloc;
use alloc::format;
use system::allocator::init_heap;
use system::{
    ports::{init_gpio, PortRegister, PORTA},
    uart::{uart0_init, write},
};

use crate::system::kernel_shell::KernelShell;
use crate::system::rtc::wait_for_rtc_init;
use crate::system::sd_mmc::SDCard;

#[no_mangle]
#[allow(unused_must_use)]
pub extern "C" fn kernel_main() -> u32 {
    init_gpio();
    let mut uart = uart0_init();
    let mut port_a = PortRegister::port(PORTA);
    port_a.set_pin_mode(10, 001);
    port_a.set_pin_output(10, true);
    write!(&mut uart, "==== SmolKernel v1.12 ====\n\r");
    write!(&mut uart, "Initializing heap... ");
    let free_bytes = init_heap();
    write!(&mut uart, "done.\n\r");
    write!(&mut uart, "Heap size: {} bytes\n\r", free_bytes);
    write!(&mut uart, "Waiting for RTC initialization... ");
    wait_for_rtc_init();
    write!(&mut uart, "done.\n\r");
    port_a.set_pin_output(10, false);
    write!(&mut uart, "Enter 'q' to quit\n\r");
    let sd_mmc = SDCard {};
    let mut shell = KernelShell::new(uart, port_a, sd_mmc);
    shell.run()
}

// #[no_mangle]
// pub extern "C" fn __aeabi_unwind_cpp_pr0() {}

// #[lang = "eh_personality"]
// pub extern "C" fn eh_personality() {}

#[allow(unused_unsafe)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    write(format!("{}\n", info.message().unwrap()).as_str());
    unsafe { abort() }
}

// #[allow(non_snake_case)]
// #[no_mangle]
// pub extern "C" fn _Unwind_Resume() {
//     loop {}
// }
