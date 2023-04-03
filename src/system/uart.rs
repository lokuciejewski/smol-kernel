use core::fmt::Write;

use super::{
    ccu::CCU,
    ports::{PortRegister, PORTA},
    rw::{memory_read, memory_write},
};

const UART0_BASE: u32 = 0x01c28000;

const UART0_RBR: u32 = UART0_BASE;
const UART0_THR: u32 = UART0_BASE;
const UART0_DLL: u32 = UART0_BASE;
#[allow(dead_code)]
const UART0_IER: u32 = UART0_BASE + 0x04;
const UART0_FCR: u32 = UART0_BASE + 0x08;
const UART0_LCR: u32 = UART0_BASE + 0x0c;
const UART0_LSR: u32 = UART0_BASE + 0x14;
const UART0_USR: u32 = UART0_BASE + 0x7c;

pub struct UART0;

impl Write for UART0 {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        write(s);
        Ok(())
    }
}

pub fn uart0_init() -> UART0 {
    let mut port_a = PortRegister::port(PORTA);
    port_a.set_pin_mode(4, 2);
    memory_write(
        CCU::BUS_CLK_GATING3 as u32,
        memory_read(CCU::BUS_CLK_GATING3 as u32) | (1 << 16),
    );
    memory_write(
        CCU::BUS_SOFT_RST4 as u32,
        memory_read(CCU::BUS_SOFT_RST4 as u32) | (1 << 16),
    );
    memory_write(UART0_LCR, (1 << 7) | 3);
    memory_write(UART0_DLL, 13);
    memory_write(UART0_LCR, 3);
    memory_write(UART0_FCR, 1);
    UART0
}

fn uart_tx_ready() -> bool {
    memory_read(UART0_USR) & 2 != 0
}

fn uart_rx_ready() -> bool {
    memory_read(UART0_LSR) & 1 != 0
}

pub fn writec(c: u8) {
    while !uart_tx_ready() {}
    memory_write(UART0_THR, c as u32);
}

pub fn getc() -> u8 {
    while !uart_rx_ready() {}
    memory_read(UART0_RBR) as u8
}

pub fn write(msg: &str) {
    for c in msg.chars() {
        writec(c as u8)
    }
}
