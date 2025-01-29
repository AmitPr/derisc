use riscv_inst::{
    codegen::rv32::{Rv32, Rv32a, Rv32c, Rv32i, Rv32m, Rv32s},
    Reg,
};

use crate::{
    error::{HartError, MachineError, MemoryAccess, MemoryError},
    machine::{Kernel, StepResult},
    memory::Memory,
};

/// A simple CPU for RV32I instructions
pub struct Hart32 {
    regs: [u32; 32],
    csrs: [u32; 4096],
    pub pc: u32,
    pub inst_count: u64,
    /// Atomic memory reservation set on this hart
    pub amo_rsv: Option<u32>,
}

impl Hart32 {
    pub fn new() -> Self {
        Hart32 {
            regs: [0; 32],
            csrs: [0; 4096],
            pc: 0,
            inst_count: 0,
            amo_rsv: None,
        }
    }

    /// Read a register. x0 is always 0 in RISC-V.
    #[inline(always)]
    pub const fn get_reg(&self, r: Reg) -> u32 {
        let idx = r as usize;
        self.regs[if idx == 0 { 0 } else { idx }]
    }

    /// Write a register (except x0).
    #[inline(always)]
    pub const fn set_reg(&mut self, r: Reg, val: u32) {
        let idx = r as usize;
        self.regs[idx] = if idx == 0 { 0 } else { val };
    }

