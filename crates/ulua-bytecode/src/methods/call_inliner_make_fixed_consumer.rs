use ulua_common::{enums::luau_opcode::LuauOpcode, macros::luau_unreachable::LUAU_UNREACHABLE};

use crate::records::{
  bc_call::BcCall, bc_call_fb::BcCallFB, bc_op::BcOp, bc_return::BcReturn, bc_set_list::BcSetList,
  call_inliner::CallInliner,
};

impl<'a> CallInliner<'a> {
  /// cpp `makeFixedConsumer(BcFunction&, BcRef<BcInst>&)`：把变参消费指令
  /// （SETLIST/RETURN/CALLFB/CALL）改写为定长形态——变参尾巴被替换掉后，
  /// 输入个数即新的固定计数。
  ///
  /// 入参由 `&mut BcRef<BcInst>` 改为 `BcOp`：`BcRef` 现在只承担只读视图，
  /// 可变访问一律经持有图的 `self.caller` 现场构造 helper。
  pub fn make_fixed_consumer(&mut self, inst_op: BcOp) {
    match self.caller.inst_op(inst_op).op {
      LuauOpcode::LOP_SETLIST => {
        let mut set_list = BcSetList::<BcOp>::from(self.caller, inst_op);
        let count = set_list.params().len() as u32;
        // cpp `makeFixedConsumer`：变参尾巴被替换成 0 个输入时，SETLIST 已无参数可写，
        // 必须整条摘掉（detach），只留下前面的 NEWTABLE。
        if count == 0 {
          set_list.base.detach();
        } else {
          set_list.set_count(count);
        }
      }
      LuauOpcode::LOP_RETURN => {
        let mut ret = BcReturn::<BcOp>::from(self.caller, inst_op);
        let count = ret.values().len() as u32;
        ret.set_return_count(count);
      }
      LuauOpcode::LOP_CALLFB => {
        let mut call_fb = BcCallFB::<BcOp>::from(self.caller, inst_op);
        let count = call_fb.params().len() as u32;
        call_fb.set_param_count(count);
      }
      LuauOpcode::LOP_CALL => {
        let mut call = BcCall::<BcOp>::from(self.caller, inst_op);
        let count = call.params().len() as u32;
        call.set_param_count(count);
      }
      _ => {
        LUAU_UNREACHABLE!();
      }
    }
  }
}
