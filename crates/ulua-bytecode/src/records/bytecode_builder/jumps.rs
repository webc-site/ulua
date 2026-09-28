//! `BytecodeBuilder` 之 跳转折叠与展开（fold_jumps / expand_jumps）。

use core::{iter::repeat_n, mem};
use std::{vec, vec::Vec};

use ulua_common::{
  enums::luau_opcode::LuauOpcode, fflag, functions::get_op_length::get_op_length,
  macros::luau_assert::LUAU_ASSERT, records::instruction::Instruction,
};

use super::{BytecodeBuilder, K_MAX_JUMP_DISTANCE, insn};
use crate::records::{debug_local_bytecode_builder::DebugLocal, typed_local::TypedLocal};

// ── abs-r139：并自 `methods/bytecode_builder_expand_jumps.rs` ──
/// PC 区间可重映射的局部记录（`DebugLocal` / `TypedLocal` 共有形状）。
trait PcRangeRemap {
  fn pc_range(&mut self) -> (&mut u32, &mut u32);
}

impl PcRangeRemap for DebugLocal {
  #[inline]
  fn pc_range(&mut self) -> (&mut u32, &mut u32) {
    (&mut self.startpc, &mut self.endpc)
  }
}

impl PcRangeRemap for TypedLocal {
  #[inline]
  fn pc_range(&mut self) -> (&mut u32, &mut u32) {
    (&mut self.startpc, &mut self.endpc)
  }
}

/// endpc 为右开区间：先映射 endpc-1 再 +1，起点直接映射。
fn remap_pc_ranges<L: PcRangeRemap>(locals: &mut [L], remap: &[u32]) {
  for local in locals {
    let (startpc, endpc) = local.pc_range();
    *endpc = if *startpc != *endpc {
      remap[(*endpc - 1) as usize] + 1
    } else {
      remap[*endpc as usize]
    };
    *startpc = remap[*startpc as usize];
  }
}

