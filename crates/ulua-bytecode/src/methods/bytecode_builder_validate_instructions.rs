use ulua_common::{
  FFlag::DebugLuauUserDefinedClasses,
  enums::{luau_capture_type::LuauCaptureType, luau_opcode::LuauOpcode},
  functions::get_op_length::get_op_length,
  macros::{
    luau_assert::LUAU_ASSERT, luau_insn_a::LUAU_INSN_A, luau_insn_aux_kv_16::LUAU_INSN_AUX_KV16,
    luau_insn_b::LUAU_INSN_B, luau_insn_c::LUAU_INSN_C, luau_insn_d::LUAU_INSN_D,
    luau_insn_e::LUAU_INSN_E, luau_insn_op::LUAU_INSN_OP,
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

    let mut i: usize = 0;
    while i < self.insns.len() {
      let insn = self.insns[i];
      let op = LuauOpcode::from(LUAU_INSN_OP(insn) as u8);

      insnvalid[i] = 1;

      let op_len = get_op_length(op) as usize;
      i += op_len;
      LUAU_ASSERT!(i <= self.insns.len());
    }

    let mut open_captures: Vec<u8> = Vec::new();

    // validate individual instructions
    i = 0;
    while i < self.insns.len() {
      let insn = self.insns[i];
      let op = LuauOpcode::from(LUAU_INSN_OP(insn) as u8);

      match op {
        LuauOpcode::LOP_LOADNIL => {
          VREG!(LUAU_INSN_A(insn) as u8, func);
        }
        LuauOpcode::LOP_LOADB => {
          VREG!(LUAU_INSN_A(insn) as u8, func);
          let b_val = LUAU_INSN_B(insn) as u8;
          LUAU_ASSERT!(b_val == 0 || b_val == 1);
          VJUMP!(LUAU_INSN_C(insn) as i32, i, self.insns, insnvalid);
        }
        LuauOpcode::LOP_LOADN => {
          VREG!(LUAU_INSN_A(insn) as u8, func);
        }
        LuauOpcode::LOP_LOADK => {
          VREG!(LUAU_INSN_A(insn) as u8, func);
          VCONSTANY!(LUAU_INSN_D(insn) as usize, self.constants);
        }
        LuauOpcode::LOP_MOVE => {
          VREG!(LUAU_INSN_A(insn) as u8, func);
          VREG!(LUAU_INSN_B(insn) as u8, func);
        }
        LuauOpcode::LOP_GETGLOBAL | LuauOpcode::LOP_SETGLOBAL => {
          VREG!(LUAU_INSN_A(insn) as u8, func);
          VCONST!(self.insns[i + 1] as usize, String, self.constants);
        }
        LuauOpcode::LOP_GETUPVAL | LuauOpcode::LOP_SETUPVAL => {
          VREG!(LUAU_INSN_A(insn) as u8, func);
          VUPVAL!(LUAU_INSN_B(insn) as u8, func);
        }
        LuauOpcode::LOP_CLOSEUPVALS => {
          VREG!(LUAU_INSN_A(insn) as u8, func);
          // 关闭所有寄存器号 >= 当前指令 A 的捕获
          while open_captures
            .last()
            .is_some_and(|&reg| reg >= LUAU_INSN_A(insn) as u8)
          {
            open_captures.pop();
          }
        }
        LuauOpcode::LOP_GETIMPORT => {
          VREG!(LUAU_INSN_A(insn) as u8, func);
          VCONST!(LUAU_INSN_D(insn) as usize, Import, self.constants);
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
          VREG!(LUAU_INSN_A(insn) as u8, func);
          VREG!(LUAU_INSN_B(insn) as u8, func);
          VREG!(LUAU_INSN_C(insn) as u8, func);
        }
        LuauOpcode::LOP_GETTABLEKS | LuauOpcode::LOP_SETTABLEKS => {
          VREG!(LUAU_INSN_A(insn) as u8, func);
          VREG!(LUAU_INSN_B(insn) as u8, func);
          VCONST!(self.insns[i + 1] as usize, String, self.constants);
        }
        LuauOpcode::LOP_GETTABLEN | LuauOpcode::LOP_SETTABLEN => {
          VREG!(LUAU_INSN_A(insn) as u8, func);
          VREG!(LUAU_INSN_B(insn) as u8, func);
        }
        LuauOpcode::LOP_NEWCLOSURE => {
          VREG!(LUAU_INSN_A(insn) as u8, func);
          let proto_idx = LUAU_INSN_D(insn) as usize;
          LUAU_ASSERT!(proto_idx < self.protos.len());
          let proto_val = self.protos[proto_idx];
          LUAU_ASSERT!(proto_val < self.functions.len() as u32);
          let numupvalues = self.functions[proto_val as usize].numupvalues as u32;

          for j in 0..numupvalues {
            LUAU_ASSERT!(i + 1 + (j as usize) < self.insns.len());
            let cinsn = self.insns[i + 1 + j as usize];
            LUAU_ASSERT!(LUAU_INSN_OP(cinsn) == LuauOpcode::LOP_CAPTURE as u32);
          }
        }
        LuauOpcode::LOP_NAMECALL => {
          VREG!(LUAU_INSN_A(insn) as u8, func);
          VREG!(LUAU_INSN_B(insn) as u8, func);
          VCONST!(self.insns[i + 1] as usize, String, self.constants);
          LUAU_ASSERT!(
            LUAU_INSN_OP(self.insns[i + 2]) == LuauOpcode::LOP_CALLFB as u32
              || LUAU_INSN_OP(self.insns[i + 2]) == LuauOpcode::LOP_CALL as u32
          );
        }
        LuauOpcode::LOP_CALL | LuauOpcode::LOP_CALLFB => {
          let nparams = (LUAU_INSN_B(insn) as i32) - 1;
          let nresults = (LUAU_INSN_C(insn) as i32) - 1;
          VREG!(LUAU_INSN_A(insn) as u8, func);
          VREGRANGE!(LUAU_INSN_A(insn) as u8 + 1, nparams, func);
          VREGRANGE!(LUAU_INSN_A(insn) as u8, nresults, func);
        }
        LuauOpcode::LOP_RETURN => {
          let nresults = (LUAU_INSN_B(insn) as i32) - 1;
          VREGRANGE!(LUAU_INSN_A(insn) as u8, nresults, func);
        }
        LuauOpcode::LOP_JUMP => {
          VJUMP!(LUAU_INSN_D(insn), i, self.insns, insnvalid);
        }
        LuauOpcode::LOP_JUMPIF | LuauOpcode::LOP_JUMPIFNOT => {
          VREG!(LUAU_INSN_A(insn) as u8, func);
          VJUMP!(LUAU_INSN_D(insn), i, self.insns, insnvalid);
        }
        LuauOpcode::LOP_JUMPIFEQ
        | LuauOpcode::LOP_JUMPIFLE
        | LuauOpcode::LOP_JUMPIFLT
        | LuauOpcode::LOP_JUMPIFNOTEQ
        | LuauOpcode::LOP_JUMPIFNOTLE
        | LuauOpcode::LOP_JUMPIFNOTLT => {
          VREG!(LUAU_INSN_A(insn) as u8, func);
          VREG!(self.insns[i + 1] as u8, func);
          VJUMP!(LUAU_INSN_D(insn), i, self.insns, insnvalid);
        }
        LuauOpcode::LOP_JUMPXEQKNIL | LuauOpcode::LOP_JUMPXEQKB => {
          VREG!(LUAU_INSN_A(insn) as u8, func);
          VJUMP!(LUAU_INSN_D(insn), i, self.insns, insnvalid);
        }
        LuauOpcode::LOP_JUMPXEQKN => {
          VREG!(LUAU_INSN_A(insn) as u8, func);
          VCONST!(
            (self.insns[i + 1] & 0xffffff) as usize,
            Number,
            self.constants
          );
          VJUMP!(LUAU_INSN_D(insn), i, self.insns, insnvalid);
        }
        LuauOpcode::LOP_JUMPXEQKS => {
          VREG!(LUAU_INSN_A(insn) as u8, func);
          VCONST!(
            (self.insns[i + 1] & 0xffffff) as usize,
            String,
            self.constants
          );
          VJUMP!(LUAU_INSN_D(insn), i, self.insns, insnvalid);
        }
        LuauOpcode::LOP_ADD
        | LuauOpcode::LOP_SUB
        | LuauOpcode::LOP_MUL
        | LuauOpcode::LOP_DIV
        | LuauOpcode::LOP_MOD
        | LuauOpcode::LOP_POW => {
          VREG!(LUAU_INSN_A(insn) as u8, func);
          VREG!(LUAU_INSN_B(insn) as u8, func);
          VREG!(LUAU_INSN_C(insn) as u8, func);
        }
        LuauOpcode::LOP_ADDK
        | LuauOpcode::LOP_SUBK
        | LuauOpcode::LOP_MULK
        | LuauOpcode::LOP_DIVK
        | LuauOpcode::LOP_MODK
        | LuauOpcode::LOP_POWK => {
          VREG!(LUAU_INSN_A(insn) as u8, func);
          VREG!(LUAU_INSN_B(insn) as u8, func);
          VCONST!(LUAU_INSN_C(insn) as usize, Number, self.constants);
        }
        LuauOpcode::LOP_SUBRK | LuauOpcode::LOP_DIVRK => {
          VREG!(LUAU_INSN_A(insn) as u8, func);
          VCONST!(LUAU_INSN_B(insn) as usize, Number, self.constants);
          VREG!(LUAU_INSN_C(insn) as u8, func);
        }
        LuauOpcode::LOP_AND | LuauOpcode::LOP_OR => {
          VREG!(LUAU_INSN_A(insn) as u8, func);
          VREG!(LUAU_INSN_B(insn) as u8, func);
          VREG!(LUAU_INSN_C(insn) as u8, func);
        }
        LuauOpcode::LOP_ANDK | LuauOpcode::LOP_ORK => {
          VREG!(LUAU_INSN_A(insn) as u8, func);
          VREG!(LUAU_INSN_B(insn) as u8, func);
          VCONSTANY!(LUAU_INSN_C(insn) as usize, self.constants);
        }
        LuauOpcode::LOP_CONCAT => {
          VREG!(LUAU_INSN_A(insn) as u8, func);
          VREG!(LUAU_INSN_B(insn) as u8, func);
          VREG!(LUAU_INSN_C(insn) as u8, func);
          LUAU_ASSERT!(LUAU_INSN_B(insn) <= LUAU_INSN_C(insn));
        }
        LuauOpcode::LOP_NOT | LuauOpcode::LOP_MINUS | LuauOpcode::LOP_LENGTH => {
          VREG!(LUAU_INSN_A(insn) as u8, func);
          VREG!(LUAU_INSN_B(insn) as u8, func);
        }
        LuauOpcode::LOP_NEWTABLE => {
          VREG!(LUAU_INSN_A(insn) as u8, func);
        }
        LuauOpcode::LOP_DUPTABLE => {
          VREG!(LUAU_INSN_A(insn) as u8, func);
          VCONST!(LUAU_INSN_D(insn) as usize, Table, self.constants);
        }
        LuauOpcode::LOP_SETLIST => {
          let count = (LUAU_INSN_C(insn) as i32) - 1;
          VREG!(LUAU_INSN_A(insn) as u8, func);
          VREGRANGE!(LUAU_INSN_B(insn) as u8, count, func);
        }
        LuauOpcode::LOP_FORNPREP | LuauOpcode::LOP_FORNLOOP => {
          VREG!(LUAU_INSN_A(insn) as u8 + 2, func);
          VJUMP!(LUAU_INSN_D(insn), i, self.insns, insnvalid);
        }
        LuauOpcode::LOP_FORGPREP => {
          VREG!(LUAU_INSN_A(insn) as u8 + 2 + 1, func);
          VJUMP!(LUAU_INSN_D(insn), i, self.insns, insnvalid);
        }
        LuauOpcode::LOP_FORGLOOP => {
          VREG!(
            LUAU_INSN_A(insn) as u8 + 2 + (self.insns[i + 1] as u8),
            func
          );
          VJUMP!(LUAU_INSN_D(insn), i, self.insns, insnvalid);
          LUAU_ASSERT!((self.insns[i + 1] as u8) >= 1);
        }
        LuauOpcode::LOP_FORGPREP_INEXT | LuauOpcode::LOP_FORGPREP_NEXT => {
          VREG!(LUAU_INSN_A(insn) as u8 + 4, func);
          VJUMP!(LUAU_INSN_D(insn), i, self.insns, insnvalid);
        }
        LuauOpcode::LOP_GETVARARGS => {
          let nresults = (LUAU_INSN_B(insn) as i32) - 1;
          VREGRANGE!(LUAU_INSN_A(insn) as u8, nresults, func);
        }
        LuauOpcode::LOP_DUPCLOSURE => {
          VREG!(LUAU_INSN_A(insn) as u8, func);
          VCONST!(LUAU_INSN_D(insn) as usize, Closure, self.constants);
          let proto = unsafe {
            self.constants[LUAU_INSN_D(insn) as usize]
              .value
              .value_closure
          };
          LUAU_ASSERT!(proto < self.functions.len() as u32);
          let numupvalues = self.functions[proto as usize].numupvalues as u32;

          for j in 0..numupvalues {
            LUAU_ASSERT!(i + 1 + (j as usize) < self.insns.len());
            let cinsn = self.insns[i + 1 + j as usize];
            LUAU_ASSERT!(LUAU_INSN_OP(cinsn) == LuauOpcode::LOP_CAPTURE as u32);
            let capture_type = LUAU_INSN_A(cinsn) as u8;
            LUAU_ASSERT!(
              capture_type == LuauCaptureType::LCT_VAL as u8
                || capture_type == LuauCaptureType::LCT_UPVAL as u8
            );
          }
        }
        LuauOpcode::LOP_PREPVARARGS => {
          LUAU_ASSERT!(LUAU_INSN_A(insn) == func.numparams as u32);
          LUAU_ASSERT!(func.isvararg);
        }
        LuauOpcode::LOP_BREAK => {}
        LuauOpcode::LOP_JUMPBACK => {
          VJUMP!(LUAU_INSN_D(insn), i, self.insns, insnvalid);
        }
        LuauOpcode::LOP_LOADKX => {
          VREG!(LUAU_INSN_A(insn) as u8, func);
          VCONSTANY!(self.insns[i + 1] as usize, self.constants);
        }
        LuauOpcode::LOP_JUMPX => {
          VJUMP!(LUAU_INSN_E(insn), i, self.insns, insnvalid);
        }
        LuauOpcode::LOP_FASTCALL => {
          VJUMP!(LUAU_INSN_C(insn), i, self.insns, insnvalid);
          LUAU_ASSERT!(
            LUAU_INSN_OP(self.insns[i + 1 + LUAU_INSN_C(insn) as usize])
              == LuauOpcode::LOP_CALL as u32
          );
        }
        LuauOpcode::LOP_FASTCALL1 => {
          VREG!(LUAU_INSN_B(insn) as u8, func);
          VJUMP!(LUAU_INSN_C(insn), i, self.insns, insnvalid);
          LUAU_ASSERT!(
            LUAU_INSN_OP(self.insns[i + 1 + LUAU_INSN_C(insn) as usize])
              == LuauOpcode::LOP_CALL as u32
          );
        }
        LuauOpcode::LOP_FASTCALL2 => {
          VREG!(LUAU_INSN_B(insn) as u8, func);
          VJUMP!(LUAU_INSN_C(insn), i, self.insns, insnvalid);
          LUAU_ASSERT!(
            LUAU_INSN_OP(self.insns[i + 1 + LUAU_INSN_C(insn) as usize])
              == LuauOpcode::LOP_CALL as u32
          );
          VREG!(self.insns[i + 1] as u8, func);
        }
        LuauOpcode::LOP_FASTCALL2K => {
          VREG!(LUAU_INSN_B(insn) as u8, func);
          VJUMP!(LUAU_INSN_C(insn), i, self.insns, insnvalid);
          LUAU_ASSERT!(
            LUAU_INSN_OP(self.insns[i + 1 + LUAU_INSN_C(insn) as usize])
              == LuauOpcode::LOP_CALL as u32
          );
          VCONSTANY!(self.insns[i + 1] as usize, self.constants);
        }
        LuauOpcode::LOP_FASTCALL3 => {
          VREG!(LUAU_INSN_B(insn) as u8, func);
          VJUMP!(LUAU_INSN_C(insn), i, self.insns, insnvalid);
          LUAU_ASSERT!(
            LUAU_INSN_OP(self.insns[i + 1 + LUAU_INSN_C(insn) as usize])
              == LuauOpcode::LOP_CALL as u32
          );
          VREG!((self.insns[i + 1] & 0xff) as u8, func);
          VREG!(((self.insns[i + 1] >> 8) & 0xff) as u8, func);
        }
        LuauOpcode::LOP_COVERAGE => {}
        LuauOpcode::LOP_CAPTURE => {
          let capture_type = LUAU_INSN_A(insn);
          match capture_type {
            a if a == LuauCaptureType::LCT_VAL as u32 => VREG!(LUAU_INSN_B(insn) as u8, func),
            a if a == LuauCaptureType::LCT_REF as u32 => {
              VREG!(LUAU_INSN_B(insn) as u8, func);
              open_captures.push(LUAU_INSN_B(insn) as u8);
            }
            a if a == LuauCaptureType::LCT_UPVAL as u32 => VUPVAL!(LUAU_INSN_B(insn) as u8, func),
            _ => LUAU_ASSERT!(false, "Unsupported capture type"),
          }
        }
        LuauOpcode::LOP_NEWCLASSMEMBER => {
          VREG!(LUAU_INSN_A(insn) as u8, func);
          LUAU_ASSERT!(LUAU_INSN_B(insn) == 0);
          VREG!(LUAU_INSN_C(insn) as u8, func);
          VCONST!(self.insns[i + 1] as usize, String, self.constants);
        }
        LuauOpcode::LOP_GETUDATAKS | LuauOpcode::LOP_SETUDATAKS => {
          VREG!(LUAU_INSN_A(insn) as u8, func);
          VREG!(LUAU_INSN_B(insn) as u8, func);
          VCONST!(
            LUAU_INSN_AUX_KV16(self.insns[i + 1]) as usize,
            String,
            self.constants
          );
        }
        LuauOpcode::LOP_NAMECALLUDATA => {
          VREG!(LUAU_INSN_A(insn) as u8, func);
          VREG!(LUAU_INSN_B(insn) as u8, func);
          VCONST!(
            LUAU_INSN_AUX_KV16(self.insns[i + 1]) as usize,
            String,
            self.constants
          );
          LUAU_ASSERT!(LUAU_INSN_OP(self.insns[i + 2]) == LuauOpcode::LOP_CALL as u32);
        }
        LuauOpcode::LOP_CMPPROTO => {
          VREG!(LUAU_INSN_A(insn) as u8, func);
          VJUMP!(LUAU_INSN_D(insn), i, self.insns, insnvalid);
        }
        LuauOpcode::LOP_FASTPCALL => {
          VJUMP!(LUAU_INSN_C(insn) as i32, i, self.insns, insnvalid);
          LUAU_ASSERT!(
            LUAU_INSN_OP(self.insns[i + 1 + LUAU_INSN_C(insn) as usize])
              == LuauOpcode::LOP_CALL as u32
          );
        }
        LuauOpcode::LOP_NEWCLASS => {
          LUAU_ASSERT!(DebugLuauUserDefinedClasses.get());
          VREG!(LUAU_INSN_A(insn) as u8, func);
          let super_reg = LUAU_INSN_B(insn) as u8;
          LUAU_ASSERT!(super_reg == 0xff || (super_reg as usize) < func.maxstacksize as usize);
          let flags = LUAU_INSN_C(insn) as u8;
          LUAU_ASSERT!(flags == 0 || flags == 1);
          VCONST!(self.insns[i + 1] as usize, ClassShape, self.constants);
        }
        _ => {
          LUAU_ASSERT!(false, "Unsupported opcode");
        }
      }

      let op_len = get_op_length(op) as usize;
      i += op_len;
      LUAU_ASSERT!(i <= self.insns.len());
    }

    // all CAPTURE REF instructions must have a CLOSEUPVALS instruction after them in the bytecode stream
    // this doesn't guarantee safety as it doesn't perform basic block based analysis, but if this fails
    // then the bytecode is definitely unsafe to run since the compiler won't generate backwards branches
    // except for loop edges
    LUAU_ASSERT!(open_captures.is_empty());
  }
}
