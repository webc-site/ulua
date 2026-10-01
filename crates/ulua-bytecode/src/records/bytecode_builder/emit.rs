//! `BytecodeBuilder` 之 指令编码与补丁：emit_*、patch_*、undo_emit。

use ulua_common::{
  enums::luau_opcode::LuauOpcode,
  fflag::LuauCompileUndoEmitAdjust,
  functions::{is_fast_call::is_fast_call, is_jump_d::is_jump_d, is_skip_c::is_skip_c},
  macros::luau_assert::LUAU_ASSERT,
  records::instruction::Instruction,
};

use super::{BytecodeBuilder, K_MAX_JUMP_DISTANCE, insn};
use crate::records::jump::Jump;

// ── abs-r139：并自 `methods/bytecode_builder_emit_abc.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn emit_abc(&mut self, op: LuauOpcode, a: u8, b: u8, c: u8) {
    let insn = (op as u32)
      | ((a as u32) << insn::A_SHIFT)
      | ((b as u32) << insn::B_SHIFT)
      | ((c as u32) << insn::C_SHIFT);

    self.push_insn(insn);
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_emit_ad.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn emit_ad(&mut self, op: LuauOpcode, a: u8, d: i16) {
    let insn = op as u32 | ((a as u32) << insn::A_SHIFT) | ((d as u16 as u32) << insn::B_SHIFT);

    self.push_insn(insn);
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_emit_aux.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn emit_aux(&mut self, aux: u32) {
    self.push_insn(aux);
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_emit_e.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub(crate) fn emit_e(&mut self, op: LuauOpcode, e: i32) {
    let insn = (op as u32) | ((e as u32) << insn::A_SHIFT);

    self.push_insn(insn);
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_emit_label.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn emit_label(&self) -> usize {
    self.insns.len()
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_patch_aux.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn patch_aux(&mut self, target_aux: usize, new_value: i32) {
    LUAU_ASSERT!(target_aux < self.insns.len());
    self.insns[target_aux] = new_value as u32;
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_patch_jump_d.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn patch_jump_d(&mut self, jump_label: usize, target_label: usize) -> bool {
    LUAU_ASSERT!(jump_label < self.insns.len());

    let jump_insn = Instruction(self.insns[jump_label]);

    LUAU_ASSERT!(is_jump_d(jump_insn.luau_opcode()));
    LUAU_ASSERT!(jump_insn.d() == 0);

    LUAU_ASSERT!(target_label <= self.insns.len());

    let offset = target_label as i32 - jump_label as i32 - 1;

    if (offset as i16) as i32 == offset {
      self.insns[jump_label] |= ((offset as u16) as u32) << insn::B_SHIFT;
    } else if offset.abs() < K_MAX_JUMP_DISTANCE {
      // 16 位放不下：改走 JUMPX 蹦床重排（见 expandJumps）。上限取 JUMPX 的
      // 24 位射程 `kMaxJumpDistance`（并非 32767），约 800 万条指令内都能兜住。
      self.has_long_jumps = true;
    } else {
      return false;
    }

    self.jumps.push(Jump {
      source: jump_label as u32,
      target: target_label as u32,
    });

    true
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_patch_skip_c.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn patch_skip_c(&mut self, jump_label: usize, target_label: usize) -> bool {
    LUAU_ASSERT!(jump_label < self.insns.len());

    let jump_insn = Instruction(self.insns[jump_label]);

    let op = jump_insn.luau_opcode();
    LUAU_ASSERT!(is_skip_c(op) || is_fast_call(op));
    LUAU_ASSERT!(jump_insn.c() == 0);

    let offset = (target_label as i32) - (jump_label as i32) - 1;

    if (offset as u8) as i32 != offset {
      return false;
    }

    self.insns[jump_label] |= (offset as u32) << insn::C_SHIFT;
    true
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_push_insn.rs` ──
impl<'a> BytecodeBuilder<'a> {
  /// 指令与行号同步落盘的唯一入口：`emit_abc` / `emit_ad` / `emit_e` / `emit_aux`
  /// 四个编码变体只负责拼字，落盘（含行信息对齐）收敛于此。
  pub(crate) fn push_insn(&mut self, insn: u32) {
    self.insns.push(insn);
    self.lines.push(self.debug_line);
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_set_debug_line.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn set_debug_line(&mut self, line: i32) {
    self.debug_line = line;
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_undo_emit.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn undo_emit(&mut self, op: LuauOpcode) {
    LUAU_ASSERT!(!self.insns.is_empty());
    LUAU_ASSERT!((self.insns[self.insns.len() - 1] & insn::OP_MASK) == op as u32);

    if LuauCompileUndoEmitAdjust.get() {
      let insns_len = self.insns.len() as u32;
      let adjust_local = |startpc: u32, endpc: &mut u32| {
        let retain = startpc != insns_len;
        if retain {
          *endpc -= (*endpc == insns_len) as u32;
        }
        retain
      };

      self
        .debug_locals
        .retain_mut(|l| adjust_local(l.startpc, &mut l.endpc));
      self
        .typed_locals
        .retain_mut(|l| adjust_local(l.startpc, &mut l.endpc));
    }

    self.insns.pop();
    self.lines.pop();
  }
}
