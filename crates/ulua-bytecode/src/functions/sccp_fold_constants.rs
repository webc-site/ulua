// 待接入锚注（R69 跨 crate 审计立案）：本出口当前仅测试消费（ulua-unit-test
// fixture / 本 crate tests），生产管线（CLI/rt）尚未接线——属 [[vm-pure-rust]]
// 分阶段策略的在途面，勿按孤儿收窄；接线时连同 cpp 对应 TEST_CASE 一并启用。
use crate::records::{
  bc_function::BcFunction,
  sccp::{Sccp, SccpScratch, SccpState, VmConstOps},
};

/// cpp `foldConstants`：Sccp 入口——先播种 def→use 反向边（cpp 由 GraphParser
/// `addUse` 建图时维护），再传播常量格，最后按格值重写图。
pub fn sccp_fold_constants<I: VmConstOps + ?Sized>(func: &mut BcFunction, vm_ops: &I) {
  let mut sccp = Sccp {
    func,
    vm_ops,
    state: SccpState::new(),
    scratch: SccpScratch::default(),
  };
  sccp.seed_uses();
  sccp.propagate();
  sccp.rewrite();
}
