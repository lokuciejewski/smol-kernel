use crate::system::ccu::CCU;
use crate::system::rw::{memory_read, memory_write};

const PIO_BASE: u32 = 0x01c20800;
pub const PORTA: u32 = PIO_BASE;
pub const PORTC: u32 = PIO_BASE + 2 * 0x24;
pub const PORTD: u32 = PIO_BASE + 3 * 0x24;
pub const PORTE: u32 = PIO_BASE + 4 * 0x24;
pub const PORTF: u32 = PIO_BASE + 5 * 0x24;
pub const PORTG: u32 = PIO_BASE + 6 * 0x24;
pub const PORTL: u32 = 0x01f02c00;

#[repr(C)]
pub struct PortRegister {
    port_address: u32,
    cfg0: u32,
    cfg1: u32,
    cfg2: u32,
    cfg3: u32,
    data: u32,
}

pub fn init_gpio() {
    memory_write(
        CCU::BUS_CLK_GATING2 as u32,
        memory_read(CCU::BUS_CLK_GATING2 as u32) | (1 << 5),
    );
    memory_write(
        CCU::APB0_CLK_GATING as u32,
        memory_read(CCU::APB0_CLK_GATING as u32) | (1 << 0),
    );
}

impl PortRegister {
    pub fn port(port_address: u32) -> PortRegister {
        PortRegister {
            port_address,
            cfg0: port_address,
            cfg1: port_address + 0x04,
            cfg2: port_address + 0x08,
            cfg3: port_address + 0x0c,
            data: port_address + 0x10,
        }
    }

    pub fn set_pin_mode(&mut self, pin: u32, mode: u32) {
        let reg: &u32;
        if pin < 8 {
            reg = &self.cfg0;
        } else if pin < 16 {
            reg = &self.cfg1;
        } else if pin < 24 {
            reg = &self.cfg2;
        } else {
            reg = &self.cfg3;
        }
        memory_write(*reg, memory_read(*reg) & !(7 << ((pin % 8) * 4)));
        memory_write(*reg, memory_read(*reg) | (mode << ((pin % 8) * 4)));
    }

    pub fn set_port_data(&mut self, data: u32) {
        memory_write(self.data, data);
    }

    pub fn set_pin_output(&mut self, pin: u32, out: bool) {
        if out {
            memory_write(self.data, memory_read(self.data) | (1 << pin));
        } else {
            memory_write(self.data, memory_read(self.data) & !(1 << pin));
        }
    }
}
