// （b）镜像定形：本件内 cpp 同形访问器/类型全仓零消费，为免降级触发 dead_code 升级而保持 pub；
// 非降级对象，勿删（批次账 b28-bc-rt-tail）。
use std::vec::Vec;

use crate::{macros::bc_inst_view::bc_inst_view, records::bc_op::BcOp};

bc_inst_view!(pub BcSetList = LOP_SETLIST, from);

impl BcSetList<'_, '_> {
  /// cpp `BcSetList::kParamStartInput`（BytecodeOps.h:322）：`ops[0]=startIndex`、
  /// `ops[1]=count`、`ops[2]=target`，参数从 `ops[3]` 起。
  pub(crate) const K_PARAM_START_INPUT: u32 = 3;

  pub fn count(&mut self) -> i32 {
    self.base.int_imm_input(1)
  }

  pub(crate) fn set_count(&mut self, value: u32) {
    self.base.set_imm_input(1, value as i32);
  }

  pub(crate) fn params(&self) -> Vec<BcOp> {
    self.base.slice_inputs(Self::K_PARAM_START_INPUT)
  }
}