impl<'a> BytecodeBuilder<'a> {
  /// 返回值：cpp 出参 `hasLongJumpError` — 展开后仍超出 JUMPX 24 位射程，
  /// 调用方（Compiler / BytecodeGraph）须按 cpp 语义报错或返回空串。
  pub fn expand_jumps(&mut self) -> bool {
    if !self.has_long_jumps {
      return false;
    }

    // we have some jump instructions that couldn't be patched which means their offset didn't fit into 16 bits
    // our strategy for replacing instructions is as follows: instead of
    //   OP jumpoffset
    // we will synthesize a jump trampoline before our instruction (note that jump offsets are relative to next instruction):
    //   JUMP +1
    //   JUMPX jumpoffset
    //   OP -2
    // the idea is that during forward execution, we will jump over JUMPX into OP; if OP decides to jump, it will jump to JUMPX
    // JUMPX can carry a 24-bit jump offset

    // jump trampolines expand the code size, which can increase existing jump distances.
    // because of this, we may need to expand jumps that previously fit into 16-bit just fine.
    // the worst-case expansion is 3x, so to be conservative we will repatch all jumps that have an offset >= 32767/3
    const K_MAX_JUMP_DISTANCE_CONSERVATIVE: i32 = 32767 / 3;

    // we will need to process jumps in order
    self.jumps.sort_by_key(|lhs| lhs.source);

    // first, let's add jump thunks for every jump with a distance that's too big
    // we will create new instruction buffers, with remap table keeping track of the moves: remap[oldpc] = newpc
    let mut remap: Vec<u32> = vec![0; self.insns.len()];

    let mut newinsns: Vec<u32> = Vec::with_capacity(self.insns.len());
    let mut newlines: Vec<i32> = Vec::with_capacity(self.insns.len());

    LUAU_ASSERT!(self.insns.len() == self.lines.len());

    let mut current_jump: usize = 0;
    let mut pending_trampolines: usize = 0;

    let mut i: usize = 0;
    while let Some(&word) = self.insns.get(i) {
      let insn = Instruction(word);
      let op = insn.op();
      LUAU_ASSERT!(op < LuauOpcode::LOP__COUNT as u8);

      if current_jump < self.jumps.len() && self.jumps[current_jump].source == i as u32 {
        let offset =
          (self.jumps[current_jump].target as i32) - (self.jumps[current_jump].source as i32) - 1;

        if offset.abs() > K_MAX_JUMP_DISTANCE_CONSERVATIVE {
          // insert jump trampoline as described above; we keep JUMPX offset uninitialized in this pass
          newinsns.push(LuauOpcode::LOP_JUMP as u32 | (1 << insn::B_SHIFT));
          newinsns.push(LuauOpcode::LOP_JUMPX as u32);

          newlines.push(self.lines[i]);
          newlines.push(self.lines[i]);

          pending_trampolines += 1;
        }

        current_jump += 1;
      }

      let oplen = get_op_length(insn.luau_opcode()) as usize;

      // copy instruction and line info to the new stream
      // remap 逐字递增：第 k 字映射到新流中的自身位置（cpp 同为 push 前取 size）
      let base = newinsns.len() as u32;
      newinsns.extend_from_slice(&self.insns[i..i + oplen]);
      newlines.extend(repeat_n(self.lines[i], oplen));
      for (k, r) in remap[i..i + oplen].iter_mut().enumerate() {
        *r = base + k as u32;
      }

      i += oplen;
    }

    LUAU_ASSERT!(current_jump == self.jumps.len());
    LUAU_ASSERT!(pending_trampolines > 0);

    // now we need to recompute offsets for jump instructions - we could not do this in the first pass because the offsets are between *target*
    // instructions
    for jump in &mut self.jumps {
      let offset = (jump.target as i32) - (jump.source as i32) - 1;
      let newoffset =
        (remap[jump.target as usize] as i32) - (remap[jump.source as usize] as i32) - 1;

      // cpp BytecodeBuilder.cpp:1407-1410：trampoline 展开后仍放不进 JUMPX 的
      // 24 位偏移 — 静默截断会产生坏字节码，立即放弃并上报
      if fflag::LuauCompileExpandLimit.get() && (newoffset.abs() + 1) >= K_MAX_JUMP_DISTANCE {
        return true;
      }

      if offset.abs() > K_MAX_JUMP_DISTANCE_CONSERVATIVE {
        // fix up jump trampoline
        let trampoline_pos = remap[jump.source as usize] as usize - 1;
        let op_pos = trampoline_pos + 1;

        let (left, right) = newinsns.split_at_mut(op_pos);
        let insnt = &mut left[trampoline_pos];
        let insnj = &mut right[0];

        LUAU_ASSERT!(Instruction(*insnt).opcode() == Some(LuauOpcode::LOP_JUMPX));

        // patch JUMPX to JUMPX to target location; note that newoffset is the offset of the jump *relative to OP*, so we need to add 1 to make it
        // relative to JUMPX
        *insnt &= insn::OP_MASK;
        *insnt |= ((newoffset + 1) as u32) << insn::A_SHIFT;

        // patch OP to OP -2
        *insnj &= insn::AD_MASK;
        *insnj |= ((-2i16) as u32) << insn::B_SHIFT;

        pending_trampolines -= 1;
      } else {
        let jump_insn = &mut newinsns[remap[jump.source as usize] as usize];

        // make sure jump instruction had the correct offset before we started
        LUAU_ASSERT!(Instruction(*jump_insn).d() as i32 == offset);

        // patch instruction with the new offset
        LUAU_ASSERT!(i32::from(newoffset as i16) == newoffset);

        *jump_insn &= insn::AD_MASK;
        *jump_insn |= (newoffset as u32) << insn::B_SHIFT;
      }
    }

    LUAU_ASSERT!(pending_trampolines == 0);

    // this was hard, but we're done.
    mem::swap(&mut self.insns, &mut newinsns);
    mem::swap(&mut self.lines, &mut newlines);

    remap_pc_ranges(&mut self.debug_locals, &remap);
    remap_pc_ranges(&mut self.typed_locals, &remap);

    false
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_fold_jumps.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn fold_jumps(&mut self) {
    // if our function has long jumps, some processing below can make jump instructions not-jumps (e.g. JUMP->RETURN)
    // it's safer to skip this processing
    if self.has_long_jumps {
      return;
    }

    for jump in &mut self.jumps {
      let jump_label: u32 = jump.source;
      let jump_insn = Instruction(self.insns[jump_label as usize]);

      // follow jump target through forward unconditional jumps
      // we only follow forward jumps to make sure the process terminates
      // NB: C++ computes this with SIGNED `int` — `LUAU_INSN_D` is the signed
      // jump offset (negative for backward jumps), so `jumpLabel + 1 + D`
      // must be signed arithmetic. The model used `u32` with `D as u32`,
      // which overflows on any backward jump (every loop's back-edge).
      let mut target_label: i32 = jump_label as i32 + 1 + jump_insn.d() as i32;
      LUAU_ASSERT!((target_label as usize) < self.insns.len());
      let mut target_insn = Instruction(self.insns[target_label as usize]);

      while target_insn.opcode() == Some(LuauOpcode::LOP_JUMP) && target_insn.d() >= 0 {
        target_label = target_label + 1 + target_insn.d() as i32;
        LUAU_ASSERT!((target_label as usize) < self.insns.len());
        target_insn = Instruction(self.insns[target_label as usize]);
      }

      let offset: i32 = target_label - jump_label as i32 - 1;

      // for unconditional jumps to RETURN, we can replace JUMP with RETURN
      if jump_insn.opcode() == Some(LuauOpcode::LOP_JUMP)
        && target_insn.opcode() == Some(LuauOpcode::LOP_RETURN)
      {
        self.insns[jump_label as usize] = target_insn.raw();
      } else if (offset as i16) as i32 == offset {
        let mut insn = self.insns[jump_label as usize];
        insn &= insn::AD_MASK;
        insn |= ((offset as u16) as u32) << insn::B_SHIFT;
        self.insns[jump_label as usize] = insn;
      }

      jump.target = target_label as u32;
    }
  }
}
