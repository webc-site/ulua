use core::ptr::NonNull;

use crate::{
  records::{
    ast_array::AstArray, cst_node::CstNode, cst_type_instantiation::CstTypeInstantiation,
    position::Position,
  },
  rtti::CstNodeClass,
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstExprCall {
  pub base: CstNode,
  pub open_parens: Position,
  pub close_parens: Position,
  pub comma_positions: AstArray<Position>,
  /// cpp `Cst.h` 的 `CstTypeInstantiation* explicitTypes = nullptr`：可空的 CST 子节点
  /// 槽，`None` 即 cpp 的 `nullptr`。只有 `storeCstData` 打开且调用带显式类型实参时由
  /// `Parser::parseMethodCall` 写入，唯一消费方是 pretty printer（判空后取只读视图）。
  /// `Option<NonNull<T>>` 在 `#[repr(C)]` 下与 `*mut T` 逐位相同（niche 复用全零），
  /// 字段偏移与 cpp 布局契约不变。
  pub explicit_types: Option<NonNull<CstTypeInstantiation>>,
}

impl_cst_node_class!(CstExprCall);

impl CstExprCall {
  pub fn new(
    open_parens: Position,
    close_parens: Position,
    comma_positions: AstArray<Position>,
  ) -> Self {
    Self {
      base: CstNode::new(<Self as CstNodeClass>::CLASS_INDEX),
      open_parens,
      close_parens,
      comma_positions,
      // 显式类型实参槽留空：cpp 的 `= nullptr` 默认成员初始化，随后仅 parseMethodCall 回填。
      explicit_types: None,
    }
  }
}
