use ulua_common::{
  enums::{luau_capture_type::LuauCaptureType, luau_opcode::LuauOpcode},
  fflag::DebugLuauUserDefinedClasses,
  functions::get_op_length::get_op_length,
  macros::{
    luau_assert::LUAU_ASSERT, luau_insn_a::luau_insn_a, luau_insn_aux_kv_16::luau_insn_aux_kv16,
    luau_insn_b::luau_insn_b, luau_insn_c::luau_insn_c, luau_insn_d::luau_insn_d,
    luau_insn_e::luau_insn_e, luau_insn_op::luau_insn_op,
  },
};

use crate::{
  macros::{
    vconst::VCONST, vconstany::VCONSTANY, vjump::VJUMP, vreg::VREG, vregrange::VREGRANGE,
    vupval::VUPVAL,
  },
  records::bytecode_builder::BytecodeBuilder,
};

impl BytecodeBuilder {
  pub fn validate_instructions(&self) {
    let current_function = self.current_function;
    LUAU_ASSERT!(current_function != !0u32);

    let func = &self.functions[current_function as usize];

    // tag instruction offsets so that we can validate jumps
    let mut insnvalid = vec![0u8; self.insns.len()];

    let mut insns = self.insns.iter().copied().enumerate();
    while let Some((i, insn)) = insns.next() {
      let op = LuauOpcode::from(luau_insn_op(insn) as u8);

      insnvalid[i] = 1;

      // 变步长推进：等价 cpp `i += getOpLength(op)`，跳过当前指令的后续槽位
      for _ in 1..get_op_length(op) as usize {
        insns.next();
      }
    }

    let mut open_captures: Vec<u8> = Vec::new();

    // validate individual instructions
    let mut insns = self.insns.iter().copied().enumerate();
    while let Some((i, insn)) = insns.next() {
      let op = LuauOpcode::from(luau_insn_op(insn) as u8);

      match op {
        LuauOpcode::LOP_LOADNIL => {
          VREG!(luau_insn_a(insn) as u8, func);
        }
        LuauOpcode::LOP_LOADB => {
          VREG!(luau_insn_a(insn) as u8, func);
          let b_val = luau_insn_b(insn) as u8;
          LUAU_ASSERT!(b_val == 0 || b_val == 1);
          VJUMP!(luau_insn_c(insn) as i32, i, self.insns, insnvalid);
        }
        LuauOpcode::LOP_LOADN => {
          VREG!(luau_insn_a(insn) as u8, func);
        }
        LuauOpcode::LOP_LOADK => {
          VREG!(luau_insn_a(insn) as u8, func);
          VCONSTANY!(luau_insn_d(insn) as usize, self.constants);
        }
        LuauOpcode::LOP_MOVE => {
          VREG!(luau_insn_a(insn) as u8, func);
          VREG!(luau_insn_b(insn) as u8, func);
        }
        LuauOpcode::LOP_GETGLOBAL | LuauOpcode::LOP_SETGLOBAL => {
          VREG!(luau_insn_a(insn) as u8, func);
          VCONST!(self.insns[i + 1] as usize, String, self.constants);
        }
        LuauOpcode::LOP_GETUPVAL | LuauOpcode::LOP_SETUPVAL => {
          VREG!(luau_insn_a(insn) as u8, func);
          VUPVAL!(luau_insn_b(insn) as u8, func);
        }
        LuauOpcode::LOP_CLOSEUPVALS => {
          VREG!(luau_insn_a(insn) as u8, func);
          // 关闭所有寄存器号 >= 当前指令 A 的捕获
          while open_captures
            .last()
            .is_some_and(|&reg| reg >= luau_insn_a(insn) as u8)
          {
            open_captures.pop();
          }
        }
        LuauOpcode::LOP_GETIMPORT => {
          VREG!(luau_insn_a(insn) as u8, func);
          VCONST!(luau_insn_d(insn) as usize, Import, self.constants);
          let id = self.insns[i + 1];
          LUAU_ASSERT!((id >> 30) != 0); // import chain with length 1-3
          for j in 0..(id >> 30) {
            VCONST!(
              ((id >> (20 - 10 * j)) & 1023) as usize,
              String,
              self.constants
            );
          }
        }
        LuauOpcode::LOP_GETTABLE | LuauOpcode::LOP_SETTABLE => {
          VREG!(luau_insn_a(insn) as u8, func);
          VREG!(luau_insn_b(insn) as u8, func);
          VREG!(luau_insn_c(insn) as u8, func);
        }
        LuauOpcode::LOP_GETTABLEKS | LuauOpcode::LOP_SETTABLEKS => {
          VREG!(luau_insn_a(insn) as u8, func);
          VREG!(luau_insn_b(insn) as u8, func);
          VCONST!(self.insns[i + 1] as usize, String, self.constants);
        }
        LuauOpcode::LOP_GETTABLEN | LuauOpcode::LOP_SETTABLEN => {
          VREG!(luau_insn_a(insn) as u8, func);
          VREG!(luau_insn_b(insn) as u8, func);
        }
        LuauOpcode::LOP_NEWCLOSURE => {
          VREG!(luau_insn_a(insn) as u8, func);
          let proto_idx = luau_insn_d(insn) as usize;
          LUAU_ASSERT!(proto_idx < self.protos.len());
          let proto_val = self.protos[proto_idx];
          LUAU_ASSERT!(proto_val < self.functions.len() as u32);
          let numupvalues = self.functions[proto_val as usize].numupvalues as u32;

          // cpp CODEGEN_ASSERT 同款：CAPTURE 序列必须完整在指令流内
          LUAU_ASSERT!(i + 1 + numupvalues as usize <= self.insns.len());
          for cinsn in self.insns[i + 1..].iter().take(numupvalues as usize) {
            LUAU_ASSERT!(luau_insn_op(*cinsn) == LuauOpcode::LOP_CAPTURE as u32);
          }
        }
        LuauOpcode::LOP_NAMECALL => {
          VREG!(luau_insn_a(insn) as u8, func);
          VREG!(luau_insn_b(insn) as u8, func);
          VCONST!(self.insns[i + 1] as usize, String, self.constants);
          LUAU_ASSERT!(
            luau_insn_op(self.insns[i + 2]) == LuauOpcode::LOP_CALLFB as u32
              || luau_insn_op(self.insns[i + 2]) == LuauOpcode::LOP_CALL as u32
          );
        }
        LuauOpcode::LOP_CALL | LuauOpcode::LOP_CALLFB => {
          let nparams = (luau_insn_b(insn) as i32) - 1;
          let nresults = (luau_insn_c(insn) as i32) - 1;
          VREG!(luau_insn_a(insn) as u8, func);
          VREGRANGE!(luau_insn_a(insn) as u8 + 1, nparams, func);
          VREGRANGE!(luau_insn_a(insn) as u8, nresults, func);
        }
        LuauOpcode::LOP_RETURN => {
          let nresults = (luau_insn_b(insn) as i32) - 1;
          VREGRANGE!(luau_insn_a(insn) as u8, nresults, func);
        }
        LuauOpcode::LOP_JUMP => {
          VJUMP!(luau_insn_d(insn), i, self.insns, insnvalid);
        }
        LuauOpcode::LOP_JUMPIF | LuauOpcode::LOP_JUMPIFNOT => {
          VREG!(luau_insn_a(insn) as u8, func);
          VJUMP!(luau_insn_d(insn), i, self.insns, insnvalid);
        }
        LuauOpcode::LOP_JUMPIFEQ
        | LuauOpcode::LOP_JUMPIFLE
        | LuauOpcode::LOP_JUMPIFLT
        | LuauOpcode::LOP_JUMPIFNOTEQ
        | LuauOpcode::LOP_JUMPIFNOTLE
        | LuauOpcode::LOP_JUMPIFNOTLT => {
          VREG!(luau_insn_a(insn) as u8, func);
          VREG!(self.insns[i + 1] as u8, func);
          VJUMP!(luau_insn_d(insn), i, self.insns, insnvalid);
        }
        LuauOpcode::LOP_JUMPXEQKNIL | LuauOpcode::LOP_JUMPXEQKB => {
          VREG!(luau_insn_a(insn) as u8, func);
          VJUMP!(luau_insn_d(insn), i, self.insns, insnvalid);
        }
        LuauOpcode::LOP_JUMPXEQKN => {
          VREG!(luau_insn_a(insn) as u8, func);
          VCONST!(
            (self.insns[i + 1] & 0xffffff) as usize,
            Number,
            self.constants
          );
          VJUMP!(luau_insn_d(insn), i, self.insns, insnvalid);
        }
        LuauOpcode::LOP_JUMPXEQKS => {
          VREG!(luau_insn_a(insn) as u8, func);
          VCONST!(
            (self.insns[i + 1] & 0xffffff) as usize,
            String,
            self.constants
          );
          VJUMP!(luau_insn_d(insn), i, self.insns, insnvalid);
        }
        LuauOpcode::LOP_ADD
        | LuauOpcode::LOP_SUB
        | LuauOpcode::LOP_MUL
        | LuauOpcode::LOP_DIV
        | LuauOpcode::LOP_IDIV
        | LuauOpcode::LOP_MOD
        | LuauOpcode::LOP_POW => {
          VREG!(luau_insn_a(insn) as u8, func);
          VREG!(luau_insn_b(insn) as u8, func);
          VREG!(luau_insn_c(insn) as u8, func);
        }
        LuauOpcode::LOP_ADDK
        | LuauOpcode::LOP_SUBK
        | LuauOpcode::LOP_MULK
        | LuauOpcode::LOP_DIVK
        | LuauOpcode::LOP_IDIVK
        | LuauOpcode::LOP_MODK
        | LuauOpcode::LOP_POWK => {
          VREG!(luau_insn_a(insn) as u8, func);
          VREG!(luau_insn_b(insn) as u8, func);
          VCONST!(luau_insn_c(insn) as usize, Number, self.constants);
        }
        LuauOpcode::LOP_SUBRK | LuauOpcode::LOP_DIVRK => {
          VREG!(luau_insn_a(insn) as u8, func);
          VCONST!(luau_insn_b(insn) as usize, Number, self.constants);
          VREG!(luau_insn_c(insn) as u8, func);
        }
        LuauOpcode::LOP_AND | LuauOpcode::LOP_OR => {
          VREG!(luau_insn_a(insn) as u8, func);
          VREG!(luau_insn_b(insn) as u8, func);
          VREG!(luau_insn_c(insn) as u8, func);
        }
        LuauOpcode::LOP_ANDK | LuauOpcode::LOP_ORK => {
          VREG!(luau_insn_a(insn) as u8, func);
          VREG!(luau_insn_b(insn) as u8, func);
          VCONSTANY!(luau_insn_c(insn) as usize, self.constants);
        }
        LuauOpcode::LOP_CONCAT => {
          VREG!(luau_insn_a(insn) as u8, func);
          VREG!(luau_insn_b(insn) as u8, func);
          VREG!(luau_insn_c(insn) as u8, func);
          LUAU_ASSERT!(luau_insn_b(insn) <= luau_insn_c(insn));
        }
        LuauOpcode::LOP_NOT | LuauOpcode::LOP_MINUS | LuauOpcode::LOP_LENGTH => {
          VREG!(luau_insn_a(insn) as u8, func);
          VREG!(luau_insn_b(insn) as u8, func);
        }
        LuauOpcode::LOP_NEWTABLE => {
          VREG!(luau_insn_a(insn) as u8, func);
        }
        LuauOpcode::LOP_DUPTABLE => {
          VREG!(luau_insn_a(insn) as u8, func);
          VCONST!(luau_insn_d(insn) as usize, Table, self.constants);
        }
        LuauOpcode::LOP_SETLIST => {
          let count = (luau_insn_c(insn) as i32) - 1;
          VREG!(luau_insn_a(insn) as u8, func);
          VREGRANGE!(luau_insn_b(insn) as u8, count, func);
        }
        LuauOpcode::LOP_FORNPREP | LuauOpcode::LOP_FORNLOOP => {
          VREG!(luau_insn_a(insn) as u8 + 2, func);
          VJUMP!(luau_insn_d(insn), i, self.insns, insnvalid);
        }
        LuauOpcode::LOP_FORGPREP => {
          VREG!(luau_insn_a(insn) as u8 + 2 + 1, func);
          VJUMP!(luau_insn_d(insn), i, self.insns, insnvalid);
        }
        LuauOpcode::LOP_FORGLOOP => {
          VREG!(
            luau_insn_a(insn) as u8 + 2 + (self.insns[i + 1] as u8),
            func
          );
          VJUMP!(luau_insn_d(insn), i, self.insns, insnvalid);
          LUAU_ASSERT!((self.insns[i + 1] as u8) >= 1);
        }
        LuauOpcode::LOP_FORGPREP_INEXT | LuauOpcode::LOP_FORGPREP_NEXT => {
          VREG!(luau_insn_a(insn) as u8 + 4, func);
          VJUMP!(luau_insn_d(insn), i, self.insns, insnvalid);
        }
        LuauOpcode::LOP_GETVARARGS => {
          let nresults = (luau_insn_b(insn) as i32) - 1;
          VREGRANGE!(luau_insn_a(insn) as u8, nresults, func);
        }
        LuauOpcode::LOP_DUPCLOSURE => {
          VREG!(luau_insn_a(insn) as u8, func);
          VCONST!(luau_insn_d(insn) as usize, Closure, self.constants);
          let proto = unsafe {
            self.constants[luau_insn_d(insn) as usize]
              .value
              .value_closure
          };
          LUAU_ASSERT!(proto < self.functions.len() as u32);
          let numupvalues = self.functions[proto as usize].numupvalues as u32;

          // cpp CODEGEN_ASSERT 同款：CAPTURE 序列必须完整在指令流内
          LUAU_ASSERT!(i + 1 + numupvalues as usize <= self.insns.len());
          for cinsn in self.insns[i + 1..].iter().take(numupvalues as usize) {
            LUAU_ASSERT!(luau_insn_op(*cinsn) == LuauOpcode::LOP_CAPTURE as u32);
            let capture_type = luau_insn_a(*cinsn) as u8;
            LUAU_ASSERT!(
              capture_type == LuauCaptureType::LCT_VAL as u8
                || capture_type == LuauCaptureType::LCT_UPVAL as u8
            );
          }
        }
        LuauOpcode::LOP_PREPVARARGS => {
          LUAU_ASSERT!(luau_insn_a(insn) == func.numparams as u32);
          LUAU_ASSERT!(func.isvararg);
        }
        LuauOpcode::LOP_BREAK => {}
        LuauOpcode::LOP_JUMPBACK => {
          VJUMP!(luau_insn_d(insn), i, self.insns, insnvalid);
        }
        LuauOpcode::LOP_LOADKX => {
          VREG!(luau_insn_a(insn) as u8, func);
          VCONSTANY!(self.insns[i + 1] as usize, self.constants);
        }
        LuauOpcode::LOP_JUMPX => {
          VJUMP!(luau_insn_e(insn), i, self.insns, insnvalid);
        }
        LuauOpcode::LOP_FASTCALL => {
          VJUMP!(luau_insn_c(insn), i, self.insns, insnvalid);
          LUAU_ASSERT!(
            luau_insn_op(self.insns[i + 1 + luau_insn_c(insn) as usize])
              == LuauOpcode::LOP_CALL as u32
          );
        }
        LuauOpcode::LOP_FASTCALL1 => {
          VREG!(luau_insn_b(insn) as u8, func);
          VJUMP!(luau_insn_c(insn), i, self.insns, insnvalid);
          LUAU_ASSERT!(
            luau_insn_op(self.insns[i + 1 + luau_insn_c(insn) as usize])
              == LuauOpcode::LOP_CALL as u32
          );
        }
        LuauOpcode::LOP_FASTCALL2 => {
          VREG!(luau_insn_b(insn) as u8, func);
          VJUMP!(luau_insn_c(insn), i, self.insns, insnvalid);
          LUAU_ASSERT!(
            luau_insn_op(self.insns[i + 1 + luau_insn_c(insn) as usize])
              == LuauOpcode::LOP_CALL as u32
          );
          VREG!(self.insns[i + 1] as u8, func);
        }
        LuauOpcode::LOP_FASTCALL2K => {
          VREG!(luau_insn_b(insn) as u8, func);
          VJUMP!(luau_insn_c(insn), i, self.insns, insnvalid);
          LUAU_ASSERT!(
            luau_insn_op(self.insns[i + 1 + luau_insn_c(insn) as usize])
              == LuauOpcode::LOP_CALL as u32
          );
          VCONSTANY!(self.insns[i + 1] as usize, self.constants);
        }
        LuauOpcode::LOP_FASTCALL3 => {
          VREG!(luau_insn_b(insn) as u8, func);
          VJUMP!(luau_insn_c(insn), i, self.insns, insnvalid);
          LUAU_ASSERT!(
            luau_insn_op(self.insns[i + 1 + luau_insn_c(insn) as usize])
              == LuauOpcode::LOP_CALL as u32
          );
          VREG!((self.insns[i + 1] & 0xff) as u8, func);
          VREG!(((self.insns[i + 1] >> 8) & 0xff) as u8, func);
        }
        LuauOpcode::LOP_COVERAGE => {}
        LuauOpcode::LOP_CAPTURE => {
          let capture_type = luau_insn_a(insn);
          match capture_type {
            a if a == LuauCaptureType::LCT_VAL as u32 => VREG!(luau_insn_b(insn) as u8, func),
            a if a == LuauCaptureType::LCT_REF as u32 => {
              VREG!(luau_insn_b(insn) as u8, func);
              open_captures.push(luau_insn_b(insn) as u8);
            }
            a if a == LuauCaptureType::LCT_UPVAL as u32 => VUPVAL!(luau_insn_b(insn) as u8, func),
            _ => LUAU_ASSERT!(false, "Unsupported capture type"),
          }
        }
        LuauOpcode::LOP_NEWCLASSMEMBER => {
          VREG!(luau_insn_a(insn) as u8, func);
          LUAU_ASSERT!(luau_insn_b(insn) == 0);
          VREG!(luau_insn_c(insn) as u8, func);
          VCONST!(self.insns[i + 1] as usize, String, self.constants);
        }
        LuauOpcode::LOP_GETUDATAKS | LuauOpcode::LOP_SETUDATAKS => {
          VREG!(luau_insn_a(insn) as u8, func);
          VREG!(luau_insn_b(insn) as u8, func);
          VCONST!(
            luau_insn_aux_kv16(self.insns[i + 1]) as usize,
            String,
            self.constants
          );
        }
        LuauOpcode::LOP_NAMECALLUDATA => {
          VREG!(luau_insn_a(insn) as u8, func);
          VREG!(luau_insn_b(insn) as u8, func);
          VCONST!(
            luau_insn_aux_kv16(self.insns[i + 1]) as usize,
            String,
            self.constants
          );
          LUAU_ASSERT!(luau_insn_op(self.insns[i + 2]) == LuauOpcode::LOP_CALL as u32);
        }
        LuauOpcode::LOP_CMPPROTO => {
          VREG!(luau_insn_a(insn) as u8, func);
          VJUMP!(luau_insn_d(insn), i, self.insns, insnvalid);
        }
        LuauOpcode::LOP_FASTPCALL => {
          VJUMP!(luau_insn_c(insn) as i32, i, self.insns, insnvalid);
          LUAU_ASSERT!(
            luau_insn_op(self.insns[i + 1 + luau_insn_c(insn) as usize])
              == LuauOpcode::LOP_CALL as u32
          );
        }
        LuauOpcode::LOP_NEWCLASS => {
          LUAU_ASSERT!(DebugLuauUserDefinedClasses.get());
          VREG!(luau_insn_a(insn) as u8, func);
          let super_reg = luau_insn_b(insn) as u8;
          LUAU_ASSERT!(super_reg == 0xff || (super_reg as usize) < func.maxstacksize as usize);
          let flags = luau_insn_c(insn) as u8;
          LUAU_ASSERT!(flags == 0 || flags == 1);
          VCONST!(self.insns[i + 1] as usize, ClassShape, self.constants);
        }
        _ => {
          LUAU_ASSERT!(false, "Unsupported opcode");
        }
      }

      // 变步长推进：等价 cpp `i += getOpLength(op)`，跳过当前指令的后续槽位
      for _ in 1..get_op_length(op) as usize {
        insns.next();
      }
    }

    // all CAPTURE REF instructions must have a CLOSEUPVALS instruction after them in the bytecode stream
    // this doesn't guarantee safety as it doesn't perform basic block based analysis, but if this fails
    // then the bytecode is definitely unsafe to run since the compiler won't generate backwards branches
    // except for loop edges
    LUAU_ASSERT!(open_captures.is_empty());
  }
}
