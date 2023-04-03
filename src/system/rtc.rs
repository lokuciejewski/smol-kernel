use super::rw::memory_read;

const RTC_REG_BASE: u32 = 0x01f00000;
const RTC_LOSC_CTRL_REG: u32 = RTC_REG_BASE;
const RTC_REG_HH_MM_SS: u32 = RTC_REG_BASE + 0x14;

pub fn wait_for_rtc_init() {
    while RTC_LOSC_CTRL_REG & (1 << 8) != 0 {}
}

pub fn get_hours() -> u8 {
    ((memory_read(RTC_REG_HH_MM_SS) >> 16) & 0x3f) as u8
}

pub fn get_minutes() -> u8 {
    ((memory_read(RTC_REG_HH_MM_SS) >> 8) & 0x3f) as u8
}

pub fn get_seconds() -> u8 {
    (memory_read(RTC_REG_HH_MM_SS) & 0x3f) as u8
}
