use alloc::{borrow::ToOwned, vec::Vec};
use core::fmt::Write;

use alloc::string::String;

use crate::system::{
    allocator::get_free_bytes,
    rtc::{get_hours, get_minutes, get_seconds},
    uart::{getc, writec},
};

use super::{ports::PortRegister, uart::UART0};

type Command = (&'static str, fn(&mut KernelShell) -> ShellReturnCode);

enum ShellReturnCode {
    Ok,
    Err,
    Exit,
}

pub struct KernelShell {
    available_commands: Vec<Command>,
    uart: UART0,
    port_a: PortRegister,
}

impl KernelShell {
    pub fn new(uart: UART0, port_a: PortRegister) -> Self {
        let mut ks = KernelShell {
            available_commands: Vec::new(),
            uart,
            port_a,
        };
        ks.available_commands.reserve_exact(1024);
        ks.available_commands.push(("q", KernelShell::shell_exit));
        ks.available_commands
            .push(("free", KernelShell::print_free_heap_memory));
        ks.available_commands
            .push(("time", KernelShell::print_time));
        ks.available_commands.push(("on", KernelShell::led_on));
        ks.available_commands.push(("off", KernelShell::led_off));
        ks
    }

    #[allow(unused)]
    pub fn run(&mut self) -> u32 {
        let mut inp: u8 = 0;
        let mut buffer = String::new();
        buffer.reserve_exact(10);
        'shell_loop: loop {
            write!(self.uart, "> ");
            while inp != 13 {
                // Carriage return
                inp = getc();
                writec(inp);
                buffer.push(inp as char);
            }
            for (command, function) in self.available_commands.to_owned() {
                if command == buffer.trim() {
                    match function(self) {
                        ShellReturnCode::Ok => {
                            inp = 0;
                            buffer.clear();
                            continue 'shell_loop;
                        }
                        ShellReturnCode::Err => {
                            write!(
                                self.uart,
                                "Command {} returned exit code {}",
                                command,
                                ShellReturnCode::Err as u32
                            );
                            inp = 0;
                            buffer.clear();
                            continue 'shell_loop;
                        }
                        ShellReturnCode::Exit => return 0,
                    }
                }
            }
            write!(self.uart, "Unknown command: `{}`\n\r", buffer.trim());
            inp = 0;
            buffer.clear();
            continue 'shell_loop;
        }
    }

    fn shell_exit(&mut self) -> ShellReturnCode {
        ShellReturnCode::Exit
    }

    fn print_time(&mut self) -> ShellReturnCode {
        match write!(
            self.uart,
            "Time: {:02}:{:02}:{:02}\n\r",
            get_hours(),
            get_minutes(),
            get_seconds()
        ) {
            Ok(_) => ShellReturnCode::Ok,
            Err(_) => ShellReturnCode::Err,
        }
    }

    fn led_on(&mut self) -> ShellReturnCode {
        self.port_a.set_pin_output(10, true);
        match write!(self.uart, "Led on!\n\r") {
            Ok(_) => ShellReturnCode::Ok,
            Err(_) => ShellReturnCode::Err,
        }
    }

    fn led_off(&mut self) -> ShellReturnCode {
        self.port_a.set_pin_output(10, false);
        match write!(self.uart, "Led off!\n\r") {
            Ok(_) => ShellReturnCode::Ok,
            Err(_) => ShellReturnCode::Err,
        }
    }

    fn print_free_heap_memory(&mut self) -> ShellReturnCode {
        match get_free_bytes() {
            Some(fb) => write!(&mut self.uart, "Free memory: {} bytes\n\r", fb).unwrap(),
            None => return ShellReturnCode::Err,
        }
        ShellReturnCode::Ok
    }
}
