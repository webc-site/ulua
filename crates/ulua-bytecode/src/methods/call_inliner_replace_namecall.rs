use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{
    bc_block::BcBlock,
    bc_function::{BcFunction, VmConst},
    bc_get_table_ks::BcGetTableKS,
    bc_inst_helper::BcInstHelper,
    bc_move::BcMove,
    bc_op::BcOp,
    bc_ref::BcRef,
    call_inliner::CallInliner,
  },
  type_aliases::reg::Reg,
};

impl<'a> CallInliner<'a> {
  pub fn replace_namecall(&mut self, namecall: BcOp, prev_block: &mut BcRef<'a, BcBlock>) -> BcOp {
    // move LOP_NAMECALL to call block
    let caller_ptr: *mut BcFunction = self.caller;
    let inst_ref = unsafe { (&*caller_ptr).inst(namecall) };
    let mut helper = BcInstHelper {
      graph: unsafe { &mut *caller_ptr },
      inst: inst_ref,
    };
    let table = helper.get_bc_op(0);
    let hint_op = helper.get_bc_op(1);
    let hint = unsafe { helper.graph.imm_op(hint_op).value.value_int as u32 };
    let key = helper.get_bc_op(2).index;
    helper.prepend_to(self.call.base.operator_deref().block);
    prev_block.operator_deref_mut().ops.pop_back();
    LUAU_ASSERT!(self.target_reg == helper.get_out_reg());
    let table_reg: Reg = helper.get_out_reg() + 1;

    // and replace it with LOP_MOVE + LOP_GETTABLEKS
    let mut move_helper = BcMove::<VmConst>::create(self.caller);
    move_helper.set_out_reg(table_reg);
    move_helper.set_src(table);
    move_helper.append_to(prev_block.op);
    let move_op = move_helper.op();

    let mut get_table_ks_helper = BcGetTableKS::<VmConst>::create(self.caller);
    get_table_ks_helper.set_source(move_op);
    get_table_ks_helper.set_hint(hint);
    get_table_ks_helper.set_key(key);
    get_table_ks_helper.set_out_reg(self.target_reg);
    get_table_ks_helper.append_to(prev_block.op);

    get_table_ks_helper.op()
  }
}
