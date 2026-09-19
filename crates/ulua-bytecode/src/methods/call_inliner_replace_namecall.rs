use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::{
  bc_function::VmConst,
  bc_get_table_ks::BcGetTableKS,
  bc_inst_helper::BcInstHelper,
  bc_move::BcMove,
  bc_op::BcOp,
  call_inliner::CallInliner,
};

impl<'a> CallInliner<'a> {
  /// cpp `replaceNamecall(BcNamecall&, BcRef<BcBlock>& prevBlock)`：
  /// 把 `NAMECALL` 摘出 `prevBlock` 挪到调用块，原位换成 `MOVE + GETTABLEKS`。
  ///
  /// `prev_block_op` 为块句柄（cpp 传 `BcRef&` 只为写 `ops.pop_back()`）。
  pub fn replace_namecall(&mut self, namecall: BcOp, prev_block_op: BcOp) -> BcOp {
    let call_block = self.call_block_op();

    // NAMECALL 视图仅在此块作用内存活，出来后 `self.caller` 重新可用
    let (table, hint, key, out_reg) = {
      let mut helper = BcInstHelper::new(self.caller, namecall);
      let table = helper.get_bc_op(0);
      let hint_op = helper.get_bc_op(1);
      let key = helper.get_bc_op(2).index;
      // SAFETY：NAMECALL 的 hint 输入在图构建期只可能以 `BcImmKind::Int` 写入。
      let hint = unsafe { helper.graph.imm_op(hint_op).value.value_int } as u32;
      let out_reg = helper.get_out_reg();
      helper.prepend_to(call_block);
      (table, hint, key, out_reg)
    };

    self.caller.block_op(prev_block_op).ops.pop_back();
    LUAU_ASSERT!(self.target_reg == out_reg);
    let table_reg = out_reg + 1;

    // and replace it with LOP_MOVE + LOP_GETTABLEKS
    let mut move_helper = BcMove::<VmConst>::create(self.caller);
    move_helper.set_out_reg(table_reg);
    move_helper.set_src(table);
    move_helper.append_to(prev_block_op);
    let move_op = move_helper.op();

    let mut get_table_ks_helper = BcGetTableKS::<VmConst>::create(self.caller);
    get_table_ks_helper.set_source(move_op);
    get_table_ks_helper.set_hint(hint);
    get_table_ks_helper.set_key(key);
    get_table_ks_helper.set_out_reg(self.target_reg);
    get_table_ks_helper.append_to(prev_block_op);

    get_table_ks_helper.op()
  }
}
