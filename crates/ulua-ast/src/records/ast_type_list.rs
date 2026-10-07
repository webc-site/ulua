use core::ptr::NonNull;

use crate::{
  functions::optional_node::{node_opt, opt_node},
  records::{ast_array::AstArray, ast_type::AstType, ast_type_pack::AstTypePack},
};

/// cpp `AstTypeList`（`Ast/include/Luau/Ast.h:130`）：类型序列 + 可选尾注。
///
/// `tail_type` 的「空」是 cpp 明示的合法状态——头注释 `Null indicates no tail, not an
/// untyped tail.`（`Ast.h:133`），且默认成员初始化就是 `= nullptr`；`parseTypeList` 在
/// 列表不以 `...` 收尾时同样返回 `nullptr`（`Parser.cpp:2558`）。字段本体保留裸指针
/// （消费点散布 `ulua-analysis`），构造一律走 [`Self::new`]，`nullptr` 由
/// [`opt_node`] 单点写出。
#[derive(Debug, Clone, Copy)]
pub struct AstTypeList {
  pub types: AstArray<*mut AstType>,
  pub tail_type: *mut AstTypePack,
}

impl AstTypeList {
  /// `None` 即 cpp 的 `AstTypeList{types, nullptr}`（无尾注，非「无类型尾注」）。
  #[inline]
  pub fn new(types: AstArray<*mut AstType>, tail_type: Option<NonNull<AstTypePack>>) -> Self {
    Self {
      types,
      tail_type: opt_node(tail_type),
    }
  }

  /// 尾注槽的 `Option` 读法，替代调用点手写 `tail_type.is_null()`。
  #[inline]
  pub fn tail(&self) -> Option<NonNull<AstTypePack>> {
    node_opt(self.tail_type)
  }
}