    /// Dump registers to stdout
    pub fn regs(&self) -> impl Iterator<Item = (Reg, u32)> + use<'_> {
        (0..32).map(|i| (unsafe { Reg::from_u5(i as u8) }, self.regs[i]))
    }

    pub fn regs_range(&self, start: Reg, end: Reg) -> impl Iterator<Item = (Reg, u32)> + use<'_> {
        (start as usize..=end as usize).map(|i| (unsafe { Reg::from_u5(i as u8) }, self.regs[i]))
    }

    pub fn step<K: Kernel>(
        &mut self,
        mem: &mut Memory,
        kernel: &mut K,
    ) -> Result<StepResult, MachineError<K::Error>> {
        let inst = mem.load::<u32>(self.pc);
        let op = Rv32::parse(inst).ok_or(HartError::InvalidInstruction {
            addr: self.pc,
            inst,
        })?;
        let inc = if matches!(op, Rv32::Rv32c(_)) { 2 } else { 4 };

        let mut next_pc = self.pc.wrapping_add(inc);

        macro_rules! reg {
            ($reg: expr) => {
                self.get_reg($reg)
            };
            ($reg: expr, $val: expr) => {
                self.set_reg($reg, $val as u32)
            };
        }

        macro_rules! reg_imm_op {
            (|$inst:ident.$rs1:ident, $inst2:ident.$imm:ident| $body:expr) => {{
                let $rs1 = reg!($inst.$rs1());
                let $imm = $inst2.$imm();
                reg!($inst.rd(), { $body } as u32);
            }};
        }

        macro_rules! imm_op {
            (|$inst:ident.$imm:ident| $body:expr) => {{
                let $imm = $inst.$imm();
                reg!($inst.rd(), { $body } as u32);
            }};
        }

        macro_rules! reg_reg_op {
            (|$inst:ident.$rs1:ident, $inst2:ident.$rs2:ident| $body:expr) => {{
                let $rs1 = reg!($inst.$rs1());
                let $rs2 = reg!($inst2.$rs2());
                reg!($inst.rd(), { $body } as u32);
            }};
        }

        macro_rules! store_op {
            (|$inst:ident.$rs1:ident, $inst2:ident.$rs2:ident, $addr:ident| $body:expr) => {{
                let $rs1 = reg!($inst.$rs1());
                let $rs2 = reg!($inst2.$rs2());
                let $addr = $rs1.wrapping_add_signed($inst.imm());
                $body;
            }};
        }

        macro_rules! branch_op {
            (|$inst:ident.$rs1:ident, $inst2:ident.$rs2:ident| $body:expr) => {{
                let $rs1 = reg!($inst.$rs1());
                let $rs2 = reg!($inst2.$rs2());
                let offset = $inst.imm();
                if $body {
                    next_pc = self.pc.wrapping_add_signed(offset);
                }
            }};
        }

        macro_rules! csr_op {
            (|$inst:ident.$csr:ident, $inst2:ident.$rs1:ident| $body:expr) => {{
                let $csr = $inst.$csr() as usize;
                let $rs1 = reg!($inst2.$rs1());
                reg!($inst.rd(), self.csrs[$csr]);
                $body;
            }};
        }

        macro_rules! csr_imm_op {
            (|$inst:ident.$csr:ident, $inst2:ident.$imm:ident| $body:expr) => {{
                let $csr = $inst.$csr() as usize;
                let $imm = $inst2.$imm();
                reg!($inst.rd(), self.csrs[$csr]);
                $body;
            }};
        }

        macro_rules! amo_op {
            (|$inst:ident, $old:ident, $rs2:ident| $body:expr) => {{
                let addr = reg!($inst.rs1());
                if addr & 3 != 0 {
                    return Err(MemoryError::UnalignedMemoryAccess {
                        access: MemoryAccess::Load,
                        addr,
                        required: 4,
                    }
                    .into());
                }
                let $old = mem.load::<u32>(addr);
                reg!($inst.rd(), $old);
                let $rs2 = reg!($inst.rs2());
                mem.store::<u32>(addr, $body as u32);
            }};
        }

        match op {
            Rv32::Rv32i(i) => match i {
                Rv32i::Lui(lui) => imm_op!(|lui.imm| imm),
                Rv32i::Auipc(auipc) => imm_op!(|auipc.imm| self.pc.wrapping_add_signed(imm)),
                Rv32i::Jal(jal) => imm_op!(|jal.imm| {
                    let res = next_pc;
                    next_pc = self.pc.wrapping_add_signed(imm);
                    res
                }),
                Rv32i::Jalr(jalr) => reg_imm_op!(|jalr.rs1, jalr.imm| {
                    let res = next_pc;
                    next_pc = rs1.wrapping_add_signed(imm);
                    res
                }),
                Rv32i::Beq(beq) => branch_op!(|beq.rs1, beq.rs2| rs1 == rs2),
                Rv32i::Bne(bne) => branch_op!(|bne.rs1, bne.rs2| rs1 != rs2),
                Rv32i::Blt(blt) => branch_op!(|blt.rs1, blt.rs2| (rs1 as i32) < (rs2 as i32)),
                Rv32i::Bge(bge) => branch_op!(|bge.rs1, bge.rs2| (rs1 as i32) >= (rs2 as i32)),
                Rv32i::Bltu(bltu) => branch_op!(|bltu.rs1, bltu.rs2| rs1 < rs2),
                Rv32i::Bgeu(bgeu) => branch_op!(|bgeu.rs1, bgeu.rs2| rs1 >= rs2),
                Rv32i::Lb(lb) => reg_imm_op!(
                    |lb.rs1, lb.imm| mem.load::<i8>(rs1.wrapping_add_signed(imm)) as i32
                ),
                Rv32i::Lh(lh) => reg_imm_op!(
                    |lh.rs1, lh.imm| mem.load::<i16>(rs1.wrapping_add_signed(imm)) as i32
                ),
                Rv32i::Lw(lw) => reg_imm_op!(
                    |lw.rs1, lw.imm| mem.load::<u32>(rs1.wrapping_add_signed(imm))
                ),
                Rv32i::Lbu(lbu) => reg_imm_op!(
                    |lbu.rs1, lbu.imm| mem.load::<u8>(rs1.wrapping_add_signed(imm))
                ),
                Rv32i::Lhu(lhu) => reg_imm_op!(
                    |lhu.rs1, lhu.imm| mem.load::<u16>(rs1.wrapping_add_signed(imm))
                ),
                Rv32i::Sb(sb) => {
                    store_op!(|sb.rs1, sb.rs2, addr| mem.store::<u8>(addr, rs2 as u8))
                }
                Rv32i::Sh(sh) => {
                    store_op!(|sh.rs1, sh.rs2, addr| mem.store::<u16>(addr, rs2 as u16))
                }
                Rv32i::Sw(sw) => {
                    store_op!(|sw.rs1, sw.rs2, addr| mem.store::<u32>(addr, rs2))
                }
                Rv32i::Addi(addi) => {
                    reg_imm_op!(|addi.rs1, addi.imm| rs1.wrapping_add_signed(imm))
                }
                Rv32i::Slti(slti) => reg_imm_op!(|slti.rs1, slti.imm| (rs1 as i32) < imm),
                Rv32i::Sltiu(sltiu) => reg_imm_op!(|sltiu.rs1, sltiu.imm| rs1 < (imm as u32)),
                Rv32i::Xori(xori) => reg_imm_op!(|xori.rs1, xori.imm| rs1 ^ (imm as u32)),
                Rv32i::Ori(ori) => reg_imm_op!(|ori.rs1, ori.imm| rs1 | (imm as u32)),
                Rv32i::Andi(andi) => reg_imm_op!(|andi.rs1, andi.imm| rs1 & (imm as u32)),
                Rv32i::Slli(slli) => reg_imm_op!(|slli.rs1, slli.shamt| rs1 << shamt),
                Rv32i::Srli(srli) => reg_imm_op!(|srli.rs1, srli.shamt| rs1 >> shamt),
                Rv32i::Srai(srai) => reg_imm_op!(|srai.rs1, srai.shamt| rs1 as i32 >> shamt),
                Rv32i::Add(add) => reg_reg_op!(|add.rs1, add.rs2| rs1.wrapping_add(rs2)),
                Rv32i::Sub(sub) => reg_reg_op!(|sub.rs1, sub.rs2| rs1.wrapping_sub(rs2)),
                Rv32i::Sll(sll) => reg_reg_op!(|sll.rs1, sll.rs2| rs1 << (rs2 & 0x1f)),
                Rv32i::Slt(slt) => reg_reg_op!(|slt.rs1, slt.rs2| (rs1 as i32) < (rs2 as i32)),
                Rv32i::Sltu(sltu) => reg_reg_op!(|sltu.rs1, sltu.rs2| rs1 < rs2),
                Rv32i::Xor(xor) => reg_reg_op!(|xor.rs1, xor.rs2| rs1 ^ rs2),
                Rv32i::Srl(srl) => reg_reg_op!(|srl.rs1, srl.rs2| rs1 >> (rs2 & 0x1f)),
                Rv32i::Sra(sra) => {
                    reg_reg_op!(|sra.rs1, sra.rs2| (rs1 as i32 >> (rs2 & 0x1f)) as u32)
                }
                Rv32i::Or(or) => reg_reg_op!(|or.rs1, or.rs2| rs1 | rs2),
                Rv32i::And(and) => reg_reg_op!(|and.rs1, and.rs2| rs1 & rs2),
                Rv32i::Fence(_) => {}
                Rv32i::FenceI(_) => {}
                Rv32i::Ecall(_) => match kernel.syscall(self, mem)? {
                    StepResult::Ok => {}
                    res => return Ok(res),
                },
                Rv32i::Ebreak(_) => match kernel.ebreak(self, mem)? {
                    StepResult::Ok => {}
                    res => return Ok(res),
                },
                Rv32i::Unimp(_) => {
                    return Err(HartError::IllegalInstruction { addr: self.pc, op }.into())
                }
            },
            Rv32::Rv32m(m) => match m {
                Rv32m::Mul(mul) => reg_reg_op!(|mul.rs1, mul.rs2| rs1.wrapping_mul(rs2)),
                Rv32m::Mulh(mulh) => reg_reg_op!(
                    |mulh.rs1, mulh.rs2| (rs1 as i32 as i64).wrapping_mul(rs2 as i32 as i64) >> 32
                ),
                Rv32m::Mulhsu(mulhsu) => reg_reg_op!(
                    |mulhsu.rs1, mulhsu.rs2| (((rs1 as i32 as i64) * (rs2 as i64)) >> 32) as u32
                ),
                Rv32m::Mulhu(mulhu) => reg_reg_op!(
                    |mulhu.rs1, mulhu.rs2| ((rs1 as u64 * rs2 as u64) >> 32) as u32
                ),
                Rv32m::Div(div) => reg_reg_op!(
                    |div.rs1, div.rs2| {
                        let rs1 = rs1 as i32;
                        let rs2 = rs2 as i32;
                        if rs2 == 0 {
                            // Division by zero returns -1
                            u32::MAX
                        } else if rs1 == i32::MIN && rs2 == -1 {
                            // Handle signed division overflow
                            rs1 as u32
                        } else {
                            rs1.wrapping_div(rs2) as u32
                        }
                    }
                ),
                Rv32m::Divu(divu) => reg_reg_op!(
                    |divu.rs1, divu.rs2| {
                        let rs2 = rs2 as u32;
                        if rs2 == 0 {
                            // Division by zero returns MAX
                            u32::MAX
                        } else {
                            rs1.wrapping_div(rs2)
                        }
                    }
                ),
                Rv32m::Rem(rem) => reg_reg_op!(
                    |rem.rs1, rem.rs2| {
                        let rs1 = rs1 as i32;
                        let rs2 = rs2 as i32;
                        if rs2 == 0 {
                            // Remainder of division by zero returns the dividend
                            rs1 as u32
                        } else if rs1 == i32::MIN && rs2 == -1 {
                            // Handle signed division overflow - remainder is 0
                            0
                        } else {
                            rs1.wrapping_rem(rs2) as u32
                        }
                    }
                ),
                Rv32m::Remu(remu) => reg_reg_op!(
                    |remu.rs1, remu.rs2| {
                        let rs2 = rs2 as u32;
                        if rs2 == 0 {
                            // Remainder of division by zero returns the dividend
                            rs1
                        } else {
                            rs1.wrapping_rem(rs2)
                        }
                    }
                ),
            },
            Rv32::Rv32s(s) => match s {
                Rv32s::Uret(_) => {
                    return Err(HartError::UnimplementedInstruction { addr: self.pc, op }.into())
                }
                Rv32s::Sret(_) => {
                    return Err(HartError::UnimplementedInstruction { addr: self.pc, op }.into())
                }
                Rv32s::Hret(_) => {
                    return Err(HartError::UnimplementedInstruction { addr: self.pc, op }.into())
                }
                Rv32s::Mret(_) => {
                    // TODO: Not erroring because the ISA tests use this.
                    // But we haven't implemented privilege levels yet.
                }
                Rv32s::Dret(_) => {
                    return Err(HartError::UnimplementedInstruction { addr: self.pc, op }.into())
                }
                Rv32s::SfenceVm(_) => {
                    return Err(HartError::UnimplementedInstruction { addr: self.pc, op }.into())
                }
                Rv32s::SfenceVma(_) => {
                    return Err(HartError::UnimplementedInstruction { addr: self.pc, op }.into())
                }
                Rv32s::Wfi(_) => {
                    return Err(HartError::UnimplementedInstruction { addr: self.pc, op }.into())
                }
                Rv32s::Csrrw(rw) => csr_op!(|rw.csr12, rw.rs1| self.csrs[csr12] = rs1),
                Rv32s::Csrrs(rs) => csr_op!(|rs.csr12, rs.rs1| self.csrs[csr12] |= rs1),
                Rv32s::Csrrc(rc) => csr_op!(|rc.csr12, rc.rs1| self.csrs[csr12] &= !rs1),
                Rv32s::Csrrwi(wi) => csr_imm_op!(|wi.csr12, wi.imm| self.csrs[csr12] = imm),
                Rv32s::Csrrsi(ri) => csr_imm_op!(|ri.csr12, ri.imm| self.csrs[csr12] |= imm),
                Rv32s::Csrrci(ci) => csr_imm_op!(|ci.csr12, ci.imm| self.csrs[csr12] &= !imm),
            },
            Rv32::Rv32a(a) => match a {
                // We don't care about reservation set on single-hart ( i think )
                Rv32a::LrW(lr_w) => {
                    let addr = reg!(lr_w.rs1());
                    if addr & 3 != 0 {
                        return Err(MemoryError::UnalignedMemoryAccess {
                            access: MemoryAccess::Load,
                            addr,
                            required: 4,
                        }
                        .into());
                    }
                    // TODO: Not sure if this is how the spec defines the
                    // "reservation set" for lr/sc
                    self.amo_rsv = Some(addr);
                    reg!(lr_w.rd(), mem.load::<u32>(addr));
                }
                Rv32a::ScW(sc_w) => {
                    let addr = reg!(sc_w.rs1());
                    if addr & 3 != 0 {
                        return Err(MemoryError::UnalignedMemoryAccess {
                            access: MemoryAccess::Store,
                            addr,
                            required: 4,
                        }
                        .into());
                    }
                    if self.amo_rsv.take() == Some(addr) {
                        mem.store::<u32>(addr, reg!(sc_w.rs2()));
                        reg!(sc_w.rd(), 0);
                    } else {
                        reg!(sc_w.rd(), 1);
                    }
                }
                Rv32a::AmoswapW(swap) => amo_op!(|swap, old, rs2| rs2),
                Rv32a::AmoaddW(add) => amo_op!(|add, old, rs2| old.wrapping_add(rs2)),
                Rv32a::AmoxorW(xor) => amo_op!(|xor, old, rs2| old ^ rs2),
                Rv32a::AmoorW(or) => amo_op!(|or, old, rs2| old | rs2),
                Rv32a::AmoandW(and) => amo_op!(|and, old, rs2| old & rs2),
                Rv32a::AmominW(min) => amo_op!(|min, old, rs2| (old as i32).min(rs2 as i32)),
                Rv32a::AmomaxW(max) => amo_op!(|max, old, rs2| (old as i32).max(rs2 as i32)),
                Rv32a::AmominuW(minu) => amo_op!(|minu, old, rs2| old.min(rs2)),
                Rv32a::AmomaxuW(maxu) => amo_op!(|maxu, old, rs2| old.max(rs2)),
            },
            Rv32::Rv32c(c) => match c {
                Rv32c::CAddi4spn(addi4spn) => {
                    let imm = addi4spn.imm();
                    let rd = addi4spn.rd();
                    self.set_reg(rd, reg!(Reg::Sp).wrapping_add(imm));
                }
                Rv32c::CLw(lw) => {
                    let addr = reg!(lw.rs1()).wrapping_add(lw.imm());
                    reg!(lw.rd(), mem.load::<u32>(addr));
                }
                Rv32c::CSw(sw) => {
                    let addr = reg!(sw.rs1()).wrapping_add(sw.imm());
                    mem.store::<u32>(addr, reg!(sw.rs2()));
                }
                Rv32c::CAddi(caddi) => {
                    let rs1rd = caddi.rs1rd();
                    self.set_reg(rs1rd, reg!(rs1rd).wrapping_add_signed(caddi.imm()));
                }
                Rv32c::CAddi16sp(caddi16sp) => {
                    let imm = caddi16sp.imm();
                    let rs1rd = caddi16sp.rs1rd();
                    self.set_reg(rs1rd, reg!(Reg::Sp).wrapping_add_signed(imm));
                }
                Rv32c::CLwsp(lwsp) => {
                    let addr = reg!(Reg::Sp).wrapping_add(lwsp.imm());
                    reg!(lwsp.rd(), mem.load::<u32>(addr));
                }
                Rv32c::CSwsp(swsp) => {
                    let addr = reg!(Reg::Sp).wrapping_add(swsp.imm());
                    mem.store::<u32>(addr, reg!(swsp.rs2()));
                }
                Rv32c::CNop(_) => {}
                Rv32c::CJal(cjal) => {
                    reg!(Reg::Ra, next_pc);
                    next_pc = self.pc.wrapping_add_signed(cjal.imm());
                }
                Rv32c::CLi(cli) => reg!(cli.rs1rd(), cli.imm()),
                Rv32c::CLui(clui) => reg!(clui.rd(), clui.imm()),
                Rv32c::CSrli(csrli) => {
                    let rd = csrli.rs1rd();
                    reg!(rd, reg!(rd) >> csrli.shamt());
                }
                Rv32c::CSrai(csrai) => {
                    let rd = csrai.rs1rd();
                    reg!(rd, (reg!(rd) as i32) >> csrai.shamt());
                }
                Rv32c::CAndi(candi) => {
                    let rd = candi.rs1rd();
                    reg!(rd, reg!(rd) & candi.imm() as u32);
                }
                Rv32c::CSub(csub) => {
                    let rs1rd = csub.rs1rd();
                    let rs2 = reg!(csub.rs2());
                    reg!(rs1rd, reg!(rs1rd).wrapping_sub(rs2));
                }
                Rv32c::CXor(cxor) => {
                    let rs1rd = cxor.rs1rd();
                    let rs2 = reg!(cxor.rs2());
                    reg!(rs1rd, reg!(rs1rd) ^ rs2);
                }
                Rv32c::COr(cor) => {
                    let rs1rd = cor.rs1rd();
                    let rs2 = reg!(cor.rs2());
                    reg!(rs1rd, reg!(rs1rd) | rs2);
                }
                Rv32c::CAnd(cand) => {
                    let rs1rd = cand.rs1rd();
                    let rs2 = reg!(cand.rs2());
                    reg!(rs1rd, reg!(rs1rd) & rs2);
                }
                Rv32c::CJ(cj) => {
                    next_pc = self.pc.wrapping_add_signed(cj.imm());
                }
                Rv32c::CBeqz(cbeqz) => {
                    if reg!(cbeqz.rs1()) == 0 {
                        next_pc = self.pc.wrapping_add_signed(cbeqz.imm());
                    }
                }
                Rv32c::CBnez(cbnez) => {
                    if reg!(cbnez.rs1()) != 0 {
                        next_pc = self.pc.wrapping_add_signed(cbnez.imm());
                    }
                }
                Rv32c::CSlli(cslli) => {
                    let rd = cslli.rs1rd();
                    reg!(rd, reg!(rd) << cslli.shamt());
                }
                Rv32c::CJr(cjr) => {
                    next_pc = reg!(cjr.rs1());
                }
                Rv32c::CMv(cmv) => reg!(cmv.rd(), reg!(cmv.rs2())),
                Rv32c::CEbreak(_) => match kernel.ebreak(self, mem)? {
                    StepResult::Ok => {}
                    res => return Ok(res),
                },
                Rv32c::CJalr(cjalr) => {
                    reg!(Reg::Ra, next_pc);
                    next_pc = reg!(cjalr.rs1());
                }
                Rv32c::CAdd(cadd) => {
                    let rs1rd = cadd.rs1rd();
                    reg!(rs1rd, reg!(rs1rd).wrapping_add(reg!(cadd.rs2())));
                }
                Rv32c::CUnimp(_) => {
                    return Err(HartError::IllegalInstruction { addr: self.pc, op }.into())
                }
            },
            _ => {
                return Err(HartError::UnimplementedInstruction { addr: self.pc, op }.into());
            }
        }

        self.inst_count += 1;
        self.pc = next_pc;

        Ok(StepResult::Ok)
    }
}

impl Default for Hart32 {
    fn default() -> Self {
        Self::new()
    }
}
