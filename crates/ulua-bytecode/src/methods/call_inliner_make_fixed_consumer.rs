use ulua_common::{enums::luau_opcode::LuauOpcode, macros::luau_unreachable::LUAU_UNREACHABLE};

use crate::records::{
  bc_call::BcCall, bc_call_fb::BcCallFB, bc_inst::BcInst, bc_op::BcOp, bc_ref::BcRef,
  bc_return::BcReturn, bc_set_list::BcSetList, call_inliner::CallInliner,
};

impl<'a> CallInliner<'a> {
  pub fn make_fixed_consumer(&mut self, inst: &mut BcRef<'a, BcInst>) {
    match inst.operator_deref().op {
      LuauOpcode::LOP_SETLIST => {
        let mut set_list = unsafe { BcSetList::<BcOp>::from(self.caller, *inst) };
        let count = set_list.params().len() as u32;
        set_list.set_count(count);
      }
      LuauOpcode::LOP_RETURN => {
        let mut ret = unsafe { BcReturn::<BcOp>::from(self.caller, *inst) };
        let count = ret.values().len() as u32;
        ret.set_return_count(count);
      }
      LuauOpcode::LOP_CALLFB => {
        let mut call_fb = unsafe { BcCallFB::<BcOp>::from(self.caller, *inst) };
        let count = call_fb.params().len() as u32;
        call_fb.set_param_count(count);
      }
      LuauOpcode::LOP_CALL => {
        let mut call = unsafe { BcCall::<BcOp>::from(self.caller, *inst) };
        let count = call.params().len() as u32;
        call.set_param_count(count);
      }
      _ => {
        LUAU_UNREACHABLE!();
      }
    }
  }
}
