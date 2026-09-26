//! insn 解码访问器一族（r7-insn14 单宿主收敛）：`luau_insn_a/b/c/d/e/op`、
//! `luau_insn_aux_a/b/kb/kv/kv16/not/slot` 共 13 枚 const fn 与常量
//! `LUAU_INSN_FBSLOT_SEALED`。原一函数一文件碎片归并，签名与函数体逐行保真。

use crate::records::instruction::Instruction;

#[inline(always)]
pub const fn luau_insn_a(insn: u32) -> u32 {
  Instruction(insn).a() as u32
}

#[inline(always)]
pub const fn luau_insn_b(insn: u32) -> u32 {
  Instruction(insn).b() as u32
}

#[inline(always)]
pub const fn luau_insn_c(insn: u32) -> u32 {
  Instruction(insn).c() as u32
}

#[inline(always)]
pub const fn luau_insn_d(insn: u32) -> i32 {
  Instruction(insn).d() as i32
}

#[inline(always)]
pub const fn luau_insn_e(insn: u32) -> i32 {
  Instruction(insn).e()
}

#[inline(always)]
pub const fn luau_insn_op(insn: u32) -> u32 {
  Instruction(insn).op() as u32
}

#[inline(always)]
pub const fn luau_insn_aux_a(aux: u32) -> u32 {
  Instruction(aux).aux_a() as u32
}

#[inline(always)]
pub const fn luau_insn_aux_b(aux: u32) -> u32 {
  Instruction(aux).aux_b() as u32
}

#[inline(always)]
pub const fn luau_insn_aux_kb(aux: u32) -> u32 {
  Instruction(aux).aux_kb()
}

#[inline(always)]
pub const fn luau_insn_aux_kv(aux: u32) -> u32 {
  Instruction(aux).aux_kv()
}

#[inline(always)]
pub const fn luau_insn_aux_kv16(aux: u32) -> u32 {
  Instruction(aux).aux_kv16() as u32
}

#[inline(always)]
pub const fn luau_insn_aux_not(aux: u32) -> u32 {
  Instruction(aux).aux_not()
}

#[inline(always)]
pub const fn luau_insn_aux_slot(aux: u32) -> u32 {
  Instruction(aux).aux_slot()
}

pub const LUAU_INSN_FBSLOT_SEALED: u32 = 0xFFFFFFFF;
