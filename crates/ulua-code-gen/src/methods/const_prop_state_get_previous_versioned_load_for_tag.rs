use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::ir_cmd::IrCmd,
  functions::{is_gco::is_gco, vm_reg_op::vm_reg_op},
  records::{const_prop_state::ConstPropState, ir_op::IrOp},
};

impl ConstPropState {
  pub fn get_previous_versioned_load_for_tag(&mut self, tag: u8, vm_reg: IrOp) -> (IrCmd, u32) {
    if !self.function.is_null() {
      let cfg = unsafe { &(*self.function).cfg };
      let reg_index = vm_reg_op(vm_reg) as usize;
      let reg_bit = reg_index % 64;
      let reg_array_index = reg_index / 64;
      if (cfg.captured.regs[reg_array_index] & (1u64 << reg_bit)) == 0 {
        if tag == LuaType::Boolean as u8 {
          if let Some(prev_idx) = self.get_previous_versioned_load_index(IrCmd::LoadInt, vm_reg) {
            return (IrCmd::LoadInt, unsafe { *prev_idx });
          }
        } else if tag == LuaType::Number as u8 {
          if let Some(prev_idx) = self.get_previous_versioned_load_index(IrCmd::LoadDouble, vm_reg)
          {
            return (IrCmd::LoadDouble, unsafe { *prev_idx });
          }
        } else if tag == LuaType::Integer as u8 {
          if let Some(prev_idx) = self.get_previous_versioned_load_index(IrCmd::LoadInt64, vm_reg) {
            return (IrCmd::LoadInt64, unsafe { *prev_idx });
          }
        } else if tag == LuaType::Vector as u8 {
          if let Some(prev_idx) = self.get_previous_versioned_load_index(IrCmd::LoadFloat, vm_reg) {
            return (IrCmd::LoadFloat, unsafe { *prev_idx });
          }
        } else if is_gco(tag)
          && let Some(prev_idx) = self.get_previous_versioned_load_index(IrCmd::LoadPointer, vm_reg)
        {
          return (IrCmd::LoadPointer, unsafe { *prev_idx });
        }
      }
    }

    (IrCmd::NOP, !0u32)
  }
}
