#![allow(dead_code)]

#[derive(Copy, Clone, Debug)]
pub enum Op {
    NULL,
    CLS,
    RET,
    JMP { address: u16 },
    EXEC { address: u16 },
    SEV { reg: u16, val: u16 },
    SNEV { reg: u16, val: u16 },
    SE { reg: u16, val: u16 },
    SETV { reg: u16, val: u16 },
    ADDV { reg: u16, val: u16 },
    SET { reg: u16, val: u16 },
    OR { reg0: u16, reg1: u16 },
    AND { reg0: u16, reg1: u16 },
    XOR { reg0: u16, reg1: u16 },
    ADD { reg0: u16, reg1: u16 },
    SUBL { reg0: u16, reg1: u16 },
    SHR { reg: u16 },
    SUBR { reg0: u16, reg1: u16 },
    SHL { reg: u16 },
    SNE { reg: u16, val: u16 },
    LIV { val: u16 },
    RAND { reg: u16, and: u16 },
    DRAW { x: u16, y: u16, val: u16 },
    SKP { key: u16 },
    SNKP { key: u16 },
    LDT { reg: u16 },
    WKP { reg: u16 },
    SDT { reg: u16 },
    SST { reg: u16 },
    LI { sprite: u16 },
    SBCD { val: u16 },
    SRI { end_reg: u16 },
    LRI { end_reg: u16 },
}

impl Op {
    pub fn from_bin(bin: u16) -> Self {
        match (bin & 0xF000) >> 12 {
            0x0 => {
                return match bin & 0xFFFF {
                    0x00E0 => Self::CLS,
                    0x00EE => Self::RET,
                    _ => Self::NULL,
                }
            }
            0x1 => Self::JMP { address: bin & 0x0FFF },
            0x2 => Self::EXEC { address: bin & 0x0FFF },
            0x3 => Self::SEV { reg: (bin & 0x0F00) >> 8, val: bin & 0x00FF },
            0x4 => Self::SNEV { reg: (bin & 0x0F00) >> 8, val: bin & 0x00FF },
            0x5 => Self::SE { reg: (bin & 0x0F00) >> 8, val: (bin & 0x00F0) >> 4 },
            0x6 => Self::SETV { reg: (bin & 0x0F00) >> 8, val: bin & 0x00FF },
            0x7 => Self::ADDV { reg: (bin & 0x0F00) >> 8, val: bin & 0x00FF },
            0x8 => {
                return match bin & 0x000F {
                    0x0 => Self::SET { reg: (bin & 0x0F00) >> 8, val: (bin & 0x00F0) >> 4 },
                    0x1 => Self::OR { reg0: (bin & 0x0F00) >> 8, reg1: (bin & 0x00F0) >> 4 },
                    0x2 => Self::AND { reg0: (bin & 0x0F00) >> 8, reg1: (bin & 0x00F0) >> 4 },
                    0x3 => Self::XOR { reg0: (bin & 0x0F00) >> 8, reg1: (bin & 0x00F0) >> 4 },
                    0x4 => Self::ADD { reg0: (bin & 0x0F00) >> 8, reg1: (bin & 0x00F0) >> 4 },
                    0x5 => Self::SUBL { reg0: (bin & 0x0F00) >> 8, reg1: (bin & 0x00F0) >> 4 },
                    0x6 => Self::SHR { reg: (bin & 0x0F00) >> 8 },
                    0x7 => Self::SUBR { reg0: (bin & 0x0F00) >> 8, reg1: (bin & 0x00F0) >> 4 },
                    0xE => Self::SHL { reg: (bin & 0x0F00) >> 8 },
                    _ => Self::NULL,
                }
            }
            0x9 => Self::SNE { reg: (bin & 0x0F00) >> 8, val: (bin & 0x00F) >> 4 },
            0xA => Self::LIV { val: bin & 0x0FFF },
            0xC => Self::RAND { reg: (bin & 0x0F00) >> 8, and: bin & 0x00FF },
            0xD => Self::DRAW { val: (bin & 0x0F00) >> 8, x: (bin & 0x00F0) >> 4, y: bin & 0x000F },
            0xE => {
                return match bin & 0x00FF {
                    0x9E => Self::SKP { key: (bin & 0x0F00) >> 8 },
                    0xA1 => Self::SNKP { key: (bin & 0x0F00) >> 8 },
                    _ => Self::NULL,
                }
            }
            0xF => {
                return match bin & 0x00FF {
                    0x07 => Self::LDT { reg: (bin & 0x0F00) >> 8 },
                    0x0A => Self::WKP { reg: (bin & 0x0F00) >> 8 },
                    0x15 => Self::SDT { reg: (bin & 0x0F00) >> 8 },
                    0x18 => Self::SST { reg: (bin & 0x0F00) >> 8 },
                    0x29 => Self::LI { sprite: (bin & 0x0F00) >> 8 },
                    0x33 => Self::SBCD { val: (bin & 0x0F00) >> 8 },
                    0x55 => Self::SRI { end_reg: (bin & 0x0F00) >> 8 },
                    0x65 => Self::LRI { end_reg: (bin & 0x0F00) >> 8 },
                    _ => Self::NULL,
                }
            }
            _ => Self::NULL,
        }
    }
}