use super::rw;

#[allow(non_camel_case_types, unused)]
const SD_MMC0_BASE: u32 = 0x01c0f000;
#[allow(non_camel_case_types, unused)]
const SD_MMC1_BASE: u32 = 0x01c10000;
#[allow(non_camel_case_types, unused)]
const SD_MMC2_BASE: u32 = 0x01c11000;
#[allow(non_camel_case_types, unused)]
enum SDRegistersOffsets {
    /// Control register
    SD_GCTL = 0x0,
    /// Clock control register
    SD_CKCR = 0x04,
    /// Time out register
    SD_TMOR = 0x08,
    /// Bus width register
    SD_BWDR = 0x0c,
    /// Block size register
    SD_BKSR = 0x10,
    /// Byte count register
    SD_BYCR = 0x14,
    /// Command register
    SD_CMDR = 0x18,
    /// Command argument register
    SD_CAGR = 0x1c,
    ///  Response 0 register
    SD_RESP0 = 0x20,
    ///  Response 1 register
    SD_RESP1 = 0x24,
    ///  Response 2 register
    SD_RESP2 = 0x28,
    ///  Response 3 register
    SD_RESP3 = 0x2c,
    /// Interrupt mask register
    SD_IMKR = 0x30,
    /// Masked interrupt status register
    SD_MISR = 0x34,
    /// Raw interrupt status register
    SD_RISR = 0x38,
    /// Status register
    SD_STAR = 0x3C,
    /// FIFO Water Level register
    SD_FWLR = 0x40,
    /// FIFO Function Select register
    SD_FUNS = 0x44,
    /// Auto command 12 argument
    SD_A12A = 0x58,
    /// SD NewTiming Set Register
    SD_NTSR = 0x5C,
    /// SD NewTiming Set Debug Register
    SD_SDBG = 0x60,
    /// Hardware Reset Register
    SD_HWRST = 0x78,
    /// BUS Mode Control
    SD_DMAC = 0x80,
    /// Descriptor List Base Address
    SD_DLBA = 0x84,
    /// DMAC Status
    SD_IDST = 0x88,
    /// DMAC Interrupt Enable
    SD_IDIE = 0x8C,
    /// Card Threshold Control register
    SD_THLDC = 0x100,
    /// eMMC4.41 DDR Start Bit Detection Control
    SD_DSBD = 0x10C,
    /// CRC status from card/eMMC in write operation
    SD_RES_CRC = 0x110,
    /// CRC Data7 from card/eMMC
    SD_DATA7_CRC = 0x114,
    /// CRC Data7 from card/eMMC
    SD_DATA6_CRC = 0x118,
    /// CRC Data7 from card/eMMC
    SD_DATA5_CRC = 0x11C,
    /// CRC Data7 from card/eMMC
    SD_DATA4_CRC = 0x120,
    /// CRC Data7 from card/eMMC
    SD_DATA3_CRC = 0x124,
    /// CRC Data7 from card/eMMC
    SD_DATA2_CRC = 0x128,
    /// CRC Data7 from card/eMMC
    SD_DATA1_CRC = 0x12C,
    /// CRC Data7 from card/eMMC
    SD_DATA0_CRC = 0x130,
    /// Response CRC from card/eMMC  
    SD_CRC_STA = 0x134,
    /// Read/Write FIFO,
    SD_FIFO = 0x200,
}

pub struct SDCard {}

impl SDCard {
    pub fn get_status(&self) -> u32 {
        rw::memory_read(SD_MMC0_BASE + SDRegistersOffsets::SD_STAR as u32)
    }
}
