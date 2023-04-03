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
use alloc::string::String;
use system::allocator::{get_free_bytes, init_heap};
use system::{
    ports::{init_gpio, PortRegister, PORTA},
    uart::{getc, uart0_init, write, writec},
};

use crate::system::rtc::{get_hours, get_minutes, get_seconds, wait_for_rtc_init};

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
    let mut inp: u8 = 0;
    let mut buffer = String::new();
    buffer.reserve_exact(10);
    loop {
        write!(&mut uart, "> ");
        while inp != 13 {
            // Carriage return
            inp = getc();
            writec(inp);
            buffer.push(inp as char);
        }
        match buffer.trim() {
            "q" => return 0,
            "free" => {
                let fb = get_free_bytes().unwrap_or(0);
                write!(&mut uart, "Free memory: {} bytes\n\r", fb);
            }
            "time" => {
                write!(
                    &mut uart,
                    "Time: {:02}:{:02}:{:02}\n\r",
                    get_hours(),
                    get_minutes(),
                    get_seconds()
                );
            }
            "on" => {
                port_a.set_pin_output(10, true);
                write!(&mut uart, "Led on\n\r");
            }
            "off" => {
                port_a.set_pin_output(10, false);
                write!(&mut uart, "Led off\n\r");
            }
            cmd => {
                write!(&mut uart, "Unknown command: `{}`\n\r", cmd);
            }
        }
        inp = 0;
        buffer.clear();
    }
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
