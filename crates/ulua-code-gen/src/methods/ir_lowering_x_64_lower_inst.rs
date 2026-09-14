//! Node: `cxx:Method:Luau.CodeGen:CodeGen/src/IrLoweringX64.cpp:50:lowerInst`
//! Mechanically transpiled (translation/scripts/lowerinst_rewrite.py) + compiler-driven repair.
use core::{
  ffi::c_void,
  mem::{size_of, transmute},
  ptr::{copy_nonoverlapping, null_mut},
};

use ulua_common::FFlag;
use ulua_vm::{
  enums::{lua_type::LuaType, tms::TMS},
  macros::{lua_multret::LUA_MULTRET, setnvalue::setnvalue},
  records::{
    call_info::CallInfo,
    closure::{Closure, LClosure},
    global_state::global_State,
    lua_t_value::TValue,
    proto::Proto,
    t_string::tstring,
    up_val::UpVal,
  },
  type_aliases::{
    instruction::Instruction, lua_node::LuaNode, lua_state::lua_State, lua_table::LuaTable,
    luau_fast_function::luau_FastFunction, udata::Udata,
  },
};

// local register-constant helpers (mirrors EmitCommonX64.h)
use crate::enums::size_x_64::SizeX64 as CrateSizeX64;
use crate::{
  enums::{
    condition_x_64::ConditionX64, features_x_64::FeaturesX64, ir_cmd::IrCmd,
    ir_condition::IrCondition, ir_const_kind::IrConstKind, ir_op_kind::IrOpKind,
    ir_value_kind::IrValueKind, rounding_mode_x_64::RoundingModeX64, size_x_64::SizeX64,
  },
  macros::{
    codegen_assert::CODEGEN_ASSERT, has_op_b::HAS_OP_B, has_op_c::HAS_OP_C, has_op_d::HAS_OP_D,
    has_op_e::HAS_OP_E,
  },
  records::{
    interrupt_handler_ir_lowering_x_64::InterruptHandler, ir_block::IrBlock,
    ir_call_wrapper_x_64::IrCallWrapperX64, ir_inst::IrInst, ir_lowering_x_64::IrLoweringX64,
    ir_op::IrOp, label::Label, native_context::NativeContext, operand_x_64::OperandX64,
    register_x_64::RegisterX64, scoped_reg_x_64::ScopedRegX64, scoped_spills::ScopedSpills,
  },
};
const K_TVALUE_SIZE_LOG2: i32 = 4;
const K_LUA_NODE_SIZE_LOG2: i32 = 5;
const K_OFFSET_OF_TKEY_TAG_NEXT: i32 = 12;
const K_TKEY_TAG_BITS: i32 = 4;
const K_TKEY_TAG_MASK: i32 = (1 << K_TKEY_TAG_BITS) - 1;
const K_STACK_OFFSET_TO_LOCALS: i32 = 48;
const K_INVALID_INST_IDX: u32 = IrLoweringX64::K_INVALID_INST_IDX;
const INT_MAX: i32 = i32::MAX;
const K_TSTRING_LEN_OFFSET: i32 = 36;
const K_BUFFER_LEN_OFFSET: i32 = 12;
const K_CLOSURE_LUPREFS_OFFSET: i32 =
  (core::mem::offset_of!(Closure, inner) + core::mem::offset_of!(LClosure, uprefs)) as i32;
const K_CLOSURE_LPOFFSET: i32 =
  (core::mem::offset_of!(Closure, inner) + core::mem::offset_of!(LClosure, p)) as i32;
const S_CLOSURE: OperandX64 = OperandX64::mem(
  SizeX64::Qword,
  RegisterX64::NOREG,
  1,
  RegisterX64::RSP,
  K_STACK_OFFSET_TO_LOCALS,
);
const S_CODE: OperandX64 = OperandX64::mem(
  SizeX64::Qword,
  RegisterX64::NOREG,
  1,
  RegisterX64::RSP,
  K_STACK_OFFSET_TO_LOCALS + 8,
);
const fn r_state() -> RegisterX64 {
  RegisterX64 {
    bits: (15u8 << RegisterX64::INDEX_SHIFT) | CrateSizeX64::Qword as u8,
  }
}
const fn r_base() -> RegisterX64 {
  RegisterX64 {
    bits: (14u8 << RegisterX64::INDEX_SHIFT) | CrateSizeX64::Qword as u8,
  }
}
const fn r_constants() -> RegisterX64 {
  RegisterX64 {
    bits: (12u8 << RegisterX64::INDEX_SHIFT) | CrateSizeX64::Qword as u8,
  }
}
const fn r_native_context() -> RegisterX64 {
  RegisterX64 {
    bits: (13u8 << RegisterX64::INDEX_SHIFT) | SizeX64::Qword as u8,
  }
}
const fn xmm(index: u8) -> RegisterX64 {
  RegisterX64 {
    bits: (index << RegisterX64::INDEX_SHIFT) | CrateSizeX64::Xmmword as u8,
  }
}
const fn xmm0() -> RegisterX64 {
  xmm(0)
}
fn sized_mem(mut op: OperandX64, size: SizeX64) -> OperandX64 {
  op.mem_size = size;
  op
}
use crate::functions::{
  byte_reg::byte_reg, call_arith_helper::call_arith_helper,
  call_barrier_object::call_barrier_object, call_barrier_table_fast::call_barrier_table_fast,
  call_get_table::call_get_table, call_length_helper::call_length_helper,
  call_set_table::call_set_table, call_step_gc::call_step_gc,
  check_object_barrier_conditions::check_object_barrier_conditions, condition_op::condition_op,
  convert_number_to_index_or_jump::convert_number_to_index_or_jump, dword_reg::dword_reg,
  emit_builtin_emit_builtins_x_64::emit_builtin_ir_reg_alloc_x_64_assembly_builder_x_64_i32_i32_i32_i32 as emit_builtin,
  emit_fallback_emit_common_x_64::emit_fallback, emit_inst_call::emit_inst_call,
  emit_inst_for_g_loop::emit_inst_for_g_loop, emit_inst_return::emit_inst_return,
  emit_inst_set_list::emit_inst_set_list, emit_update_base_emit_common_x_64::emit_update_base,
  get_cmd_value_kind::get_cmd_value_kind, get_condition_int_emit_common_x_64::get_condition_int,
  get_inverse_condition_condition_x_64::get_inverse_condition,
  get_native_context_offset::get_native_context_offset,
  get_negated_condition_ir_utils::get_negated_condition_ir_condition, get_op_ir_data::get_op_mut,
  get_table_node_at_cached_slot::get_table_node_at_cached_slot, is_gco::is_gco,
  jump_if_falsy::jump_if_falsy, jump_if_truthy::jump_if_truthy,
  jump_on_number_cmp::jump_on_number_cmp, luau_constant::luau_constant,
  luau_constant_address::luau_constant_address, luau_constant_tag::luau_constant_tag,
  luau_constant_value::luau_constant_value, luau_node_key_tag::luau_node_key_tag,
  luau_node_key_value::luau_node_key_value, luau_reg::luau_reg, luau_reg_address::luau_reg_address,
  luau_reg_extra::luau_reg_extra, luau_reg_tag::luau_reg_tag, luau_reg_value::luau_reg_value,
  luau_reg_value_int::luau_reg_value_int, luau_reg_value_int_64::luau_reg_value_int_64,
  luau_reg_value_vector::luau_reg_value_vector,
  produces_dirty_high_register_bits::produces_dirty_high_register_bits, qword_reg::qword_reg,
  vm_const_op::vm_const_op, vm_reg_op::vm_reg_op, vm_upvalue_op::vm_upvalue_op, word_reg::word_reg,
};

