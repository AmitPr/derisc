//! Autogenerated by riscv-inst-codegen
//! DO NOT EDIT

#[allow(unused_imports)]
use super::{FReg, Reg};
pub enum Rv64i {
    Lui(Lui),
    Auipc(Auipc),
    Jal(Jal),
    Jalr(Jalr),
    Beq(Beq),
    Bne(Bne),
    Blt(Blt),
    Bge(Bge),
    Bltu(Bltu),
    Bgeu(Bgeu),
    Lb(Lb),
    Lh(Lh),
    Lw(Lw),
    Lbu(Lbu),
    Lhu(Lhu),
    Sb(Sb),
    Sh(Sh),
    Sw(Sw),
    Addi(Addi),
    Slti(Slti),
    Sltiu(Sltiu),
    Xori(Xori),
    Ori(Ori),
    Andi(Andi),
    Add(Add),
    Sub(Sub),
    Sll(Sll),
    Slt(Slt),
    Sltu(Sltu),
    Xor(Xor),
    Srl(Srl),
    Sra(Sra),
    Or(Or),
    And(And),
    Fence(Fence),
    FenceI(FenceI),
    Lwu(Lwu),
    Ld(Ld),
    Sd(Sd),
    Slli(Slli),
    Srli(Srli),
    Srai(Srai),
    Addiw(Addiw),
    Slliw(Slliw),
    Srliw(Srliw),
    Sraiw(Sraiw),
    Addw(Addw),
    Subw(Subw),
    Sllw(Sllw),
    Srlw(Srlw),
    Sraw(Sraw),
}
pub struct Lui(pub u32);
impl Lui {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn imm(&self) -> i32 {
        (((self.0 & 0b11111111111111111111000000000000) << 1) as i32) >> 1
    }
}
pub struct Auipc(pub u32);
impl Auipc {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn imm(&self) -> i32 {
        (((self.0 & 0b11111111111111111111000000000000) << 1) as i32) >> 1
    }
}
pub struct Jal(pub u32);
impl Jal {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn imm(&self) -> i32 {
        (((((self.0 >> 11) & 0b100000000000000000000)
            | ((self.0 >> 20) & 0b11111111110)
            | ((self.0 >> 9) & 0b100000000000)
            | (self.0 & 0b11111111000000000000))
            << 12) as i32)
            >> 12
    }
}
pub struct Jalr(pub u32);
impl Jalr {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
pub struct Beq(pub u32);
impl Beq {
    #[inline]
    pub const fn rs1(&self) -> Reg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn imm(&self) -> i32 {
        ((((((self.0 >> 19) & 0b1000000000000)
            | ((self.0 >> 20) & 0b11111100000))
            | (((self.0 >> 7) & 0b11110) | ((self.0 << 4) & 0b100000000000)))
            << 20) as i32)
            >> 20
    }
}
pub struct Bne(pub u32);
impl Bne {
    #[inline]
    pub const fn rs1(&self) -> Reg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn imm(&self) -> i32 {
        ((((((self.0 >> 19) & 0b1000000000000)
            | ((self.0 >> 20) & 0b11111100000))
            | (((self.0 >> 7) & 0b11110) | ((self.0 << 4) & 0b100000000000)))
            << 20) as i32)
            >> 20
    }
}
pub struct Blt(pub u32);
impl Blt {
    #[inline]
    pub const fn rs1(&self) -> Reg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn imm(&self) -> i32 {
        ((((((self.0 >> 19) & 0b1000000000000)
            | ((self.0 >> 20) & 0b11111100000))
            | (((self.0 >> 7) & 0b11110) | ((self.0 << 4) & 0b100000000000)))
            << 20) as i32)
            >> 20
    }
}
pub struct Bge(pub u32);
impl Bge {
    #[inline]
    pub const fn rs1(&self) -> Reg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn imm(&self) -> i32 {
        ((((((self.0 >> 19) & 0b1000000000000)
            | ((self.0 >> 20) & 0b11111100000))
            | (((self.0 >> 7) & 0b11110) | ((self.0 << 4) & 0b100000000000)))
            << 20) as i32)
            >> 20
    }
}
pub struct Bltu(pub u32);
impl Bltu {
    #[inline]
    pub const fn rs1(&self) -> Reg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn imm(&self) -> i32 {
        ((((((self.0 >> 19) & 0b1000000000000)
            | ((self.0 >> 20) & 0b11111100000))
            | (((self.0 >> 7) & 0b11110) | ((self.0 << 4) & 0b100000000000)))
            << 20) as i32)
            >> 20
    }
}
pub struct Bgeu(pub u32);
impl Bgeu {
    #[inline]
    pub const fn rs1(&self) -> Reg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn imm(&self) -> i32 {
        ((((((self.0 >> 19) & 0b1000000000000)
            | ((self.0 >> 20) & 0b11111100000))
            | (((self.0 >> 7) & 0b11110) | ((self.0 << 4) & 0b100000000000)))
            << 20) as i32)
            >> 20
    }
}
pub struct Lb(pub u32);
impl Lb {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
pub struct Lh(pub u32);
impl Lh {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
pub struct Lw(pub u32);
impl Lw {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
pub struct Lbu(pub u32);
impl Lbu {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
pub struct Lhu(pub u32);
impl Lhu {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
pub struct Sb(pub u32);
impl Sb {
    #[inline]
    pub const fn rs1(&self) -> Reg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn imm(&self) -> i32 {
        (((((self.0 >> 20) & 0b111111100000) | ((self.0 >> 7) & 0b11111)) << 21)
            as i32)
            >> 21
    }
}
pub struct Sh(pub u32);
impl Sh {
    #[inline]
    pub const fn rs1(&self) -> Reg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn imm(&self) -> i32 {
        (((((self.0 >> 20) & 0b111111100000) | ((self.0 >> 7) & 0b11111)) << 21)
            as i32)
            >> 21
    }
}
pub struct Sw(pub u32);
impl Sw {
    #[inline]
    pub const fn rs1(&self) -> Reg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn imm(&self) -> i32 {
        (((((self.0 >> 20) & 0b111111100000) | ((self.0 >> 7) & 0b11111)) << 21)
            as i32)
            >> 21
    }
}
pub struct Addi(pub u32);
impl Addi {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
pub struct Slti(pub u32);
impl Slti {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
pub struct Sltiu(pub u32);
impl Sltiu {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
pub struct Xori(pub u32);
impl Xori {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
pub struct Ori(pub u32);
impl Ori {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
pub struct Andi(pub u32);
impl Andi {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
pub struct Add(pub u32);
impl Add {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
}
pub struct Sub(pub u32);
impl Sub {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
}
pub struct Sll(pub u32);
impl Sll {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
}
pub struct Slt(pub u32);
impl Slt {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
}
pub struct Sltu(pub u32);
impl Sltu {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
}
pub struct Xor(pub u32);
impl Xor {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
}
pub struct Srl(pub u32);
impl Srl {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
}
pub struct Sra(pub u32);
impl Sra {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
}
pub struct Or(pub u32);
impl Or {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
}
pub struct And(pub u32);
impl And {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
}
pub struct Fence(pub u32);
impl Fence {
    #[inline]
    pub const fn pred(&self) -> u32 {
        (self.0 >> 24) & 0b1111
    }
    #[inline]
    pub const fn succ(&self) -> u32 {
        (self.0 >> 20) & 0b1111
    }
}
pub struct FenceI(pub u32);
impl FenceI {}
pub struct Lwu(pub u32);
impl Lwu {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
pub struct Ld(pub u32);
impl Ld {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
pub struct Sd(pub u32);
impl Sd {
    #[inline]
    pub const fn rs1(&self) -> Reg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn imm(&self) -> i32 {
        (((((self.0 >> 20) & 0b111111100000) | ((self.0 >> 7) & 0b11111)) << 21)
            as i32)
            >> 21
    }
}
pub struct Slli(pub u32);
impl Slli {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn shamt(&self) -> u32 {
        (self.0 >> 20) & 0b111111
    }
}
pub struct Srli(pub u32);
impl Srli {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn shamt(&self) -> u32 {
        (self.0 >> 20) & 0b111111
    }
}
pub struct Srai(pub u32);
impl Srai {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn shamt(&self) -> u32 {
        (self.0 >> 20) & 0b111111
    }
}
pub struct Addiw(pub u32);
impl Addiw {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
pub struct Slliw(pub u32);
impl Slliw {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn shamt(&self) -> u32 {
        (self.0 >> 20) & 0b11111
    }
}
pub struct Srliw(pub u32);
impl Srliw {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn shamt(&self) -> u32 {
        (self.0 >> 20) & 0b11111
    }
}
pub struct Sraiw(pub u32);
impl Sraiw {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn shamt(&self) -> u32 {
        (self.0 >> 20) & 0b11111
    }
}
pub struct Addw(pub u32);
impl Addw {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
}
pub struct Subw(pub u32);
impl Subw {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
}
pub struct Sllw(pub u32);
impl Sllw {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
}
pub struct Srlw(pub u32);
impl Srlw {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
}
pub struct Sraw(pub u32);
impl Sraw {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
}
