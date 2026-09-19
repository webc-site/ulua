use alloc::{string::String, vec::Vec};

use ulua_common::{
  enums::{
    luau_bytecode_type::{LBC_TYPE_OPTIONAL_BIT, LuauBytecodeType},
    luau_opcode::LuauOpcode,
  },
  functions::{
    format_append::format_append, get_jump_target::get_jump_target, get_op_length::get_op_length,
  },
  macros::{luau_assert::LUAU_ASSERT, luau_insn_op::luau_insn_op},
};

use crate::{
  enums::dump_flags::DumpFlags, functions::get_base_type_string::get_base_type_string,
  records::bytecode_builder::BytecodeBuilder,
};

impl BytecodeBuilder {
  /// 类型名 + 可选 '?' 后缀（userdata 名优先，回落到基础类型名）。
  /// C++ `name = userdata ? userdata : getBaseTypeString(et)`。
  fn type_name_and_optional(&self, ty: LuauBytecodeType) -> (&str, &'static str) {
    let name = self
      .try_get_userdata_type_name(ty)
      .unwrap_or_else(|| get_base_type_string(ty.0 as u8));
    let optional = if (ty.0 & LBC_TYPE_OPTIONAL_BIT.0) != 0 {
      "?"
    } else {
      ""
    };
    (name, optional)
  }