impl IrLoweringX64 {
  pub fn lower_inst(&mut self, inst: &mut IrInst, index: u32, next: &IrBlock) {
    unsafe {
      self.regs.curr_inst_idx = index;

      self.value_tracker.before_inst_lowering(inst);
      match inst.cmd {
        IrCmd::LoadTag => {
          inst.reg_x64 = self.regs.alloc_reg(SizeX64::Dword, index);

          if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::VmReg {
            (*self.build).mov(
              OperandX64::reg(inst.reg_x64),
              luau_reg_tag(vm_reg_op(*get_op_mut(inst, 0))),
            );
          } else if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::VmConst {
            (*self.build).mov(
              OperandX64::reg(inst.reg_x64),
              luau_constant_tag(vm_const_op(*get_op_mut(inst, 0))),
            );
          }
          // If we have a register, we assume it's a pointer to TValue
          // We might introduce explicit operand types in the future to make this more robust
          else if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Inst {
            (*self.build).mov(
              OperandX64::reg(inst.reg_x64),
              OperandX64::mem(
                SizeX64::Dword,
                RegisterX64::NOREG,
                1,
                self.reg_op(*get_op_mut(inst, 0)),
                (core::mem::offset_of!(TValue, tt) as i32),
              ),
            );
          } else {
            CODEGEN_ASSERT!(false, "Unsupported instruction form");
          }
        }
        IrCmd::LoadPointer => {
          inst.reg_x64 = self.regs.alloc_reg(SizeX64::Qword, index);

          if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::VmReg {
            (*self.build).mov(
              OperandX64::reg(inst.reg_x64),
              luau_reg_value(vm_reg_op(*get_op_mut(inst, 0))),
            );
          } else if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::VmConst {
            (*self.build).mov(
              OperandX64::reg(inst.reg_x64),
              luau_constant_value(vm_const_op(*get_op_mut(inst, 0))),
            );
          }
          // If we have a register, we assume it's a pointer to TValue
          // We might introduce explicit operand types in the future to make this more robust
          else if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Inst {
            (*self.build).mov(
              OperandX64::reg(inst.reg_x64),
              OperandX64::mem(
                SizeX64::Qword,
                RegisterX64::NOREG,
                1,
                self.reg_op(*get_op_mut(inst, 0)),
                (core::mem::offset_of!(TValue, value) as i32),
              ),
            );
          } else {
            CODEGEN_ASSERT!(false, "Unsupported instruction form");
          }
        }
        IrCmd::LoadDouble => {
          inst.reg_x64 = self.regs.alloc_reg(SizeX64::Xmmword, index);

          if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::VmReg {
            (*self.build).vmovsd_operand_x_64_operand_x_64(
              OperandX64::reg(inst.reg_x64),
              luau_reg_value(vm_reg_op(*get_op_mut(inst, 0))),
            );
          } else if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::VmConst {
            (*self.build).vmovsd_operand_x_64_operand_x_64(
              OperandX64::reg(inst.reg_x64),
              luau_constant_value(vm_const_op(*get_op_mut(inst, 0))),
            );
          } else {
            CODEGEN_ASSERT!(false, "Unsupported instruction form");
          }
        }
        IrCmd::LoadInt => {
          inst.reg_x64 = self.regs.alloc_reg(SizeX64::Dword, index);

          (*self.build).mov(
            OperandX64::reg(inst.reg_x64),
            luau_reg_value_int(vm_reg_op(*get_op_mut(inst, 0))),
          );
        }
        IrCmd::LoadInt64 => {
          inst.reg_x64 = self.regs.alloc_reg(SizeX64::Qword, index);

          if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::VmReg {
            (*self.build).mov(
              OperandX64::reg(inst.reg_x64),
              luau_reg_value_int_64(vm_reg_op(*get_op_mut(inst, 0))),
            );
          } else if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::VmConst {
            (*self.build).mov(
              OperandX64::reg(inst.reg_x64),
              luau_constant_value(vm_const_op(*get_op_mut(inst, 0))),
            );
          } else {
            CODEGEN_ASSERT!(false, "Unsupported instruction form");
          }
        }
        IrCmd::LoadFloat => {
          inst.reg_x64 = self.regs.alloc_reg(SizeX64::Xmmword, index);

          if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::VmReg {
            (*self.build).vmovss_operand_x_64_operand_x_64(
              OperandX64::reg(inst.reg_x64),
              OperandX64::mem(
                SizeX64::Dword,
                RegisterX64::NOREG,
                1,
                r_base(),
                vm_reg_op(*get_op_mut(inst, 0)) * (size_of::<TValue>() as i32)
                  + (core::mem::offset_of!(TValue, value) as i32)
                  + self.int_op(*get_op_mut(inst, 1)),
              ),
            );
          } else if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::VmConst {
            (*self.build).vmovss_operand_x_64_operand_x_64(
              OperandX64::reg(inst.reg_x64),
              OperandX64::mem(
                SizeX64::Dword,
                RegisterX64::NOREG,
                1,
                r_constants(),
                vm_const_op(*get_op_mut(inst, 0)) * (size_of::<TValue>() as i32)
                  + (core::mem::offset_of!(TValue, value) as i32)
                  + self.int_op(*get_op_mut(inst, 1)),
              ),
            );
          } else {
            CODEGEN_ASSERT!(false, "Unsupported instruction form");
          }
        }
        IrCmd::LoadTvalue => {
          inst.reg_x64 = self.regs.alloc_reg(SizeX64::Xmmword, index);

          let addr_offset = if HAS_OP_B!(inst) {
            self.int_op(*get_op_mut(inst, 1))
          } else {
            0
          };

          if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::VmReg {
            (*self.build).vmovups(
              OperandX64::reg(inst.reg_x64),
              luau_reg(vm_reg_op(*get_op_mut(inst, 0))),
            );
          } else if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::VmConst {
            (*self.build).vmovups(
              OperandX64::reg(inst.reg_x64),
              luau_constant(vm_const_op(*get_op_mut(inst, 0))),
            );
          } else if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Inst {
            (*self.build).vmovups(
              OperandX64::reg(inst.reg_x64),
              OperandX64::mem(
                SizeX64::Xmmword,
                RegisterX64::NOREG,
                1,
                self.reg_op(*get_op_mut(inst, 0)),
                addr_offset,
              ),
            );
          } else {
            CODEGEN_ASSERT!(false, "Unsupported instruction form");
          }
        }
        IrCmd::LoadEnv => {
          inst.reg_x64 = self.regs.alloc_reg(SizeX64::Qword, index);

          (*self.build).mov(OperandX64::reg(inst.reg_x64), S_CLOSURE);
          (*self.build).mov(
            OperandX64::reg(inst.reg_x64),
            OperandX64::mem(
              SizeX64::Qword,
              RegisterX64::NOREG,
              1,
              inst.reg_x64,
              (core::mem::offset_of!(Closure, env) as i32),
            ),
          );
        }
        IrCmd::GetArrAddr => {
          if (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Inst {
            inst.reg_x64 =
              self
                .regs
                .alloc_reg_or_reuse(SizeX64::Qword, index, &[(*get_op_mut(inst, 1_u32))]);

            if dword_reg(inst.reg_x64) != self.reg_op(*get_op_mut(inst, 1)) {
              (*self.build).mov(
                OperandX64::reg(dword_reg(inst.reg_x64)),
                OperandX64::reg(self.reg_op(*get_op_mut(inst, 1))),
              );
            }

            (*self.build).shl(
              OperandX64::reg(dword_reg(inst.reg_x64)),
              OperandX64::imm(K_TVALUE_SIZE_LOG2),
            );
            (*self.build).add(
              OperandX64::reg(inst.reg_x64),
              OperandX64::mem(
                SizeX64::Qword,
                RegisterX64::NOREG,
                1,
                self.reg_op(*get_op_mut(inst, 0)),
                (core::mem::offset_of!(LuaTable, array) as i32),
              ),
            );
          } else if (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Constant {
            inst.reg_x64 =
              self
                .regs
                .alloc_reg_or_reuse(SizeX64::Qword, index, &[(*get_op_mut(inst, 0_u32))]);

            (*self.build).mov(
              OperandX64::reg(inst.reg_x64),
              OperandX64::mem(
                SizeX64::Qword,
                RegisterX64::NOREG,
                1,
                self.reg_op(*get_op_mut(inst, 0)),
                (core::mem::offset_of!(LuaTable, array) as i32),
              ),
            );

            if self.int_op(*get_op_mut(inst, 1)) != 0 {
              (*self.build).lea_operand_x_64_operand_x_64(
                OperandX64::reg(inst.reg_x64),
                OperandX64::mem(
                  SizeX64::None,
                  RegisterX64::NOREG,
                  1,
                  inst.reg_x64,
                  self.int_op(*get_op_mut(inst, 1)) * (size_of::<TValue>() as i32),
                ),
              );
            }
          } else {
            CODEGEN_ASSERT!(false, "Unsupported instruction form");
          }
        }
        IrCmd::GetSlotNodeAddr => {
          inst.reg_x64 = self.regs.alloc_reg(SizeX64::Qword, index);

          let mut tmp = ScopedRegX64 {
            owner: &mut self.regs,
            reg: RegisterX64::NOREG,
          };
          tmp.alloc(SizeX64::Qword);

          get_table_node_at_cached_slot(
            &mut *self.build,
            tmp.reg,
            inst.reg_x64,
            self.reg_op(*get_op_mut(inst, 0)),
            (self.uint_op(*get_op_mut(inst, 1))) as i32,
          );
        }
        IrCmd::GetHashNodeAddr => {
          {
            // Custom bit shift value can only be placed in RegisterX64::CL
            let shift_tmp = ScopedRegX64 {
              owner: &mut self.regs,
              reg: self.regs.take_reg(RegisterX64::RCX, K_INVALID_INST_IDX),
            };

            inst.reg_x64 = self.regs.alloc_reg(SizeX64::Qword, index);

            let mut tmp = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp.alloc(SizeX64::Qword);

            (*self.build).mov(
              OperandX64::reg(inst.reg_x64),
              OperandX64::mem(
                SizeX64::Qword,
                RegisterX64::NOREG,
                1,
                self.reg_op(*get_op_mut(inst, 0)),
                (core::mem::offset_of!(LuaTable, node) as i32),
              ),
            );
            (*self.build).mov(OperandX64::reg(dword_reg(tmp.reg)), OperandX64::imm(1_i32));
            (*self.build).mov(
              OperandX64::reg(byte_reg(shift_tmp.reg)),
              OperandX64::mem(
                SizeX64::Byte,
                RegisterX64::NOREG,
                1,
                self.reg_op(*get_op_mut(inst, 0)),
                (core::mem::offset_of!(LuaTable, lsizenode) as i32),
              ),
            );
            (*self.build).shl(
              OperandX64::reg(dword_reg(tmp.reg)),
              OperandX64::reg(byte_reg(shift_tmp.reg)),
            );
            (*self.build).dec(OperandX64::reg(dword_reg(tmp.reg)));
            (*self.build).and_(
              OperandX64::reg(dword_reg(tmp.reg)),
              OperandX64::imm((self.uint_op(*get_op_mut(inst, 1))) as i32),
            );
            (*self.build).shl(
              OperandX64::reg(tmp.reg),
              OperandX64::imm(K_LUA_NODE_SIZE_LOG2),
            );
            (*self.build).add(OperandX64::reg(inst.reg_x64), OperandX64::reg(tmp.reg));
          };
        }
        IrCmd::GetClosureUpvalAddr => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Qword, index, &[(*get_op_mut(inst, 0_u32))]);

          if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Undef {
            (*self.build).mov(OperandX64::reg(inst.reg_x64), S_CLOSURE);
          } else {
            let cl = self.reg_op(*get_op_mut(inst, 0));
            if inst.reg_x64 != cl {
              (*self.build).mov(OperandX64::reg(inst.reg_x64), OperandX64::reg(cl));
            }
          }

          (*self.build).add(
            OperandX64::reg(inst.reg_x64),
            OperandX64::imm(
              (core::mem::offset_of!(Closure, inner) as i32)
                + (core::mem::offset_of!(LClosure, uprefs) as i32)
                + (size_of::<TValue>() as i32) * vm_upvalue_op(*get_op_mut(inst, 1)) as i32,
            ),
          );
        }
        IrCmd::StoreTag => {
          if (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Constant {
            if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Inst {
              (*self.build).mov(
                OperandX64::mem(
                  SizeX64::Dword,
                  RegisterX64::NOREG,
                  1,
                  self.reg_op(*get_op_mut(inst, 0)),
                  (core::mem::offset_of!(TValue, tt) as i32),
                ),
                OperandX64::imm((self.tag_op(*get_op_mut(inst, 1))) as i32),
              );
            } else {
              (*self.build).mov(
                luau_reg_tag(vm_reg_op(*get_op_mut(inst, 0))),
                OperandX64::imm((self.tag_op(*get_op_mut(inst, 1))) as i32),
              );
            }
          } else {
            CODEGEN_ASSERT!(false, "Unsupported instruction form");
          }
        }
        IrCmd::StorePointer => {
          let value_lhs = if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Inst {
            OperandX64::mem(
              SizeX64::Qword,
              RegisterX64::NOREG,
              1,
              self.reg_op(*get_op_mut(inst, 0)),
              (core::mem::offset_of!(TValue, value) as i32),
            )
          } else {
            luau_reg_value(vm_reg_op(*get_op_mut(inst, 0)))
          };

          if (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Constant {
            CODEGEN_ASSERT!(self.int_op(*get_op_mut(inst, 1)) == 0);
            (*self.build).mov(value_lhs, OperandX64::imm(0_i32));
          } else if (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Inst {
            (*self.build).mov(
              value_lhs,
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 1))),
            );
          } else {
            CODEGEN_ASSERT!(false, "Unsupported instruction form");
          }
        }
        IrCmd::StoreExtra => {
          if (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Constant {
            if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Inst {
              (*self.build).mov(
                OperandX64::mem(
                  SizeX64::Dword,
                  RegisterX64::NOREG,
                  1,
                  self.reg_op(*get_op_mut(inst, 0)),
                  (core::mem::offset_of!(TValue, extra) as i32),
                ),
                OperandX64::imm(self.int_op(*get_op_mut(inst, 1))),
              );
            } else {
              (*self.build).mov(
                luau_reg_extra(vm_reg_op(*get_op_mut(inst, 0))),
                OperandX64::imm(self.int_op(*get_op_mut(inst, 1))),
              );
            }
          } else {
            CODEGEN_ASSERT!(false, "Unsupported instruction form");
          }
        }
        IrCmd::StoreDouble => {
          let value_lhs = if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Inst {
            OperandX64::mem(
              SizeX64::Qword,
              RegisterX64::NOREG,
              1,
              self.reg_op(*get_op_mut(inst, 0)),
              (core::mem::offset_of!(TValue, value) as i32),
            )
          } else {
            luau_reg_value(vm_reg_op(*get_op_mut(inst, 0)))
          };

          if (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Constant {
            let mut tmp = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp.alloc(SizeX64::Xmmword);

            (*self.build).vmovsd_operand_x_64_operand_x_64(
              OperandX64::reg(tmp.reg),
              (*self.build).f64(self.double_op(*get_op_mut(inst, 1))),
            );
            (*self.build).vmovsd_operand_x_64_operand_x_64(value_lhs, OperandX64::reg(tmp.reg));
          } else if (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Inst {
            (*self.build).vmovsd_operand_x_64_operand_x_64(
              value_lhs,
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 1))),
            );
          } else {
            CODEGEN_ASSERT!(false, "Unsupported instruction form");
          }
        }
        IrCmd::StoreInt => {
          if (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Constant {
            (*self.build).mov(
              luau_reg_value_int(vm_reg_op(*get_op_mut(inst, 0))),
              OperandX64::imm(self.int_op(*get_op_mut(inst, 1))),
            );
          } else if (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Inst {
            (*self.build).mov(
              luau_reg_value_int(vm_reg_op(*get_op_mut(inst, 0))),
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 1))),
            );
          } else {
            CODEGEN_ASSERT!(false, "Unsupported instruction form");
          }
        }
        IrCmd::StoreInt64 => {
          if (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Constant {
            let value = self.int64_op(*get_op_mut(inst, 1));

            // x64 mov r/m64, imm32 sign-extends
            // otherwise we use register for values outside that range
            if value >= i32::MIN as i64 && value <= i32::MAX as i64 {
              (*self.build).mov(
                luau_reg_value_int_64(vm_reg_op(*get_op_mut(inst, 0))),
                OperandX64::imm((value) as i32),
              );
            } else {
              let mut tmp = ScopedRegX64 {
                owner: &mut self.regs,
                reg: RegisterX64::NOREG,
              };
              tmp.alloc(SizeX64::Qword);
              (*self.build).mov64(tmp.reg, value);
              (*self.build).mov(
                luau_reg_value_int_64(vm_reg_op(*get_op_mut(inst, 0))),
                OperandX64::reg(tmp.reg),
              );
            }
          } else if (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Inst {
            (*self.build).mov(
              luau_reg_value_int_64(vm_reg_op(*get_op_mut(inst, 0))),
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 1))),
            );
          } else {
            CODEGEN_ASSERT!(false, "Unsupported instruction form");
          }
        }
        IrCmd::StoreVector => {
          self.store_float(
            luau_reg_value_vector(vm_reg_op(*get_op_mut(inst, 0)), 0),
            *get_op_mut(inst, 1),
          );
          self.store_float(
            luau_reg_value_vector(vm_reg_op(*get_op_mut(inst, 0)), 1),
            *get_op_mut(inst, 2),
          );
          self.store_float(
            luau_reg_value_vector(vm_reg_op(*get_op_mut(inst, 0)), 2),
            *get_op_mut(inst, 3),
          );

          if HAS_OP_E!(inst) {
            (*self.build).mov(
              luau_reg_tag(vm_reg_op(*get_op_mut(inst, 0))),
              OperandX64::imm((self.tag_op(*get_op_mut(inst, 4))) as i32),
            );
          }
        }
        IrCmd::StoreTvalue => {
          let addr_offset = if HAS_OP_C!(inst) {
            self.int_op(*get_op_mut(inst, 2))
          } else {
            0
          };

          if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::VmReg {
            (*self.build).vmovups(
              luau_reg(vm_reg_op(*get_op_mut(inst, 0))),
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 1))),
            );
          } else if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Inst {
            (*self.build).vmovups(
              OperandX64::mem(
                SizeX64::Xmmword,
                RegisterX64::NOREG,
                1,
                self.reg_op(*get_op_mut(inst, 0)),
                addr_offset,
              ),
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 1))),
            );
          } else {
            CODEGEN_ASSERT!(false, "Unsupported instruction form");
          }
        }
        IrCmd::StoreSplitTvalue => {
          {
            let addr_offset = if HAS_OP_D!(inst) {
              self.int_op(*get_op_mut(inst, 3))
            } else {
              0
            };

            let tag_lhs = if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Inst {
              OperandX64::mem(
                SizeX64::Dword,
                RegisterX64::NOREG,
                1,
                self.reg_op(*get_op_mut(inst, 0)),
                (core::mem::offset_of!(TValue, tt) as i32) + addr_offset,
              )
            } else {
              luau_reg_tag(vm_reg_op(*get_op_mut(inst, 0)))
            };
            (*self.build).mov(
              tag_lhs,
              OperandX64::imm((self.tag_op(*get_op_mut(inst, 1))) as i32),
            );

            if self.tag_op(*get_op_mut(inst, 1)) == LuaType::Boolean as u8 {
              let value_lhs = if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Inst {
                OperandX64::mem(
                  SizeX64::Dword,
                  RegisterX64::NOREG,
                  1,
                  self.reg_op(*get_op_mut(inst, 0)),
                  (core::mem::offset_of!(TValue, value) as i32) + addr_offset,
                )
              } else {
                luau_reg_value_int(vm_reg_op(*get_op_mut(inst, 0)))
              };
              (*self.build).mov(
                value_lhs,
                if (*get_op_mut(inst, 2)).kind() == IrOpKind::Constant {
                  OperandX64::imm(self.int_op(*get_op_mut(inst, 2)))
                } else {
                  OperandX64::reg(self.reg_op(*get_op_mut(inst, 2)))
                },
              );
            } else if self.tag_op(*get_op_mut(inst, 1)) == LuaType::Number as u8 {
              let value_lhs = if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Inst {
                OperandX64::mem(
                  SizeX64::Qword,
                  RegisterX64::NOREG,
                  1,
                  self.reg_op(*get_op_mut(inst, 0)),
                  (core::mem::offset_of!(TValue, value) as i32) + addr_offset,
                )
              } else {
                luau_reg_value(vm_reg_op(*get_op_mut(inst, 0)))
              };

              if (*get_op_mut(inst, 2_u32)).kind() == IrOpKind::Constant {
                let mut tmp = ScopedRegX64 {
                  owner: &mut self.regs,
                  reg: RegisterX64::NOREG,
                };
                tmp.alloc(SizeX64::Xmmword);

                (*self.build).vmovsd_operand_x_64_operand_x_64(
                  OperandX64::reg(tmp.reg),
                  (*self.build).f64(self.double_op(*get_op_mut(inst, 2))),
                );
                (*self.build).vmovsd_operand_x_64_operand_x_64(value_lhs, OperandX64::reg(tmp.reg));
              } else {
                (*self.build).vmovsd_operand_x_64_operand_x_64(
                  value_lhs,
                  OperandX64::reg(self.reg_op(*get_op_mut(inst, 2))),
                );
              }
            } else if self.tag_op(*get_op_mut(inst, 1)) == LuaType::Integer as u8 {
              let value_lhs = if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Inst {
                OperandX64::mem(
                  SizeX64::Qword,
                  RegisterX64::NOREG,
                  1,
                  self.reg_op(*get_op_mut(inst, 0)),
                  (core::mem::offset_of!(TValue, value) as i32) + addr_offset,
                )
              } else {
                luau_reg_value_int_64(vm_reg_op(*get_op_mut(inst, 0)))
              };

              if (*get_op_mut(inst, 2_u32)).kind() == IrOpKind::Constant {
                let value = self.int64_op(*get_op_mut(inst, 2));

                // x64 mov r/m64, imm32 sign-extends
                if value >= i32::MIN as i64 && value <= i32::MAX as i64 {
                  (*self.build).mov(value_lhs, OperandX64::imm((value) as i32));
                } else {
                  let mut tmp = ScopedRegX64 {
                    owner: &mut self.regs,
                    reg: RegisterX64::NOREG,
                  };
                  tmp.alloc(SizeX64::Qword);
                  (*self.build).mov64(tmp.reg, value);
                  (*self.build).mov(value_lhs, OperandX64::reg(tmp.reg));
                }
              } else {
                (*self.build).mov(
                  value_lhs,
                  OperandX64::reg(self.reg_op(*get_op_mut(inst, 2))),
                );
              }
            } else if is_gco(self.tag_op(*get_op_mut(inst, 1))) {
              let value_lhs = if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Inst {
                OperandX64::mem(
                  SizeX64::Qword,
                  RegisterX64::NOREG,
                  1,
                  self.reg_op(*get_op_mut(inst, 0)),
                  (core::mem::offset_of!(TValue, value) as i32) + addr_offset,
                )
              } else {
                luau_reg_value(vm_reg_op(*get_op_mut(inst, 0)))
              };
              (*self.build).mov(
                value_lhs,
                OperandX64::reg(self.reg_op(*get_op_mut(inst, 2))),
              );
            } else {
              CODEGEN_ASSERT!(false, "Unsupported instruction form");
            }
          }
        }
        IrCmd::AddInt => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Dword, index, &[(*get_op_mut(inst, 0_u32))]);
          let op0 = *get_op_mut(inst, 0);
          let op1 = *get_op_mut(inst, 1);

          if op0.kind() == IrOpKind::Constant {
            (*self.build).lea_operand_x_64_operand_x_64(
              OperandX64::reg(inst.reg_x64),
              OperandX64::mem(
                SizeX64::None,
                RegisterX64::NOREG,
                1,
                self.reg_op(op1),
                self.int_op(op0),
              ),
            );
          } else if op0.kind() == IrOpKind::Inst {
            if inst.reg_x64 == self.reg_op(op0) {
              if op1.kind() == IrOpKind::Inst {
                (*self.build).add(
                  OperandX64::reg(inst.reg_x64),
                  OperandX64::reg(self.reg_op(op1)),
                );
              } else if self.int_op(op1) == 1 {
                (*self.build).inc(OperandX64::reg(inst.reg_x64));
              } else {
                (*self.build).add(
                  OperandX64::reg(inst.reg_x64),
                  OperandX64::imm(self.int_op(op1)),
                );
              }
            } else {
              if op1.kind() == IrOpKind::Inst {
                (*self.build).lea_operand_x_64_operand_x_64(
                  OperandX64::reg(inst.reg_x64),
                  OperandX64::mem(SizeX64::None, self.reg_op(op1), 1, self.reg_op(op0), 0),
                );
              } else {
                (*self.build).lea_operand_x_64_operand_x_64(
                  OperandX64::reg(inst.reg_x64),
                  OperandX64::mem(
                    SizeX64::None,
                    RegisterX64::NOREG,
                    1,
                    self.reg_op(op0),
                    self.int_op(op1),
                  ),
                );
              }
            }
          } else {
            CODEGEN_ASSERT!(false, "Unsupported instruction form");
          }
        }
        IrCmd::AddInt64 => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Qword, index, &[(*get_op_mut(inst, 0_u32))]);
          let op0 = *get_op_mut(inst, 0);
          let op1 = *get_op_mut(inst, 1);

          if op0.kind() == IrOpKind::Constant {
            let value = self.int64_op(op0);

            if value >= i32::MIN as i64 && value <= i32::MAX as i64 {
              (*self.build).lea_operand_x_64_operand_x_64(
                OperandX64::reg(inst.reg_x64),
                OperandX64::mem(
                  SizeX64::None,
                  RegisterX64::NOREG,
                  1,
                  self.reg_op(op1),
                  (value) as i32,
                ),
              );
            } else {
              (*self.build).mov64(inst.reg_x64, value);
              (*self.build).add(
                OperandX64::reg(inst.reg_x64),
                OperandX64::reg(self.reg_op(op1)),
              );
            }
          } else if op0.kind() == IrOpKind::Inst {
            if inst.reg_x64 == self.reg_op(op0) {
              if op1.kind() == IrOpKind::Inst {
                (*self.build).add(
                  OperandX64::reg(inst.reg_x64),
                  OperandX64::reg(self.reg_op(op1)),
                );
              } else if self.int64_op(op1) == 1 {
                (*self.build).inc(OperandX64::reg(inst.reg_x64));
              } else {
                let value = self.int64_op(op1);

                if value >= i32::MIN as i64 && value <= i32::MAX as i64 {
                  (*self.build).add(
                    OperandX64::reg(inst.reg_x64),
                    OperandX64::imm((value) as i32),
                  );
                } else {
                  let mut tmp = ScopedRegX64 {
                    owner: &mut self.regs,
                    reg: RegisterX64::NOREG,
                  };
                  tmp.alloc(SizeX64::Qword);
                  (*self.build).mov64(tmp.reg, value);
                  (*self.build).add(OperandX64::reg(inst.reg_x64), OperandX64::reg(tmp.reg));
                }
              }
            } else {
              if op1.kind() == IrOpKind::Inst {
                (*self.build).lea_operand_x_64_operand_x_64(
                  OperandX64::reg(inst.reg_x64),
                  OperandX64::mem(SizeX64::None, self.reg_op(op1), 1, self.reg_op(op0), 0),
                );
              } else {
                let value = self.int64_op(op1);

                if value >= i32::MIN as i64 && value <= i32::MAX as i64 {
                  (*self.build).lea_operand_x_64_operand_x_64(
                    OperandX64::reg(inst.reg_x64),
                    OperandX64::mem(
                      SizeX64::None,
                      RegisterX64::NOREG,
                      1,
                      self.reg_op(op0),
                      (value) as i32,
                    ),
                  );
                } else {
                  (*self.build).mov64(inst.reg_x64, value);
                  (*self.build).add(
                    OperandX64::reg(inst.reg_x64),
                    OperandX64::reg(self.reg_op(op0)),
                  );
                }
              }
            }
          } else {
            CODEGEN_ASSERT!(false, "Unsupported instruction form");
          }
        }
        IrCmd::SubInt => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Dword, index, &[(*get_op_mut(inst, 0_u32))]);
          let op0 = *get_op_mut(inst, 0);
          let op1 = *get_op_mut(inst, 1);

          if op0.kind() == IrOpKind::Inst {
            if op1.kind() == IrOpKind::Constant {
              if inst.reg_x64 != self.reg_op(op0) {
                (*self.build).lea_operand_x_64_operand_x_64(
                  OperandX64::reg(inst.reg_x64),
                  OperandX64::mem(
                    SizeX64::None,
                    RegisterX64::NOREG,
                    1,
                    self.reg_op(op0),
                    -self.int_op(op1),
                  ),
                );
              } else {
                (*self.build).sub(
                  OperandX64::reg(inst.reg_x64),
                  OperandX64::imm(self.int_op(op1)),
                );
              }
            } else {
              // If result reuses the source, we can subtract in place, otherwise we need to setup our initial value
              if inst.reg_x64 != self.reg_op(op0) {
                (*self.build).mov(
                  OperandX64::reg(inst.reg_x64),
                  OperandX64::reg(self.reg_op(op0)),
                );
              }

              (*self.build).sub(
                OperandX64::reg(inst.reg_x64),
                OperandX64::reg(self.reg_op(op1)),
              );
            }
          } else if op1.kind() == IrOpKind::Inst {
            (*self.build).mov(
              OperandX64::reg(inst.reg_x64),
              OperandX64::imm(self.int_op(op0)),
            );
            (*self.build).sub(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(self.reg_op(op1)),
            );
          } else {
            CODEGEN_ASSERT!(false, "Unsupported instruction form");
          }
        }
        IrCmd::SubInt64 => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Qword, index, &[(*get_op_mut(inst, 0_u32))]);
          let op0 = *get_op_mut(inst, 0);
          let op1 = *get_op_mut(inst, 1);

          if op0.kind() == IrOpKind::Inst {
            if op1.kind() == IrOpKind::Constant {
              let value = self.int64_op(op1);

              if value >= i32::MIN as i64 && value <= i32::MAX as i64 {
                if inst.reg_x64 != self.reg_op(op0) {
                  (*self.build).lea_operand_x_64_operand_x_64(
                    OperandX64::reg(inst.reg_x64),
                    OperandX64::mem(
                      SizeX64::None,
                      RegisterX64::NOREG,
                      1,
                      self.reg_op(op0),
                      -((value) as i32),
                    ),
                  );
                } else {
                  (*self.build).sub(
                    OperandX64::reg(inst.reg_x64),
                    OperandX64::imm((value) as i32),
                  );
                }
              } else {
                let mut tmp = ScopedRegX64 {
                  owner: &mut self.regs,
                  reg: RegisterX64::NOREG,
                };
                tmp.alloc(SizeX64::Qword);
                (*self.build).mov64(tmp.reg, value);

                if inst.reg_x64 != self.reg_op(op0) {
                  (*self.build).mov(
                    OperandX64::reg(inst.reg_x64),
                    OperandX64::reg(self.reg_op(op0)),
                  );
                }

                (*self.build).sub(OperandX64::reg(inst.reg_x64), OperandX64::reg(tmp.reg));
              }
            } else {
              // If result reuses the source, we can subtract in place, otherwise we need to setup our initial value
              if inst.reg_x64 != self.reg_op(op0) {
                (*self.build).mov(
                  OperandX64::reg(inst.reg_x64),
                  OperandX64::reg(self.reg_op(op0)),
                );
              }

              (*self.build).sub(
                OperandX64::reg(inst.reg_x64),
                OperandX64::reg(self.reg_op(op1)),
              );
            }
          } else if op1.kind() == IrOpKind::Inst {
            (*self.build).mov64(inst.reg_x64, self.int64_op(op0));
            (*self.build).sub(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(self.reg_op(op1)),
            );
          } else {
            CODEGEN_ASSERT!(false, "Unsupported instruction form");
          }
        }
        IrCmd::Sexti8Int => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Dword, index, &[(*get_op_mut(inst, 0_u32))]);

          (*self.build).movsx(
            inst.reg_x64,
            OperandX64::reg(byte_reg(self.reg_op(*get_op_mut(inst, 0)))),
          );
        }
        IrCmd::Sexti16Int => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Dword, index, &[(*get_op_mut(inst, 0_u32))]);

          (*self.build).movsx(
            inst.reg_x64,
            OperandX64::reg(word_reg(self.reg_op(*get_op_mut(inst, 0)))),
          );
        }
        IrCmd::AddNum => {
          inst.reg_x64 = self.regs.alloc_reg_or_reuse(
            SizeX64::Xmmword,
            index,
            &[(*get_op_mut(inst, 0_u32)), (*get_op_mut(inst, 1_u32))],
          );

          if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Constant {
            let mut tmp = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp.alloc(SizeX64::Xmmword);

            (*self.build).vmovsd_operand_x_64_operand_x_64(
              OperandX64::reg(tmp.reg),
              self.mem_reg_double_op(*get_op_mut(inst, 0)),
            );
            (*self.build).vaddsd(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(tmp.reg),
              self.mem_reg_double_op(*get_op_mut(inst, 1)),
            );
          } else {
            (*self.build).vaddsd(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
              self.mem_reg_double_op(*get_op_mut(inst, 1)),
            );
          }
        }
        IrCmd::SubNum => {
          inst.reg_x64 = self.regs.alloc_reg_or_reuse(
            SizeX64::Xmmword,
            index,
            &[(*get_op_mut(inst, 0_u32)), (*get_op_mut(inst, 1_u32))],
          );

          if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Constant {
            let mut tmp = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp.alloc(SizeX64::Xmmword);

            (*self.build).vmovsd_operand_x_64_operand_x_64(
              OperandX64::reg(tmp.reg),
              self.mem_reg_double_op(*get_op_mut(inst, 0)),
            );
            (*self.build).vsubsd(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(tmp.reg),
              self.mem_reg_double_op(*get_op_mut(inst, 1)),
            );
          } else {
            (*self.build).vsubsd(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
              self.mem_reg_double_op(*get_op_mut(inst, 1)),
            );
          }
        }
        IrCmd::MulNum => {
          inst.reg_x64 = self.regs.alloc_reg_or_reuse(
            SizeX64::Xmmword,
            index,
            &[(*get_op_mut(inst, 0_u32)), (*get_op_mut(inst, 1_u32))],
          );

          if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Constant {
            let mut tmp = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp.alloc(SizeX64::Xmmword);

            (*self.build).vmovsd_operand_x_64_operand_x_64(
              OperandX64::reg(tmp.reg),
              self.mem_reg_double_op(*get_op_mut(inst, 0)),
            );
            (*self.build).vmulsd(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(tmp.reg),
              self.mem_reg_double_op(*get_op_mut(inst, 1)),
            );
          } else {
            (*self.build).vmulsd(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
              self.mem_reg_double_op(*get_op_mut(inst, 1)),
            );
          }
        }
        IrCmd::MulInt64 => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Qword, index, &[(*get_op_mut(inst, 0_u32))]);

          (*self.build).mov(
            OperandX64::reg(inst.reg_x64),
            self.mem_reg_int_64_op(*get_op_mut(inst, 0)),
          );
          (*self.build).imul_operand_x_64_operand_x_64(
            OperandX64::reg(inst.reg_x64),
            self.mem_reg_int_64_op(*get_op_mut(inst, 1)),
          );
        }
        IrCmd::DivNum => {
          inst.reg_x64 = self.regs.alloc_reg_or_reuse(
            SizeX64::Xmmword,
            index,
            &[(*get_op_mut(inst, 0_u32)), (*get_op_mut(inst, 1_u32))],
          );

          if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Constant {
            let mut tmp = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp.alloc(SizeX64::Xmmword);

            (*self.build).vmovsd_operand_x_64_operand_x_64(
              OperandX64::reg(tmp.reg),
              self.mem_reg_double_op(*get_op_mut(inst, 0)),
            );
            (*self.build).vdivsd(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(tmp.reg),
              self.mem_reg_double_op(*get_op_mut(inst, 1)),
            );
          } else {
            (*self.build).vdivsd(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
              self.mem_reg_double_op(*get_op_mut(inst, 1)),
            );
          }
        }
        IrCmd::DivInt64 => {
          {
            // idiv clobbers RegisterX64::RAX (quotient) and RegisterX64::RDX (remainder)
            let mut div_rax = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            div_rax.alloc(SizeX64::Dword);
            let mut div_rdx = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            div_rdx.alloc(SizeX64::Dword);
            div_rax.take(RegisterX64::RAX);
            div_rdx.take(RegisterX64::RDX);

            (*self.build).mov(
              OperandX64::reg(RegisterX64::RAX),
              self.mem_reg_int_64_op(*get_op_mut(inst, 0)),
            );
            (*self.build).cqo(); // sign-extend RAX into RDX:RAX
            (*self.build).idiv(self.mem_reg_int_64_op(*get_op_mut(inst, 1)));

            inst.reg_x64 = self.regs.alloc_reg_or_reuse(
              SizeX64::Qword,
              index,
              &[(*get_op_mut(inst, 0_u32)), (*get_op_mut(inst, 1_u32))],
            );
            (*self.build).mov(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(RegisterX64::RAX),
            );
          }
        }
        IrCmd::IdivNum => {
          inst.reg_x64 = self.regs.alloc_reg_or_reuse(
            SizeX64::Xmmword,
            index,
            &[(*get_op_mut(inst, 0_u32)), (*get_op_mut(inst, 1_u32))],
          );

          if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Constant {
            let mut tmp = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp.alloc(SizeX64::Xmmword);

            (*self.build).vmovsd_operand_x_64_operand_x_64(
              OperandX64::reg(tmp.reg),
              self.mem_reg_double_op(*get_op_mut(inst, 0)),
            );
            (*self.build).vdivsd(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(tmp.reg),
              self.mem_reg_double_op(*get_op_mut(inst, 1)),
            );
          } else {
            (*self.build).vdivsd(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
              self.mem_reg_double_op(*get_op_mut(inst, 1)),
            );
          }
          (*self.build).vroundsd(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(inst.reg_x64),
            RoundingModeX64::RoundToNegativeInfinity,
          );
        }
        IrCmd::IdivInt64 => {
          {
            // idiv clobbers RegisterX64::RAX (quotient) and RegisterX64::RDX (remainder)
            let mut div_rax = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            div_rax.alloc(SizeX64::Dword);
            div_rax.take(RegisterX64::RAX);
            let mut div_rdx = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            div_rdx.alloc(SizeX64::Dword);
            div_rdx.take(RegisterX64::RDX);
            let mut temp_b = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            temp_b.alloc(SizeX64::Qword);

            (*self.build).mov(
              OperandX64::reg(temp_b.reg),
              self.mem_reg_int_64_op(*get_op_mut(inst, 1)),
            );

            // idiv divides RDX:RAX by operand; quotient in RAX, remainder in RDX
            (*self.build).mov(
              OperandX64::reg(RegisterX64::RAX),
              self.mem_reg_int_64_op(*get_op_mut(inst, 0)),
            );
            (*self.build).cqo(); // sign-extend RAX into RDX:RAX
            (*self.build).idiv(OperandX64::reg(temp_b.reg));

            inst.reg_x64 = self.regs.alloc_reg_or_reuse(
              SizeX64::Qword,
              index,
              &[(*get_op_mut(inst, 0_u32)), (*get_op_mut(inst, 1_u32))],
            );
            (*self.build).mov(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(RegisterX64::RAX),
            ); // start with truncated quotient

            let mut done = Label::default();
            (*self.build).test(
              OperandX64::reg(RegisterX64::RDX),
              OperandX64::reg(RegisterX64::RDX),
            );
            (*self.build).jcc(ConditionX64::Equal, &mut done); // remainder == 0, no adjustment needed

            (*self.build).xor_(
              OperandX64::reg(RegisterX64::RDX),
              OperandX64::reg(temp_b.reg),
            );
            (*self.build).jcc(ConditionX64::GreaterEqual, &mut done); // same sign, no adjustment

            (*self.build).sub(OperandX64::reg(inst.reg_x64), OperandX64::imm(1_i32)); // floor adjustment
            (*self.build).set_label(&mut done);
          }
        }
        IrCmd::UdivInt64 => {
          {
            // div clobbers RegisterX64::RAX (quotient) and RegisterX64::RDX (remainder)
            let mut div_rax = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            div_rax.alloc(SizeX64::Dword);
            let mut div_rdx = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            div_rdx.alloc(SizeX64::Dword);
            div_rax.take(RegisterX64::RAX);
            div_rdx.take(RegisterX64::RDX);

            (*self.build).mov(
              OperandX64::reg(RegisterX64::RAX),
              self.mem_reg_int_64_op(*get_op_mut(inst, 0)),
            );
            (*self.build).xor_(
              OperandX64::reg(RegisterX64::RDX),
              OperandX64::reg(RegisterX64::RDX),
            ); // zero-extend RAX into RDX:RAX
            (*self.build).div(self.mem_reg_int_64_op(*get_op_mut(inst, 1)));

            inst.reg_x64 = self.regs.alloc_reg_or_reuse(
              SizeX64::Qword,
              index,
              &[(*get_op_mut(inst, 0_u32)), (*get_op_mut(inst, 1_u32))],
            );
            (*self.build).mov(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(RegisterX64::RAX),
            );
          }
        }
        IrCmd::RemInt64 => {
          {
            // idiv clobbers RegisterX64::RAX (quotient) and RegisterX64::RDX (remainder)
            let mut div_rax = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            div_rax.alloc(SizeX64::Dword);
            let mut div_rdx = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            div_rdx.alloc(SizeX64::Dword);
            div_rax.take(RegisterX64::RAX);
            div_rdx.take(RegisterX64::RDX);
            let mut temp_b = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            temp_b.alloc(SizeX64::Qword);
            let mut temp_a = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            temp_a.alloc(SizeX64::Qword);
            (*self.build).mov(
              OperandX64::reg(temp_a.reg),
              self.mem_reg_int_64_op(*get_op_mut(inst, 0)),
            );
            (*self.build).mov(
              OperandX64::reg(temp_b.reg),
              self.mem_reg_int_64_op(*get_op_mut(inst, 1)),
            );

            // guard against dividend == i64::MIN && divisor == -1 (signed overflow)
            // if that occurs, we must return 0
            let mut skip = Label::default();
            let mut done = Label::default();

            (*self.build).cmp(OperandX64::reg(temp_b.reg), OperandX64::imm(-1_i32));
            (*self.build).jcc(ConditionX64::NotEqual, &mut skip);

            let mut tmp_min = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp_min.alloc(SizeX64::Qword);
            (*self.build).mov(OperandX64::reg(RegisterX64::RDX), OperandX64::imm(0_i32));
            (*self.build).mov64(tmp_min.reg, i64::MIN);
            (*self.build).cmp(OperandX64::reg(temp_a.reg), OperandX64::reg(tmp_min.reg));
            (*self.build).jcc(ConditionX64::Equal, &mut done);

            (*self.build).set_label(&mut skip);

            (*self.build).mov(
              OperandX64::reg(RegisterX64::RAX),
              OperandX64::reg(temp_a.reg),
            );
            (*self.build).cqo(); // sign-extend RAX into RDX:RAX
            (*self.build).idiv(OperandX64::reg(temp_b.reg));

            (*self.build).set_label(&mut done);
            inst.reg_x64 = self.regs.alloc_reg_or_reuse(
              SizeX64::Qword,
              index,
              &[(*get_op_mut(inst, 0_u32)), (*get_op_mut(inst, 1_u32))],
            );
            (*self.build).mov(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(RegisterX64::RDX),
            );
          }
        }
        IrCmd::UremInt64 => {
          {
            // div clobbers RegisterX64::RAX (quotient) and RegisterX64::RDX (remainder)
            let mut div_rax = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            div_rax.alloc(SizeX64::Dword);
            let mut div_rdx = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            div_rdx.alloc(SizeX64::Dword);
            div_rax.take(RegisterX64::RAX);
            div_rdx.take(RegisterX64::RDX);

            (*self.build).mov(
              OperandX64::reg(RegisterX64::RAX),
              self.mem_reg_int_64_op(*get_op_mut(inst, 0)),
            );
            (*self.build).xor_(
              OperandX64::reg(RegisterX64::RDX),
              OperandX64::reg(RegisterX64::RDX),
            ); // zero-extend RAX into RDX:RAX
            (*self.build).div(self.mem_reg_int_64_op(*get_op_mut(inst, 1)));

            inst.reg_x64 = self.regs.alloc_reg_or_reuse(
              SizeX64::Qword,
              index,
              &[(*get_op_mut(inst, 0_u32)), (*get_op_mut(inst, 1_u32))],
            );
            (*self.build).mov(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(RegisterX64::RDX),
            );
          }
        }
        IrCmd::MuladdNum => {
          if ((*self.build).features & FeaturesX64::FeatureFma3 as u32) != 0 {
            if (*get_op_mut(inst, 0_u32)).kind() != IrOpKind::Inst {
              inst.reg_x64 = self.regs.alloc_reg(SizeX64::Xmmword, index);
              (*self.build).vmovsd_operand_x_64_operand_x_64(
                OperandX64::reg(inst.reg_x64),
                self.mem_reg_double_op(*get_op_mut(inst, 0)),
              );
            } else {
              inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                SizeX64::Xmmword,
                index,
                &[(*get_op_mut(inst, 0_u32))],
              );
              let a_reg = self.reg_op(*get_op_mut(inst, 0));
              if inst.reg_x64 != a_reg {
                (*self.build).vmovupd(OperandX64::reg(inst.reg_x64), OperandX64::reg(a_reg));
              }
            }

            let mut opt_btmp = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            opt_btmp.alloc(SizeX64::Dword);

            let b_reg: RegisterX64 = if (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Constant {
              opt_btmp.alloc(SizeX64::Xmmword);

              (*self.build).vmovsd_operand_x_64_operand_x_64(
                OperandX64::reg(opt_btmp.reg),
                self.mem_reg_double_op(*get_op_mut(inst, 1)),
              );
              opt_btmp.reg
            } else {
              self.reg_op(*get_op_mut(inst, 1))
            };

            (*self.build).vfmadd213pd(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(b_reg),
              self.mem_reg_double_op(*get_op_mut(inst, 2)),
            );
          } else {
            inst.reg_x64 =
              self
                .regs
                .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[(*get_op_mut(inst, 0_u32))]);

            if (*get_op_mut(inst, 0_u32)).kind() != IrOpKind::Inst
              && (*get_op_mut(inst, 1_u32)).kind() != IrOpKind::Inst
            {
              (*self.build).vmovsd_operand_x_64_operand_x_64(
                OperandX64::reg(inst.reg_x64),
                self.mem_reg_double_op(*get_op_mut(inst, 0)),
              );
              (*self.build).vmulsd(
                OperandX64::reg(inst.reg_x64),
                OperandX64::reg(inst.reg_x64),
                self.mem_reg_double_op(*get_op_mut(inst, 1)),
              );
            } else if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Inst {
              (*self.build).vmulsd(
                OperandX64::reg(inst.reg_x64),
                OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
                self.mem_reg_double_op(*get_op_mut(inst, 1)),
              );
            } else {
              CODEGEN_ASSERT!((*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Inst);
              (*self.build).vmulsd(
                OperandX64::reg(inst.reg_x64),
                OperandX64::reg(self.reg_op(*get_op_mut(inst, 1))),
                self.mem_reg_double_op(*get_op_mut(inst, 0)),
              );
            }

            (*self.build).vaddsd(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(inst.reg_x64),
              self.mem_reg_double_op(*get_op_mut(inst, 2)),
            );
          }
        }
        IrCmd::ModNum => {
          inst.reg_x64 = self.regs.alloc_reg_or_reuse(
            SizeX64::Xmmword,
            index,
            &[(*get_op_mut(inst, 0_u32)), (*get_op_mut(inst, 1_u32))],
          );

          let mut opt_lhs_tmp = ScopedRegX64 {
            owner: &mut self.regs,
            reg: RegisterX64::NOREG,
          };
          opt_lhs_tmp.alloc(SizeX64::Dword);

          let lhs: RegisterX64 = if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Constant {
            opt_lhs_tmp.alloc(SizeX64::Xmmword);

            (*self.build).vmovsd_operand_x_64_operand_x_64(
              OperandX64::reg(opt_lhs_tmp.reg),
              self.mem_reg_double_op(*get_op_mut(inst, 0)),
            );
            opt_lhs_tmp.reg
          } else {
            self.reg_op(*get_op_mut(inst, 0))
          };

          if (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Inst {
            let mut tmp = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp.alloc(SizeX64::Xmmword);

            (*self.build).vdivsd(
              OperandX64::reg(tmp.reg),
              OperandX64::reg(lhs),
              self.mem_reg_double_op(*get_op_mut(inst, 1)),
            );
            (*self.build).vroundsd(
              OperandX64::reg(tmp.reg),
              OperandX64::reg(tmp.reg),
              OperandX64::reg(tmp.reg),
              RoundingModeX64::RoundToNegativeInfinity,
            );
            (*self.build).vmulsd(
              OperandX64::reg(tmp.reg),
              OperandX64::reg(tmp.reg),
              self.mem_reg_double_op(*get_op_mut(inst, 1)),
            );
            (*self.build).vsubsd(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(lhs),
              OperandX64::reg(tmp.reg),
            );
          } else {
            let mut tmp1 = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp1.alloc(SizeX64::Xmmword);
            let mut tmp2 = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp2.alloc(SizeX64::Xmmword);

            (*self.build).vmovsd_operand_x_64_operand_x_64(
              OperandX64::reg(tmp1.reg),
              self.mem_reg_double_op(*get_op_mut(inst, 1)),
            );
            (*self.build).vdivsd(
              OperandX64::reg(tmp2.reg),
              OperandX64::reg(lhs),
              OperandX64::reg(tmp1.reg),
            );
            (*self.build).vroundsd(
              OperandX64::reg(tmp2.reg),
              OperandX64::reg(tmp2.reg),
              OperandX64::reg(tmp2.reg),
              RoundingModeX64::RoundToNegativeInfinity,
            );
            (*self.build).vmulsd(
              OperandX64::reg(tmp1.reg),
              OperandX64::reg(tmp2.reg),
              OperandX64::reg(tmp1.reg),
            );
            (*self.build).vsubsd(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(lhs),
              OperandX64::reg(tmp1.reg),
            );
          }
        }
        IrCmd::ModInt64 => {
          {
            // idiv clobbers RegisterX64::RAX (quotient) and RegisterX64::RDX (remainder)
            let mut div_rax = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            div_rax.alloc(SizeX64::Dword);
            div_rax.take(RegisterX64::RAX);
            let mut div_rdx = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            div_rdx.alloc(SizeX64::Dword);
            div_rdx.take(RegisterX64::RDX);
            let mut temp_b = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            temp_b.alloc(SizeX64::Qword);
            let mut temp_a = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            temp_a.alloc(SizeX64::Qword);
            (*self.build).mov(
              OperandX64::reg(temp_a.reg),
              self.mem_reg_int_64_op(*get_op_mut(inst, 0)),
            );
            (*self.build).mov(
              OperandX64::reg(temp_b.reg),
              self.mem_reg_int_64_op(*get_op_mut(inst, 1)),
            );

            inst.reg_x64 = self.regs.alloc_reg_or_reuse(
              SizeX64::Qword,
              index,
              &[(*get_op_mut(inst, 0_u32)), (*get_op_mut(inst, 1_u32))],
            );

            // guard against dividend == i64::MIN && divisor == -1 (signed overflow)
            // if that occurs, we must return 0
            let mut skip = Label::default();
            let mut done = Label::default();

            (*self.build).cmp(OperandX64::reg(temp_b.reg), OperandX64::imm(-1_i32));
            (*self.build).jcc(ConditionX64::NotEqual, &mut skip);

            let mut tmp_min = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp_min.alloc(SizeX64::Qword);
            (*self.build).mov(OperandX64::reg(inst.reg_x64), OperandX64::imm(0_i32));
            (*self.build).mov64(tmp_min.reg, i64::MIN);
            (*self.build).cmp(OperandX64::reg(temp_a.reg), OperandX64::reg(tmp_min.reg));
            (*self.build).jcc(ConditionX64::Equal, &mut done);

            (*self.build).set_label(&mut skip);

            (*self.build).mov(
              OperandX64::reg(RegisterX64::RAX),
              OperandX64::reg(temp_a.reg),
            );
            (*self.build).cqo();
            (*self.build).idiv(OperandX64::reg(temp_b.reg));

            (*self.build).mov(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(RegisterX64::RDX),
            );

            (*self.build).test(
              OperandX64::reg(RegisterX64::RDX),
              OperandX64::reg(RegisterX64::RDX),
            );
            (*self.build).jcc(ConditionX64::Equal, &mut done);

            (*self.build).xor_(
              OperandX64::reg(RegisterX64::RDX),
              OperandX64::reg(temp_b.reg),
            );
            (*self.build).jcc(ConditionX64::GreaterEqual, &mut done);

            (*self.build).add(OperandX64::reg(inst.reg_x64), OperandX64::reg(temp_b.reg));
            (*self.build).set_label(&mut done);
          }
        }
        IrCmd::MinNum => {
          inst.reg_x64 = self.regs.alloc_reg_or_reuse(
            SizeX64::Xmmword,
            index,
            &[(*get_op_mut(inst, 0_u32)), (*get_op_mut(inst, 1_u32))],
          );

          if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Constant {
            let mut tmp = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp.alloc(SizeX64::Xmmword);

            (*self.build).vmovsd_operand_x_64_operand_x_64(
              OperandX64::reg(tmp.reg),
              self.mem_reg_double_op(*get_op_mut(inst, 0)),
            );
            (*self.build).vminsd(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(tmp.reg),
              self.mem_reg_double_op(*get_op_mut(inst, 1)),
            );
          } else {
            (*self.build).vminsd(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
              self.mem_reg_double_op(*get_op_mut(inst, 1)),
            );
          }
        }
        IrCmd::MaxNum => {
          inst.reg_x64 = self.regs.alloc_reg_or_reuse(
            SizeX64::Xmmword,
            index,
            &[(*get_op_mut(inst, 0_u32)), (*get_op_mut(inst, 1_u32))],
          );

          if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Constant {
            let mut tmp = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp.alloc(SizeX64::Xmmword);

            (*self.build).vmovsd_operand_x_64_operand_x_64(
              OperandX64::reg(tmp.reg),
              self.mem_reg_double_op(*get_op_mut(inst, 0)),
            );
            (*self.build).vmaxsd(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(tmp.reg),
              self.mem_reg_double_op(*get_op_mut(inst, 1)),
            );
          } else {
            (*self.build).vmaxsd(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
              self.mem_reg_double_op(*get_op_mut(inst, 1)),
            );
          }
        }
        IrCmd::UnmNum => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[(*get_op_mut(inst, 0_u32))]);

          (*self.build).vxorpd(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
            (*self.build).f64(-0.0),
          );
        }
        IrCmd::FloorNum => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[(*get_op_mut(inst, 0_u32))]);

          (*self.build).vroundsd(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(inst.reg_x64),
            self.mem_reg_double_op(*get_op_mut(inst, 0)),
            RoundingModeX64::RoundToNegativeInfinity,
          );
        }
        IrCmd::CeilNum => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[(*get_op_mut(inst, 0_u32))]);

          (*self.build).vroundsd(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(inst.reg_x64),
            self.mem_reg_double_op(*get_op_mut(inst, 0)),
            RoundingModeX64::RoundToPositiveInfinity,
          );
        }
        IrCmd::RoundNum => {
          {
            inst.reg_x64 =
              self
                .regs
                .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[(*get_op_mut(inst, 0_u32))]);

            let mut tmp1 = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp1.alloc(SizeX64::Xmmword);
            let mut tmp2 = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp2.alloc(SizeX64::Xmmword);

            if (*get_op_mut(inst, 0_u32)).kind() != IrOpKind::Inst {
              (*self.build).vmovsd_operand_x_64_operand_x_64(
                OperandX64::reg(inst.reg_x64),
                self.mem_reg_double_op(*get_op_mut(inst, 0)),
              );
            } else if self.reg_op(*get_op_mut(inst, 0)) != inst.reg_x64 {
              (*self.build).vmovsd_operand_x_64_operand_x_64_operand_x_64(
                OperandX64::reg(inst.reg_x64),
                OperandX64::reg(inst.reg_x64),
                OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
              );
            }

            (*self.build).vandpd(
              OperandX64::reg(tmp1.reg),
              OperandX64::reg(inst.reg_x64),
              (*self.build).f64x2(-0.0, -0.0),
            );
            (*self.build).vmovsd_operand_x_64_operand_x_64(
              OperandX64::reg(tmp2.reg),
              (*self.build).i64(0x3fdfffffffffffff),
            ); // 0.49999999999999994
            (*self.build).vorpd(
              OperandX64::reg(tmp1.reg),
              OperandX64::reg(tmp1.reg),
              OperandX64::reg(tmp2.reg),
            );
            (*self.build).vaddsd(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(tmp1.reg),
            );
            (*self.build).vroundsd(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(inst.reg_x64),
              RoundingModeX64::RoundToZero,
            );
          }
        }
        IrCmd::SqrtNum => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[(*get_op_mut(inst, 0_u32))]);

          (*self.build).vsqrtsd(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(inst.reg_x64),
            self.mem_reg_double_op(*get_op_mut(inst, 0)),
          );
        }
        IrCmd::AbsNum => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[(*get_op_mut(inst, 0_u32))]);

          if (*get_op_mut(inst, 0_u32)).kind() != IrOpKind::Inst {
            (*self.build).vmovsd_operand_x_64_operand_x_64(
              OperandX64::reg(inst.reg_x64),
              self.mem_reg_double_op(*get_op_mut(inst, 0)),
            );
          } else if self.reg_op(*get_op_mut(inst, 0)) != inst.reg_x64 {
            (*self.build).vmovsd_operand_x_64_operand_x_64_operand_x_64(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
            );
          }

          (*self.build).vandpd(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(inst.reg_x64),
            (*self.build).i64(!(1 << 63)),
          );
        }
        IrCmd::SignNum => {
          {
            inst.reg_x64 =
              self
                .regs
                .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[(*get_op_mut(inst, 0_u32))]);

            let mut tmp0 = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp0.alloc(SizeX64::Xmmword);
            let mut tmp1 = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp1.alloc(SizeX64::Xmmword);
            let mut tmp2 = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp2.alloc(SizeX64::Xmmword);

            (*self.build).vxorpd(
              OperandX64::reg(tmp0.reg),
              OperandX64::reg(tmp0.reg),
              OperandX64::reg(tmp0.reg),
            );

            // Set tmp1 to -1 if arg < 0, else 0
            (*self.build).vcmpltsd(
              OperandX64::reg(tmp1.reg),
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
              OperandX64::reg(tmp0.reg),
            );
            (*self.build)
              .vmovsd_operand_x_64_operand_x_64(OperandX64::reg(tmp2.reg), (*self.build).f64(-1.0));
            (*self.build).vandpd(
              OperandX64::reg(tmp1.reg),
              OperandX64::reg(tmp1.reg),
              OperandX64::reg(tmp2.reg),
            );

            // Set mask bit to 1 if 0 < arg, else 0
            (*self.build).vcmpltsd(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(tmp0.reg),
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
            );

            // Result = if (mask-bit == 1) { 1.0 } else { tmp1
            // If arg < 0 then tmp1 is -1 and mask-bit is 0 }, result is -1
            // If arg == 0 then tmp1 is 0 and mask-bit is 0, result is 0
            // If arg > 0 then tmp1 is 0 and mask-bit is 1, result is 1
            (*self.build).vblendvpd(
              inst.reg_x64,
              tmp1.reg,
              (*self.build).f64x2(1.0, 1.0),
              inst.reg_x64,
            );
          }
        }
        IrCmd::AddFloat => {
          inst.reg_x64 = self.regs.alloc_reg_or_reuse(
            SizeX64::Xmmword,
            index,
            &[(*get_op_mut(inst, 0_u32)), (*get_op_mut(inst, 1_u32))],
          );

          if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Constant {
            let mut tmp = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp.alloc(SizeX64::Xmmword);

            (*self.build).vmovss_operand_x_64_operand_x_64(
              OperandX64::reg(tmp.reg),
              self.mem_reg_float_op(*get_op_mut(inst, 0)),
            );
            (*self.build).vaddss(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(tmp.reg),
              self.mem_reg_float_op(*get_op_mut(inst, 1)),
            );
          } else {
            (*self.build).vaddss(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
              self.mem_reg_float_op(*get_op_mut(inst, 1)),
            );
          }
        }
        IrCmd::SubFloat => {
          inst.reg_x64 = self.regs.alloc_reg_or_reuse(
            SizeX64::Xmmword,
            index,
            &[(*get_op_mut(inst, 0_u32)), (*get_op_mut(inst, 1_u32))],
          );

          if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Constant {
            let mut tmp = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp.alloc(SizeX64::Xmmword);

            (*self.build).vmovss_operand_x_64_operand_x_64(
              OperandX64::reg(tmp.reg),
              self.mem_reg_float_op(*get_op_mut(inst, 0)),
            );
            (*self.build).vsubss(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(tmp.reg),
              self.mem_reg_float_op(*get_op_mut(inst, 1)),
            );
          } else {
            (*self.build).vsubss(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
              self.mem_reg_float_op(*get_op_mut(inst, 1)),
            );
          }
        }
        IrCmd::MulFloat => {
          inst.reg_x64 = self.regs.alloc_reg_or_reuse(
            SizeX64::Xmmword,
            index,
            &[(*get_op_mut(inst, 0_u32)), (*get_op_mut(inst, 1_u32))],
          );

          if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Constant {
            let mut tmp = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp.alloc(SizeX64::Xmmword);

            (*self.build).vmovss_operand_x_64_operand_x_64(
              OperandX64::reg(tmp.reg),
              self.mem_reg_float_op(*get_op_mut(inst, 0)),
            );
            (*self.build).vmulss(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(tmp.reg),
              self.mem_reg_float_op(*get_op_mut(inst, 1)),
            );
          } else {
            (*self.build).vmulss(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
              self.mem_reg_float_op(*get_op_mut(inst, 1)),
            );
          }
        }
        IrCmd::DivFloat => {
          inst.reg_x64 = self.regs.alloc_reg_or_reuse(
            SizeX64::Xmmword,
            index,
            &[(*get_op_mut(inst, 0_u32)), (*get_op_mut(inst, 1_u32))],
          );

          if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Constant {
            let mut tmp = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp.alloc(SizeX64::Xmmword);

            (*self.build).vmovss_operand_x_64_operand_x_64(
              OperandX64::reg(tmp.reg),
              self.mem_reg_float_op(*get_op_mut(inst, 0)),
            );
            (*self.build).vdivss(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(tmp.reg),
              self.mem_reg_float_op(*get_op_mut(inst, 1)),
            );
          } else {
            (*self.build).vdivss(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
              self.mem_reg_float_op(*get_op_mut(inst, 1)),
            );
          }
        }
        IrCmd::MinFloat => {
          inst.reg_x64 = self.regs.alloc_reg_or_reuse(
            SizeX64::Xmmword,
            index,
            &[(*get_op_mut(inst, 0_u32)), (*get_op_mut(inst, 1_u32))],
          );

          if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Constant {
            let mut tmp = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp.alloc(SizeX64::Xmmword);

            (*self.build).vmovss_operand_x_64_operand_x_64(
              OperandX64::reg(tmp.reg),
              self.mem_reg_float_op(*get_op_mut(inst, 0)),
            );
            (*self.build).vminss(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(tmp.reg),
              self.mem_reg_float_op(*get_op_mut(inst, 1)),
            );
          } else {
            (*self.build).vminss(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
              self.mem_reg_float_op(*get_op_mut(inst, 1)),
            );
          }
        }
        IrCmd::MaxFloat => {
          inst.reg_x64 = self.regs.alloc_reg_or_reuse(
            SizeX64::Xmmword,
            index,
            &[(*get_op_mut(inst, 0_u32)), (*get_op_mut(inst, 1_u32))],
          );

          if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Constant {
            let mut tmp = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp.alloc(SizeX64::Xmmword);

            (*self.build).vmovss_operand_x_64_operand_x_64(
              OperandX64::reg(tmp.reg),
              self.mem_reg_float_op(*get_op_mut(inst, 0)),
            );
            (*self.build).vmaxss(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(tmp.reg),
              self.mem_reg_float_op(*get_op_mut(inst, 1)),
            );
          } else {
            (*self.build).vmaxss(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
              self.mem_reg_float_op(*get_op_mut(inst, 1)),
            );
          }
        }
        IrCmd::UnmFloat => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[(*get_op_mut(inst, 0_u32))]);

          (*self.build).vxorps(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
            (*self.build).f32(-0.0),
          );
        }
        IrCmd::FloorFloat => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[(*get_op_mut(inst, 0_u32))]);

          (*self.build).vroundss(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(inst.reg_x64),
            self.mem_reg_float_op(*get_op_mut(inst, 0)),
            RoundingModeX64::RoundToNegativeInfinity,
          );
        }
        IrCmd::CeilFloat => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[(*get_op_mut(inst, 0_u32))]);

          (*self.build).vroundss(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(inst.reg_x64),
            self.mem_reg_float_op(*get_op_mut(inst, 0)),
            RoundingModeX64::RoundToPositiveInfinity,
          );
        }
        IrCmd::SqrtFloat => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[(*get_op_mut(inst, 0_u32))]);

          (*self.build).vsqrtss(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(inst.reg_x64),
            self.mem_reg_float_op(*get_op_mut(inst, 0)),
          );
        }
        IrCmd::AbsFloat => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[(*get_op_mut(inst, 0_u32))]);

          if (*get_op_mut(inst, 0_u32)).kind() != IrOpKind::Inst {
            (*self.build).vmovss_operand_x_64_operand_x_64(
              OperandX64::reg(inst.reg_x64),
              self.mem_reg_float_op(*get_op_mut(inst, 0)),
            );
          } else if self.reg_op(*get_op_mut(inst, 0)) != inst.reg_x64 {
            (*self.build).vmovss_operand_x_64_operand_x_64_operand_x_64(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
            );
          }

          (*self.build).vandps(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(inst.reg_x64),
            (*self.build).i32(0x7fffffff),
          );
        }
        IrCmd::SignFloat => {
          {
            inst.reg_x64 =
              self
                .regs
                .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[(*get_op_mut(inst, 0_u32))]);

            let mut tmp0 = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp0.alloc(SizeX64::Xmmword);
            let mut tmp1 = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp1.alloc(SizeX64::Xmmword);
            let mut tmp2 = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp2.alloc(SizeX64::Xmmword);

            (*self.build).vxorps(
              OperandX64::reg(tmp0.reg),
              OperandX64::reg(tmp0.reg),
              OperandX64::reg(tmp0.reg),
            );

            // Set tmp1 to -1 if arg < 0, else 0
            (*self.build).vcmpltss(
              OperandX64::reg(tmp1.reg),
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
              OperandX64::reg(tmp0.reg),
            );
            (*self.build)
              .vmovss_operand_x_64_operand_x_64(OperandX64::reg(tmp2.reg), (*self.build).f32(-1.0));
            (*self.build).vandps(
              OperandX64::reg(tmp1.reg),
              OperandX64::reg(tmp1.reg),
              OperandX64::reg(tmp2.reg),
            );

            // Set mask bit to 1 if 0 < arg, else 0
            (*self.build).vcmpltss(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(tmp0.reg),
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
            );

            // Result = if (mask-bit == 1) { 1.0 } else { tmp1
            // If arg < 0 then tmp1 is -1 and mask-bit is 0 }, result is -1
            // If arg == 0 then tmp1 is 0 and mask-bit is 0, result is 0
            // If arg > 0 then tmp1 is 0 and mask-bit is 1, result is 1
            (*self.build).vblendvps(
              inst.reg_x64,
              tmp1.reg,
              (*self.build).f32x4(1.0, 1.0, 1.0, 1.0),
              inst.reg_x64,
            );
          }
        }
        IrCmd::SelectNum => {
          {
            inst.reg_x64 = self.regs.alloc_reg_or_reuse(
              SizeX64::Xmmword,
              index,
              &[
                (*get_op_mut(inst, 0_u32)),
                (*get_op_mut(inst, 2_u32)),
                (*get_op_mut(inst, 3_u32)),
              ],
            ); // can't reuse b if a is a memory operand

            let mut tmp = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp.alloc(SizeX64::Xmmword);

            if (*get_op_mut(inst, 2_u32)).kind() == IrOpKind::Inst {
              (*self.build).vcmpeqsd(
                OperandX64::reg(tmp.reg),
                OperandX64::reg(self.reg_op(*get_op_mut(inst, 2))),
                self.mem_reg_double_op(*get_op_mut(inst, 3)),
              );
            } else {
              (*self.build).vmovsd_operand_x_64_operand_x_64(
                OperandX64::reg(tmp.reg),
                self.mem_reg_double_op(*get_op_mut(inst, 2)),
              );
              (*self.build).vcmpeqsd(
                OperandX64::reg(tmp.reg),
                OperandX64::reg(tmp.reg),
                self.mem_reg_double_op(*get_op_mut(inst, 3)),
              );
            }

            if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Inst {
              (*self.build).vblendvpd(
                inst.reg_x64,
                self.reg_op(*get_op_mut(inst, 0)),
                self.mem_reg_double_op(*get_op_mut(inst, 1)),
                tmp.reg,
              );
            } else {
              (*self.build).vmovsd_operand_x_64_operand_x_64(
                OperandX64::reg(inst.reg_x64),
                self.mem_reg_double_op(*get_op_mut(inst, 0)),
              );
              (*self.build).vblendvpd(
                inst.reg_x64,
                inst.reg_x64,
                self.mem_reg_double_op(*get_op_mut(inst, 1)),
                tmp.reg,
              );
            }
          }
        }
        IrCmd::SelectInt64 => {
          {
            // Select B if C cond D, otherwise select A
            // A, B: int64 (endpoints), C, D: int64 (condition arguments), E: condition
            inst.reg_x64 =
              self
                .regs
                .alloc_reg_or_reuse(SizeX64::Qword, index, &[(*get_op_mut(inst, 0_u32))]);

            let cond = condition_op(*get_op_mut(inst, 4));

            // Start with falseVal (A), conditionally replace with trueVal (B)
            (*self.build).mov(
              OperandX64::reg(inst.reg_x64),
              self.mem_reg_int_64_op(*get_op_mut(inst, 0)),
            );

            let mut tmp = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp.alloc(SizeX64::Qword);
            // Compare C vs D
            if (*get_op_mut(inst, 2_u32)).kind() == IrOpKind::Inst {
              (*self.build).cmp(
                OperandX64::reg(self.reg_op(*get_op_mut(inst, 2))),
                self.mem_reg_int_64_op(*get_op_mut(inst, 3)),
              );
            } else {
              (*self.build).mov(
                OperandX64::reg(tmp.reg),
                self.mem_reg_int_64_op(*get_op_mut(inst, 2)),
              );
              (*self.build).cmp(
                OperandX64::reg(tmp.reg),
                self.mem_reg_int_64_op(*get_op_mut(inst, 3)),
              );
            }

            // If condition is true, select B instead
            (*self.build).cmov(
              get_condition_int(cond),
              inst.reg_x64,
              self.mem_reg_int_64_op(*get_op_mut(inst, 1)),
            );
          }
        }
        IrCmd::SelectVec => {
          inst.reg_x64 = self.regs.alloc_reg_or_reuse(
            SizeX64::Xmmword,
            index,
            &[(*get_op_mut(inst, 2_u32)), (*get_op_mut(inst, 3_u32))],
          );

          let mut tmp1 = ScopedRegX64 {
            owner: &mut self.regs,
            reg: RegisterX64::NOREG,
          };
          let mut tmp2 = ScopedRegX64 {
            owner: &mut self.regs,
            reg: RegisterX64::NOREG,
          };
          let tmpc = self.vec_op(*get_op_mut(inst, 2), &mut tmp1);
          let tmpd = self.vec_op(*get_op_mut(inst, 3), &mut tmp2);

          (*self.build).vcmpeqps(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(tmpc),
            OperandX64::reg(tmpd),
          );
          (*self.build).vblendvps(
            inst.reg_x64,
            self.vec_op(*get_op_mut(inst, 0), &mut tmp1),
            OperandX64::reg(self.vec_op(*get_op_mut(inst, 1), &mut tmp2)),
            inst.reg_x64,
          );
        }
        IrCmd::SelectIfTruthy => {
          {
            inst.reg_x64 = self.regs.alloc_reg(SizeX64::Xmmword, index); // No reuse since multiple inputs can be shared

            // Place lhs as the result, we will overwrite it with rhs if 'A' is falsy later
            (*self.build).vmovaps(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 1))),
            );

            // Get rhs register early, so a potential restore happens on both sides of a conditional control flow
            let c = self.reg_op(*get_op_mut(inst, 2));

            let mut tmp = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp.alloc(SizeX64::Dword);
            let mut save_rhs = Label::default();
            let mut exit = Label::default();

            // Check tag first
            (*self.build).vpextrd(tmp.reg, self.reg_op(*get_op_mut(inst, 0)), 3_u8);
            (*self.build).cmp(
              OperandX64::reg(tmp.reg),
              OperandX64::imm((LuaType::Boolean as u8) as i32),
            );

            (*self.build).jcc(ConditionX64::Below, &mut save_rhs); // rhs if 'A' is nil
            (*self.build).jcc(ConditionX64::Above, &mut exit); // Keep lhs if 'A' is not a boolean

            // Check the boolean value
            (*self.build).vpextrd(tmp.reg, self.reg_op(*get_op_mut(inst, 0)), 0_u8);
            (*self.build).test(OperandX64::reg(tmp.reg), OperandX64::reg(tmp.reg));
            (*self.build).jcc(ConditionX64::NotZero, &mut exit); // Keep lhs if 'A' is true

            (*self.build).set_label(&mut save_rhs);
            (*self.build).vmovaps(OperandX64::reg(inst.reg_x64), OperandX64::reg(c));

            (*self.build).set_label(&mut exit);
          }
        }
        IrCmd::AddVec => {
          inst.reg_x64 = self.regs.alloc_reg_or_reuse(
            SizeX64::Xmmword,
            index,
            &[(*get_op_mut(inst, 0_u32)), (*get_op_mut(inst, 1_u32))],
          );

          let mut tmp1 = ScopedRegX64 {
            owner: &mut self.regs,
            reg: RegisterX64::NOREG,
          };
          let mut tmp2 = ScopedRegX64 {
            owner: &mut self.regs,
            reg: RegisterX64::NOREG,
          };

          let op0 = *get_op_mut(inst, 0);
          let op1 = *get_op_mut(inst, 1);
          let tmpa = self.vec_op(op0, &mut tmp1);
          let tmpb = if op0 == op1 {
            tmpa
          } else {
            self.vec_op(op1, &mut tmp2)
          };

          (*self.build).vaddps(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(tmpa),
            OperandX64::reg(tmpb),
          );
        }
        IrCmd::SubVec => {
          inst.reg_x64 = self.regs.alloc_reg_or_reuse(
            SizeX64::Xmmword,
            index,
            &[(*get_op_mut(inst, 0_u32)), (*get_op_mut(inst, 1_u32))],
          );

          let mut tmp1 = ScopedRegX64 {
            owner: &mut self.regs,
            reg: RegisterX64::NOREG,
          };
          let mut tmp2 = ScopedRegX64 {
            owner: &mut self.regs,
            reg: RegisterX64::NOREG,
          };

          let op0 = *get_op_mut(inst, 0);
          let op1 = *get_op_mut(inst, 1);
          let tmpa = self.vec_op(op0, &mut tmp1);
          let tmpb = if op0 == op1 {
            tmpa
          } else {
            self.vec_op(op1, &mut tmp2)
          };

          (*self.build).vsubps(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(tmpa),
            OperandX64::reg(tmpb),
          );
        }
        IrCmd::MulVec => {
          inst.reg_x64 = self.regs.alloc_reg_or_reuse(
            SizeX64::Xmmword,
            index,
            &[(*get_op_mut(inst, 0_u32)), (*get_op_mut(inst, 1_u32))],
          );

          let mut tmp1 = ScopedRegX64 {
            owner: &mut self.regs,
            reg: RegisterX64::NOREG,
          };
          let mut tmp2 = ScopedRegX64 {
            owner: &mut self.regs,
            reg: RegisterX64::NOREG,
          };

          let op0 = *get_op_mut(inst, 0);
          let op1 = *get_op_mut(inst, 1);
          let tmpa = self.vec_op(op0, &mut tmp1);
          let tmpb = if op0 == op1 {
            tmpa
          } else {
            self.vec_op(op1, &mut tmp2)
          };

          (*self.build).vmulps(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(tmpa),
            OperandX64::reg(tmpb),
          );
        }
        IrCmd::DivVec => {
          inst.reg_x64 = self.regs.alloc_reg_or_reuse(
            SizeX64::Xmmword,
            index,
            &[(*get_op_mut(inst, 0_u32)), (*get_op_mut(inst, 1_u32))],
          );

          let mut tmp1 = ScopedRegX64 {
            owner: &mut self.regs,
            reg: RegisterX64::NOREG,
          };
          let mut tmp2 = ScopedRegX64 {
            owner: &mut self.regs,
            reg: RegisterX64::NOREG,
          };

          let op0 = *get_op_mut(inst, 0);
          let op1 = *get_op_mut(inst, 1);
          let tmpa = self.vec_op(op0, &mut tmp1);
          let tmpb = if op0 == op1 {
            tmpa
          } else {
            self.vec_op(op1, &mut tmp2)
          };

          (*self.build).vdivps(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(tmpa),
            OperandX64::reg(tmpb),
          );
        }
        IrCmd::IdivVec => {
          inst.reg_x64 = self.regs.alloc_reg_or_reuse(
            SizeX64::Xmmword,
            index,
            &[(*get_op_mut(inst, 0_u32)), (*get_op_mut(inst, 1_u32))],
          );

          let mut tmp1 = ScopedRegX64 {
            owner: &mut self.regs,
            reg: RegisterX64::NOREG,
          };
          let mut tmp2 = ScopedRegX64 {
            owner: &mut self.regs,
            reg: RegisterX64::NOREG,
          };

          let op0 = *get_op_mut(inst, 0);
          let op1 = *get_op_mut(inst, 1);
          let tmpa = self.vec_op(op0, &mut tmp1);
          let tmpb = if op0 == op1 {
            tmpa
          } else {
            self.vec_op(op1, &mut tmp2)
          };

          (*self.build).vdivps(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(tmpa),
            OperandX64::reg(tmpb),
          );
          (*self.build).vroundps(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(inst.reg_x64),
            RoundingModeX64::RoundToNegativeInfinity,
          );
        }
        IrCmd::MuladdVec => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[(*get_op_mut(inst, 0_u32))]);
          let mut tmp1 = ScopedRegX64 {
            owner: &mut self.regs,
            reg: RegisterX64::NOREG,
          };
          let mut tmp2 = ScopedRegX64 {
            owner: &mut self.regs,
            reg: RegisterX64::NOREG,
          };
          let mut tmp3 = ScopedRegX64 {
            owner: &mut self.regs,
            reg: RegisterX64::NOREG,
          };

          let tmpa = self.vec_op(*get_op_mut(inst, 0), &mut tmp1);
          let tmpb = self.vec_op(*get_op_mut(inst, 1), &mut tmp2);
          let tmpc = self.vec_op(*get_op_mut(inst, 2), &mut tmp3);

          if ((*self.build).features & FeaturesX64::FeatureFma3 as u32) != 0 {
            if inst.reg_x64 != tmpa {
              (*self.build).vmovups(OperandX64::reg(inst.reg_x64), OperandX64::reg(tmpa));
            }

            (*self.build).vfmadd213ps(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(tmpb),
              OperandX64::reg(tmpc),
            );
          } else {
            (*self.build).vmulps(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(tmpa),
              OperandX64::reg(tmpb),
            );
            (*self.build).vaddps(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(tmpc),
            );
          }
        }
        IrCmd::UnmVec => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[(*get_op_mut(inst, 0_u32))]);

          (*self.build).vxorpd(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
            (*self.build).f32x4(-0.0, -0.0, -0.0, -0.0),
          );
        }
        IrCmd::MinVec => {
          inst.reg_x64 = self.regs.alloc_reg_or_reuse(
            SizeX64::Xmmword,
            index,
            &[(*get_op_mut(inst, 0_u32)), (*get_op_mut(inst, 1_u32))],
          );

          let mut tmp1 = ScopedRegX64 {
            owner: &mut self.regs,
            reg: RegisterX64::NOREG,
          };
          let mut tmp2 = ScopedRegX64 {
            owner: &mut self.regs,
            reg: RegisterX64::NOREG,
          };

          let op0 = *get_op_mut(inst, 0);
          let op1 = *get_op_mut(inst, 1);
          let tmpa = self.vec_op(op0, &mut tmp1);
          let tmpb = if op0 == op1 {
            tmpa
          } else {
            self.vec_op(op1, &mut tmp2)
          };

          (*self.build).vminps(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(tmpa),
            OperandX64::reg(tmpb),
          );
        }
        IrCmd::MaxVec => {
          inst.reg_x64 = self.regs.alloc_reg_or_reuse(
            SizeX64::Xmmword,
            index,
            &[(*get_op_mut(inst, 0_u32)), (*get_op_mut(inst, 1_u32))],
          );

          let mut tmp1 = ScopedRegX64 {
            owner: &mut self.regs,
            reg: RegisterX64::NOREG,
          };
          let mut tmp2 = ScopedRegX64 {
            owner: &mut self.regs,
            reg: RegisterX64::NOREG,
          };

          let op0 = *get_op_mut(inst, 0);
          let op1 = *get_op_mut(inst, 1);
          let tmpa = self.vec_op(op0, &mut tmp1);
          let tmpb = if op0 == op1 {
            tmpa
          } else {
            self.vec_op(op1, &mut tmp2)
          };

          (*self.build).vmaxps(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(tmpa),
            OperandX64::reg(tmpb),
          );
        }
        IrCmd::FloorVec => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[(*get_op_mut(inst, 0_u32))]);

          let mut tmp1 = ScopedRegX64 {
            owner: &mut self.regs,
            reg: RegisterX64::NOREG,
          };
          let tmpa = self.vec_op(*get_op_mut(inst, 0), &mut tmp1);

          (*self.build).vroundps(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(tmpa),
            RoundingModeX64::RoundToNegativeInfinity,
          );
        }
        IrCmd::CeilVec => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[(*get_op_mut(inst, 0_u32))]);

          let mut tmp1 = ScopedRegX64 {
            owner: &mut self.regs,
            reg: RegisterX64::NOREG,
          };
          let tmpa = self.vec_op(*get_op_mut(inst, 0), &mut tmp1);

          (*self.build).vroundps(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(tmpa),
            RoundingModeX64::RoundToPositiveInfinity,
          );
        }
        IrCmd::AbsVec => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[(*get_op_mut(inst, 0_u32))]);

          let mut tmp1 = ScopedRegX64 {
            owner: &mut self.regs,
            reg: RegisterX64::NOREG,
          };
          let tmpa = self.vec_op(*get_op_mut(inst, 0), &mut tmp1);

          (*self.build).vandps(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(tmpa),
            (*self.build).u32x4(0x7fffffff, 0x7fffffff, 0x7fffffff, 0x7fffffff),
          );
        }
        IrCmd::DotVec => {
          {
            inst.reg_x64 = self.regs.alloc_reg_or_reuse(
              SizeX64::Xmmword,
              index,
              &[(*get_op_mut(inst, 0_u32)), (*get_op_mut(inst, 1_u32))],
            );

            let mut tmp1 = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            let mut tmp2 = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };

            let op0 = *get_op_mut(inst, 0);
            let op1 = *get_op_mut(inst, 1);
            let tmpa = self.vec_op(op0, &mut tmp1);
            let tmpb = if op0 == op1 {
              tmpa
            } else {
              self.vec_op(op1, &mut tmp2)
            };

            (*self.build).vdpps(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(tmpa),
              OperandX64::reg(tmpb),
              0x71,
            ); // 7 = 0b0111, sum first 3 products into first float
          }
        }
        IrCmd::ExtractVec => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[(*get_op_mut(inst, 0_u32))]);

          (*self.build).vpshufps(
            inst.reg_x64,
            self.reg_op(*get_op_mut(inst, 0)),
            OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
            (self.int_op(*get_op_mut(inst, 1))) as u8,
          );
        }
        IrCmd::NotAny => {
          {
            // TODO: if we have a single user which is a STORE_INT, we are missing the opportunity to write directly to target
            inst.reg_x64 = self.regs.alloc_reg_or_reuse(
              SizeX64::Dword,
              index,
              &[(*get_op_mut(inst, 0_u32)), (*get_op_mut(inst, 1_u32))],
            );

            let mut save_one = Label::default();
            let mut save_zero = Label::default();
            let mut exit = Label::default();

            if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Constant {
              // Other cases should've been constant folded
              CODEGEN_ASSERT!(self.tag_op(*get_op_mut(inst, 0)) == LuaType::Boolean as u8);
            } else {
              (*self.build).cmp(
                OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
                OperandX64::imm((LuaType::Nil as u8) as i32),
              );
              (*self.build).jcc(ConditionX64::Equal, &mut save_one);

              (*self.build).cmp(
                OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
                OperandX64::imm((LuaType::Boolean as u8) as i32),
              );
              (*self.build).jcc(ConditionX64::NotEqual, &mut save_zero);
            }

            if (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Constant {
              // If value is 1, we fallthrough to storing 0
              if self.int_op(*get_op_mut(inst, 1)) == 0 {
                (*self.build).jmp_label(&mut save_one);
              }
            } else {
              (*self.build).cmp(
                OperandX64::reg(self.reg_op(*get_op_mut(inst, 1))),
                OperandX64::imm(0_i32),
              );
              (*self.build).jcc(ConditionX64::Equal, &mut save_one);
            }

            (*self.build).set_label(&mut save_zero);
            (*self.build).mov(OperandX64::reg(inst.reg_x64), OperandX64::imm(0_i32));
            (*self.build).jmp_label(&mut exit);

            (*self.build).set_label(&mut save_one);
            (*self.build).mov(OperandX64::reg(inst.reg_x64), OperandX64::imm(1_i32));

            (*self.build).set_label(&mut exit);
          }
        }
        IrCmd::CmpInt => {
          {
            // Cannot reuse operand registers as a target because we have to modify it before the comparison
            inst.reg_x64 = self.regs.alloc_reg(SizeX64::Dword, index);

            // We are going to operate on byte register, those do not clear high bits on write
            (*self.build).xor_(OperandX64::reg(inst.reg_x64), OperandX64::reg(inst.reg_x64));

            let cond = condition_op(*get_op_mut(inst, 2));

            if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Constant {
              (*self.build).cmp(
                OperandX64::reg(self.reg_op(*get_op_mut(inst, 1))),
                OperandX64::imm(self.int_op(*get_op_mut(inst, 0))),
              );
              (*self.build).setcc(
                get_inverse_condition(get_condition_int(cond)),
                OperandX64::reg(byte_reg(inst.reg_x64)),
              );
            } else if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Inst {
              (*self.build).cmp(
                OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
                OperandX64::imm(self.int_op(*get_op_mut(inst, 1))),
              );
              (*self.build).setcc(
                get_condition_int(cond),
                OperandX64::reg(byte_reg(inst.reg_x64)),
              );
            } else {
              CODEGEN_ASSERT!(false, "Unsupported instruction form");
            }
          }
        }
        IrCmd::CmpAny => {
          {
            CODEGEN_ASSERT!(
              (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::VmReg
                && (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::VmReg
            );
            let cond = condition_op(*get_op_mut(inst, 2));

            inst.reg_x64 = self.regs.alloc_reg(SizeX64::Dword, index);

            let mut skip = Label::default();
            let mut exit = Label::default();

            // For equality comparison, 'luaV_equalval' expects tag to be equal before the call
            if cond == IrCondition::Equal {
              let mut tmp = ScopedRegX64 {
                owner: &mut self.regs,
                reg: RegisterX64::NOREG,
              };
              tmp.alloc(SizeX64::Dword);

              (*self.build).mov(
                OperandX64::reg(tmp.reg),
                self.mem_reg_tag_op(*get_op_mut(inst, 0)),
              );
              (*self.build).cmp(
                self.mem_reg_tag_op(*get_op_mut(inst, 1)),
                OperandX64::reg(tmp.reg),
              );

              // If the tags are not equal, skip the call and set result to 0
              (*self.build).jcc(ConditionX64::NotEqual, &mut skip);
            }

            {
              let mut spill_guard = ScopedSpills {
                owner: null_mut(),
                start_spill_id: 0,
              };
              spill_guard.scoped_spills_scoped_spills_ir_reg_alloc_x_64(&mut self.regs);

              let mut call_wrap = IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
                &mut self.regs,
                &mut *self.build,
                index,
              );
              call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
                SizeX64::Qword,
                OperandX64::reg(r_state()),
                IrOp::default(),
              );
              call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
                SizeX64::Qword,
                luau_reg_address(vm_reg_op(*get_op_mut(inst, 0))),
                IrOp::default(),
              );
              call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
                SizeX64::Qword,
                luau_reg_address(vm_reg_op(*get_op_mut(inst, 1))),
                IrOp::default(),
              );
              call_wrap.set_result_register(inst.reg_x64, index);

              if cond == IrCondition::LessEqual {
                call_wrap.call(&OperandX64::mem(
                  SizeX64::Qword,
                  RegisterX64::NOREG,
                  1,
                  r_native_context(),
                  (core::mem::offset_of!(NativeContext, lua_v_lessequal) as i32),
                ));
              } else if cond == IrCondition::Less {
                call_wrap.call(&OperandX64::mem(
                  SizeX64::Qword,
                  RegisterX64::NOREG,
                  1,
                  r_native_context(),
                  (core::mem::offset_of!(NativeContext, lua_v_lessthan) as i32),
                ));
              } else if cond == IrCondition::Equal {
                call_wrap.call(&OperandX64::mem(
                  SizeX64::Qword,
                  RegisterX64::NOREG,
                  1,
                  r_native_context(),
                  (core::mem::offset_of!(NativeContext, lua_v_equalval) as i32),
                ));
              } else {
                CODEGEN_ASSERT!(false, "Unsupported condition");
              }

              emit_update_base(&mut *self.build);
            }

            if cond == IrCondition::Equal {
              (*self.build).jmp_label(&mut exit);
              (*self.build).set_label(&mut skip);

              (*self.build).xor_(OperandX64::reg(inst.reg_x64), OperandX64::reg(inst.reg_x64));
              (*self.build).set_label(&mut exit);
            }

            // If case we made a call, skip high register bits clear, only consumer is JUMP_CMP_INT which doesn't read them
          }
        }
        IrCmd::CmpTag => {
          {
            // Cannot reuse operand registers as a target because we have to modify it before the comparison
            inst.reg_x64 = self.regs.alloc_reg(SizeX64::Dword, index);

            // We are going to operate on byte register, those do not clear high bits on write
            (*self.build).xor_(OperandX64::reg(inst.reg_x64), OperandX64::reg(inst.reg_x64));

            let cond = condition_op(*get_op_mut(inst, 2));
            CODEGEN_ASSERT!(cond == IrCondition::Equal || cond == IrCondition::NotEqual);
            let cond_x64 = get_condition_int(cond);

            if self.tag_op(*get_op_mut(inst, 1)) == LuaType::Nil as u8
              && (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Inst
            {
              (*self.build).test(
                OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
                OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
              );
            } else {
              (*self.build).cmp(
                self.mem_reg_tag_op(*get_op_mut(inst, 0)),
                OperandX64::imm((self.tag_op(*get_op_mut(inst, 1))) as i32),
              );
            }

            (*self.build).setcc(cond_x64, OperandX64::reg(byte_reg(inst.reg_x64)));
          }
        }
        IrCmd::CmpSplitTvalue => {
          {
            // Cannot reuse operand registers as a target because we have to modify it before the comparison
            inst.reg_x64 = self.regs.alloc_reg(SizeX64::Dword, index);

            // Second operand of this instruction must be a constant
            // Without a constant type, we wouldn't know the correct way to compare the values at lowering time
            CODEGEN_ASSERT!((*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Constant);

            // We are going to operate on byte registers, those do not clear high bits on write
            (*self.build).xor_(OperandX64::reg(inst.reg_x64), OperandX64::reg(inst.reg_x64));

            let cond = condition_op(*get_op_mut(inst, 4));
            CODEGEN_ASSERT!(cond == IrCondition::Equal || cond == IrCondition::NotEqual);

            // Check tag equality first
            let mut tmp1 = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp1.alloc(SizeX64::Byte);

            if (*get_op_mut(inst, 0_u32)).kind() != IrOpKind::Constant {
              (*self.build).cmp(
                OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
                OperandX64::imm((self.tag_op(*get_op_mut(inst, 1))) as i32),
              );
              (*self.build).setcc(get_condition_int(cond), OperandX64::reg(byte_reg(tmp1.reg)));
            } else {
              // Constant folding had to handle different constant tags
              CODEGEN_ASSERT!(
                self.tag_op(*get_op_mut(inst, 0)) == self.tag_op(*get_op_mut(inst, 1))
              );
            }

            if self.tag_op(*get_op_mut(inst, 1)) == LuaType::Boolean as u8 {
              if (*get_op_mut(inst, 2_u32)).kind() == IrOpKind::Constant {
                (*self.build).cmp(
                  OperandX64::reg(self.reg_op(*get_op_mut(inst, 3))),
                  OperandX64::imm(self.int_op(*get_op_mut(inst, 2))),
                );
              }
              // swapped arguments
              else if (*get_op_mut(inst, 3_u32)).kind() == IrOpKind::Constant {
                (*self.build).cmp(
                  OperandX64::reg(self.reg_op(*get_op_mut(inst, 2))),
                  OperandX64::imm(self.int_op(*get_op_mut(inst, 3))),
                );
              } else {
                (*self.build).cmp(
                  OperandX64::reg(self.reg_op(*get_op_mut(inst, 2))),
                  OperandX64::reg(self.reg_op(*get_op_mut(inst, 3))),
                );
              }

              (*self.build).setcc(
                get_condition_int(cond),
                OperandX64::reg(byte_reg(inst.reg_x64)),
              );
            } else if self.tag_op(*get_op_mut(inst, 1)) == LuaType::String as u8 {
              (*self.build).cmp(
                OperandX64::reg(self.reg_op(*get_op_mut(inst, 2))),
                OperandX64::reg(self.reg_op(*get_op_mut(inst, 3))),
              );
              (*self.build).setcc(
                get_condition_int(cond),
                OperandX64::reg(byte_reg(inst.reg_x64)),
              );
            } else if self.tag_op(*get_op_mut(inst, 1)) == LuaType::Number as u8 {
              if (*get_op_mut(inst, 2_u32)).kind() == IrOpKind::Constant {
                (*self.build).vucomisd(
                  OperandX64::reg(self.reg_op(*get_op_mut(inst, 3))),
                  self.mem_reg_double_op(*get_op_mut(inst, 2)),
                );
              }
              // swapped arguments
              else if (*get_op_mut(inst, 3_u32)).kind() == IrOpKind::Constant {
                (*self.build).vucomisd(
                  OperandX64::reg(self.reg_op(*get_op_mut(inst, 2))),
                  self.mem_reg_double_op(*get_op_mut(inst, 3)),
                );
              } else {
                (*self.build).vucomisd(
                  OperandX64::reg(self.reg_op(*get_op_mut(inst, 2))),
                  OperandX64::reg(self.reg_op(*get_op_mut(inst, 3))),
                );
              }

              let op2 = *get_op_mut(inst, 2);
              let op3 = *get_op_mut(inst, 3);
              if op2 == op3 {
                // When numbers are the same, we only need to check parity to detect NaN
                if cond == IrCondition::Equal {
                  (*self.build).setcc(
                    ConditionX64::NotParity,
                    OperandX64::reg(byte_reg(inst.reg_x64)),
                  );
                } else {
                  (*self.build).setcc(
                    ConditionX64::Parity,
                    OperandX64::reg(byte_reg(inst.reg_x64)),
                  );
                }
              } else {
                let mut tmp2 = ScopedRegX64 {
                  owner: &mut self.regs,
                  reg: RegisterX64::NOREG,
                };
                tmp2.alloc(SizeX64::Dword);

                if cond == IrCondition::Equal {
                  (*self.build).mov(OperandX64::reg(tmp2.reg), OperandX64::imm(0_i32));
                  (*self.build).setcc(
                    ConditionX64::NotParity,
                    OperandX64::reg(byte_reg(inst.reg_x64)),
                  );
                  (*self.build).cmov(
                    ConditionX64::NotEqual,
                    inst.reg_x64,
                    OperandX64::reg(tmp2.reg),
                  );
                } else {
                  (*self.build).mov(OperandX64::reg(tmp2.reg), OperandX64::imm(1_i32));
                  (*self.build).setcc(
                    ConditionX64::Parity,
                    OperandX64::reg(byte_reg(inst.reg_x64)),
                  );
                  (*self.build).cmov(
                    ConditionX64::NotEqual,
                    inst.reg_x64,
                    OperandX64::reg(tmp2.reg),
                  );
                }
              }
            } else if self.tag_op(*get_op_mut(inst, 1)) == LuaType::Integer as u8 {
              if (*get_op_mut(inst, 2_u32)).kind() == IrOpKind::Constant {
                (*self.build).cmp(
                  OperandX64::reg(self.reg_op(*get_op_mut(inst, 3))),
                  self.mem_reg_int_64_op(*get_op_mut(inst, 2)),
                );
              }
              // swapped arguments
              else if (*get_op_mut(inst, 3_u32)).kind() == IrOpKind::Constant {
                (*self.build).cmp(
                  OperandX64::reg(self.reg_op(*get_op_mut(inst, 2))),
                  self.mem_reg_int_64_op(*get_op_mut(inst, 3)),
                );
              } else {
                (*self.build).cmp(
                  OperandX64::reg(self.reg_op(*get_op_mut(inst, 2))),
                  OperandX64::reg(self.reg_op(*get_op_mut(inst, 3))),
                );
              }

              (*self.build).setcc(
                get_condition_int(cond),
                OperandX64::reg(byte_reg(inst.reg_x64)),
              );
            } else {
              CODEGEN_ASSERT!(false, "unsupported type tag in CMP_SPLIT_TVALUE");
            }

            if (*get_op_mut(inst, 0_u32)).kind() != IrOpKind::Constant {
              if cond == IrCondition::Equal {
                (*self.build).and_(
                  OperandX64::reg(byte_reg(inst.reg_x64)),
                  OperandX64::reg(byte_reg(tmp1.reg)),
                );
              } else {
                (*self.build).or_(
                  OperandX64::reg(byte_reg(inst.reg_x64)),
                  OperandX64::reg(byte_reg(tmp1.reg)),
                );
              }
            }
          }
        }
        IrCmd::JUMP => {
          self.jump_or_abort_on_undef_ir_op_u32_ir_block(*get_op_mut(inst, 0), index, next);
        }
        IrCmd::JumpIfTruthy => {
          jump_if_truthy(
            &mut *self.build,
            vm_reg_op(*get_op_mut(inst, 0)),
            &mut *self.label_op(*get_op_mut(inst, 1)),
            &mut *self.label_op(*get_op_mut(inst, 2)),
          );
          let target_block = self.block_op(*get_op_mut(inst, 2));
          self.jump_or_fallthrough(&mut *target_block, next);
        }
        IrCmd::JumpIfFalsy => {
          jump_if_falsy(
            &mut *self.build,
            vm_reg_op(*get_op_mut(inst, 0)),
            &mut *self.label_op(*get_op_mut(inst, 1)),
            &mut *self.label_op(*get_op_mut(inst, 2)),
          );
          let target_block = self.block_op(*get_op_mut(inst, 2));
          self.jump_or_fallthrough(&mut *target_block, next);
        }
        IrCmd::JumpEqTag => {
          CODEGEN_ASSERT!(
            (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Inst
              || (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Constant
          );
          let opb = if (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Inst {
            OperandX64::reg(self.reg_op(*get_op_mut(inst, 1)))
          } else {
            OperandX64::imm(self.tag_op(*get_op_mut(inst, 1)) as i32)
          };

          if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Constant {
            (*self.build).cmp(
              opb,
              OperandX64::imm((self.tag_op(*get_op_mut(inst, 0))) as i32),
            );
          } else {
            (*self.build).cmp(self.mem_reg_tag_op(*get_op_mut(inst, 0)), opb);
          }

          if self.is_fallthrough_block(&*self.block_op(*get_op_mut(inst, 3)), next) {
            (*self.build).jcc(
              ConditionX64::Equal,
              &mut *self.label_op(*get_op_mut(inst, 2)),
            );
            let target_block = self.block_op(*get_op_mut(inst, 3));
            self.jump_or_fallthrough(&mut *target_block, next);
          } else {
            (*self.build).jcc(
              ConditionX64::NotEqual,
              &mut *self.label_op(*get_op_mut(inst, 3)),
            );
            let target_block = self.block_op(*get_op_mut(inst, 2));
            self.jump_or_fallthrough(&mut *target_block, next);
          }
        }
        IrCmd::JumpCmpInt => {
          let cond = condition_op(*get_op_mut(inst, 2));

          if (cond == IrCondition::Equal || cond == IrCondition::NotEqual)
            && self.int_op(*get_op_mut(inst, 1)) == 0
          {
            let invert = cond == IrCondition::NotEqual;

            (*self.build).test(
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
            );

            if self.is_fallthrough_block(&*self.block_op(*get_op_mut(inst, 3)), next) {
              (*self.build).jcc(
                if invert {
                  ConditionX64::Zero
                } else {
                  ConditionX64::NotZero
                },
                &mut *self.label_op(*get_op_mut(inst, 4)),
              );
              let target_block = self.block_op(*get_op_mut(inst, 3));
              self.jump_or_fallthrough(&mut *target_block, next);
            } else {
              (*self.build).jcc(
                if invert {
                  ConditionX64::NotZero
                } else {
                  ConditionX64::Zero
                },
                &mut *self.label_op(*get_op_mut(inst, 3)),
              );
              let target_block = self.block_op(*get_op_mut(inst, 4));
              self.jump_or_fallthrough(&mut *target_block, next);
            }
          } else {
            (*self.build).cmp(
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
              OperandX64::imm(self.int_op(*get_op_mut(inst, 1))),
            );

            (*self.build).jcc(
              get_condition_int(cond),
              &mut *self.label_op(*get_op_mut(inst, 3)),
            );
            let target_block = self.block_op(*get_op_mut(inst, 4));
            self.jump_or_fallthrough(&mut *target_block, next);
          }
        }
        IrCmd::JumpEqPointer => {
          (*self.build).cmp(
            OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
            OperandX64::reg(self.reg_op(*get_op_mut(inst, 1))),
          );

          (*self.build).jcc(
            ConditionX64::Equal,
            &mut *self.label_op(*get_op_mut(inst, 2)),
          );
          let target_block = self.block_op(*get_op_mut(inst, 3));
          self.jump_or_fallthrough(&mut *target_block, next);
        }
        IrCmd::JumpCmpNum => {
          {
            let cond = condition_op(*get_op_mut(inst, 2));

            let mut tmp = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp.alloc(SizeX64::Xmmword);

            jump_on_number_cmp(
              &mut *self.build,
              tmp.reg,
              self.mem_reg_double_op(*get_op_mut(inst, 0)),
              self.mem_reg_double_op(*get_op_mut(inst, 1)),
              cond,
              &mut *self.label_op(*get_op_mut(inst, 3)),
              /* floatPrecision */ false,
            );
            let target_block = self.block_op(*get_op_mut(inst, 4));
            self.jump_or_fallthrough(&mut *target_block, next);
          }
        }
        IrCmd::JumpCmpFloat => {
          {
            let cond = condition_op(*get_op_mut(inst, 2));

            let mut tmp = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp.alloc(SizeX64::Xmmword);

            jump_on_number_cmp(
              &mut *self.build,
              tmp.reg,
              self.mem_reg_float_op(*get_op_mut(inst, 0)),
              self.mem_reg_float_op(*get_op_mut(inst, 1)),
              cond,
              &mut *self.label_op(*get_op_mut(inst, 3)),
              /* floatPrecision */ true,
            );
            let target_block = self.block_op(*get_op_mut(inst, 4));
            self.jump_or_fallthrough(&mut *target_block, next);
          }
        }
        IrCmd::JumpFornLoopCond => {
          {
            let mut tmp1 = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp1.alloc(SizeX64::Xmmword);
            let mut tmp2 = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp2.alloc(SizeX64::Xmmword);
            let mut tmp3 = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp3.alloc(SizeX64::Xmmword);

            let index = if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Inst {
              self.reg_op(*get_op_mut(inst, 0))
            } else {
              tmp1.reg
            };
            let limit = if (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Inst {
              self.reg_op(*get_op_mut(inst, 1))
            } else {
              tmp2.reg
            };

            if (*get_op_mut(inst, 0_u32)).kind() != IrOpKind::Inst {
              (*self.build).vmovsd_operand_x_64_operand_x_64(
                OperandX64::reg(tmp1.reg),
                self.mem_reg_double_op(*get_op_mut(inst, 0)),
              );
            }

            if (*get_op_mut(inst, 1_u32)).kind() != IrOpKind::Inst {
              (*self.build).vmovsd_operand_x_64_operand_x_64(
                OperandX64::reg(tmp2.reg),
                self.mem_reg_double_op(*get_op_mut(inst, 1)),
              );
            }

            let mut direct = Label::default();

            // step > 0
            jump_on_number_cmp(
              &mut *self.build,
              tmp3.reg,
              self.mem_reg_double_op(*get_op_mut(inst, 2)),
              (*self.build).f64(0.0),
              IrCondition::Greater,
              &mut direct,
              /* floatPrecision */ false,
            );

            // !(limit <= index)
            jump_on_number_cmp(
              &mut *self.build,
              RegisterX64::NOREG,
              OperandX64::reg(limit),
              OperandX64::reg(index),
              IrCondition::NotLessEqual,
              &mut *self.label_op(*get_op_mut(inst, 4)),
              /* floatPrecision */ false,
            );
            (*self.build).jmp_label(&mut *self.label_op(*get_op_mut(inst, 3)));

            // !(index <= limit)
            (*self.build).set_label(&mut direct);
            jump_on_number_cmp(
              &mut *self.build,
              RegisterX64::NOREG,
              OperandX64::reg(index),
              OperandX64::reg(limit),
              IrCondition::NotLessEqual,
              &mut *self.label_op(*get_op_mut(inst, 4)),
              /* floatPrecision */ false,
            );
            let target_block = self.block_op(*get_op_mut(inst, 3));
            self.jump_or_fallthrough(&mut *target_block, next);
          }
        }
        IrCmd::TableLen => {
          {
            let mut call_wrap = IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
              &mut self.regs,
              &mut *self.build,
              index,
            );
            call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
              SizeX64::Qword,
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
              *get_op_mut(inst, 0_u32),
            );
            call_wrap.call(&OperandX64::mem(
              SizeX64::Qword,
              RegisterX64::NOREG,
              1,
              r_native_context(),
              (core::mem::offset_of!(NativeContext, lua_h_getn) as i32),
            ));

            inst.reg_x64 = self.regs.take_reg(dword_reg(RegisterX64::RAX), index);

            (*self.build).mov(OperandX64::reg(inst.reg_x64), OperandX64::reg(inst.reg_x64));
            // Ensure high register bits are cleared
          }
        }
        IrCmd::TableSetnum => {
          let mut call_wrap = IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
            &mut self.regs,
            &mut *self.build,
            index,
          );
          call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::Qword,
            OperandX64::reg(r_state()),
            IrOp::default(),
          );
          call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::Qword,
            OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
            *get_op_mut(inst, 0_u32),
          );
          call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::Dword,
            OperandX64::reg(self.reg_op(*get_op_mut(inst, 1))),
            *get_op_mut(inst, 1_u32),
          );
          call_wrap.call(&OperandX64::mem(
            SizeX64::Qword,
            RegisterX64::NOREG,
            1,
            r_native_context(),
            (core::mem::offset_of!(NativeContext, lua_h_setnum) as i32),
          ));
          inst.reg_x64 = self.regs.take_reg(RegisterX64::RAX, index);
        }
        IrCmd::StringLen => {
          let ptr = self.reg_op(*get_op_mut(inst, 0));
          inst.reg_x64 = self.regs.alloc_reg(SizeX64::Dword, index);
          (*self.build).mov(
            OperandX64::reg(inst.reg_x64),
            OperandX64::mem(
              SizeX64::Dword,
              RegisterX64::NOREG,
              1,
              ptr,
              K_TSTRING_LEN_OFFSET,
            ),
          );
        }
        IrCmd::NewTable => {
          let mut call_wrap = IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
            &mut self.regs,
            &mut *self.build,
            index,
          );
          call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::Qword,
            OperandX64::reg(r_state()),
            IrOp::default(),
          );
          call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::Dword,
            OperandX64::imm(self.uint_op(*get_op_mut(inst, 0)) as i32),
            IrOp::default(),
          );
          call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::Dword,
            OperandX64::imm(self.uint_op(*get_op_mut(inst, 1)) as i32),
            IrOp::default(),
          );
          call_wrap.call(&OperandX64::mem(
            SizeX64::Qword,
            RegisterX64::NOREG,
            1,
            r_native_context(),
            (core::mem::offset_of!(NativeContext, lua_h_new) as i32),
          ));
          inst.reg_x64 = self.regs.take_reg(RegisterX64::RAX, index);
        }
        IrCmd::DupTable => {
          let mut call_wrap = IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
            &mut self.regs,
            &mut *self.build,
            index,
          );
          call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::Qword,
            OperandX64::reg(r_state()),
            IrOp::default(),
          );
          call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::Qword,
            OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
            *get_op_mut(inst, 0_u32),
          );
          call_wrap.call(&OperandX64::mem(
            SizeX64::Qword,
            RegisterX64::NOREG,
            1,
            r_native_context(),
            (core::mem::offset_of!(NativeContext, lua_h_clone) as i32),
          ));
          inst.reg_x64 = self.regs.take_reg(RegisterX64::RAX, index);
        }
        IrCmd::TryNumToIndex => {
          inst.reg_x64 = self.regs.alloc_reg(SizeX64::Dword, index);

          let mut tmp = ScopedRegX64 {
            owner: &mut self.regs,
            reg: RegisterX64::NOREG,
          };
          tmp.alloc(SizeX64::Xmmword);

          convert_number_to_index_or_jump(
            &mut *self.build,
            tmp.reg,
            self.reg_op(*get_op_mut(inst, 0)),
            inst.reg_x64,
            &mut *self.label_op(*get_op_mut(inst, 1)),
          );
        }
        IrCmd::TryCallFastgettm => {
          {
            inst.reg_x64 = self.regs.alloc_reg(SizeX64::Qword, index);

            let mut tmp = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp.alloc(SizeX64::Qword);

            (*self.build).mov(
              OperandX64::reg(tmp.reg),
              OperandX64::mem(
                SizeX64::Qword,
                RegisterX64::NOREG,
                1,
                self.reg_op(*get_op_mut(inst, 0)),
                (core::mem::offset_of!(LuaTable, metatable) as i32),
              ),
            );
            self
              .regs
              .free_last_use_reg((*self.function).inst_op(*get_op_mut(inst, 0_u32)), index); // Release before the call if it's the last use

            (*self.build).test(OperandX64::reg(tmp.reg), OperandX64::reg(tmp.reg));
            (*self.build).jcc(
              ConditionX64::Zero,
              &mut *self.label_op(*get_op_mut(inst, 2)),
            ); // No metatable

            (*self.build).test(
              OperandX64::mem(
                SizeX64::Byte,
                RegisterX64::NOREG,
                1,
                tmp.reg,
                (core::mem::offset_of!(LuaTable, tmcache) as i32),
              ),
              OperandX64::imm(1 << self.int_op(*get_op_mut(inst, 1))),
            );
            (*self.build).jcc(
              ConditionX64::NotZero,
              &mut *self.label_op(*get_op_mut(inst, 2)),
            ); // No tag method

            let mut tmp2 = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp2.alloc(SizeX64::Qword);
            (*self.build).mov(
              OperandX64::reg(tmp2.reg),
              OperandX64::mem(
                SizeX64::Qword,
                RegisterX64::NOREG,
                1,
                r_state(),
                (core::mem::offset_of!(lua_State, global) as i32),
              ),
            );

            {
              let mut spill_guard = ScopedSpills {
                owner: null_mut(),
                start_spill_id: 0,
              };
              spill_guard.scoped_spills_scoped_spills_ir_reg_alloc_x_64(&mut self.regs);

              let mut call_wrap = IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
                &mut self.regs,
                &mut *self.build,
                index,
              );
              call_wrap.add_argument_size_x_64_scoped_reg_x_64(SizeX64::Qword, &mut tmp);
              call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
                SizeX64::Qword,
                OperandX64::imm(self.int_op(*get_op_mut(inst, 1))),
                IrOp::default(),
              );
              call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
                SizeX64::Qword,
                OperandX64::mem(
                  SizeX64::Qword,
                  RegisterX64::NOREG,
                  1,
                  tmp2.release(),
                  (core::mem::offset_of!(global_State, tmname) as i32)
                    + self.int_op(*get_op_mut(inst, 1)) * (size_of::<*mut tstring>() as i32),
                ),
                IrOp::default(),
              );
              call_wrap.set_result_register(inst.reg_x64, index);
              call_wrap.call(&OperandX64::mem(
                SizeX64::Qword,
                RegisterX64::NOREG,
                1,
                r_native_context(),
                (core::mem::offset_of!(NativeContext, lua_t_gettm) as i32),
              ));
            }

            (*self.build).test(OperandX64::reg(inst.reg_x64), OperandX64::reg(inst.reg_x64));
            (*self.build).jcc(
              ConditionX64::Zero,
              &mut *self.label_op(*get_op_mut(inst, 2)),
            );
            // No tag method
          }
        }
        IrCmd::NewUserdata => {
          let mut call_wrap = IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
            &mut self.regs,
            &mut *self.build,
            index,
          );
          call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::Qword,
            OperandX64::reg(r_state()),
            IrOp::default(),
          );
          call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::Qword,
            OperandX64::imm(self.int_op(*get_op_mut(inst, 0))),
            IrOp::default(),
          );
          call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::Dword,
            OperandX64::imm(self.int_op(*get_op_mut(inst, 1))),
            IrOp::default(),
          );
          call_wrap.call(&OperandX64::mem(
            SizeX64::Qword,
            RegisterX64::NOREG,
            1,
            r_native_context(),
            (core::mem::offset_of!(NativeContext, new_userdata) as i32),
          ));
          inst.reg_x64 = self.regs.take_reg(RegisterX64::RAX, index);
        }
        IrCmd::IntToNum => {
          inst.reg_x64 = self.regs.alloc_reg(SizeX64::Xmmword, index);

          (*self.build).vcvtsi2sd(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
          );
        }
        IrCmd::UintToNum => {
          inst.reg_x64 = self.regs.alloc_reg(SizeX64::Xmmword, index);

          // AVX has no uint->double conversion; the source must come from UINT op and they all should clear top 32 bits so we can usually
          // use 64-bit reg; the one exception is NUM_TO_UINT which doesn't clear top bits
          let source = (*self.function).inst_op(*get_op_mut(inst, 0_u32)).cmd;
          if source == IrCmd::NumToUint {
            let mut tmp = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp.alloc(SizeX64::Dword);
            (*self.build).mov(
              OperandX64::reg(tmp.reg),
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
            );
            (*self.build).vcvtsi2sd(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(qword_reg(tmp.reg)),
            );
          } else {
            CODEGEN_ASSERT!(source != IrCmd::SUBSTITUTE); // we don't process substitutions
            (*self.build).vcvtsi2sd(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(qword_reg(self.reg_op(*get_op_mut(inst, 0)))),
            );
          }
        }
        IrCmd::UintToFloat => {
          inst.reg_x64 = self.regs.alloc_reg(SizeX64::Xmmword, index);

          // AVX has no uint->float conversion; the source must come from UINT op and they all should clear top 32 bits so we can usually
          // use 64-bit reg; the one exception is NUM_TO_UINT which doesn't clear top bits
          let source = (*self.function).inst_op(*get_op_mut(inst, 0_u32)).cmd;
          if source == IrCmd::NumToUint {
            let mut tmp = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp.alloc(SizeX64::Dword);
            (*self.build).mov(
              OperandX64::reg(tmp.reg),
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
            );
            (*self.build).vcvtsi2ss(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(qword_reg(tmp.reg)),
            );
          } else {
            CODEGEN_ASSERT!(source != IrCmd::SUBSTITUTE); // we don't process substitutions
            (*self.build).vcvtsi2ss(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(qword_reg(self.reg_op(*get_op_mut(inst, 0)))),
            );
          }
        }
        IrCmd::NumToInt => {
          inst.reg_x64 = self.regs.alloc_reg(SizeX64::Dword, index);

          (*self.build).vcvttsd2si(
            OperandX64::reg(inst.reg_x64),
            self.mem_reg_double_op(*get_op_mut(inst, 0)),
          );
        }
        IrCmd::NumToUint => {
          inst.reg_x64 = self.regs.alloc_reg(SizeX64::Dword, index);

          // Note: we perform 'uint64_t = (long long)double' for consistency with C++ code
          (*self.build).vcvttsd2si(
            OperandX64::reg(qword_reg(inst.reg_x64)),
            self.mem_reg_double_op(*get_op_mut(inst, 0)),
          );
        }
        IrCmd::FloatToNum => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[(*get_op_mut(inst, 0_u32))]);

          (*self.build).vcvtss2sd(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(inst.reg_x64),
            self.mem_reg_double_op(*get_op_mut(inst, 0)),
          );
        }
        IrCmd::NumToFloat => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[(*get_op_mut(inst, 0_u32))]);

          (*self.build).vcvtsd2ss(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(inst.reg_x64),
            self.mem_reg_double_op(*get_op_mut(inst, 0)),
          );
        }
        IrCmd::FloatToVec => {
          inst.reg_x64 = self.regs.alloc_reg(SizeX64::Xmmword, index);

          if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Constant {
            let value = (self.double_op(*get_op_mut(inst, 0))) as f32;
            let mut as_u32: u32 = 0;
            const _: () = assert!(size_of::<u32>() == size_of::<f32>());
            copy_nonoverlapping(
              &value as *const f32 as *const u32,
              &mut as_u32 as *mut u32,
              1,
            );

            (*self.build).vmovaps(
              OperandX64::reg(inst.reg_x64),
              (*self.build).u32x4(as_u32, as_u32, as_u32, 0),
            );
          } else {
            (*self.build).vpshufps(
              inst.reg_x64,
              self.reg_op(*get_op_mut(inst, 0)),
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
              0b00_00_00_00,
            );
          }
        }
        IrCmd::TagVector => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[(*get_op_mut(inst, 0_u32))]);

          (*self.build).vpinsrd(
            inst.reg_x64,
            self.reg_op(*get_op_mut(inst, 0)),
            (*self.build).i32(LuaType::Vector as i32),
            3_u8,
          );
        }
        IrCmd::TruncateUint => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Dword, index, &[(*get_op_mut(inst, 0_u32))]);

          // Might generate mov with the same source and destination register which is not a no-op
          (*self.build).mov(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
          );
        }
        IrCmd::AdjustStackToReg => {
          let mut tmp = ScopedRegX64 {
            owner: &mut self.regs,
            reg: RegisterX64::NOREG,
          };
          tmp.alloc(SizeX64::Qword);

          if (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Constant {
            (*self.build).lea_operand_x_64_operand_x_64(
              OperandX64::reg(tmp.reg),
              OperandX64::mem(
                SizeX64::None,
                RegisterX64::NOREG,
                1,
                r_base(),
                (vm_reg_op(*get_op_mut(inst, 0)) + self.int_op(*get_op_mut(inst, 1)))
                  * (size_of::<TValue>() as i32),
              ),
            );
            (*self.build).mov(
              OperandX64::mem(
                SizeX64::Qword,
                RegisterX64::NOREG,
                1,
                r_state(),
                (core::mem::offset_of!(lua_State, top) as i32),
              ),
              OperandX64::reg(tmp.reg),
            );
          } else if (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Inst {
            (*self.build).mov(
              OperandX64::reg(dword_reg(tmp.reg)),
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 1))),
            );
            (*self.build).shl(
              OperandX64::reg(tmp.reg),
              OperandX64::imm(K_TVALUE_SIZE_LOG2),
            );
            (*self.build).lea_operand_x_64_operand_x_64(
              OperandX64::reg(tmp.reg),
              OperandX64::mem(
                SizeX64::None,
                tmp.reg,
                1,
                r_base(),
                vm_reg_op(*get_op_mut(inst, 0)) * (size_of::<TValue>() as i32),
              ),
            );
            (*self.build).mov(
              OperandX64::mem(
                SizeX64::Qword,
                RegisterX64::NOREG,
                1,
                r_state(),
                (core::mem::offset_of!(lua_State, top) as i32),
              ),
              OperandX64::reg(tmp.reg),
            );
          } else {
            CODEGEN_ASSERT!(false, "Unsupported instruction form");
          }
        }
        IrCmd::AdjustStackToTop => {
          let mut tmp = ScopedRegX64 {
            owner: &mut self.regs,
            reg: RegisterX64::NOREG,
          };
          tmp.alloc(SizeX64::Qword);
          (*self.build).mov(
            OperandX64::reg(tmp.reg),
            OperandX64::mem(
              SizeX64::Qword,
              RegisterX64::NOREG,
              1,
              r_state(),
              (core::mem::offset_of!(lua_State, ci) as i32),
            ),
          );
          (*self.build).mov(
            OperandX64::reg(tmp.reg),
            OperandX64::mem(
              SizeX64::Qword,
              RegisterX64::NOREG,
              1,
              tmp.reg,
              (core::mem::offset_of!(CallInfo, top) as i32),
            ),
          );
          (*self.build).mov(
            OperandX64::mem(
              SizeX64::Qword,
              RegisterX64::NOREG,
              1,
              r_state(),
              (core::mem::offset_of!(lua_State, top) as i32),
            ),
            OperandX64::reg(tmp.reg),
          );
        }
        IrCmd::FASTCALL => {
          let bfid = self.uint_op(*get_op_mut(inst, 0)) as i32;
          let ra = vm_reg_op(*get_op_mut(inst, 1));
          let arg = vm_reg_op(*get_op_mut(inst, 2));
          let nparams = self.int_op(*get_op_mut(inst, 3));
          emit_builtin(&mut self.regs, &mut *self.build, bfid, ra, arg, nparams);
        }
        IrCmd::InvokeFastcall => {
          {
            let bfid = self.uint_op(*get_op_mut(inst, 0));

            let mut args = OperandX64::imm(0);
            let mut args_alt = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };

            // 'E' argument can only be produced by LOP_FASTCALL3
            if (*get_op_mut(inst, 4_u32)).kind() != IrOpKind::Undef {
              CODEGEN_ASSERT!(self.int_op(*get_op_mut(inst, 5)) == 3);

              let mut tmp = ScopedRegX64 {
                owner: &mut self.regs,
                reg: RegisterX64::NOREG,
              };
              tmp.alloc(SizeX64::Xmmword);
              args_alt.alloc(SizeX64::Qword);

              (*self.build).mov(
                OperandX64::reg(args_alt.reg),
                OperandX64::mem(
                  SizeX64::Qword,
                  RegisterX64::NOREG,
                  1,
                  r_state(),
                  (core::mem::offset_of!(lua_State, top) as i32),
                ),
              );

              (*self.build).vmovups(
                OperandX64::reg(tmp.reg),
                luau_reg(vm_reg_op(*get_op_mut(inst, 3))),
              );
              (*self.build).vmovups(
                OperandX64::mem(SizeX64::Xmmword, RegisterX64::NOREG, 1, args_alt.reg, 0),
                OperandX64::reg(tmp.reg),
              );

              (*self.build).vmovups(
                OperandX64::reg(tmp.reg),
                luau_reg(vm_reg_op(*get_op_mut(inst, 4))),
              );
              (*self.build).vmovups(
                OperandX64::mem(
                  SizeX64::Xmmword,
                  RegisterX64::NOREG,
                  1,
                  args_alt.reg,
                  size_of::<TValue>() as i32,
                ),
                OperandX64::reg(tmp.reg),
              );
            } else {
              if (*get_op_mut(inst, 3_u32)).kind() == IrOpKind::VmReg {
                args = luau_reg_address(vm_reg_op(*get_op_mut(inst, 3)));
              } else if (*get_op_mut(inst, 3_u32)).kind() == IrOpKind::VmConst {
                args = luau_constant_address(vm_const_op(*get_op_mut(inst, 3)));
              } else {
                CODEGEN_ASSERT!((*get_op_mut(inst, 3_u32)).kind() == IrOpKind::Undef);
              }
            }

            let ra = vm_reg_op(*get_op_mut(inst, 1));
            let arg = vm_reg_op(*get_op_mut(inst, 2));
            let nparams = self.int_op(*get_op_mut(inst, 5));
            let nresults = self.int_op(*get_op_mut(inst, 6));

            let mut call_wrap = IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
              &mut self.regs,
              &mut *self.build,
              index,
            );
            call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
              SizeX64::Qword,
              OperandX64::reg(r_state()),
              IrOp::default(),
            );
            call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
              SizeX64::Qword,
              luau_reg_address(ra),
              IrOp::default(),
            );
            call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
              SizeX64::Qword,
              luau_reg_address(arg),
              IrOp::default(),
            );
            call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
              SizeX64::Dword,
              OperandX64::imm(nresults),
              IrOp::default(),
            );

            if (*get_op_mut(inst, 4_u32)).kind() != IrOpKind::Undef {
              call_wrap.add_argument_size_x_64_scoped_reg_x_64(SizeX64::Qword, &mut args_alt);
            } else {
              call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
                SizeX64::Qword,
                args,
                IrOp::default(),
              );
            }

            if nparams == LUA_MULTRET {
              let reg = call_wrap.suggest_next_argument_register(SizeX64::Qword);
              let mut tmp = ScopedRegX64 {
                owner: &mut self.regs,
                reg: RegisterX64::NOREG,
              };
              tmp.alloc(SizeX64::Qword);

              // l->top - (ra + 1)
              (*self.build).mov(
                OperandX64::reg(reg),
                OperandX64::mem(
                  SizeX64::Qword,
                  RegisterX64::NOREG,
                  1,
                  r_state(),
                  (core::mem::offset_of!(lua_State, top) as i32),
                ),
              );
              (*self.build).lea_operand_x_64_operand_x_64(
                OperandX64::reg(tmp.reg),
                OperandX64::mem(
                  SizeX64::None,
                  RegisterX64::NOREG,
                  1,
                  r_base(),
                  (ra + 1) * (size_of::<TValue>() as i32),
                ),
              );
              (*self.build).sub(OperandX64::reg(reg), OperandX64::reg(tmp.reg));
              (*self.build).shr(OperandX64::reg(reg), OperandX64::imm(K_TVALUE_SIZE_LOG2));

              call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
                SizeX64::Dword,
                OperandX64::reg(dword_reg(reg)),
                IrOp::default(),
              );
            } else {
              call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
                SizeX64::Dword,
                OperandX64::imm(nparams),
                IrOp::default(),
              );
            }

            let mut func = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            func.alloc(SizeX64::Qword);
            (*self.build).mov(
              OperandX64::reg(func.reg),
              OperandX64::mem(
                SizeX64::Qword,
                RegisterX64::NOREG,
                1,
                r_native_context(),
                (core::mem::offset_of!(NativeContext, luau_f_table) as i32)
                  + (bfid as i32) * (size_of::<luau_FastFunction>() as i32),
              ),
            );

            call_wrap.call(&OperandX64::reg(func.release()));
            inst.reg_x64 = self.regs.take_reg(dword_reg(RegisterX64::RAX), index);
            // Result of a builtin call is returned in eax
            // Skipping high register bits clear, only consumer is CHECK_FASTCALL_RES which doesn't read them
          }
        }
        IrCmd::CheckFastcallRes => {
          {
            let res = self.reg_op(*get_op_mut(inst, 0));

            (*self.build).test(OperandX64::reg(res), OperandX64::reg(res)); // test here will set SF=1 for a negative number and it always sets OF to 0
            (*self.build).jcc(
              ConditionX64::Less,
              &mut *self.label_op(*get_op_mut(inst, 1)),
            );
            // jl jumps if SF != OF
          }
        }
        IrCmd::DoArith => {
          let opb = if (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::VmReg {
            luau_reg_address(vm_reg_op(*get_op_mut(inst, 1)))
          } else {
            luau_constant_address(vm_const_op(*get_op_mut(inst, 1)))
          };
          let opc = if (*get_op_mut(inst, 2_u32)).kind() == IrOpKind::VmReg {
            luau_reg_address(vm_reg_op(*get_op_mut(inst, 2)))
          } else {
            luau_constant_address(vm_const_op(*get_op_mut(inst, 2)))
          };
          let ra = vm_reg_op(*get_op_mut(inst, 0));
          let tm = transmute::<u32, TMS>(self.int_op(*get_op_mut(inst, 3)) as u32);
          call_arith_helper(&mut self.regs, &mut *self.build, ra, opb, opc, tm);
        }
        IrCmd::DoLen => {
          call_length_helper(
            &mut self.regs,
            &mut *self.build,
            vm_reg_op(*get_op_mut(inst, 0)),
            vm_reg_op(*get_op_mut(inst, 1)),
          );
        }
        IrCmd::GetTable => {
          if (*get_op_mut(inst, 2_u32)).kind() == IrOpKind::VmReg {
            call_get_table(
              &mut self.regs,
              &mut *self.build,
              vm_reg_op(*get_op_mut(inst, 1)),
              luau_reg_address(vm_reg_op(*get_op_mut(inst, 2))),
              vm_reg_op(*get_op_mut(inst, 0)),
            );
          } else if (*get_op_mut(inst, 2_u32)).kind() == IrOpKind::Constant {
            let mut n = TValue::default();
            setnvalue!(
              &mut n as *mut TValue,
              self.uint_op(*get_op_mut(inst, 2)) as f64
            );
            call_get_table(
              &mut self.regs,
              &mut *self.build,
              vm_reg_op(*get_op_mut(inst, 1)),
              (*self.build).bytes(&n as *const TValue as *const c_void, size_of::<TValue>(), 8),
              vm_reg_op(*get_op_mut(inst, 0)),
            );
          } else {
            CODEGEN_ASSERT!(false, "Unsupported instruction form");
          }
        }
        IrCmd::SetTable => {
          if (*get_op_mut(inst, 2_u32)).kind() == IrOpKind::VmReg {
            call_set_table(
              &mut self.regs,
              &mut *self.build,
              vm_reg_op(*get_op_mut(inst, 1)),
              luau_reg_address(vm_reg_op(*get_op_mut(inst, 2))),
              vm_reg_op(*get_op_mut(inst, 0)),
            );
          } else if (*get_op_mut(inst, 2_u32)).kind() == IrOpKind::Constant {
            let mut n = TValue::default();
            setnvalue!(
              &mut n as *mut TValue,
              self.uint_op(*get_op_mut(inst, 2)) as f64
            );
            call_set_table(
              &mut self.regs,
              &mut *self.build,
              vm_reg_op(*get_op_mut(inst, 1)),
              (*self.build).bytes(&n as *const TValue as *const c_void, size_of::<TValue>(), 8),
              vm_reg_op(*get_op_mut(inst, 0)),
            );
          } else {
            CODEGEN_ASSERT!(false, "Unsupported instruction form");
          }
        }
        IrCmd::GetCachedImport => {
          {
            self.regs.assert_all_free();
            self.regs.assert_no_spills();

            let mut skip = Label::default();
            let mut exit = Label::default();

            // If the constant for the import is set, we will use it directly, otherwise we have to call an import path lookup function
            (*self.build).cmp(
              luau_constant_tag(vm_const_op(*get_op_mut(inst, 1))),
              OperandX64::imm((LuaType::Nil as u8) as i32),
            );
            (*self.build).jcc(ConditionX64::NotEqual, &mut skip);

            {
              let mut spill_guard = ScopedSpills {
                owner: null_mut(),
                start_spill_id: 0,
              };
              spill_guard.scoped_spills_scoped_spills_ir_reg_alloc_x_64(&mut self.regs);

              let mut call_wrap = IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
                &mut self.regs,
                &mut *self.build,
                index,
              );
              call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
                SizeX64::Qword,
                OperandX64::reg(r_state()),
                IrOp::default(),
              );
              call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
                SizeX64::Qword,
                luau_reg_address(vm_reg_op(*get_op_mut(inst, 0))),
                IrOp::default(),
              );
              call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
                SizeX64::Dword,
                OperandX64::imm(self.import_op(*get_op_mut(inst, 2)) as i32),
                IrOp::default(),
              );
              call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
                SizeX64::Dword,
                OperandX64::imm(self.uint_op(*get_op_mut(inst, 3)) as i32),
                IrOp::default(),
              );
              call_wrap.call(&OperandX64::mem(
                SizeX64::Qword,
                RegisterX64::NOREG,
                1,
                r_native_context(),
                (core::mem::offset_of!(NativeContext, get_import) as i32),
              ));

              emit_update_base(&mut *self.build);
            }

            (*self.build).jmp_label(&mut exit);

            (*self.build).set_label(&mut skip);

            let mut tmp1 = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp1.alloc(SizeX64::Xmmword);

            (*self.build).vmovups(
              OperandX64::reg(tmp1.reg),
              luau_constant(vm_const_op(*get_op_mut(inst, 1))),
            );
            (*self.build).vmovups(
              luau_reg(vm_reg_op(*get_op_mut(inst, 0))),
              OperandX64::reg(tmp1.reg),
            );
            (*self.build).set_label(&mut exit);
          }
        }
        IrCmd::CONCAT => {
          let mut call_wrap = IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
            &mut self.regs,
            &mut *self.build,
            index,
          );
          call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::Qword,
            OperandX64::reg(r_state()),
            IrOp::default(),
          );
          call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::Dword,
            OperandX64::imm(self.uint_op(*get_op_mut(inst, 1)) as i32),
            IrOp::default(),
          );
          call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::Dword,
            OperandX64::imm(
              vm_reg_op(*get_op_mut(inst, 0)) + self.uint_op(*get_op_mut(inst, 1)) as i32 - 1,
            ),
            IrOp::default(),
          );
          call_wrap.call(&OperandX64::mem(
            SizeX64::Qword,
            RegisterX64::NOREG,
            1,
            r_native_context(),
            (core::mem::offset_of!(NativeContext, lua_v_concat) as i32),
          ));

          emit_update_base(&mut *self.build);
        }
        IrCmd::GetUpvalue => {
          {
            inst.reg_x64 = self.regs.alloc_reg(SizeX64::Xmmword, index);

            let mut tmp1 = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp1.alloc(SizeX64::Qword);

            (*self.build).mov(OperandX64::reg(tmp1.reg), S_CLOSURE);
            (*self.build).add(
              OperandX64::reg(tmp1.reg),
              OperandX64::imm(
                K_CLOSURE_LUPREFS_OFFSET
                  + (size_of::<TValue>() as i32) * vm_upvalue_op(*get_op_mut(inst, 0)) as i32,
              ),
            );

            // uprefs[] is either an actual value, or it points to UpVal object which has a pointer to value
            let mut skip = Label::default();
            (*self.build).cmp(
              OperandX64::mem(
                SizeX64::Dword,
                RegisterX64::NOREG,
                1,
                tmp1.reg,
                (core::mem::offset_of!(TValue, tt) as i32),
              ),
              OperandX64::imm((LuaType::Upval as u8) as i32),
            );
            (*self.build).jcc(ConditionX64::NotEqual, &mut skip);

            // UpVal.v points to the value (either on stack, or on heap inside each UpVal, but we can deref it unconditionally)
            (*self.build).mov(
              OperandX64::reg(tmp1.reg),
              OperandX64::mem(
                SizeX64::Qword,
                RegisterX64::NOREG,
                1,
                tmp1.reg,
                (core::mem::offset_of!(TValue, value.gc) as i32),
              ),
            );
            (*self.build).mov(
              OperandX64::reg(tmp1.reg),
              OperandX64::mem(
                SizeX64::Qword,
                RegisterX64::NOREG,
                1,
                tmp1.reg,
                (core::mem::offset_of!(UpVal, v) as i32),
              ),
            );

            (*self.build).set_label(&mut skip);

            (*self.build).vmovups(
              OperandX64::reg(inst.reg_x64),
              OperandX64::mem(SizeX64::Xmmword, RegisterX64::NOREG, 1, tmp1.reg, 0),
            );
          }
        }
        IrCmd::SetUpvalue => {
          let mut tmp1 = ScopedRegX64 {
            owner: &mut self.regs,
            reg: RegisterX64::NOREG,
          };
          tmp1.alloc(SizeX64::Qword);
          let mut tmp2 = ScopedRegX64 {
            owner: &mut self.regs,
            reg: RegisterX64::NOREG,
          };
          tmp2.alloc(SizeX64::Qword);

          (*self.build).mov(OperandX64::reg(tmp1.reg), S_CLOSURE);
          (*self.build).mov(
            OperandX64::reg(tmp2.reg),
            OperandX64::mem(
              SizeX64::Qword,
              RegisterX64::NOREG,
              1,
              tmp1.reg,
              K_CLOSURE_LUPREFS_OFFSET
                + (size_of::<TValue>() as i32) * vm_upvalue_op(*get_op_mut(inst, 0)) as i32
                + (core::mem::offset_of!(TValue, value.gc) as i32),
            ),
          );

          (*self.build).mov(
            OperandX64::reg(tmp1.reg),
            OperandX64::mem(
              SizeX64::Qword,
              RegisterX64::NOREG,
              1,
              tmp2.reg,
              (core::mem::offset_of!(UpVal, v) as i32),
            ),
          );
          (*self.build).vmovups(
            OperandX64::mem(SizeX64::Xmmword, RegisterX64::NOREG, 1, tmp1.reg, 0),
            OperandX64::reg(self.reg_op(*get_op_mut(inst, 1))),
          );

          tmp1.free();

          if (*get_op_mut(inst, 2_u32)).kind() == IrOpKind::Undef
            || is_gco(self.tag_op(*get_op_mut(inst, 2)))
          {
            let object = tmp2.release();
            let value_op = *get_op_mut(inst, 1);
            let value = self.reg_op(value_op);
            let tag_op = *get_op_mut(inst, 2);
            let ratag = if tag_op.kind() == IrOpKind::Undef {
              -1
            } else {
              self.tag_op(tag_op) as i32
            };
            call_barrier_object(
              &mut self.regs,
              &mut *self.build,
              object,
              IrOp::default(),
              value,
              value_op,
              ratag,
            );
          }
        }
        IrCmd::CheckTag => {
          (*self.build).cmp(
            self.mem_reg_tag_op(*get_op_mut(inst, 0)),
            OperandX64::imm((self.tag_op(*get_op_mut(inst, 1))) as i32),
          );
          self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
            ConditionX64::NotEqual,
            *get_op_mut(inst, 2),
            index,
            next,
          );
        }
        IrCmd::CheckTruthy => {
          {
            // Constant tags which don't require boolean value check should've been removed in constant folding
            CODEGEN_ASSERT!(
              (*get_op_mut(inst, 0_u32)).kind() != IrOpKind::Constant
                || self.tag_op(*get_op_mut(inst, 0)) == LuaType::Boolean as u8
            );

            let mut skip = Label::default();

            if (*get_op_mut(inst, 0_u32)).kind() != IrOpKind::Constant {
              // Fail to fallback on 'nil' (falsy)
              (*self.build).cmp(
                self.mem_reg_tag_op(*get_op_mut(inst, 0)),
                OperandX64::imm((LuaType::Nil as u8) as i32),
              );
              self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
                ConditionX64::Equal,
                *get_op_mut(inst, 2),
                index,
                next,
              );

              // Skip value test if it's not a boolean (truthy)
              (*self.build).cmp(
                self.mem_reg_tag_op(*get_op_mut(inst, 0)),
                OperandX64::imm((LuaType::Boolean as u8) as i32),
              );
              (*self.build).jcc(ConditionX64::NotEqual, &mut skip);
            }

            // fail to fallback on 'false' boolean value (falsy)
            if (*get_op_mut(inst, 1_u32)).kind() != IrOpKind::Constant {
              (*self.build).cmp(
                self.mem_reg_uint_op(*get_op_mut(inst, 1)),
                OperandX64::imm(0_i32),
              );
              self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
                ConditionX64::Equal,
                *get_op_mut(inst, 2),
                index,
                next,
              );
            } else {
              if self.int_op(*get_op_mut(inst, 1)) == 0 {
                self.jump_or_abort_on_undef_ir_op_u32_ir_block(*get_op_mut(inst, 2), index, next);
              }
            }

            if (*get_op_mut(inst, 0_u32)).kind() != IrOpKind::Constant {
              (*self.build).set_label(&mut skip);
            }
          }
        }
        IrCmd::CheckReadonly => {
          (*self.build).cmp(
            OperandX64::mem(
              SizeX64::Byte,
              RegisterX64::NOREG,
              1,
              self.reg_op(*get_op_mut(inst, 0)),
              (core::mem::offset_of!(LuaTable, readonly) as i32),
            ),
            OperandX64::imm(0_i32),
          );
          self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
            ConditionX64::NotEqual,
            *get_op_mut(inst, 1),
            index,
            next,
          );
        }
        IrCmd::CheckNoMetatable => {
          (*self.build).cmp(
            OperandX64::mem(
              SizeX64::Qword,
              RegisterX64::NOREG,
              1,
              self.reg_op(*get_op_mut(inst, 0)),
              (core::mem::offset_of!(LuaTable, metatable) as i32),
            ),
            OperandX64::imm(0_i32),
          );
          self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
            ConditionX64::NotEqual,
            *get_op_mut(inst, 1),
            index,
            next,
          );
        }
        IrCmd::CheckSafeEnv => {
          self.check_safe_env(*get_op_mut(inst, 0), index, next);
        }
        IrCmd::CheckArraySize => {
          if (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Inst {
            (*self.build).cmp(
              OperandX64::mem(
                SizeX64::Dword,
                RegisterX64::NOREG,
                1,
                self.reg_op(*get_op_mut(inst, 0)),
                (core::mem::offset_of!(LuaTable, sizearray) as i32),
              ),
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 1))),
            );
          } else if (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Constant {
            (*self.build).cmp(
              OperandX64::mem(
                SizeX64::Dword,
                RegisterX64::NOREG,
                1,
                self.reg_op(*get_op_mut(inst, 0)),
                (core::mem::offset_of!(LuaTable, sizearray) as i32),
              ),
              OperandX64::imm(self.int_op(*get_op_mut(inst, 1))),
            );
          } else {
            CODEGEN_ASSERT!(false, "Unsupported instruction form");
          }

          self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
            ConditionX64::BelowEqual,
            *get_op_mut(inst, 2),
            index,
            next,
          );
        }
        IrCmd::JumpSlotMatch | IrCmd::CheckSlotMatch => {
          {
            let mut abort = Label { id: 0, location: 0 }; // Used when guard aborts execution
            let mismatch_op = if inst.cmd == IrCmd::JumpSlotMatch {
              *get_op_mut(inst, 3_u32)
            } else {
              *get_op_mut(inst, 2_u32)
            };
            let mismatch = if mismatch_op.kind() == IrOpKind::Undef {
              &mut abort as *mut Label
            } else {
              (&mut *self.label_op(mismatch_op)) as *mut Label
            };

            let mut tmp = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp.alloc(SizeX64::Qword);

            // Check if node key tag is a string
            (*self.build).mov(
              OperandX64::reg(dword_reg(tmp.reg)),
              luau_node_key_tag(self.reg_op(*get_op_mut(inst, 0))),
            );
            (*self.build).and_(
              OperandX64::reg(dword_reg(tmp.reg)),
              OperandX64::imm(K_TKEY_TAG_MASK),
            );
            (*self.build).cmp(
              OperandX64::reg(dword_reg(tmp.reg)),
              OperandX64::imm((LuaType::String as u8) as i32),
            );
            (*self.build).jcc(ConditionX64::NotEqual, &mut *mismatch);

            // Check that node key value matches the expected one
            (*self.build).mov(
              OperandX64::reg(tmp.reg),
              luau_constant_value(vm_const_op(*get_op_mut(inst, 1))),
            );
            (*self.build).cmp(
              OperandX64::reg(tmp.reg),
              luau_node_key_value(self.reg_op(*get_op_mut(inst, 0))),
            );
            (*self.build).jcc(ConditionX64::NotEqual, &mut *mismatch);

            // Check that node value is not nil
            (*self.build).cmp(
              OperandX64::mem(
                SizeX64::Dword,
                RegisterX64::NOREG,
                1,
                self.reg_op(*get_op_mut(inst, 0)),
                (core::mem::offset_of!(LuaNode, val) as i32)
                  + (core::mem::offset_of!(TValue, tt) as i32),
              ),
              OperandX64::imm((LuaType::Nil as u8) as i32),
            );
            (*self.build).jcc(ConditionX64::Equal, &mut *mismatch);

            if inst.cmd == IrCmd::JumpSlotMatch {
              let target_block = self.block_op(*get_op_mut(inst, 2));
              self.jump_or_fallthrough(&mut *target_block, next);
            } else if mismatch_op.kind() == IrOpKind::Undef {
              let mut skip = Label { id: 0, location: 0 };
              (*self.build).jmp_label(&mut skip);
              (*self.build).set_label(&mut abort);
              (*self.build).ud_2();
              (*self.build).set_label(&mut skip);
            }
          }
        }
        IrCmd::CheckNodeNoNext => {
          let mut tmp = ScopedRegX64 {
            owner: &mut self.regs,
            reg: RegisterX64::NOREG,
          };
          tmp.alloc(SizeX64::Dword);

          (*self.build).mov(
            OperandX64::reg(tmp.reg),
            OperandX64::mem(
              SizeX64::Dword,
              RegisterX64::NOREG,
              1,
              self.reg_op(*get_op_mut(inst, 0)),
              (core::mem::offset_of!(LuaNode, key) as i32) + K_OFFSET_OF_TKEY_TAG_NEXT,
            ),
          );
          (*self.build).shr(OperandX64::reg(tmp.reg), OperandX64::imm(K_TKEY_TAG_BITS));
          self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
            ConditionX64::NotZero,
            *get_op_mut(inst, 1),
            index,
            next,
          );
        }
        IrCmd::CheckNodeValue => {
          (*self.build).cmp(
            OperandX64::mem(
              SizeX64::Dword,
              RegisterX64::NOREG,
              1,
              self.reg_op(*get_op_mut(inst, 0)),
              (core::mem::offset_of!(LuaNode, val) as i32)
                + (core::mem::offset_of!(TValue, tt) as i32),
            ),
            OperandX64::imm((LuaType::Nil as u8) as i32),
          );
          self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
            ConditionX64::Equal,
            *get_op_mut(inst, 1),
            index,
            next,
          );
        }
        IrCmd::CheckBufferLen => {
          {
            if FFlag::LuauCodegenVmExitSync.get() {
              let min_offset = self.int_op(*get_op_mut(inst, 2));
              let max_offset = self.int_op(*get_op_mut(inst, 3));
              CODEGEN_ASSERT!(min_offset < max_offset);

              let access_size = max_offset - min_offset;
              CODEGEN_ASSERT!(access_size > 0);

              // Determine which registers we will need
              let has_integer_check = (*get_op_mut(inst, 4_u32)).kind() != IrOpKind::Undef;
              let needs_extended_bounds_regs = (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Inst
                && !(access_size == 1 && min_offset == 0);

              // For jumps to exit sync blocks to work, we need the same register allocation state at each potential taken branch
              let reg_a = if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Inst {
                self.reg_op(*get_op_mut(inst, 0))
              } else {
                RegisterX64::NOREG
              };
              let reg_b = if (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Inst {
                self.reg_op(*get_op_mut(inst, 1))
              } else {
                RegisterX64::NOREG
              };
              let reg_e = if has_integer_check {
                self.reg_op(*get_op_mut(inst, 4))
              } else {
                RegisterX64::NOREG
              };

              let mut tmp_xmm = ScopedRegX64 {
                owner: &mut self.regs,
                reg: RegisterX64::NOREG,
              };
              let mut tmp1 = ScopedRegX64 {
                owner: &mut self.regs,
                reg: RegisterX64::NOREG,
              };
              let mut tmp2 = ScopedRegX64 {
                owner: &mut self.regs,
                reg: RegisterX64::NOREG,
              };

              if has_integer_check {
                tmp_xmm.alloc(SizeX64::Xmmword);
              }

              if needs_extended_bounds_regs {
                tmp1.alloc(SizeX64::Qword);
                tmp2.alloc(SizeX64::Dword);
              }

              let mut fresh = Label { id: 0, location: 0 };

              // Check if we are acting not only as a guard for the size, but as a guard that offset represents an exact integer
              if has_integer_check {
                CODEGEN_ASSERT!(
                  get_cmd_value_kind((*self.function).inst_op(*get_op_mut(inst, 1)).cmd)
                    == IrValueKind::Int
                );
                CODEGEN_ASSERT!(!produces_dirty_high_register_bits(
                  (*self.function).inst_op(*get_op_mut(inst, 1)).cmd
                )); // Ensure that high register bits are cleared

                // Convert integer back to double
                (*self.build).vcvtsi2sd(
                  OperandX64::reg(tmp_xmm.reg),
                  OperandX64::reg(tmp_xmm.reg),
                  OperandX64::reg(reg_b),
                );

                (*self.build).vucomisd(OperandX64::reg(tmp_xmm.reg), OperandX64::reg(reg_e)); // Sets ZF=1 if equal or NaN, PF=1 on NaN

                // We don't allow non-integer values
                self.jump_or_abort_on_undef_no_finalize(
                  ConditionX64::NotZero,
                  *get_op_mut(inst, 5),
                  index,
                  next,
                  &mut fresh,
                ); // exit on ZF=0
                self.jump_or_abort_on_undef_no_finalize(
                  ConditionX64::Parity,
                  *get_op_mut(inst, 5),
                  index,
                  next,
                  &mut fresh,
                ); // exit on PF=1
              }

              if (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Inst {
                CODEGEN_ASSERT!(!produces_dirty_high_register_bits(
                  (*self.function).inst_op(*get_op_mut(inst, 1)).cmd
                )); // Ensure that high register bits are cleared

                if access_size == 1 && min_offset == 0 {
                  // Simpler check for a single byte access
                  (*self.build).cmp(
                    OperandX64::mem(
                      SizeX64::Dword,
                      RegisterX64::NOREG,
                      1,
                      reg_a,
                      K_BUFFER_LEN_OFFSET,
                    ),
                    OperandX64::reg(reg_b),
                  );
                  self.jump_or_abort_on_undef_no_finalize(
                    ConditionX64::BelowEqual,
                    *get_op_mut(inst, 5),
                    index,
                    next,
                    &mut fresh,
                  );
                } else {
                  // To perform the bounds check using a single branch, we take index that is limited to a 32 bit int
                  // Max offset is then added using a 64 bit addition
                  // This will make sure that addition will not wrap around for values like 0xffffffff

                  if min_offset >= 0 {
                    (*self.build).lea_operand_x_64_operand_x_64(
                      OperandX64::reg(tmp1.reg),
                      OperandX64::mem(
                        SizeX64::None,
                        RegisterX64::NOREG,
                        1,
                        qword_reg(reg_b),
                        max_offset,
                      ),
                    );
                  } else {
                    // When the min offset is negative, we subtract it from offset first (in 32 bits)
                    (*self.build).lea_operand_x_64_operand_x_64(
                      OperandX64::reg(dword_reg(tmp1.reg)),
                      OperandX64::mem(SizeX64::None, RegisterX64::NOREG, 1, reg_b, min_offset),
                    );

                    // And then add the full access size like before
                    (*self.build).lea_operand_x_64_operand_x_64(
                      OperandX64::reg(tmp1.reg),
                      OperandX64::mem(SizeX64::None, RegisterX64::NOREG, 1, tmp1.reg, access_size),
                    );
                  }

                  (*self.build).mov(
                    OperandX64::reg(tmp2.reg),
                    OperandX64::mem(
                      SizeX64::Dword,
                      RegisterX64::NOREG,
                      1,
                      reg_a,
                      K_BUFFER_LEN_OFFSET,
                    ),
                  );
                  (*self.build).cmp(
                    OperandX64::reg(qword_reg(tmp2.reg)),
                    OperandX64::reg(tmp1.reg),
                  );
                  self.jump_or_abort_on_undef_no_finalize(
                    ConditionX64::Below,
                    *get_op_mut(inst, 5),
                    index,
                    next,
                    &mut fresh,
                  );
                }
              } else if (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Constant {
                let offset = self.int_op(*get_op_mut(inst, 1));

                let end_offset = if FFlag::LuauCodegenFixBufferLenCheck.get() {
                  max_offset
                } else {
                  access_size
                };

                // Constant folding can take care of it, but for safety we avoid overflow/underflow cases here
                if offset < 0 || ((offset) as u32) + ((end_offset) as u32) >= ((INT_MAX) as u32) {
                  self.jump_or_abort_on_undef_no_finalize(
                    ConditionX64::Count,
                    *get_op_mut(inst, 5),
                    index,
                    next,
                    &mut fresh,
                  );
                } else {
                  (*self.build).cmp(
                    OperandX64::mem(
                      SizeX64::Dword,
                      RegisterX64::NOREG,
                      1,
                      reg_a,
                      K_BUFFER_LEN_OFFSET,
                    ),
                    OperandX64::imm(offset + end_offset),
                  );
                }

                self.jump_or_abort_on_undef_no_finalize(
                  ConditionX64::Below,
                  *get_op_mut(inst, 5),
                  index,
                  next,
                  &mut fresh,
                );
              } else {
                CODEGEN_ASSERT!(false, "Unsupported instruction form");
              }

              self.finalize_target_label(*get_op_mut(inst, 5), index, &mut fresh);
            } else {
              let min_offset = self.int_op(*get_op_mut(inst, 2));
              let max_offset = self.int_op(*get_op_mut(inst, 3));
              CODEGEN_ASSERT!(min_offset < max_offset);

              let access_size = max_offset - min_offset;
              CODEGEN_ASSERT!(access_size > 0);

              // Check if we are acting not only as a guard for the size, but as a guard that offset represents an exact integer
              if (*get_op_mut(inst, 4_u32)).kind() != IrOpKind::Undef {
                CODEGEN_ASSERT!(
                  get_cmd_value_kind((*self.function).inst_op(*get_op_mut(inst, 1)).cmd)
                    == IrValueKind::Int
                );
                CODEGEN_ASSERT!(!produces_dirty_high_register_bits(
                  (*self.function).inst_op(*get_op_mut(inst, 1)).cmd
                )); // Ensure that high register bits are cleared

                let mut tmp = ScopedRegX64 {
                  owner: &mut self.regs,
                  reg: RegisterX64::NOREG,
                };
                tmp.alloc(SizeX64::Xmmword);

                // Convert integer back to double
                (*self.build).vcvtsi2sd(
                  OperandX64::reg(tmp.reg),
                  OperandX64::reg(tmp.reg),
                  OperandX64::reg(self.reg_op(*get_op_mut(inst, 1))),
                );

                (*self.build).vucomisd(
                  OperandX64::reg(tmp.reg),
                  OperandX64::reg(self.reg_op(*get_op_mut(inst, 4))),
                ); // Sets ZF=1 if equal or NaN, PF=1 on NaN

                // We don't allow non-integer values
                self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
                  ConditionX64::NotZero,
                  *get_op_mut(inst, 5),
                  index,
                  next,
                ); // exit on ZF=0
                self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
                  ConditionX64::Parity,
                  *get_op_mut(inst, 5),
                  index,
                  next,
                ); // exit on PF=1
              }

              if (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Inst {
                CODEGEN_ASSERT!(!produces_dirty_high_register_bits(
                  (*self.function).inst_op(*get_op_mut(inst, 1)).cmd
                )); // Ensure that high register bits are cleared

                if access_size == 1 && min_offset == 0 {
                  // Simpler check for a single byte access
                  (*self.build).cmp(
                    OperandX64::mem(
                      SizeX64::Dword,
                      RegisterX64::NOREG,
                      1,
                      self.reg_op(*get_op_mut(inst, 0)),
                      K_BUFFER_LEN_OFFSET,
                    ),
                    OperandX64::reg(self.reg_op(*get_op_mut(inst, 1))),
                  );
                  self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
                    ConditionX64::BelowEqual,
                    *get_op_mut(inst, 5),
                    index,
                    next,
                  );
                } else {
                  let mut tmp1 = ScopedRegX64 {
                    owner: &mut self.regs,
                    reg: RegisterX64::NOREG,
                  };
                  tmp1.alloc(SizeX64::Qword);
                  let mut tmp2 = ScopedRegX64 {
                    owner: &mut self.regs,
                    reg: RegisterX64::NOREG,
                  };
                  tmp2.alloc(SizeX64::Dword);

                  // To perform the bounds check using a single branch, we take index that is limited to a 32 bit int
                  // Max offset is then added using a 64 bit addition
                  // This will make sure that addition will not wrap around for values like 0xffffffff

                  if min_offset >= 0 {
                    (*self.build).lea_operand_x_64_operand_x_64(
                      OperandX64::reg(tmp1.reg),
                      OperandX64::mem(
                        SizeX64::None,
                        RegisterX64::NOREG,
                        1,
                        qword_reg(self.reg_op(*get_op_mut(inst, 1))),
                        max_offset,
                      ),
                    );
                  } else {
                    // When the min offset is negative, we subtract it from offset first (in 32 bits)
                    (*self.build).lea_operand_x_64_operand_x_64(
                      OperandX64::reg(dword_reg(tmp1.reg)),
                      OperandX64::mem(
                        SizeX64::None,
                        RegisterX64::NOREG,
                        1,
                        self.reg_op(*get_op_mut(inst, 1)),
                        min_offset,
                      ),
                    );

                    // And then add the full access size like before
                    (*self.build).lea_operand_x_64_operand_x_64(
                      OperandX64::reg(tmp1.reg),
                      OperandX64::mem(SizeX64::None, RegisterX64::NOREG, 1, tmp1.reg, access_size),
                    );
                  }

                  (*self.build).mov(
                    OperandX64::reg(tmp2.reg),
                    OperandX64::mem(
                      SizeX64::Dword,
                      RegisterX64::NOREG,
                      1,
                      self.reg_op(*get_op_mut(inst, 0)),
                      K_BUFFER_LEN_OFFSET,
                    ),
                  );
                  (*self.build).cmp(
                    OperandX64::reg(qword_reg(tmp2.reg)),
                    OperandX64::reg(tmp1.reg),
                  );

                  self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
                    ConditionX64::Below,
                    *get_op_mut(inst, 5),
                    index,
                    next,
                  );
                }
              } else if (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Constant {
                let offset = self.int_op(*get_op_mut(inst, 1));

                let end_offset = if FFlag::LuauCodegenFixBufferLenCheck.get() {
                  max_offset
                } else {
                  access_size
                };

                // Constant folding can take care of it, but for safety we avoid overflow/underflow cases here
                if offset < 0 || ((offset) as u32) + ((end_offset) as u32) >= ((INT_MAX) as u32) {
                  self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
                    ConditionX64::Count,
                    *get_op_mut(inst, 5),
                    index,
                    next,
                  );
                } else {
                  (*self.build).cmp(
                    OperandX64::mem(
                      SizeX64::Dword,
                      RegisterX64::NOREG,
                      1,
                      self.reg_op(*get_op_mut(inst, 0)),
                      K_BUFFER_LEN_OFFSET,
                    ),
                    OperandX64::imm(offset + end_offset),
                  );
                }

                self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
                  ConditionX64::Below,
                  *get_op_mut(inst, 5),
                  index,
                  next,
                );
              } else {
                CODEGEN_ASSERT!(false, "Unsupported instruction form");
              }
            }
          }
        }
        IrCmd::CheckUserdataTag => {
          (*self.build).cmp(
            OperandX64::mem(
              SizeX64::Byte,
              RegisterX64::NOREG,
              1,
              self.reg_op(*get_op_mut(inst, 0)),
              (core::mem::offset_of!(Udata, tag) as i32),
            ),
            OperandX64::imm(self.int_op(*get_op_mut(inst, 1))),
          );
          self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
            ConditionX64::NotEqual,
            *get_op_mut(inst, 2),
            index,
            next,
          );
        }
        IrCmd::CheckCmpNum => {
          let cond = condition_op(*get_op_mut(inst, 2));

          let mut fresh = Label { id: 0, location: 0 };
          let fail = self.get_target_label(*get_op_mut(inst, 3), index, &mut fresh) as *mut Label;

          let mut tmp = ScopedRegX64 {
            owner: &mut self.regs,
            reg: RegisterX64::NOREG,
          };
          tmp.alloc(SizeX64::Xmmword);

          jump_on_number_cmp(
            &mut *self.build,
            tmp.reg,
            self.mem_reg_double_op(*get_op_mut(inst, 0)),
            self.mem_reg_double_op(*get_op_mut(inst, 1)),
            get_negated_condition_ir_condition(cond),
            &mut *fail,
            false,
          );

          self.finalize_target_label(*get_op_mut(inst, 3), index, &mut fresh);
        }
        IrCmd::CheckCmpInt => {
          let cond = condition_op(*get_op_mut(inst, 2));

          if (cond == IrCondition::Equal || cond == IrCondition::NotEqual)
            && (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Constant
            && self.int_op(*get_op_mut(inst, 1)) == 0
          {
            (*self.build).test(
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
            );
            self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
              if cond == IrCondition::Equal {
                ConditionX64::NotZero
              } else {
                ConditionX64::Zero
              },
              *get_op_mut(inst, 3),
              index,
              next,
            );
          } else if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Constant {
            let mut tmp = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp.alloc(SizeX64::Dword);
            (*self.build).mov(
              OperandX64::reg(tmp.reg),
              self.mem_reg_int_op(*get_op_mut(inst, 0)),
            );
            (*self.build).cmp(
              OperandX64::reg(tmp.reg),
              self.mem_reg_int_op(*get_op_mut(inst, 1)),
            );
            self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
              get_condition_int(get_negated_condition_ir_condition(cond)),
              *get_op_mut(inst, 3),
              index,
              next,
            );
          } else {
            (*self.build).cmp(
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
              self.mem_reg_int_op(*get_op_mut(inst, 1)),
            );
            self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
              get_condition_int(get_negated_condition_ir_condition(cond)),
              *get_op_mut(inst, 3),
              index,
              next,
            );
          }
        }
        IrCmd::INTERRUPT => {
          {
            let pcpos = self.uint_op(*get_op_mut(inst, 0));

            // We unconditionally spill values here because that allows us to ignore register state when we synthesize interrupt handler
            // This can be changed in the future if we can somehow record interrupt handler code separately
            // Since interrupts are loop edges or call/ret, we don't have a significant opportunity for register reuse here anyway
            self.regs.preserve_and_free_inst_values();

            let mut tmp = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp.alloc(SizeX64::Qword);

            let mut self_lbl = Label::default();

            (*self.build).mov(
              OperandX64::reg(tmp.reg),
              OperandX64::mem(
                SizeX64::Qword,
                RegisterX64::NOREG,
                1,
                r_state(),
                (core::mem::offset_of!(lua_State, global) as i32),
              ),
            );
            (*self.build).cmp(
              OperandX64::mem(
                SizeX64::Qword,
                RegisterX64::NOREG,
                1,
                tmp.reg,
                (core::mem::offset_of!(global_State, cb.interrupt) as i32),
              ),
              OperandX64::imm(0_i32),
            );
            (*self.build).jcc(ConditionX64::NotEqual, &mut self_lbl);

            let mut next = Label::default();
            (*self.build).set_label(&mut next);

            self.interrupt_handlers.push(InterruptHandler {
              self_: self_lbl,
              pcpos,
              next,
            });
          }
        }
        IrCmd::CheckGc => {
          call_step_gc(&mut self.regs, &mut *self.build);
        }
        IrCmd::BarrierObj => {
          let object_op = *get_op_mut(inst, 0);
          let object = self.reg_op(object_op);
          let value_op = *get_op_mut(inst, 1);
          let tag_op = *get_op_mut(inst, 2);
          let ratag = if tag_op.kind() == IrOpKind::Undef {
            -1
          } else {
            self.tag_op(tag_op) as i32
          };
          call_barrier_object(
            &mut self.regs,
            &mut *self.build,
            object,
            object_op,
            RegisterX64::NOREG,
            value_op,
            ratag,
          );
        }
        IrCmd::BarrierTableBack => {
          let table_op = *get_op_mut(inst, 0);
          let table = self.reg_op(table_op);
          call_barrier_table_fast(&mut self.regs, &mut *self.build, table, table_op);
        }
        IrCmd::BarrierTableForward => {
          let mut skip = Label::default();

          let mut tmp = ScopedRegX64 {
            owner: &mut self.regs,
            reg: RegisterX64::NOREG,
          };
          tmp.alloc(SizeX64::Qword);

          check_object_barrier_conditions(
            &mut *self.build,
            tmp.reg,
            self.reg_op(*get_op_mut(inst, 0)),
            RegisterX64::NOREG,
            *get_op_mut(inst, 1),
            if (*get_op_mut(inst, 2)).kind() == IrOpKind::Undef {
              -1
            } else {
              self.tag_op(*get_op_mut(inst, 2)) as i32
            },
            &mut skip,
          );

          {
            let mut spill_guard = ScopedSpills {
              owner: null_mut(),
              start_spill_id: 0,
            };
            spill_guard.scoped_spills_scoped_spills_ir_reg_alloc_x_64(&mut self.regs);

            let mut call_wrap = IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
              &mut self.regs,
              &mut *self.build,
              index,
            );
            call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
              SizeX64::Qword,
              OperandX64::reg(r_state()),
              IrOp::default(),
            );
            call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
              SizeX64::Qword,
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
              *get_op_mut(inst, 0_u32),
            );
            call_wrap.add_argument_size_x_64_scoped_reg_x_64(SizeX64::Qword, &mut tmp);
            call_wrap.call(&OperandX64::mem(
              SizeX64::Qword,
              RegisterX64::NOREG,
              1,
              r_native_context(),
              (core::mem::offset_of!(NativeContext, lua_c_barriertable) as i32),
            ));
          }

          (*self.build).set_label(&mut skip);
        }
        IrCmd::SetSavedpc => {
          let mut tmp1 = ScopedRegX64 {
            owner: &mut self.regs,
            reg: RegisterX64::NOREG,
          };
          tmp1.alloc(SizeX64::Qword);
          let mut tmp2 = ScopedRegX64 {
            owner: &mut self.regs,
            reg: RegisterX64::NOREG,
          };
          tmp2.alloc(SizeX64::Qword);

          (*self.build).mov(OperandX64::reg(tmp2.reg), S_CODE);
          (*self.build).add(
            OperandX64::reg(tmp2.reg),
            OperandX64::imm(
              (self.uint_op(*get_op_mut(inst, 0)) as i32) * (size_of::<Instruction>() as i32),
            ),
          );
          (*self.build).mov(
            OperandX64::reg(tmp1.reg),
            OperandX64::mem(
              SizeX64::Qword,
              RegisterX64::NOREG,
              1,
              r_state(),
              (core::mem::offset_of!(lua_State, ci) as i32),
            ),
          );
          (*self.build).mov(
            OperandX64::mem(
              SizeX64::Qword,
              RegisterX64::NOREG,
              1,
              tmp1.reg,
              (core::mem::offset_of!(CallInfo, savedpc) as i32),
            ),
            OperandX64::reg(tmp2.reg),
          );
        }
        IrCmd::CloseUpvals => {
          {
            let mut next = Label::default();
            let mut tmp1 = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp1.alloc(SizeX64::Qword);
            let mut tmp2 = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp2.alloc(SizeX64::Qword);

            // l->openupval != 0
            (*self.build).mov(
              OperandX64::reg(tmp1.reg),
              OperandX64::mem(
                SizeX64::Qword,
                RegisterX64::NOREG,
                1,
                r_state(),
                (core::mem::offset_of!(lua_State, openupval) as i32),
              ),
            );
            (*self.build).test(OperandX64::reg(tmp1.reg), OperandX64::reg(tmp1.reg));
            (*self.build).jcc(ConditionX64::Zero, &mut next);

            // ra <= l->openupval->v
            (*self.build).lea_operand_x_64_operand_x_64(
              OperandX64::reg(tmp2.reg),
              OperandX64::mem(
                SizeX64::None,
                RegisterX64::NOREG,
                1,
                r_base(),
                vm_reg_op(*get_op_mut(inst, 0)) * (size_of::<TValue>() as i32),
              ),
            );
            (*self.build).cmp(
              OperandX64::reg(tmp2.reg),
              OperandX64::mem(
                SizeX64::Qword,
                RegisterX64::NOREG,
                1,
                tmp1.reg,
                (core::mem::offset_of!(UpVal, v) as i32),
              ),
            );
            (*self.build).jcc(ConditionX64::Above, &mut next);

            tmp1.free();

            {
              let mut spill_guard = ScopedSpills {
                owner: null_mut(),
                start_spill_id: 0,
              };
              spill_guard.scoped_spills_scoped_spills_ir_reg_alloc_x_64(&mut self.regs);

              let mut call_wrap = IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
                &mut self.regs,
                &mut *self.build,
                index,
              );
              call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
                SizeX64::Qword,
                OperandX64::reg(r_state()),
                IrOp::default(),
              );
              call_wrap.add_argument_size_x_64_scoped_reg_x_64(SizeX64::Qword, &mut tmp2);
              call_wrap.call(&OperandX64::mem(
                SizeX64::Qword,
                RegisterX64::NOREG,
                1,
                r_native_context(),
                (core::mem::offset_of!(NativeContext, lua_f_close) as i32),
              ));
            }

            (*self.build).set_label(&mut next);
          }
        }
        IrCmd::CAPTURE => {
          // No-op right now

          // Fallbacks to non-IR instruction implementations
        }
        IrCmd::SETLIST => {
          self.regs.assert_all_free();
          let ra = vm_reg_op(*get_op_mut(inst, 1));
          let rb = vm_reg_op(*get_op_mut(inst, 2));
          let count = self.int_op(*get_op_mut(inst, 3));
          let index = self.uint_op(*get_op_mut(inst, 4));
          let aux = if (*get_op_mut(inst, 5)).kind() == IrOpKind::Undef {
            -1
          } else {
            self.uint_op(*get_op_mut(inst, 5)) as i32
          };
          emit_inst_set_list(&mut self.regs, &mut *self.build, ra, rb, count, index, aux);
        }
        IrCmd::CALL => {
          self.regs.assert_all_free();
          self.regs.assert_no_spills();
          let ra = vm_reg_op(*get_op_mut(inst, 0));
          let nparams = self.int_op(*get_op_mut(inst, 1));
          let nresults = self.int_op(*get_op_mut(inst, 2));
          emit_inst_call(
            &mut self.regs,
            &mut *self.build,
            &mut *self.helpers,
            ra,
            nparams,
            nresults,
          );
        }
        IrCmd::RETURN => {
          self.regs.assert_all_free();
          self.regs.assert_no_spills();
          emit_inst_return(
            &mut *self.build,
            &mut *self.helpers,
            vm_reg_op(*get_op_mut(inst, 0)),
            self.int_op(*get_op_mut(inst, 1)),
            (*self.function).variadic,
          );
        }
        IrCmd::FORGLOOP => {
          self.regs.assert_all_free();
          let ra = vm_reg_op(*get_op_mut(inst, 0));
          let aux = self.int_op(*get_op_mut(inst, 1));
          let target = &mut *self.label_op(*get_op_mut(inst, 2)) as *mut Label;
          emit_inst_for_g_loop(&mut self.regs, &mut *self.build, ra, aux, &mut *target);
          let target_block = self.block_op(*get_op_mut(inst, 3));
          self.jump_or_fallthrough(&mut *target_block, next);
        }
        IrCmd::ForgloopFallback => {
          let mut call_wrap = IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
            &mut self.regs,
            &mut *self.build,
            index,
          );
          call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::Qword,
            OperandX64::reg(r_state()),
            IrOp::default(),
          );
          call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::Dword,
            OperandX64::imm(vm_reg_op(*get_op_mut(inst, 0))),
            IrOp::default(),
          );
          call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::Dword,
            OperandX64::imm(self.int_op(*get_op_mut(inst, 1))),
            IrOp::default(),
          );

          if FFlag::LuauYieldIter2.get() {
            call_wrap.call(&OperandX64::mem(
              SizeX64::Qword,
              RegisterX64::NOREG,
              1,
              r_native_context(),
              (core::mem::offset_of!(NativeContext, forg_loop_non_table_fallback) as i32),
            ));

            emit_update_base(&mut *self.build);

            (*self.build).test(
              OperandX64::reg(dword_reg(RegisterX64::RAX)),
              OperandX64::reg(dword_reg(RegisterX64::RAX)),
            );
            (*self.build).jcc(ConditionX64::Less, &mut (*self.helpers).exit_no_continue_vm);
            (*self.build).jcc(
              ConditionX64::Greater,
              &mut *self.label_op(*get_op_mut(inst, 2)),
            );
          } else {
            call_wrap.call(&OperandX64::mem(
              SizeX64::Qword,
              RegisterX64::NOREG,
              1,
              r_native_context(),
              (core::mem::offset_of!(NativeContext, forg_loop_non_table_fallback_deprecated)
                as i32),
            ));

            emit_update_base(&mut *self.build);

            (*self.build).test(
              OperandX64::reg(byte_reg(RegisterX64::RAX)),
              OperandX64::reg(byte_reg(RegisterX64::RAX)),
            );
            (*self.build).jcc(
              ConditionX64::NotZero,
              &mut *self.label_op(*get_op_mut(inst, 2)),
            );
          }

          let target_block = self.block_op(*get_op_mut(inst, 3));
          self.jump_or_fallthrough(&mut *target_block, next);
        }
        IrCmd::ForgprepXnextFallback => {
          let mut call_wrap = IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
            &mut self.regs,
            &mut *self.build,
            index,
          );
          call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::Qword,
            OperandX64::reg(r_state()),
            IrOp::default(),
          );
          call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::Qword,
            luau_reg_address(vm_reg_op(*get_op_mut(inst, 1))),
            IrOp::default(),
          );
          call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::Dword,
            OperandX64::imm(self.uint_op(*get_op_mut(inst, 0)) as i32 + 1),
            IrOp::default(),
          );
          call_wrap.call(&OperandX64::mem(
            SizeX64::Qword,
            RegisterX64::NOREG,
            1,
            r_native_context(),
            (core::mem::offset_of!(NativeContext, forg_prep_xnext_fallback) as i32),
          ));
          let target_block = self.block_op(*get_op_mut(inst, 2));
          self.jump_or_fallthrough(&mut *target_block, next);
        }
        IrCmd::COVERAGE => {
          {
            let mut tmp1 = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp1.alloc(SizeX64::Qword);
            let mut tmp2 = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp2.alloc(SizeX64::Dword);
            let mut tmp3 = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp3.alloc(SizeX64::Dword);

            (*self.build).mov(OperandX64::reg(tmp1.reg), S_CODE);
            (*self.build).add(
              OperandX64::reg(tmp1.reg),
              OperandX64::imm(
                (self.uint_op(*get_op_mut(inst, 0)) as i32) * (size_of::<Instruction>() as i32),
              ),
            );

            // hits = LUAU_INSN_E(*pc)
            (*self.build).mov(
              OperandX64::reg(tmp2.reg),
              OperandX64::mem(SizeX64::Dword, RegisterX64::NOREG, 1, tmp1.reg, 0),
            );
            (*self.build).sar(OperandX64::reg(tmp2.reg), OperandX64::imm(8_i32));

            // hits = if (hits < (1 << 23) - 1) { hits + 1 } else { hits };
            (*self.build).xor_(OperandX64::reg(tmp3.reg), OperandX64::reg(tmp3.reg));
            (*self.build).cmp(OperandX64::reg(tmp2.reg), OperandX64::imm((1 << 23) - 1));
            (*self.build).setcc(ConditionX64::NotEqual, OperandX64::reg(byte_reg(tmp3.reg)));
            (*self.build).add(OperandX64::reg(tmp2.reg), OperandX64::reg(tmp3.reg));

            // vm_patch_e(pc, hits);
            (*self.build).sal(OperandX64::reg(tmp2.reg), OperandX64::imm(8_i32));
            (*self.build).movzx(
              tmp3.reg,
              OperandX64::mem(SizeX64::Byte, RegisterX64::NOREG, 1, tmp1.reg, 0),
            );
            (*self.build).or_(OperandX64::reg(tmp3.reg), OperandX64::reg(tmp2.reg));
            (*self.build).mov(
              OperandX64::mem(SizeX64::Dword, RegisterX64::NOREG, 1, tmp1.reg, 0),
              OperandX64::reg(tmp3.reg),
            );
          }

          // Full instruction fallbacks
        }
        IrCmd::FallbackGetglobal => {
          CODEGEN_ASSERT!((*get_op_mut(inst, 1_u32)).kind() == IrOpKind::VmReg);
          CODEGEN_ASSERT!((*get_op_mut(inst, 2_u32)).kind() == IrOpKind::VmConst);

          let pcpos = self.uint_op(*get_op_mut(inst, 0)) as i32;
          emit_fallback(
            &mut self.regs,
            &mut *self.build,
            core::mem::offset_of!(NativeContext, execute_getglobal) as i32,
            pcpos,
          );
        }
        IrCmd::FallbackSetglobal => {
          CODEGEN_ASSERT!((*get_op_mut(inst, 1_u32)).kind() == IrOpKind::VmReg);
          CODEGEN_ASSERT!((*get_op_mut(inst, 2_u32)).kind() == IrOpKind::VmConst);

          let pcpos = self.uint_op(*get_op_mut(inst, 0)) as i32;
          emit_fallback(
            &mut self.regs,
            &mut *self.build,
            core::mem::offset_of!(NativeContext, execute_setglobal) as i32,
            pcpos,
          );
        }
        IrCmd::FallbackGettableks => {
          CODEGEN_ASSERT!((*get_op_mut(inst, 1_u32)).kind() == IrOpKind::VmReg);
          CODEGEN_ASSERT!((*get_op_mut(inst, 2_u32)).kind() == IrOpKind::VmReg);
          CODEGEN_ASSERT!((*get_op_mut(inst, 3_u32)).kind() == IrOpKind::VmConst);

          let pcpos = self.uint_op(*get_op_mut(inst, 0)) as i32;
          emit_fallback(
            &mut self.regs,
            &mut *self.build,
            core::mem::offset_of!(NativeContext, execute_gettableks) as i32,
            pcpos,
          );
        }
        IrCmd::FallbackSettableks => {
          CODEGEN_ASSERT!((*get_op_mut(inst, 1_u32)).kind() == IrOpKind::VmReg);
          CODEGEN_ASSERT!((*get_op_mut(inst, 2_u32)).kind() == IrOpKind::VmReg);
          CODEGEN_ASSERT!((*get_op_mut(inst, 3_u32)).kind() == IrOpKind::VmConst);

          let pcpos = self.uint_op(*get_op_mut(inst, 0)) as i32;
          emit_fallback(
            &mut self.regs,
            &mut *self.build,
            core::mem::offset_of!(NativeContext, execute_settableks) as i32,
            pcpos,
          );
        }
        IrCmd::FallbackNamecall => {
          CODEGEN_ASSERT!((*get_op_mut(inst, 1_u32)).kind() == IrOpKind::VmReg);
          CODEGEN_ASSERT!((*get_op_mut(inst, 2_u32)).kind() == IrOpKind::VmReg);
          CODEGEN_ASSERT!((*get_op_mut(inst, 3_u32)).kind() == IrOpKind::VmConst);

          let pcpos = self.uint_op(*get_op_mut(inst, 0)) as i32;
          emit_fallback(
            &mut self.regs,
            &mut *self.build,
            core::mem::offset_of!(NativeContext, execute_namecall) as i32,
            pcpos,
          );
        }
        IrCmd::FallbackPrepvarargs => {
          CODEGEN_ASSERT!((*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Constant);

          let pcpos = self.uint_op(*get_op_mut(inst, 0)) as i32;
          emit_fallback(
            &mut self.regs,
            &mut *self.build,
            core::mem::offset_of!(NativeContext, execute_prepvarargs) as i32,
            pcpos,
          );
        }
        IrCmd::FallbackGetvarargs => {
          CODEGEN_ASSERT!((*get_op_mut(inst, 1_u32)).kind() == IrOpKind::VmReg);
          CODEGEN_ASSERT!((*get_op_mut(inst, 2_u32)).kind() == IrOpKind::Constant);

          if self.int_op(*get_op_mut(inst, 2)) == LUA_MULTRET {
            let mut call_wrap = IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
              &mut self.regs,
              &mut *self.build,
              index,
            );
            call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
              SizeX64::Qword,
              OperandX64::reg(r_state()),
              IrOp::default(),
            );

            let reg = call_wrap.suggest_next_argument_register(SizeX64::Qword);
            (*self.build).mov(OperandX64::reg(reg), S_CODE);
            call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
              SizeX64::Qword,
              OperandX64::mem(
                SizeX64::Qword,
                RegisterX64::NOREG,
                1,
                reg,
                (self.uint_op(*get_op_mut(inst, 0)) as i32) * (size_of::<Instruction>() as i32),
              ),
              IrOp::default(),
            );

            call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
              SizeX64::Qword,
              OperandX64::reg(r_base()),
              IrOp::default(),
            );
            call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
              SizeX64::Dword,
              OperandX64::imm(vm_reg_op(*get_op_mut(inst, 1))),
              IrOp::default(),
            );
            call_wrap.call(&OperandX64::mem(
              SizeX64::Qword,
              RegisterX64::NOREG,
              1,
              r_native_context(),
              (core::mem::offset_of!(NativeContext, execute_getvarargsmult_ret) as i32),
            ));

            emit_update_base(&mut *self.build);
          } else {
            let mut call_wrap = IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
              &mut self.regs,
              &mut *self.build,
              index,
            );
            call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
              SizeX64::Qword,
              OperandX64::reg(r_state()),
              IrOp::default(),
            );
            call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
              SizeX64::Qword,
              OperandX64::reg(r_base()),
              IrOp::default(),
            );
            call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
              SizeX64::Dword,
              OperandX64::imm(vm_reg_op(*get_op_mut(inst, 1))),
              IrOp::default(),
            );
            call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
              SizeX64::Dword,
              OperandX64::imm(self.int_op(*get_op_mut(inst, 2))),
              IrOp::default(),
            );
            call_wrap.call(&OperandX64::mem(
              SizeX64::Qword,
              RegisterX64::NOREG,
              1,
              r_native_context(),
              (core::mem::offset_of!(NativeContext, execute_getvarargsconst) as i32),
            ));
          }
        }
        IrCmd::NEWCLOSURE => {
          let mut tmp2 = ScopedRegX64 {
            owner: &mut self.regs,
            reg: RegisterX64::NOREG,
          };
          tmp2.alloc(SizeX64::Qword);
          (*self.build).mov(OperandX64::reg(tmp2.reg), S_CLOSURE);
          (*self.build).mov(
            OperandX64::reg(tmp2.reg),
            OperandX64::mem(
              SizeX64::Qword,
              RegisterX64::NOREG,
              1,
              tmp2.reg,
              K_CLOSURE_LPOFFSET,
            ),
          );
          (*self.build).mov(
            OperandX64::reg(tmp2.reg),
            OperandX64::mem(
              SizeX64::Qword,
              RegisterX64::NOREG,
              1,
              tmp2.reg,
              (core::mem::offset_of!(Proto, p) as i32),
            ),
          );
          (*self.build).mov(
            OperandX64::reg(tmp2.reg),
            OperandX64::mem(
              SizeX64::Qword,
              RegisterX64::NOREG,
              1,
              tmp2.reg,
              (self.uint_op(*get_op_mut(inst, 2)) as i32) * (size_of::<*mut Proto>() as i32),
            ),
          );

          let mut call_wrap = IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
            &mut self.regs,
            &mut *self.build,
            index,
          );
          call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::Qword,
            OperandX64::reg(r_state()),
            IrOp::default(),
          );
          call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::Dword,
            OperandX64::imm(self.uint_op(*get_op_mut(inst, 0)) as i32),
            *get_op_mut(inst, 0_u32),
          );
          call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::Qword,
            OperandX64::reg(self.reg_op(*get_op_mut(inst, 1))),
            *get_op_mut(inst, 1_u32),
          );
          call_wrap.add_argument_size_x_64_scoped_reg_x_64(SizeX64::Qword, &mut tmp2);

          call_wrap.call(&OperandX64::mem(
            SizeX64::Qword,
            RegisterX64::NOREG,
            1,
            r_native_context(),
            (core::mem::offset_of!(NativeContext, lua_f_new_lclosure) as i32),
          ));

          inst.reg_x64 = self.regs.take_reg(RegisterX64::RAX, index);
        }
        IrCmd::FallbackDupclosure => {
          CODEGEN_ASSERT!((*get_op_mut(inst, 1_u32)).kind() == IrOpKind::VmReg);
          CODEGEN_ASSERT!((*get_op_mut(inst, 2_u32)).kind() == IrOpKind::VmConst);

          let pcpos = self.uint_op(*get_op_mut(inst, 0)) as i32;
          emit_fallback(
            &mut self.regs,
            &mut *self.build,
            core::mem::offset_of!(NativeContext, execute_dupclosure) as i32,
            pcpos,
          );
        }
        IrCmd::FallbackForgprep => {
          let pcpos = self.uint_op(*get_op_mut(inst, 0)) as i32;
          emit_fallback(
            &mut self.regs,
            &mut *self.build,
            core::mem::offset_of!(NativeContext, execute_forgprep) as i32,
            pcpos,
          );
          let target_block = self.block_op(*get_op_mut(inst, 2));
          self.jump_or_fallthrough(&mut *target_block, next);
        }
        IrCmd::BitandUint => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Dword, index, &[(*get_op_mut(inst, 0_u32))]);
          let op0 = *get_op_mut(inst, 0);

          if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
            (*self.build).mov(OperandX64::reg(inst.reg_x64), self.mem_reg_uint_op(op0));
          }

          (*self.build).and_(
            OperandX64::reg(inst.reg_x64),
            self.mem_reg_uint_op(*get_op_mut(inst, 1)),
          );
        }
        IrCmd::BitxorUint => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Dword, index, &[(*get_op_mut(inst, 0_u32))]);
          let op0 = *get_op_mut(inst, 0);

          if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
            (*self.build).mov(OperandX64::reg(inst.reg_x64), self.mem_reg_uint_op(op0));
          }

          (*self.build).xor_(
            OperandX64::reg(inst.reg_x64),
            self.mem_reg_uint_op(*get_op_mut(inst, 1)),
          );
        }
        IrCmd::BitorUint => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Dword, index, &[(*get_op_mut(inst, 0_u32))]);
          let op0 = *get_op_mut(inst, 0);

          if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
            (*self.build).mov(OperandX64::reg(inst.reg_x64), self.mem_reg_uint_op(op0));
          }

          (*self.build).or_(
            OperandX64::reg(inst.reg_x64),
            self.mem_reg_uint_op(*get_op_mut(inst, 1)),
          );
        }
        IrCmd::BitnotUint => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Dword, index, &[(*get_op_mut(inst, 0_u32))]);
          let op0 = *get_op_mut(inst, 0);

          if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
            (*self.build).mov(OperandX64::reg(inst.reg_x64), self.mem_reg_uint_op(op0));
          }

          (*self.build).not_(OperandX64::reg(inst.reg_x64));
        }
        IrCmd::BitlshiftUint => {
          {
            let mut shift_tmp = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };

            // Custom bit shift value can only be placed in RegisterX64::CL
            // but we use it if the shift value is not a constant stored in b
            if (*get_op_mut(inst, 1_u32)).kind() != IrOpKind::Constant {
              shift_tmp.take(dword_reg(RegisterX64::RCX));
            }

            inst.reg_x64 =
              self
                .regs
                .alloc_reg_or_reuse(SizeX64::Dword, index, &[(*get_op_mut(inst, 0_u32))]);
            let op0 = *get_op_mut(inst, 0);

            if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
              (*self.build).mov(OperandX64::reg(inst.reg_x64), self.mem_reg_uint_op(op0));
            }

            if (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Constant {
              // if shift value is a constant, we extract the byte-sized shift amount
              let shift = ((self.int_op(*get_op_mut(inst, 1))) as u32) as i8;
              (*self.build).shl(OperandX64::reg(inst.reg_x64), OperandX64::imm(shift as i32));
            } else {
              (*self.build).mov(
                OperandX64::reg(shift_tmp.reg),
                self.mem_reg_uint_op(*get_op_mut(inst, 1)),
              );
              (*self.build).shl(
                OperandX64::reg(inst.reg_x64),
                OperandX64::reg(byte_reg(shift_tmp.reg)),
              );
            }
          }
        }
        IrCmd::BitrshiftUint => {
          {
            let mut shift_tmp = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };

            // Custom bit shift value can only be placed in RegisterX64::CL
            // but we use it if the shift value is not a constant stored in b
            if (*get_op_mut(inst, 1_u32)).kind() != IrOpKind::Constant {
              shift_tmp.take(dword_reg(RegisterX64::RCX));
            }

            inst.reg_x64 =
              self
                .regs
                .alloc_reg_or_reuse(SizeX64::Dword, index, &[(*get_op_mut(inst, 0_u32))]);
            let op0 = *get_op_mut(inst, 0);

            if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
              (*self.build).mov(OperandX64::reg(inst.reg_x64), self.mem_reg_uint_op(op0));
            }

            if (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Constant {
              // if shift value is a constant, we extract the byte-sized shift amount
              let shift = ((self.int_op(*get_op_mut(inst, 1))) as u32) as i8;
              (*self.build).shr(OperandX64::reg(inst.reg_x64), OperandX64::imm(shift as i32));
            } else {
              (*self.build).mov(
                OperandX64::reg(shift_tmp.reg),
                self.mem_reg_uint_op(*get_op_mut(inst, 1)),
              );
              (*self.build).shr(
                OperandX64::reg(inst.reg_x64),
                OperandX64::reg(byte_reg(shift_tmp.reg)),
              );
            }
          }
        }
        IrCmd::BitarshiftUint => {
          {
            let mut shift_tmp = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };

            // Custom bit shift value can only be placed in RegisterX64::CL
            // but we use it if the shift value is not a constant stored in b
            if (*get_op_mut(inst, 1_u32)).kind() != IrOpKind::Constant {
              shift_tmp.take(dword_reg(RegisterX64::RCX));
            }

            inst.reg_x64 =
              self
                .regs
                .alloc_reg_or_reuse(SizeX64::Dword, index, &[(*get_op_mut(inst, 0_u32))]);
            let op0 = *get_op_mut(inst, 0);

            if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
              (*self.build).mov(OperandX64::reg(inst.reg_x64), self.mem_reg_uint_op(op0));
            }

            if (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Constant {
              // if shift value is a constant, we extract the byte-sized shift amount
              let shift = ((self.int_op(*get_op_mut(inst, 1))) as u32) as i8;
              (*self.build).sar(OperandX64::reg(inst.reg_x64), OperandX64::imm(shift as i32));
            } else {
              (*self.build).mov(
                OperandX64::reg(shift_tmp.reg),
                self.mem_reg_uint_op(*get_op_mut(inst, 1)),
              );
              (*self.build).sar(
                OperandX64::reg(inst.reg_x64),
                OperandX64::reg(byte_reg(shift_tmp.reg)),
              );
            }
          }
        }
        IrCmd::BitlrotateUint => {
          {
            let mut shift_tmp = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };

            // Custom bit shift value can only be placed in RegisterX64::CL
            // but we use it if the shift value is not a constant stored in b
            if (*get_op_mut(inst, 1_u32)).kind() != IrOpKind::Constant {
              shift_tmp.take(dword_reg(RegisterX64::RCX));
            }

            inst.reg_x64 =
              self
                .regs
                .alloc_reg_or_reuse(SizeX64::Dword, index, &[(*get_op_mut(inst, 0_u32))]);
            let op0 = *get_op_mut(inst, 0);

            if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
              (*self.build).mov(OperandX64::reg(inst.reg_x64), self.mem_reg_uint_op(op0));
            }

            if (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Constant {
              // if shift value is a constant, we extract the byte-sized shift amount
              let shift = ((self.int_op(*get_op_mut(inst, 1))) as u32) as i8;
              (*self.build).rol(OperandX64::reg(inst.reg_x64), OperandX64::imm(shift as i32));
            } else {
              (*self.build).mov(
                OperandX64::reg(shift_tmp.reg),
                self.mem_reg_uint_op(*get_op_mut(inst, 1)),
              );
              (*self.build).rol(
                OperandX64::reg(inst.reg_x64),
                OperandX64::reg(byte_reg(shift_tmp.reg)),
              );
            }
          }
        }
        IrCmd::BitrrotateUint => {
          {
            let mut shift_tmp = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };

            // Custom bit shift value can only be placed in RegisterX64::CL
            // but we use it if the shift value is not a constant stored in b
            if (*get_op_mut(inst, 1_u32)).kind() != IrOpKind::Constant {
              shift_tmp.take(dword_reg(RegisterX64::RCX));
            }

            inst.reg_x64 =
              self
                .regs
                .alloc_reg_or_reuse(SizeX64::Dword, index, &[(*get_op_mut(inst, 0_u32))]);
            let op0 = *get_op_mut(inst, 0);

            if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
              (*self.build).mov(OperandX64::reg(inst.reg_x64), self.mem_reg_uint_op(op0));
            }

            if (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Constant {
              // if shift value is a constant, we extract the byte-sized shift amount
              let shift = ((self.int_op(*get_op_mut(inst, 1))) as u32) as i8;
              (*self.build).ror(OperandX64::reg(inst.reg_x64), OperandX64::imm(shift as i32));
            } else {
              (*self.build).mov(
                OperandX64::reg(shift_tmp.reg),
                self.mem_reg_uint_op(*get_op_mut(inst, 1)),
              );
              (*self.build).ror(
                OperandX64::reg(inst.reg_x64),
                OperandX64::reg(byte_reg(shift_tmp.reg)),
              );
            }
          }
        }
        IrCmd::BitcountlzUint => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Dword, index, &[(*get_op_mut(inst, 0_u32))]);

          let mut zero = Label::default();
          let mut exit = Label::default();

          (*self.build).test(
            OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
            OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
          );
          (*self.build).jcc(ConditionX64::Equal, &mut zero);

          (*self.build).bsr(
            inst.reg_x64,
            OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
          );
          (*self.build).xor_(OperandX64::reg(inst.reg_x64), OperandX64::imm(0x1f));
          (*self.build).jmp_label(&mut exit);

          (*self.build).set_label(&mut zero);
          (*self.build).mov(OperandX64::reg(inst.reg_x64), OperandX64::imm(32_i32));

          (*self.build).set_label(&mut exit);
        }
        IrCmd::BitcountrzUint => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Dword, index, &[(*get_op_mut(inst, 0_u32))]);

          let mut zero = Label::default();
          let mut exit = Label::default();

          (*self.build).test(
            OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
            OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
          );
          (*self.build).jcc(ConditionX64::Equal, &mut zero);

          (*self.build).bsf(
            inst.reg_x64,
            OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
          );
          (*self.build).jmp_label(&mut exit);

          (*self.build).set_label(&mut zero);
          (*self.build).mov(OperandX64::reg(inst.reg_x64), OperandX64::imm(32_i32));

          (*self.build).set_label(&mut exit);
        }
        IrCmd::ByteswapUint => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Dword, index, &[(*get_op_mut(inst, 0_u32))]);
          let op0 = *get_op_mut(inst, 0);

          if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
            (*self.build).mov(OperandX64::reg(inst.reg_x64), self.mem_reg_uint_op(op0));
          }

          (*self.build).bswap(inst.reg_x64);
        }
        IrCmd::InvokeLibm => {
          let mut call_wrap = IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
            &mut self.regs,
            &mut *self.build,
            index,
          );
          call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::Xmmword,
            self.mem_reg_double_op(*get_op_mut(inst, 1)),
            *get_op_mut(inst, 1_u32),
          );

          if HAS_OP_C!(inst) {
            let is_int = if (*get_op_mut(inst, 2_u32)).kind() == IrOpKind::Constant {
              self.ir_lowering_x_64_const_op(*get_op_mut(inst, 2)).kind == IrConstKind::Int
            } else {
              get_cmd_value_kind((*self.function).inst_op(*get_op_mut(inst, 2)).cmd)
                == IrValueKind::Int
            };

            if is_int {
              call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
                SizeX64::Dword,
                self.mem_reg_uint_op(*get_op_mut(inst, 2)),
                *get_op_mut(inst, 2_u32),
              );
            } else {
              call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
                SizeX64::Xmmword,
                self.mem_reg_double_op(*get_op_mut(inst, 2)),
                *get_op_mut(inst, 2_u32),
              );
            }
          }

          call_wrap.call(&OperandX64::mem(
            SizeX64::Qword,
            RegisterX64::NOREG,
            1,
            r_native_context(),
            get_native_context_offset(self.uint_op(*get_op_mut(inst, 0)) as i32) as i32,
          ));
          inst.reg_x64 = self.regs.take_reg(xmm0(), index);
        }
        IrCmd::GetType => {
          inst.reg_x64 = self.regs.alloc_reg(SizeX64::Qword, index);

          (*self.build).mov(
            OperandX64::reg(inst.reg_x64),
            OperandX64::mem(
              SizeX64::Qword,
              RegisterX64::NOREG,
              1,
              r_state(),
              (core::mem::offset_of!(lua_State, global) as i32),
            ),
          );

          if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Inst {
            (*self.build).mov(
              OperandX64::reg(inst.reg_x64),
              OperandX64::mem(
                SizeX64::Qword,
                qword_reg(self.reg_op(*get_op_mut(inst, 0))),
                size_of::<*mut tstring>() as u8,
                inst.reg_x64,
                (core::mem::offset_of!(global_State, ttname) as i32),
              ),
            );
          } else if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Constant {
            (*self.build).mov(
              OperandX64::reg(inst.reg_x64),
              OperandX64::mem(
                SizeX64::Qword,
                RegisterX64::NOREG,
                1,
                inst.reg_x64,
                (core::mem::offset_of!(global_State, ttname) as i32)
                  + (self.tag_op(*get_op_mut(inst, 0)) as i32) * (size_of::<*mut tstring>() as i32),
              ),
            );
          } else {
            CODEGEN_ASSERT!(false, "Unsupported instruction form");
          }
        }
        IrCmd::GetTypeof => {
          let mut call_wrap = IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
            &mut self.regs,
            &mut *self.build,
            index,
          );
          call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::Qword,
            OperandX64::reg(r_state()),
            IrOp::default(),
          );
          call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::Qword,
            luau_reg_address(vm_reg_op(*get_op_mut(inst, 0))),
            IrOp::default(),
          );
          call_wrap.call(&OperandX64::mem(
            SizeX64::Qword,
            RegisterX64::NOREG,
            1,
            r_native_context(),
            (core::mem::offset_of!(NativeContext, lua_t_objtypenamestr) as i32),
          ));

          inst.reg_x64 = self.regs.take_reg(RegisterX64::RAX, index);
        }
        IrCmd::FINDUPVAL => {
          let mut call_wrap = IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
            &mut self.regs,
            &mut *self.build,
            index,
          );
          call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::Qword,
            OperandX64::reg(r_state()),
            IrOp::default(),
          );
          call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::Qword,
            luau_reg_address(vm_reg_op(*get_op_mut(inst, 0))),
            IrOp::default(),
          );
          call_wrap.call(&OperandX64::mem(
            SizeX64::Qword,
            RegisterX64::NOREG,
            1,
            r_native_context(),
            (core::mem::offset_of!(NativeContext, lua_f_findupval) as i32),
          ));

          inst.reg_x64 = self.regs.take_reg(RegisterX64::RAX, index);
        }
        IrCmd::BufferReadi8 => {
          inst.reg_x64 = self.regs.alloc_reg_or_reuse(
            SizeX64::Dword,
            index,
            &[(*get_op_mut(inst, 0_u32)), (*get_op_mut(inst, 1_u32))],
          );

          (*self.build).movsx(
            inst.reg_x64,
            sized_mem(
              self.buffer_addr_op(
                *get_op_mut(inst, 0),
                *get_op_mut(inst, 1),
                self.tag_op(*get_op_mut(inst, 2)),
              ),
              SizeX64::Byte,
            ),
          );
        }
        IrCmd::BufferReadu8 => {
          inst.reg_x64 = self.regs.alloc_reg_or_reuse(
            SizeX64::Dword,
            index,
            &[(*get_op_mut(inst, 0_u32)), (*get_op_mut(inst, 1_u32))],
          );

          (*self.build).movzx(
            inst.reg_x64,
            sized_mem(
              self.buffer_addr_op(
                *get_op_mut(inst, 0),
                *get_op_mut(inst, 1),
                self.tag_op(*get_op_mut(inst, 2)),
              ),
              SizeX64::Byte,
            ),
          );
        }
        IrCmd::BufferWritei8 => {
          let value = if (*get_op_mut(inst, 2_u32)).kind() == IrOpKind::Inst {
            OperandX64::reg(byte_reg(self.reg_op(*get_op_mut(inst, 2))))
          } else {
            OperandX64::imm(self.int_op(*get_op_mut(inst, 2)) as i8 as i32)
          };

          (*self.build).mov(
            sized_mem(
              self.buffer_addr_op(
                *get_op_mut(inst, 0),
                *get_op_mut(inst, 1),
                self.tag_op(*get_op_mut(inst, 3)),
              ),
              SizeX64::Byte,
            ),
            value,
          );
        }
        IrCmd::BufferReadi16 => {
          inst.reg_x64 = self.regs.alloc_reg_or_reuse(
            SizeX64::Dword,
            index,
            &[(*get_op_mut(inst, 0_u32)), (*get_op_mut(inst, 1_u32))],
          );

          (*self.build).movsx(
            inst.reg_x64,
            sized_mem(
              self.buffer_addr_op(
                *get_op_mut(inst, 0),
                *get_op_mut(inst, 1),
                self.tag_op(*get_op_mut(inst, 2)),
              ),
              SizeX64::Word,
            ),
          );
        }
        IrCmd::BufferReadu16 => {
          inst.reg_x64 = self.regs.alloc_reg_or_reuse(
            SizeX64::Dword,
            index,
            &[(*get_op_mut(inst, 0_u32)), (*get_op_mut(inst, 1_u32))],
          );

          (*self.build).movzx(
            inst.reg_x64,
            sized_mem(
              self.buffer_addr_op(
                *get_op_mut(inst, 0),
                *get_op_mut(inst, 1),
                self.tag_op(*get_op_mut(inst, 2)),
              ),
              SizeX64::Word,
            ),
          );
        }
        IrCmd::BufferWritei16 => {
          let value = if (*get_op_mut(inst, 2_u32)).kind() == IrOpKind::Inst {
            OperandX64::reg(word_reg(self.reg_op(*get_op_mut(inst, 2))))
          } else {
            OperandX64::imm(self.int_op(*get_op_mut(inst, 2)) as i16 as i32)
          };

          (*self.build).mov(
            sized_mem(
              self.buffer_addr_op(
                *get_op_mut(inst, 0),
                *get_op_mut(inst, 1),
                self.tag_op(*get_op_mut(inst, 3)),
              ),
              SizeX64::Word,
            ),
            value,
          );
        }
        IrCmd::BufferReadi32 => {
          inst.reg_x64 = self.regs.alloc_reg_or_reuse(
            SizeX64::Dword,
            index,
            &[(*get_op_mut(inst, 0_u32)), (*get_op_mut(inst, 1_u32))],
          );

          (*self.build).mov(
            OperandX64::reg(inst.reg_x64),
            sized_mem(
              self.buffer_addr_op(
                *get_op_mut(inst, 0),
                *get_op_mut(inst, 1),
                self.tag_op(*get_op_mut(inst, 2)),
              ),
              SizeX64::Dword,
            ),
          );
        }
        IrCmd::BufferWritei32 => {
          let value = if (*get_op_mut(inst, 2_u32)).kind() == IrOpKind::Inst {
            OperandX64::reg(self.reg_op(*get_op_mut(inst, 2)))
          } else {
            OperandX64::imm(self.int_op(*get_op_mut(inst, 2)))
          };

          (*self.build).mov(
            sized_mem(
              self.buffer_addr_op(
                *get_op_mut(inst, 0),
                *get_op_mut(inst, 1),
                self.tag_op(*get_op_mut(inst, 3)),
              ),
              SizeX64::Dword,
            ),
            value,
          );
        }
        IrCmd::BufferReadf32 => {
          inst.reg_x64 = self.regs.alloc_reg(SizeX64::Xmmword, index);

          (*self.build).vmovss_operand_x_64_operand_x_64(
            OperandX64::reg(inst.reg_x64),
            sized_mem(
              self.buffer_addr_op(
                *get_op_mut(inst, 0),
                *get_op_mut(inst, 1),
                self.tag_op(*get_op_mut(inst, 2)),
              ),
              SizeX64::Dword,
            ),
          );
        }
        IrCmd::BufferWritef32 => {
          let dst = sized_mem(
            self.buffer_addr_op(
              *get_op_mut(inst, 0),
              *get_op_mut(inst, 1),
              self.tag_op(*get_op_mut(inst, 3)),
            ),
            SizeX64::Dword,
          );
          let src = *get_op_mut(inst, 2);
          self.store_float(dst, src);
        }
        IrCmd::BufferReadf64 => {
          inst.reg_x64 = self.regs.alloc_reg(SizeX64::Xmmword, index);

          (*self.build).vmovsd_operand_x_64_operand_x_64(
            OperandX64::reg(inst.reg_x64),
            sized_mem(
              self.buffer_addr_op(
                *get_op_mut(inst, 0),
                *get_op_mut(inst, 1),
                self.tag_op(*get_op_mut(inst, 2)),
              ),
              SizeX64::Qword,
            ),
          );
        }
        IrCmd::BufferWritef64 => {
          if (*get_op_mut(inst, 2_u32)).kind() == IrOpKind::Constant {
            let mut tmp = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp.alloc(SizeX64::Xmmword);
            (*self.build).vmovsd_operand_x_64_operand_x_64(
              OperandX64::reg(tmp.reg),
              (*self.build).f64(self.double_op(*get_op_mut(inst, 2))),
            );

            (*self.build).vmovsd_operand_x_64_operand_x_64(
              sized_mem(
                self.buffer_addr_op(
                  *get_op_mut(inst, 0),
                  *get_op_mut(inst, 1),
                  self.tag_op(*get_op_mut(inst, 3)),
                ),
                SizeX64::Qword,
              ),
              OperandX64::reg(tmp.reg),
            );
          } else if (*get_op_mut(inst, 2_u32)).kind() == IrOpKind::Inst {
            (*self.build).vmovsd_operand_x_64_operand_x_64(
              sized_mem(
                self.buffer_addr_op(
                  *get_op_mut(inst, 0),
                  *get_op_mut(inst, 1),
                  self.tag_op(*get_op_mut(inst, 3)),
                ),
                SizeX64::Qword,
              ),
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 2))),
            );
          } else {
            CODEGEN_ASSERT!(false, "Unsupported instruction form");
          }
        }
        IrCmd::BufferReadi64 => {
          inst.reg_x64 = self.regs.alloc_reg(SizeX64::Qword, index);

          (*self.build).mov(
            OperandX64::reg(inst.reg_x64),
            sized_mem(
              self.buffer_addr_op(
                *get_op_mut(inst, 0),
                *get_op_mut(inst, 1),
                self.tag_op(*get_op_mut(inst, 2)),
              ),
              SizeX64::Qword,
            ),
          );
        }
        IrCmd::BufferWritei64 => {
          if (*get_op_mut(inst, 2_u32)).kind() == IrOpKind::Constant {
            let mut tmp = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp.alloc(SizeX64::Qword);
            (*self.build).mov(
              OperandX64::reg(tmp.reg),
              (*self.build).i64(self.int64_op(*get_op_mut(inst, 2))),
            );

            (*self.build).mov(
              sized_mem(
                self.buffer_addr_op(
                  *get_op_mut(inst, 0),
                  *get_op_mut(inst, 1),
                  self.tag_op(*get_op_mut(inst, 3)),
                ),
                SizeX64::Qword,
              ),
              OperandX64::reg(tmp.reg),
            );
          } else if (*get_op_mut(inst, 2_u32)).kind() == IrOpKind::Inst {
            (*self.build).mov(
              sized_mem(
                self.buffer_addr_op(
                  *get_op_mut(inst, 0),
                  *get_op_mut(inst, 1),
                  self.tag_op(*get_op_mut(inst, 3)),
                ),
                SizeX64::Qword,
              ),
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 2))),
            );
          } else {
            CODEGEN_ASSERT!(false, "Unsupported instruction form");
          }
        }
        IrCmd::CheckDivInt64 => {
          {
            let mut tmp_a = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp_a.alloc(SizeX64::Qword);
            let mut tmp_b = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp_b.alloc(SizeX64::Qword);
            (*self.build).mov(
              OperandX64::reg(tmp_a.reg),
              self.mem_reg_int_64_op(*get_op_mut(inst, 0)),
            );
            (*self.build).mov(
              OperandX64::reg(tmp_b.reg),
              self.mem_reg_int_64_op(*get_op_mut(inst, 1)),
            );

            // guard against division by zero
            (*self.build).test(OperandX64::reg(tmp_b.reg), OperandX64::reg(tmp_b.reg));
            self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
              ConditionX64::Equal,
              *get_op_mut(inst, 2),
              index,
              next,
            );

            // guard against dividend == i64::MIN && divisor == -1 (signed overflow)
            {
              let mut skip = Label::default();

              (*self.build).cmp(OperandX64::reg(tmp_b.reg), OperandX64::imm(-1_i32));
              (*self.build).jcc(ConditionX64::NotEqual, &mut skip);

              let mut tmp_min = ScopedRegX64 {
                owner: &mut self.regs,
                reg: RegisterX64::NOREG,
              };
              tmp_min.alloc(SizeX64::Qword);
              (*self.build).mov64(tmp_min.reg, i64::MIN);
              (*self.build).cmp(OperandX64::reg(tmp_a.reg), OperandX64::reg(tmp_min.reg));
              self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
                ConditionX64::Equal,
                *get_op_mut(inst, 2),
                index,
                next,
              );

              (*self.build).set_label(&mut skip);
            }
          }
        }
        IrCmd::CheckCmpInt64 => {
          let cond = condition_op(*get_op_mut(inst, 2));

          if (cond == IrCondition::Equal || cond == IrCondition::NotEqual)
            && (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Constant
            && self.int64_op(*get_op_mut(inst, 1)) == 0
          {
            (*self.build).test(
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
            );
            self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
              if cond == IrCondition::Equal {
                ConditionX64::NotZero
              } else {
                ConditionX64::Zero
              },
              *get_op_mut(inst, 3),
              index,
              next,
            );
          } else if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Constant {
            let mut tmp = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };
            tmp.alloc(SizeX64::Qword);
            (*self.build).mov(
              OperandX64::reg(tmp.reg),
              self.mem_reg_int_64_op(*get_op_mut(inst, 0)),
            );
            (*self.build).cmp(
              OperandX64::reg(tmp.reg),
              self.mem_reg_int_64_op(*get_op_mut(inst, 1)),
            );
            self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
              get_condition_int(get_negated_condition_ir_condition(cond)),
              *get_op_mut(inst, 3),
              index,
              next,
            );
          } else {
            (*self.build).cmp(
              OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
              self.mem_reg_int_64_op(*get_op_mut(inst, 1)),
            );
            self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
              get_condition_int(get_negated_condition_ir_condition(cond)),
              *get_op_mut(inst, 3),
              index,
              next,
            );
          }
        }
        IrCmd::CmpInt64 => {
          {
            // cannot reuse operand registers as a target because we have to modify it before the comparison
            inst.reg_x64 = self.regs.alloc_reg(SizeX64::Dword, index);

            // We are going to operate on byte register, those do not clear high bits on write
            (*self.build).xor_(OperandX64::reg(inst.reg_x64), OperandX64::reg(inst.reg_x64));

            let cond = condition_op(*get_op_mut(inst, 2));

            if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Constant {
              (*self.build).cmp(
                OperandX64::reg(self.reg_op(*get_op_mut(inst, 1))),
                self.mem_reg_int_64_op(*get_op_mut(inst, 0)),
              );
              (*self.build).setcc(
                get_inverse_condition(get_condition_int(cond)),
                OperandX64::reg(byte_reg(inst.reg_x64)),
              );
            } else if (*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Inst {
              (*self.build).cmp(
                OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
                self.mem_reg_int_64_op(*get_op_mut(inst, 1)),
              );
              (*self.build).setcc(
                get_condition_int(cond),
                OperandX64::reg(byte_reg(inst.reg_x64)),
              );
            } else {
              CODEGEN_ASSERT!(false, "Unsupported instruction form");
            }
          }
        }
        IrCmd::Int64ToNum => {
          inst.reg_x64 = self.regs.alloc_reg(SizeX64::Xmmword, index);

          (*self.build).vcvtsi2sd(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
          );
        }
        IrCmd::NumToInt64 => {
          inst.reg_x64 = self.regs.alloc_reg(SizeX64::Qword, index);

          (*self.build).vcvttsd2si(
            OperandX64::reg(inst.reg_x64),
            self.mem_reg_double_op(*get_op_mut(inst, 0)),
          );
        }
        IrCmd::BitandInt64 => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Qword, index, &[(*get_op_mut(inst, 0_u32))]);
          let op0 = *get_op_mut(inst, 0);

          if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
            (*self.build).mov(OperandX64::reg(inst.reg_x64), self.mem_reg_int_64_op(op0));
          }

          (*self.build).and_(
            OperandX64::reg(inst.reg_x64),
            self.mem_reg_int_64_op(*get_op_mut(inst, 1)),
          );
        }
        IrCmd::BitxorInt64 => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Qword, index, &[(*get_op_mut(inst, 0_u32))]);
          let op0 = *get_op_mut(inst, 0);

          if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
            (*self.build).mov(OperandX64::reg(inst.reg_x64), self.mem_reg_int_64_op(op0));
          }

          (*self.build).xor_(
            OperandX64::reg(inst.reg_x64),
            self.mem_reg_int_64_op(*get_op_mut(inst, 1)),
          );
        }
        IrCmd::BitorInt64 => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Qword, index, &[(*get_op_mut(inst, 0_u32))]);
          let op0 = *get_op_mut(inst, 0);

          if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
            (*self.build).mov(OperandX64::reg(inst.reg_x64), self.mem_reg_int_64_op(op0));
          }

          (*self.build).or_(
            OperandX64::reg(inst.reg_x64),
            self.mem_reg_int_64_op(*get_op_mut(inst, 1)),
          );
        }
        IrCmd::BitnotInt64 => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Qword, index, &[(*get_op_mut(inst, 0_u32))]);
          let op0 = *get_op_mut(inst, 0);

          if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
            (*self.build).mov(OperandX64::reg(inst.reg_x64), self.mem_reg_int_64_op(op0));
          }

          (*self.build).not_(OperandX64::reg(inst.reg_x64));
        }
        IrCmd::BitlshiftInt64 => {
          {
            let mut shift_tmp = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };

            if (*get_op_mut(inst, 1_u32)).kind() != IrOpKind::Constant {
              shift_tmp.take(RegisterX64::RCX);
            }

            inst.reg_x64 =
              self
                .regs
                .alloc_reg_or_reuse(SizeX64::Qword, index, &[(*get_op_mut(inst, 0_u32))]);
            let op0 = *get_op_mut(inst, 0);

            if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
              (*self.build).mov(OperandX64::reg(inst.reg_x64), self.mem_reg_int_64_op(op0));
            }

            if (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Constant {
              let shift = self.int64_op(*get_op_mut(inst, 1));

              if shift < 0 {
                // Negative left shift = right shift by -amount
                let amount = (-shift) as u8;
                if amount > 63 {
                  (*self.build).xor_(OperandX64::reg(inst.reg_x64), OperandX64::reg(inst.reg_x64));
                } else {
                  (*self.build).shr(
                    OperandX64::reg(inst.reg_x64),
                    OperandX64::imm(((amount) as i8) as i32),
                  );
                }
              } else if shift > 63 {
                (*self.build).xor_(OperandX64::reg(inst.reg_x64), OperandX64::reg(inst.reg_x64));
              } else {
                (*self.build).shl(
                  OperandX64::reg(inst.reg_x64),
                  OperandX64::imm(((shift) as i8) as i32),
                );
              }
            } else {
              let mut tmp = ScopedRegX64 {
                owner: &mut self.regs,
                reg: RegisterX64::NOREG,
              };
              tmp.alloc(SizeX64::Qword);

              let mut negative = Label::default();
              let mut out_of_range = Label::default();
              let mut done = Label::default();

              (*self.build).mov(
                OperandX64::reg(shift_tmp.reg),
                self.mem_reg_int_64_op(*get_op_mut(inst, 1)),
              );

              // Check |amount| > 63: (amount + 63) unsigned > 126
              (*self.build).lea_operand_x_64_operand_x_64(
                OperandX64::reg(tmp.reg),
                OperandX64::mem(SizeX64::None, RegisterX64::NOREG, 1, shift_tmp.reg, 63),
              );
              (*self.build).cmp(OperandX64::reg(tmp.reg), OperandX64::imm(126_i32));
              (*self.build).jcc(ConditionX64::Above, &mut out_of_range);

              // Check sign of amount
              (*self.build).test(
                OperandX64::reg(shift_tmp.reg),
                OperandX64::reg(shift_tmp.reg),
              );
              (*self.build).jcc(ConditionX64::Less, &mut negative);

              // Left shift
              (*self.build).shl(
                OperandX64::reg(inst.reg_x64),
                OperandX64::reg(byte_reg(shift_tmp.reg)),
              );
              (*self.build).jmp_label(&mut done);

              // Right shift by -amount
              (*self.build).set_label(&mut negative);
              (*self.build).neg(OperandX64::reg(shift_tmp.reg));
              (*self.build).shr(
                OperandX64::reg(inst.reg_x64),
                OperandX64::reg(byte_reg(shift_tmp.reg)),
              );
              (*self.build).jmp_label(&mut done);

              (*self.build).set_label(&mut out_of_range);
              (*self.build).xor_(OperandX64::reg(inst.reg_x64), OperandX64::reg(inst.reg_x64));

              (*self.build).set_label(&mut done);
            }
          }
        }
        IrCmd::BitrshiftInt64 => {
          {
            let mut shift_tmp = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };

            if (*get_op_mut(inst, 1_u32)).kind() != IrOpKind::Constant {
              shift_tmp.take(RegisterX64::RCX);
            }

            inst.reg_x64 =
              self
                .regs
                .alloc_reg_or_reuse(SizeX64::Qword, index, &[(*get_op_mut(inst, 0_u32))]);
            let op0 = *get_op_mut(inst, 0);

            if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
              (*self.build).mov(OperandX64::reg(inst.reg_x64), self.mem_reg_int_64_op(op0));
            }

            if (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Constant {
              let shift = self.int64_op(*get_op_mut(inst, 1));

              if shift < 0 {
                // Negative right shift = left shift by -amount
                let amount = (-shift) as u8;
                if amount > 63 {
                  (*self.build).xor_(OperandX64::reg(inst.reg_x64), OperandX64::reg(inst.reg_x64));
                } else {
                  (*self.build).shl(
                    OperandX64::reg(inst.reg_x64),
                    OperandX64::imm(((amount) as i8) as i32),
                  );
                }
              } else if shift > 63 {
                (*self.build).xor_(OperandX64::reg(inst.reg_x64), OperandX64::reg(inst.reg_x64));
              } else {
                (*self.build).shr(
                  OperandX64::reg(inst.reg_x64),
                  OperandX64::imm(((shift) as i8) as i32),
                );
              }
            } else {
              let mut tmp = ScopedRegX64 {
                owner: &mut self.regs,
                reg: RegisterX64::NOREG,
              };
              tmp.alloc(SizeX64::Qword);

              let mut negative = Label::default();
              let mut out_of_range = Label::default();
              let mut done = Label::default();

              (*self.build).mov(
                OperandX64::reg(shift_tmp.reg),
                self.mem_reg_int_64_op(*get_op_mut(inst, 1)),
              );

              // Check |amount| > 63: (amount + 63) unsigned > 126
              (*self.build).lea_operand_x_64_operand_x_64(
                OperandX64::reg(tmp.reg),
                OperandX64::mem(SizeX64::None, RegisterX64::NOREG, 1, shift_tmp.reg, 63),
              );
              (*self.build).cmp(OperandX64::reg(tmp.reg), OperandX64::imm(126_i32));
              (*self.build).jcc(ConditionX64::Above, &mut out_of_range);

              // Check sign of amount
              (*self.build).test(
                OperandX64::reg(shift_tmp.reg),
                OperandX64::reg(shift_tmp.reg),
              );
              (*self.build).jcc(ConditionX64::Less, &mut negative);

              // Unsigned right shift
              (*self.build).shr(
                OperandX64::reg(inst.reg_x64),
                OperandX64::reg(byte_reg(shift_tmp.reg)),
              );
              (*self.build).jmp_label(&mut done);

              // Left shift by -amount
              (*self.build).set_label(&mut negative);
              (*self.build).neg(OperandX64::reg(shift_tmp.reg));
              (*self.build).shl(
                OperandX64::reg(inst.reg_x64),
                OperandX64::reg(byte_reg(shift_tmp.reg)),
              );
              (*self.build).jmp_label(&mut done);

              (*self.build).set_label(&mut out_of_range);
              (*self.build).xor_(OperandX64::reg(inst.reg_x64), OperandX64::reg(inst.reg_x64));

              (*self.build).set_label(&mut done);
            }
          }
        }
        IrCmd::BitarshiftInt64 => {
          {
            let mut shift_tmp = ScopedRegX64 {
              owner: &mut self.regs,
              reg: RegisterX64::NOREG,
            };

            if (*get_op_mut(inst, 1_u32)).kind() != IrOpKind::Constant {
              shift_tmp.take(RegisterX64::RCX);
            }

            inst.reg_x64 =
              self
                .regs
                .alloc_reg_or_reuse(SizeX64::Qword, index, &[(*get_op_mut(inst, 0_u32))]);
            let op0 = *get_op_mut(inst, 0);

            if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
              (*self.build).mov(OperandX64::reg(inst.reg_x64), self.mem_reg_int_64_op(op0));
            }

            if (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Constant {
              let shift = self.int64_op(*get_op_mut(inst, 1));

              if shift < -63 {
                // Left shift by > 63 = 0
                (*self.build).xor_(OperandX64::reg(inst.reg_x64), OperandX64::reg(inst.reg_x64));
              } else if shift < 0 {
                // Negative arshift = left shift by -amount
                (*self.build).shl(
                  OperandX64::reg(inst.reg_x64),
                  OperandX64::imm(((-shift) as i8) as i32),
                );
              } else if shift > 63 {
                // Arithmetic right shift by > 63 = sign-fill
                (*self.build).sar(OperandX64::reg(inst.reg_x64), OperandX64::imm(63_i8 as i32));
              } else {
                (*self.build).sar(
                  OperandX64::reg(inst.reg_x64),
                  OperandX64::imm(((shift) as i8) as i32),
                );
              }
            } else {
              let mut tmp = ScopedRegX64 {
                owner: &mut self.regs,
                reg: RegisterX64::NOREG,
              };
              tmp.alloc(SizeX64::Qword);

              let mut negative = Label::default();
              let mut out_of_range_positive = Label::default();
              let mut out_of_range_negative = Label::default();
              let mut done = Label::default();

              (*self.build).mov(
                OperandX64::reg(shift_tmp.reg),
                self.mem_reg_int_64_op(*get_op_mut(inst, 1)),
              );

              // amount > 63: sign-fill
              (*self.build).cmp(OperandX64::reg(shift_tmp.reg), OperandX64::imm(63_i32));
              (*self.build).jcc(ConditionX64::Greater, &mut out_of_range_positive);

              // Check amount < -63: (amount + 63) < 0
              (*self.build).lea_operand_x_64_operand_x_64(
                OperandX64::reg(tmp.reg),
                OperandX64::mem(SizeX64::None, RegisterX64::NOREG, 1, shift_tmp.reg, 63),
              );
              (*self.build).test(OperandX64::reg(tmp.reg), OperandX64::reg(tmp.reg));
              (*self.build).jcc(ConditionX64::Less, &mut out_of_range_negative);

              // Check sign of amount
              (*self.build).test(
                OperandX64::reg(shift_tmp.reg),
                OperandX64::reg(shift_tmp.reg),
              );
              (*self.build).jcc(ConditionX64::Less, &mut negative);

              // Arithmetic right shift
              (*self.build).sar(
                OperandX64::reg(inst.reg_x64),
                OperandX64::reg(byte_reg(shift_tmp.reg)),
              );
              (*self.build).jmp_label(&mut done);

              // Left shift by -amount
              (*self.build).set_label(&mut negative);
              (*self.build).neg(OperandX64::reg(shift_tmp.reg));
              (*self.build).shl(
                OperandX64::reg(inst.reg_x64),
                OperandX64::reg(byte_reg(shift_tmp.reg)),
              );
              (*self.build).jmp_label(&mut done);

              // amount > 63: sign-fill ( if n < 0 { -1 } else { 0 })
              (*self.build).set_label(&mut out_of_range_positive);
              (*self.build).sar(OperandX64::reg(inst.reg_x64), OperandX64::imm(63_i8 as i32));
              (*self.build).jmp_label(&mut done);

              // amount < -63: result is 0
              (*self.build).set_label(&mut out_of_range_negative);
              (*self.build).xor_(OperandX64::reg(inst.reg_x64), OperandX64::reg(inst.reg_x64));

              (*self.build).set_label(&mut done);
            }
          }
        }
        IrCmd::BitlrotateInt64 => {
          let mut shift_tmp = ScopedRegX64 {
            owner: &mut self.regs,
            reg: RegisterX64::NOREG,
          };

          if (*get_op_mut(inst, 1_u32)).kind() != IrOpKind::Constant {
            shift_tmp.take(RegisterX64::RCX);
          }

          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Qword, index, &[(*get_op_mut(inst, 0_u32))]);
          let op0 = *get_op_mut(inst, 0);

          if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
            (*self.build).mov(OperandX64::reg(inst.reg_x64), self.mem_reg_int_64_op(op0));
          }

          if (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Constant {
            let shift = ((self.int64_op(*get_op_mut(inst, 1))) as u32) as i8;
            (*self.build).rol(OperandX64::reg(inst.reg_x64), OperandX64::imm(shift as i32));
          } else {
            (*self.build).mov(
              OperandX64::reg(shift_tmp.reg),
              self.mem_reg_int_64_op(*get_op_mut(inst, 1)),
            );
            (*self.build).rol(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(byte_reg(shift_tmp.reg)),
            );
          }
        }
        IrCmd::BitrrotateInt64 => {
          let mut shift_tmp = ScopedRegX64 {
            owner: &mut self.regs,
            reg: RegisterX64::NOREG,
          };

          if (*get_op_mut(inst, 1_u32)).kind() != IrOpKind::Constant {
            shift_tmp.take(RegisterX64::RCX);
          }

          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Qword, index, &[(*get_op_mut(inst, 0_u32))]);
          let op0 = *get_op_mut(inst, 0);

          if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
            (*self.build).mov(OperandX64::reg(inst.reg_x64), self.mem_reg_int_64_op(op0));
          }

          if (*get_op_mut(inst, 1_u32)).kind() == IrOpKind::Constant {
            let shift = ((self.int64_op(*get_op_mut(inst, 1))) as u32) as i8;
            (*self.build).ror(OperandX64::reg(inst.reg_x64), OperandX64::imm(shift as i32));
          } else {
            (*self.build).mov(
              OperandX64::reg(shift_tmp.reg),
              self.mem_reg_int_64_op(*get_op_mut(inst, 1)),
            );
            (*self.build).ror(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(byte_reg(shift_tmp.reg)),
            );
          }
        }
        IrCmd::BitcountlzInt64 => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Qword, index, &[(*get_op_mut(inst, 0_u32))]);

          let mut zero = Label::default();
          let mut exit = Label::default();

          (*self.build).test(
            OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
            OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
          );
          (*self.build).jcc(ConditionX64::Equal, &mut zero);

          (*self.build).bsr(
            inst.reg_x64,
            OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
          );
          (*self.build).xor_(OperandX64::reg(inst.reg_x64), OperandX64::imm(0x3f));
          (*self.build).jmp_label(&mut exit);

          (*self.build).set_label(&mut zero);
          (*self.build).mov(OperandX64::reg(inst.reg_x64), OperandX64::imm(64_i32));

          (*self.build).set_label(&mut exit);
        }
        IrCmd::BitcountrzInt64 => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Qword, index, &[(*get_op_mut(inst, 0_u32))]);

          let mut zero = Label::default();
          let mut exit = Label::default();

          (*self.build).test(
            OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
            OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
          );
          (*self.build).jcc(ConditionX64::Equal, &mut zero);

          (*self.build).bsf(
            inst.reg_x64,
            OperandX64::reg(self.reg_op(*get_op_mut(inst, 0))),
          );
          (*self.build).jmp_label(&mut exit);

          (*self.build).set_label(&mut zero);
          (*self.build).mov(OperandX64::reg(inst.reg_x64), OperandX64::imm(64_i32));

          (*self.build).set_label(&mut exit);
        }
        IrCmd::ByteswapInt64 => {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Qword, index, &[(*get_op_mut(inst, 0_u32))]);
          let op0 = *get_op_mut(inst, 0);

          if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
            (*self.build).mov(OperandX64::reg(inst.reg_x64), self.mem_reg_int_64_op(op0));
          }

          (*self.build).bswap(inst.reg_x64);
        }
        IrCmd::JumpCmpProtoid => {
          {
            CODEGEN_ASSERT!((*get_op_mut(inst, 0_u32)).kind() == IrOpKind::Inst);
            (*self.build).cmp(
              OperandX64::mem(
                SizeX64::Byte,
                RegisterX64::NOREG,
                1,
                self.reg_op(*get_op_mut(inst, 0)),
                (core::mem::offset_of!(Closure, is_c) as i32),
              ),
              OperandX64::imm(1_i32),
            );
            (*self.build).jcc(
              ConditionX64::Equal,
              &mut *self.label_op(*get_op_mut(inst, 3)),
            );
            {
              let mut tmp = ScopedRegX64 {
                owner: &mut self.regs,
                reg: RegisterX64::NOREG,
              };
              tmp.alloc(SizeX64::Qword);
              (*self.build).mov(
                OperandX64::reg(tmp.reg),
                OperandX64::mem(
                  SizeX64::Qword,
                  RegisterX64::NOREG,
                  1,
                  self.reg_op(*get_op_mut(inst, 0)),
                  K_CLOSURE_LPOFFSET,
                ),
              );
              (*self.build).cmp(
                OperandX64::mem(
                  SizeX64::Dword,
                  RegisterX64::NOREG,
                  1,
                  tmp.reg,
                  (core::mem::offset_of!(Proto, funid) as i32),
                ),
                OperandX64::imm((self.uint_op(*get_op_mut(inst, 1))) as i32),
              );
              (*self.build).jcc(
                ConditionX64::NotEqual,
                &mut *self.label_op(*get_op_mut(inst, 3)),
              );
            }
            let target_block = self.block_op(*get_op_mut(inst, 2));
            self.jump_or_fallthrough(&mut *target_block, next);
          }

          // Pseudo instructions
        }
        IrCmd::NOP | IrCmd::SUBSTITUTE | IrCmd::MarkUsed | IrCmd::MarkDead => {
          CODEGEN_ASSERT!(false, "Pseudo instructions should not be lowered");
        }
      }
      self.value_tracker.after_inst_lowering(inst, index);

      self.regs.curr_inst_idx = K_INVALID_INST_IDX;

      self.regs.free_last_use_regs(inst, index);
    }
  }
}
