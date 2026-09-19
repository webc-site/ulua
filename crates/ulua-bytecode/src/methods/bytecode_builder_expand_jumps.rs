use core::{iter::repeat_n, mem};

use ulua_common::{
  enums::luau_opcode::LuauOpcode,
  fflag,
  functions::get_op_length::get_op_length,
  macros::{luau_assert::LUAU_ASSERT, luau_insn_d::luau_insn_d, luau_insn_op::luau_insn_op},
};

use crate::records::{
  bytecode_builder::BytecodeBuilder, debug_local_bytecode_builder::DebugLocal,
  typed_local_bytecode_builder::TypedLocal,
};

/// cpp `BytecodeBuilder.h:23` `kMaxJumpDistance`：JUMPX 24 位偏移上限
const K_MAX_JUMP_DISTANCE: i32 = 1 << 23;

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

impl BytecodeBuilder {
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
    while i < self.insns.len() {
      let op = luau_insn_op(self.insns[i]) as u8;
      LUAU_ASSERT!(op < LuauOpcode::LOP__COUNT as u8);

      if current_jump < self.jumps.len() && self.jumps[current_jump].source == i as u32 {
        let offset =
          (self.jumps[current_jump].target as i32) - (self.jumps[current_jump].source as i32) - 1;

        if offset.abs() > K_MAX_JUMP_DISTANCE_CONSERVATIVE {
          // insert jump trampoline as described above; we keep JUMPX offset uninitialized in this pass
          newinsns.push(LuauOpcode::LOP_JUMP as u32 | (1 << 16));
          newinsns.push(LuauOpcode::LOP_JUMPX as u32);

          newlines.push(self.lines[i]);
          newlines.push(self.lines[i]);

          pending_trampolines += 1;
        }

        current_jump += 1;
      }

      let oplen = get_op_length(LuauOpcode::from(op)) as usize;

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

        LUAU_ASSERT!(luau_insn_op(*insnt) == LuauOpcode::LOP_JUMPX as u32);

        // patch JUMPX to JUMPX to target location; note that newoffset is the offset of the jump *relative to OP*, so we need to add 1 to make it
        // relative to JUMPX
        *insnt &= 0xff;
        *insnt |= ((newoffset + 1) as u32) << 8;

        // patch OP to OP -2
        *insnj &= 0xffff;
        *insnj |= ((-2i16) as u32) << 16;

        pending_trampolines -= 1;
      } else {
        let insn = &mut newinsns[remap[jump.source as usize] as usize];

        // make sure jump instruction had the correct offset before we started
        LUAU_ASSERT!(luau_insn_d(*insn) == offset);

        // patch instruction with the new offset
        LUAU_ASSERT!(i32::from(newoffset as i16) == newoffset);

        *insn &= 0xffff;
        *insn |= (newoffset as u32) << 16;
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
