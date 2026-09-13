use ulua_common::{
  FFlag,
  enums::luau_opcode::LuauOpcode,
  macros::{
    luau_insn_a::LUAU_INSN_A, luau_insn_b::LUAU_INSN_B, luau_insn_c::LUAU_INSN_C,
    luau_insn_d::LUAU_INSN_D, luau_insn_op::LUAU_INSN_OP,
  },
};
use ulua_vm::enums::{lua_type::LuaType, tms::TMS};

use crate::{
  enums::{ir_cmd::IrCmd, ir_condition::IrCondition},
  functions::{
    get_op_length::get_op_length, is_direct_compare::is_direct_compare,
    translate_fast_call_n::translate_fast_call_n, translate_inst_and_x::translate_inst_and_x,
    translate_inst_binary::translate_inst_binary, translate_inst_binary_k::translate_inst_binary_k,
    translate_inst_binary_rk::translate_inst_binary_rk,
    translate_inst_capture::translate_inst_capture,
    translate_inst_close_upvals::translate_inst_close_upvals,
    translate_inst_cmp_proto::translate_inst_cmp_proto,
    translate_inst_concat::translate_inst_concat,
    translate_inst_dup_table::translate_inst_dup_table,
    translate_inst_for_g_loop_ipairs::translate_inst_for_g_loop_ipairs,
    translate_inst_for_g_prep_inext::translate_inst_for_g_prep_inext,
    translate_inst_for_g_prep_next::translate_inst_for_g_prep_next,
    translate_inst_for_n_loop::translate_inst_for_n_loop,
    translate_inst_for_n_prep::translate_inst_for_n_prep,
    translate_inst_get_global::translate_inst_get_global,
    translate_inst_get_import::translate_inst_get_import,
    translate_inst_get_table::translate_inst_get_table,
    translate_inst_get_table_ks::translate_inst_get_table_ks,
    translate_inst_get_table_n::translate_inst_get_table_n,
    translate_inst_get_upval::translate_inst_get_upval, translate_inst_jump::translate_inst_jump,
    translate_inst_jump_back::translate_inst_jump_back,
    translate_inst_jump_if::translate_inst_jump_if,
    translate_inst_jump_if_cond::translate_inst_jump_if_cond,
    translate_inst_jump_if_eq::translate_inst_jump_if_eq,
    translate_inst_jump_if_eq_shortcut::translate_inst_jump_if_eq_shortcut,
    translate_inst_jump_x::translate_inst_jump_x,
    translate_inst_jumpx_eq_b::translate_inst_jumpx_eq_b,
    translate_inst_jumpx_eq_b_shortcut::translate_inst_jumpx_eq_b_shortcut,
    translate_inst_jumpx_eq_n::translate_inst_jumpx_eq_n,
    translate_inst_jumpx_eq_n_shortcut::translate_inst_jumpx_eq_n_shortcut,
    translate_inst_jumpx_eq_nil::translate_inst_jumpx_eq_nil,
    translate_inst_jumpx_eq_nil_shortcut::translate_inst_jumpx_eq_nil_shortcut,
    translate_inst_jumpx_eq_s::translate_inst_jumpx_eq_s,
    translate_inst_jumpx_eq_s_shortcut::translate_inst_jumpx_eq_s_shortcut,
    translate_inst_length::translate_inst_length, translate_inst_load_b::translate_inst_load_b,
    translate_inst_load_k::translate_inst_load_k, translate_inst_load_kx::translate_inst_load_kx,
    translate_inst_load_n::translate_inst_load_n, translate_inst_load_nil::translate_inst_load_nil,
    translate_inst_minus::translate_inst_minus, translate_inst_move::translate_inst_move,
    translate_inst_namecall::translate_inst_namecall,
    translate_inst_new_closure::translate_inst_new_closure,
    translate_inst_new_table::translate_inst_new_table, translate_inst_not::translate_inst_not,
    translate_inst_or_x::translate_inst_or_x, translate_inst_set_global::translate_inst_set_global,
    translate_inst_set_table::translate_inst_set_table,
    translate_inst_set_table_ks::translate_inst_set_table_ks,
    translate_inst_set_table_n::translate_inst_set_table_n,
    translate_inst_set_upval::translate_inst_set_upval,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_builder::Instruction,
};

impl IrBuilder {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn translate_inst(&mut self, op: LuauOpcode, pc: *const Instruction, i: i32) {
    unsafe {
      match op {
        LuauOpcode::LOP_NOP => {}
        LuauOpcode::LOP_LOADNIL => translate_inst_load_nil(self, pc),
        LuauOpcode::LOP_LOADB => translate_inst_load_b(self, pc, i),
        LuauOpcode::LOP_LOADN => translate_inst_load_n(self, pc),
        LuauOpcode::LOP_LOADK => translate_inst_load_k(self, pc),
        LuauOpcode::LOP_LOADKX => translate_inst_load_kx(self, pc),
        LuauOpcode::LOP_MOVE => translate_inst_move(self, pc),
        LuauOpcode::LOP_GETGLOBAL => translate_inst_get_global(self, pc, i),
        LuauOpcode::LOP_SETGLOBAL => translate_inst_set_global(self, pc, i),
        LuauOpcode::LOP_CALL | LuauOpcode::LOP_CALLFB => {
          let interrupt_pc = self.const_uint(i as u32);
          self.inst_ir_cmd_ir_op(IrCmd::INTERRUPT, interrupt_pc);

          let savedpc = if FFlag::LuauCallFeedback.get() {
            i + get_op_length(op)
          } else {
            i + 1
          };
          let savedpc = self.const_uint(savedpc as u32);
          self.inst_ir_cmd_ir_op(IrCmd::SetSavedpc, savedpc);

          let ra = self.vm_reg(LUAU_INSN_A(*pc) as u8);
          let b = self.const_int(LUAU_INSN_B(*pc) as i32 - 1);
          let c = self.const_int(LUAU_INSN_C(*pc) as i32 - 1);
          self.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CALL, ra, b, c);

          if self.active_fastcall_fallback {
            let ret = self.fastcall_fallback_return;
            self.inst_ir_cmd_ir_op(IrCmd::JUMP, ret);
            self.begin_block(ret);
            self.active_fastcall_fallback = false;
          }
        }
        LuauOpcode::LOP_RETURN => {
          let interrupt_pc = self.const_uint(i as u32);
          self.inst_ir_cmd_ir_op(IrCmd::INTERRUPT, interrupt_pc);
          let ra = self.vm_reg(LUAU_INSN_A(*pc) as u8);
          let b = self.const_int(LUAU_INSN_B(*pc) as i32 - 1);
          self.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, ra, b);
        }
        LuauOpcode::LOP_GETTABLE => translate_inst_get_table(self, pc, i),
        LuauOpcode::LOP_SETTABLE => translate_inst_set_table(self, pc, i),
        LuauOpcode::LOP_GETTABLEKS | LuauOpcode::LOP_GETUDATAKS => {
          translate_inst_get_table_ks(self, pc, i)
        }
        LuauOpcode::LOP_SETTABLEKS | LuauOpcode::LOP_SETUDATAKS => {
          translate_inst_set_table_ks(self, pc, i)
        }
        LuauOpcode::LOP_GETTABLEN => translate_inst_get_table_n(self, pc, i),
        LuauOpcode::LOP_SETTABLEN => translate_inst_set_table_n(self, pc, i),
        LuauOpcode::LOP_JUMP => translate_inst_jump(self, pc, i),
        LuauOpcode::LOP_JUMPBACK => translate_inst_jump_back(self, pc, i),
        LuauOpcode::LOP_JUMPIF => translate_inst_jump_if(self, pc, i, false),
        LuauOpcode::LOP_JUMPIFNOT => translate_inst_jump_if(self, pc, i, true),
        LuauOpcode::LOP_JUMPIFEQ => {
          if is_direct_compare(self.function.proto, pc, i) {
            translate_inst_jump_if_eq_shortcut(self, pc, i, false);
            self.cmd_skip_target = i + 3;
          } else {
            translate_inst_jump_if_eq(self, pc, i, false);
          }
        }
        LuauOpcode::LOP_JUMPIFLE => {
          translate_inst_jump_if_cond(self, pc, i, IrCondition::LessEqual)
        }
        LuauOpcode::LOP_JUMPIFLT => translate_inst_jump_if_cond(self, pc, i, IrCondition::Less),
        LuauOpcode::LOP_JUMPIFNOTEQ => {
          if is_direct_compare(self.function.proto, pc, i) {
            translate_inst_jump_if_eq_shortcut(self, pc, i, true);
            self.cmd_skip_target = i + 3;
          } else {
            translate_inst_jump_if_eq(self, pc, i, true);
          }
        }
        LuauOpcode::LOP_JUMPIFNOTLE => {
          translate_inst_jump_if_cond(self, pc, i, IrCondition::NotLessEqual)
        }
        LuauOpcode::LOP_JUMPIFNOTLT => {
          translate_inst_jump_if_cond(self, pc, i, IrCondition::NotLess)
        }
        LuauOpcode::LOP_JUMPX => translate_inst_jump_x(self, pc, i),
        LuauOpcode::LOP_JUMPXEQKNIL => {
          if is_direct_compare(self.function.proto, pc, i) {
            translate_inst_jumpx_eq_nil_shortcut(self, pc, i);
            self.cmd_skip_target = i + 3;
          } else {
            translate_inst_jumpx_eq_nil(self, pc, i);
          }
        }
        LuauOpcode::LOP_JUMPXEQKB => {
          if is_direct_compare(self.function.proto, pc, i) {
            translate_inst_jumpx_eq_b_shortcut(self, pc, i);
            self.cmd_skip_target = i + 3;
          } else {
            translate_inst_jumpx_eq_b(self, pc, i);
          }
        }
        LuauOpcode::LOP_JUMPXEQKN => {
          if is_direct_compare(self.function.proto, pc, i) {
            translate_inst_jumpx_eq_n_shortcut(self, pc, i);
            self.cmd_skip_target = i + 3;
          } else {
            translate_inst_jumpx_eq_n(self, pc, i);
          }
        }
        LuauOpcode::LOP_JUMPXEQKS => {
          if is_direct_compare(self.function.proto, pc, i) {
            translate_inst_jumpx_eq_s_shortcut(self, pc, i);
            self.cmd_skip_target = i + 3;
          } else {
            translate_inst_jumpx_eq_s(self, pc, i);
          }
        }
        LuauOpcode::LOP_ADD => translate_inst_binary(self, pc, i, TMS::TmAdd),
        LuauOpcode::LOP_SUB => translate_inst_binary(self, pc, i, TMS::TmSub),
        LuauOpcode::LOP_MUL => translate_inst_binary(self, pc, i, TMS::TmMul),
        LuauOpcode::LOP_DIV => translate_inst_binary(self, pc, i, TMS::TmDiv),
        LuauOpcode::LOP_IDIV => translate_inst_binary(self, pc, i, TMS::TmIDiv),
        LuauOpcode::LOP_MOD => translate_inst_binary(self, pc, i, TMS::TmMod),
        LuauOpcode::LOP_POW => translate_inst_binary(self, pc, i, TMS::TmPow),
        LuauOpcode::LOP_ADDK => translate_inst_binary_k(self, pc, i, TMS::TmAdd),
        LuauOpcode::LOP_SUBK => translate_inst_binary_k(self, pc, i, TMS::TmSub),
        LuauOpcode::LOP_MULK => translate_inst_binary_k(self, pc, i, TMS::TmMul),
        LuauOpcode::LOP_DIVK => translate_inst_binary_k(self, pc, i, TMS::TmDiv),
        LuauOpcode::LOP_IDIVK => translate_inst_binary_k(self, pc, i, TMS::TmIDiv),
        LuauOpcode::LOP_MODK => translate_inst_binary_k(self, pc, i, TMS::TmMod),
        LuauOpcode::LOP_POWK => translate_inst_binary_k(self, pc, i, TMS::TmPow),
        LuauOpcode::LOP_SUBRK => translate_inst_binary_rk(self, pc, i, TMS::TmSub),
        LuauOpcode::LOP_DIVRK => translate_inst_binary_rk(self, pc, i, TMS::TmDiv),
        LuauOpcode::LOP_NOT => translate_inst_not(self, pc),
        LuauOpcode::LOP_MINUS => translate_inst_minus(self, pc, i),
        LuauOpcode::LOP_LENGTH => translate_inst_length(self, pc, i),
        LuauOpcode::LOP_NEWTABLE => translate_inst_new_table(self, pc, i),
        LuauOpcode::LOP_DUPTABLE => translate_inst_dup_table(self, pc, i),
        LuauOpcode::LOP_SETLIST => {
          let pcpos = self.const_uint(i as u32);
          let ra = self.vm_reg(LUAU_INSN_A(*pc) as u8);
          let rb = self.vm_reg(LUAU_INSN_B(*pc) as u8);
          let c = self.const_int(LUAU_INSN_C(*pc) as i32 - 1);
          let aux = self.const_uint(*pc.add(1));
          let undef = self.undef();
          self.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op_ir_op(
            IrCmd::SETLIST,
            pcpos,
            ra,
            rb,
            c,
            aux,
            undef,
          );
        }
        LuauOpcode::LOP_GETUPVAL => translate_inst_get_upval(self, pc, i),
        LuauOpcode::LOP_SETUPVAL => translate_inst_set_upval(self, pc, i),
        LuauOpcode::LOP_CLOSEUPVALS => translate_inst_close_upvals(self, pc),
        LuauOpcode::LOP_FASTCALL => {
          let undef1 = self.undef();
          let undef2 = self.undef();
          let fallback = translate_fast_call_n(self, pc, i, false, 0, undef1, undef2);
          self.handle_fastcall_fallback(fallback, pc, i);
        }
        LuauOpcode::LOP_FASTCALL1 => {
          let undef1 = self.undef();
          let undef2 = self.undef();
          let fallback = translate_fast_call_n(self, pc, i, true, 1, undef1, undef2);
          self.handle_fastcall_fallback(fallback, pc, i);
        }
        LuauOpcode::LOP_FASTCALL2 => {
          let arg = self.vm_reg(*pc.add(1) as u8);
          let undef = self.undef();
          let fallback = translate_fast_call_n(self, pc, i, true, 2, arg, undef);
          self.handle_fastcall_fallback(fallback, pc, i);
        }
        LuauOpcode::LOP_FASTCALL2K => {
          let arg = self.vm_const(*pc.add(1));
          let undef = self.undef();
          let fallback = translate_fast_call_n(self, pc, i, true, 2, arg, undef);
          self.handle_fastcall_fallback(fallback, pc, i);
        }
        LuauOpcode::LOP_FASTCALL3 => {
          let aux = *pc.add(1);
          let arg2 = self.vm_reg((aux & 0xff) as u8);
          let arg3 = self.vm_reg(((aux >> 8) & 0xff) as u8);
          let fallback = translate_fast_call_n(self, pc, i, true, 3, arg2, arg3);
          self.handle_fastcall_fallback(fallback, pc, i);
        }
        LuauOpcode::LOP_FORNPREP => translate_inst_for_n_prep(self, pc, i),
        LuauOpcode::LOP_FORNLOOP => translate_inst_for_n_loop(self, pc, i),
        LuauOpcode::LOP_FORGLOOP => {
          let aux = *pc.add(1) as i32;
          if aux < 0 {
            translate_inst_for_g_loop_ipairs(self, pc, i);
          } else {
            let ra = LUAU_INSN_A(*pc) as u8;
            let loop_repeat = self.block_at_inst((i + 1 + LUAU_INSN_D(*pc)) as u32);
            let loop_exit =
              self.block_at_inst((i + get_op_length(LuauOpcode::LOP_FORGLOOP)) as u32);
            let fallback = self.fallback_block(i as u32);

            let pcpos = self.const_uint(i as u32);
            self.inst_ir_cmd_ir_op(IrCmd::INTERRUPT, pcpos);
            let reg_ra = self.vm_reg(ra);
            self.load_and_check_tag(reg_ra, LuaType::Nil as u8, fallback);

            let reg_ra = self.vm_reg(ra);
            let aux_op = self.const_int(aux);
            self.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
              IrCmd::FORGLOOP,
              reg_ra,
              aux_op,
              loop_repeat,
              loop_exit,
            );

            self.begin_block(fallback);
            let savedpc = self.const_uint((i + 1) as u32);
            self.inst_ir_cmd_ir_op(IrCmd::SetSavedpc, savedpc);
            let reg_ra = self.vm_reg(ra);
            let aux_op = self.const_int(aux);
            self.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
              IrCmd::ForgloopFallback,
              reg_ra,
              aux_op,
              loop_repeat,
              loop_exit,
            );

            self.begin_block(loop_exit);
          }
        }
        LuauOpcode::LOP_FORGPREP_NEXT => translate_inst_for_g_prep_next(self, pc, i),
        LuauOpcode::LOP_FORGPREP_INEXT => translate_inst_for_g_prep_inext(self, pc, i),
        LuauOpcode::LOP_AND => {
          let c = self.vm_reg(LUAU_INSN_C(*pc) as u8);
          translate_inst_and_x(self, pc, i, c);
        }
        LuauOpcode::LOP_ANDK => {
          let c = self.vm_const(LUAU_INSN_C(*pc) as u32);
          translate_inst_and_x(self, pc, i, c);
        }
        LuauOpcode::LOP_OR => {
          let c = self.vm_reg(LUAU_INSN_C(*pc) as u8);
          translate_inst_or_x(self, pc, i, c);
        }
        LuauOpcode::LOP_ORK => {
          let c = self.vm_const(LUAU_INSN_C(*pc) as u32);
          translate_inst_or_x(self, pc, i, c);
        }
        LuauOpcode::LOP_COVERAGE => {
          let pcpos = self.const_uint(i as u32);
          self.inst_ir_cmd_ir_op(IrCmd::COVERAGE, pcpos);
        }
        LuauOpcode::LOP_GETIMPORT => translate_inst_get_import(self, pc, i),
        LuauOpcode::LOP_CONCAT => translate_inst_concat(self, pc, i),
        LuauOpcode::LOP_CAPTURE => translate_inst_capture(self, pc, i),
        LuauOpcode::LOP_NAMECALL | LuauOpcode::LOP_NAMECALLUDATA => {
          if translate_inst_namecall(self, pc, i) {
            if FFlag::LuauCallFeedback.get() {
              let namecall = get_op_length(LuauOpcode::LOP_NAMECALL);
              let call_op = LuauOpcode::from(LUAU_INSN_OP(*pc.add(namecall as usize)) as u8);
              CODEGEN_ASSERT!(call_op == LuauOpcode::LOP_CALL || call_op == LuauOpcode::LOP_CALLFB);
              let call = get_op_length(call_op);
              self.cmd_skip_target = i + namecall + call;
            } else {
              self.cmd_skip_target = i + 3;
            }
          }
        }
        LuauOpcode::LOP_PREPVARARGS => {
          let pcpos = self.const_uint(i as u32);
          let a = self.const_int(LUAU_INSN_A(*pc) as i32);
          self.inst_ir_cmd_ir_op_ir_op(IrCmd::FallbackPrepvarargs, pcpos, a);
        }
        LuauOpcode::LOP_GETVARARGS => {
          let pcpos = self.const_uint(i as u32);
          let ra = self.vm_reg(LUAU_INSN_A(*pc) as u8);
          let b = self.const_int(LUAU_INSN_B(*pc) as i32 - 1);
          self.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::FallbackGetvarargs, pcpos, ra, b);
        }
        LuauOpcode::LOP_NEWCLOSURE => translate_inst_new_closure(self, pc, i),
        LuauOpcode::LOP_DUPCLOSURE => {
          let pcpos = self.const_uint(i as u32);
          let ra = self.vm_reg(LUAU_INSN_A(*pc) as u8);
          let kd = self.vm_const(LUAU_INSN_D(*pc) as u32);
          self.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::FallbackDupclosure, pcpos, ra, kd);
        }
        LuauOpcode::LOP_FORGPREP => {
          let loop_start = self.block_at_inst((i + 1 + LUAU_INSN_D(*pc)) as u32);
          let pcpos = self.const_uint(i as u32);
          let ra = self.vm_reg(LUAU_INSN_A(*pc) as u8);
          self.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::FallbackForgprep, pcpos, ra, loop_start);
        }
        LuauOpcode::LOP_NEWCLASSMEMBER => {
          let exit = self.vm_exit(i as u32);
          self.inst_ir_cmd_ir_op(IrCmd::JUMP, exit);
        }
        LuauOpcode::LOP_CMPPROTO => translate_inst_cmp_proto(self, pc, i),
        _ => CODEGEN_ASSERT!(false),
      }
    }
  }
}
