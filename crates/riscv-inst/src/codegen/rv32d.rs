//! Autogenerated by riscv-inst-codegen
//! DO NOT EDIT

#[allow(unused_imports)]
use super::{FReg, Reg};
pub enum Rv32d {
    Fld(Fld),
    Fsd(Fsd),
    FmaddD(FmaddD),
    FmsubD(FmsubD),
    FnmsubD(FnmsubD),
    FnmaddD(FnmaddD),
    FaddD(FaddD),
    FsubD(FsubD),
    FmulD(FmulD),
    FdivD(FdivD),
    FsgnjD(FsgnjD),
    FsgnjnD(FsgnjnD),
    FsgnjxD(FsgnjxD),
    FminD(FminD),
    FmaxD(FmaxD),
    FcvtSD(FcvtSD),
    FcvtDS(FcvtDS),
    FsqrtD(FsqrtD),
    FleD(FleD),
    FltD(FltD),
    FeqD(FeqD),
    FcvtWD(FcvtWD),
    FcvtWuD(FcvtWuD),
    FcvtDW(FcvtDW),
    FcvtDWu(FcvtDWu),
    FclassD(FclassD),
}
pub struct Fld(pub u32);
impl Fld {
    #[inline]
    pub const fn frd(&self) -> FReg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rs1(&self) -> Reg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn imm(&self) -> i32 {
        ((((self.0 >> 20) & 0b111111111111) << 21) as i32) >> 21
    }
}
pub struct Fsd(pub u32);
impl Fsd {
    #[inline]
    pub const fn rs1(&self) -> Reg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs2(&self) -> FReg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn imm(&self) -> i32 {
        (((((self.0 >> 20) & 0b111111100000) | ((self.0 >> 7) & 0b11111)) << 21)
            as i32)
            >> 21
    }
}
pub struct FmaddD(pub u32);
impl FmaddD {
    #[inline]
    pub const fn frd(&self) -> FReg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs2(&self) -> FReg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs3(&self) -> FReg {
        {
            let acc = (self.0 >> 27) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rm(&self) -> u32 {
        (self.0 >> 12) & 0b111
    }
}
pub struct FmsubD(pub u32);
impl FmsubD {
    #[inline]
    pub const fn frd(&self) -> FReg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs2(&self) -> FReg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs3(&self) -> FReg {
        {
            let acc = (self.0 >> 27) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rm(&self) -> u32 {
        (self.0 >> 12) & 0b111
    }
}
pub struct FnmsubD(pub u32);
impl FnmsubD {
    #[inline]
    pub const fn frd(&self) -> FReg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs2(&self) -> FReg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs3(&self) -> FReg {
        {
            let acc = (self.0 >> 27) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rm(&self) -> u32 {
        (self.0 >> 12) & 0b111
    }
}
pub struct FnmaddD(pub u32);
impl FnmaddD {
    #[inline]
    pub const fn frd(&self) -> FReg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs2(&self) -> FReg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs3(&self) -> FReg {
        {
            let acc = (self.0 >> 27) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rm(&self) -> u32 {
        (self.0 >> 12) & 0b111
    }
}
pub struct FaddD(pub u32);
impl FaddD {
    #[inline]
    pub const fn frd(&self) -> FReg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs2(&self) -> FReg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rm(&self) -> u32 {
        (self.0 >> 12) & 0b111
    }
}
pub struct FsubD(pub u32);
impl FsubD {
    #[inline]
    pub const fn frd(&self) -> FReg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs2(&self) -> FReg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rm(&self) -> u32 {
        (self.0 >> 12) & 0b111
    }
}
pub struct FmulD(pub u32);
impl FmulD {
    #[inline]
    pub const fn frd(&self) -> FReg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs2(&self) -> FReg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rm(&self) -> u32 {
        (self.0 >> 12) & 0b111
    }
}
pub struct FdivD(pub u32);
impl FdivD {
    #[inline]
    pub const fn frd(&self) -> FReg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs2(&self) -> FReg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rm(&self) -> u32 {
        (self.0 >> 12) & 0b111
    }
}
pub struct FsgnjD(pub u32);
impl FsgnjD {
    #[inline]
    pub const fn frd(&self) -> FReg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs2(&self) -> FReg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
}
pub struct FsgnjnD(pub u32);
impl FsgnjnD {
    #[inline]
    pub const fn frd(&self) -> FReg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs2(&self) -> FReg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
}
pub struct FsgnjxD(pub u32);
impl FsgnjxD {
    #[inline]
    pub const fn frd(&self) -> FReg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs2(&self) -> FReg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
}
pub struct FminD(pub u32);
impl FminD {
    #[inline]
    pub const fn frd(&self) -> FReg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs2(&self) -> FReg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
}
pub struct FmaxD(pub u32);
impl FmaxD {
    #[inline]
    pub const fn frd(&self) -> FReg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs2(&self) -> FReg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
}
pub struct FcvtSD(pub u32);
impl FcvtSD {
    #[inline]
    pub const fn frd(&self) -> FReg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rm(&self) -> u32 {
        (self.0 >> 12) & 0b111
    }
}
pub struct FcvtDS(pub u32);
impl FcvtDS {
    #[inline]
    pub const fn frd(&self) -> FReg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rm(&self) -> u32 {
        (self.0 >> 12) & 0b111
    }
}
pub struct FsqrtD(pub u32);
impl FsqrtD {
    #[inline]
    pub const fn frd(&self) -> FReg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rm(&self) -> u32 {
        (self.0 >> 12) & 0b111
    }
}
pub struct FleD(pub u32);
impl FleD {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs2(&self) -> FReg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
}
pub struct FltD(pub u32);
impl FltD {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs2(&self) -> FReg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
}
pub struct FeqD(pub u32);
impl FeqD {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs2(&self) -> FReg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
}
pub struct FcvtWD(pub u32);
impl FcvtWD {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rm(&self) -> u32 {
        (self.0 >> 12) & 0b111
    }
}
pub struct FcvtWuD(pub u32);
impl FcvtWuD {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rm(&self) -> u32 {
        (self.0 >> 12) & 0b111
    }
}
pub struct FcvtDW(pub u32);
impl FcvtDW {
    #[inline]
    pub const fn frd(&self) -> FReg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rs1(&self) -> Reg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rm(&self) -> u32 {
        (self.0 >> 12) & 0b111
    }
}
pub struct FcvtDWu(pub u32);
impl FcvtDWu {
    #[inline]
    pub const fn frd(&self) -> FReg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rs1(&self) -> Reg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rm(&self) -> u32 {
        (self.0 >> 12) & 0b111
    }
}
pub struct FclassD(pub u32);
impl FclassD {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
}
