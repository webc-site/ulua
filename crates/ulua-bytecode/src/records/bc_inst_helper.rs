use crate::records::{bc_function::BcFunction, bc_op::BcOp};

/// cpp `BcInstHelper`（`BytecodeOps.h:44-75`）。
///
/// 与 cpp 的 `BcInst*` 成员对应，这里只保存指令的 `BcOp` 下标 + 对图的**唯一**
/// 可变借用：可变访问一律经 `self.graph`（`BcFunction::inst_op` / 本类型的
/// `operator_deref_mut`）现取现用，不再从 `&Vec<T>` 伪造 `*mut T`（旧实现经
/// `BcRef::operator_arrow` 与 `graph as *mut BcFunction` 构成双 `&mut` 回环，
/// 属 Stacked Borrows UB，且 `instructions` 重分配后会悬垂）。
#[derive(Debug)]
pub struct BcInstHelper<'a> {
  pub(crate) graph: &'a mut BcFunction,
  pub(crate) inst: BcOp,
}

impl<'a> BcInstHelper<'a> {
  pub(crate) fn new(graph: &'a mut BcFunction, inst: BcOp) -> Self {
    Self { graph, inst }
  }
}
