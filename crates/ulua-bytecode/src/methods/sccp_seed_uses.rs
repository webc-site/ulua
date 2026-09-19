use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{
    bc_function::BcFunction,
    sccp::{Sccp, SccpState, VmConstOps},
  },
};

/// 播种一个函数的全部 def→use 反向边。
///
/// 拆成自由函数是为了一次性拿到 `&BcFunction` 与 `&mut SccpState` 两个**不相交**
/// 借用：成员视图只读，写侧仅落在 state，迭代器可安全跨 `record_use` 存活，
/// 不再需要旧实现「把每条 ops 拷成 Vec 绕借用检查」的热路径堆分配。
fn seed_block_uses(func: &BcFunction, state: &mut SccpState) {
  for block_idx in 0..func.blocks.len() {
    for phi_op in func.blocks[block_idx].phis.iter().copied() {
      LUAU_ASSERT!(phi_op.kind == BcOpKind::Phi);
      let phi = func.phi(phi_op);
      for phi_operand in phi.operator_deref().ops.iter().copied() {
        state.record_use(phi_operand, phi_op);
      }
    }

    for inst_op in func.blocks[block_idx].ops.iter().copied() {
      LUAU_ASSERT!(inst_op.kind == BcOpKind::Inst);
      let inst = func.inst(inst_op);
      for inst_operand in inst.operator_deref().ops.iter().copied() {
        state.record_use(inst_operand, inst_op);
      }
    }
  }
}

impl<I: VmConstOps + ?Sized> Sccp<'_, '_, I> {
  /// cpp 侧 def→use 边由 GraphParser 的 `addUse` 建图时同步写入 `BcInst::uses`/
  /// `BcPhi::uses`，内联器经 `addUse`/`eraseUse` 增量维护；Rust 侧反向边收敛于
  /// `SccpState.op_uses`，进入传播前必须先从最终图一次性播种，否则 SSA 工作表
  /// 永远为空，visit 检测到格值变化后无法重估消费者（循环回边场景会把陈旧常量
  /// 错误折叠），与 cpp 的不动点语义不等价。
  pub fn seed_uses(&mut self) {
    // 字段级拆分借用：func 只读重借用、state 独占可变，二者不相交
    let Sccp { func, state, .. } = self;
    seed_block_uses(func, state);
  }
}
