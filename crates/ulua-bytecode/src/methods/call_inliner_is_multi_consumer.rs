use ulua_common::enums::luau_opcode::LuauOpcode;

use crate::records::{
  bc_call::BcCall,
  bc_call_fb::BcCallFB,
  bc_function::{BcFunction, VmConst},
  bc_inst::BcInst,
  bc_ref::BcRef,
  bc_return::BcReturn,
  bc_set_list::BcSetList,
  call_inliner::CallInliner,
};

impl<'a> CallInliner<'a> {
  pub fn is_multi_consumer(&self, inst: &BcRef<'a, BcInst>) -> bool {
    let op = inst.operator_deref().op;
    let caller = self.caller as *const BcFunction as *mut BcFunction;
    match op {
      LuauOpcode::LOP_SETLIST => unsafe { BcSetList::<VmConst>::from(caller, *inst).count() < 0 },
      LuauOpcode::LOP_RETURN => unsafe {
        BcReturn::<VmConst>::from(caller, *inst).return_count() < 0
      },
      LuauOpcode::LOP_CALLFB => unsafe {
        BcCallFB::<VmConst>::from(caller, *inst).param_count() < 0
      },
      LuauOpcode::LOP_CALL => unsafe { BcCall::<VmConst>::from(caller, *inst).param_count() < 0 },
      _ => false,
    }
  }
}