  pub fn dump_current_function(&self, dumpinstoffs: &mut Vec<i32>) -> String {
    if (self.dump_flags & (DumpFlags::Code as u32 | DumpFlags::Constants as u32)) == 0 {
      return String::new();
    }

    let mut last_line = -1;
    let mut result = String::new();

    if self.dump_flags & DumpFlags::Locals as u32 != 0 {
      for (i, l) in self.debug_locals.iter().enumerate() {
        if l.startpc == l.endpc {
          LUAU_ASSERT!(l.startpc < self.lines.len() as u32);

          format_append(
            &mut result,
            format_args!(
              "local {}: reg {}, start pc {} line {}, no live range\n",
              i, l.reg, l.startpc, self.lines[l.startpc as usize]
            ),
          );
        } else {
          LUAU_ASSERT!(l.startpc < l.endpc);
          LUAU_ASSERT!(l.startpc < self.lines.len() as u32);
          LUAU_ASSERT!(l.endpc <= self.lines.len() as u32);

          format_append(
            &mut result,
            format_args!(
              "local {}: reg {}, start pc {} line {}, end pc {} line {}\n",
              i,
              l.reg,
              l.startpc,
              self.lines[l.startpc as usize],
              l.endpc - 1,
              self.lines[(l.endpc - 1) as usize]
            ),
          );
        }
      }
    }

    if self.dump_flags & DumpFlags::Types as u32 != 0 {
      let typeinfo_bytes = &self.functions.last().unwrap().typeinfo;

      if typeinfo_bytes.len() >= 2 {
        for (i, &et) in typeinfo_bytes[2..].iter().enumerate() {
          let (name, optional) = self.type_name_and_optional(LuauBytecodeType(et as u16));
          format_append(
            &mut result,
            format_args!("R{}: {}{} [argument]\n", i, name, optional),
          );
        }
      }

      for (i, l) in self.typed_upvals.iter().enumerate() {
        let (name, optional) = self.type_name_and_optional(l.r#type);
        format_append(&mut result, format_args!("U{}: {}{}\n", i, name, optional));
      }

      for l in &self.typed_locals {
        let (name, optional) = self.type_name_and_optional(l.r#type);
        format_append(
          &mut result,
          format_args!(
            "R{}: {}{} from {} to {}\n",
            l.reg, name, optional, l.startpc, l.endpc
          ),
        );
      }
    }

    if self.dump_flags & DumpFlags::Constants as u32 != 0 {
      for (i, _) in self.constants.iter().enumerate() {
        format_append(&mut result, format_args!("K{}: ", i));
        self.dump_constant(&mut result, i as i32, true);
        result.push('\n');
      }
    }

    if self.dump_flags & DumpFlags::Code as u32 != 0 {
      let mut labels = vec![-1; self.insns.len()];

      // 第一遍：标记跳转目标槽位（变步长遍历）
      let mut insns = self.insns.iter().copied().enumerate();
      while let Some((i, insn)) = insns.next() {
        let target = get_jump_target(insn, i as u32);

        if target >= 0 {
          LUAU_ASSERT!((target as usize) < self.insns.len());
          labels[target as usize] = 0;
        }

        let op = LuauOpcode::from(luau_insn_op(insn) as u8);
        // 变步长推进：等价 cpp `i += getOpLength(op)`
        for _ in 1..get_op_length(op) as usize {
          insns.next();
        }
      }

      let mut next_label = 0;

      for label in &mut labels {
        if *label == 0 {
          *label = next_label;
          next_label += 1;
        }
      }

      dumpinstoffs.resize(self.insns.len() + 1, -1);

      let mut remarks = self.debug_remarks.iter().copied().peekable();

      let mut insns = self.insns.iter().copied().enumerate();
      while let Some((i, code)) = insns.next() {
        let op = luau_insn_op(code) as u8;

        dumpinstoffs[i] = result.len() as i32;

        // C++: `if (op == LOP_PREPVARARGS) { i++; continue; }` — the vararg
        // prologue is a call-dispatch Header with no "interesting" info and
        // is never disassembled. (The prior `op == 32` was a mistranslated
        // literal; LOP_PREPVARARGS is 65, so the skip never fired and the
        // Header reached `dump_instruction`'s unsupported-opcode assert.)
        if op == LuauOpcode::LOP_PREPVARARGS as u8 {
          continue;
        }

        if self.dump_flags & DumpFlags::Remarks as u32 != 0 {
          // 归并输出起始于当前指令位置的 remark
          while let Some((_, remark_start)) = remarks.next_if(|(pos, _)| *pos == i as u32) {
            // C++ reads `debugRemarkBuffer.c_str() + offset` — a C-string that stops
            // at the null terminator. remark_end points at the *next* remark (past this
            // remark's '\0'), so slice only up to the terminator.
            let remark_end = remarks
              .peek()
              .map_or(self.debug_remark_buffer.len(), |&(_, end)| end as usize);
            let remark_str = &self.debug_remark_buffer[remark_start as usize..remark_end];
            let remark_str = remark_str.split('\0').next().unwrap_or("");
            format_append(&mut result, format_args!("REMARK {}\n", remark_str));
          }
        }

        if self.dump_flags & DumpFlags::Source as u32 != 0 {
          let line = self.lines[i];

          if line > 0 && line != last_line {
            LUAU_ASSERT!(((line - 1) as usize) < self.dump_source.len());
            format_append(
              &mut result,
              format_args!("{:5}: {}\n", line, self.dump_source[(line - 1) as usize]),
            );
            last_line = line;
          }
        }

        if self.dump_flags & DumpFlags::Lines as u32 != 0 {
          format_append(&mut result, format_args!("{}: ", self.lines[i]));
        }

        if labels[i] != -1 {
          format_append(&mut result, format_args!("L{}: ", labels[i]));
        }

        let target = get_jump_target(code, i as u32);
        let target_label = if target >= 0 {
          labels[target as usize]
        } else {
          -1
        };

        // Pass the full remaining instruction stream (not just one word):
        // multi-word ops (LOADKX, GETIMPORT, FASTCALL2K, NEWCLASSMEMBER,
        // CMPPROTO, …) read their aux word via `code[1]`, which would be
        // out of bounds on a length-1 slice. C++ passes a bare pointer.
        self.dump_instruction(&self.insns[i..], &mut result, target_label);

        let op = LuauOpcode::from(op);
        // 变步长推进：等价 cpp `i += getOpLength(op)`
        for _ in 1..get_op_length(op) as usize {
          insns.next();
        }
      }

      dumpinstoffs[self.insns.len()] = result.len() as i32;
    }

    result
  }
}
