use ulua_common::enums::luau_opcode::LuauOpcode;

use crate::records::{bc_function::BcFunction, bc_inst_helper::BcInstHelper};

pub trait BcInstHelperCreate {
  const OPCODE: LuauOpcode;
}

impl<'a> BcInstHelper<'a> {
  /// cpp `BcInstHelper::create<T>(graph)`：新增一条 `T::OPCODE` 指令并返回持有图的
  /// 唯一可变借用的 helper。
  ///
  /// 旧实现在此用 `graph as *mut BcFunction` 把同一个图再借两次（`inst(op)` 的
  /// `&`、`BcInstHelper::new` 的 `&mut`），是 Stacked Borrows UB；现在 helper 只存
  /// `BcOp` 下标，无需二次借用。
  pub fn create<T>(graph: &'a mut BcFunction) -> Self
  where
    T: BcInstHelperCreate,
  {
    let op = graph.add_inst();
    graph.inst_op(op).op = T::OPCODE;
    BcInstHelper::new(graph, op)
  }
}
