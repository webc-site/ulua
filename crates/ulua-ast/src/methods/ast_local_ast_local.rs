use core::ptr::null_mut;

use crate::records::{
  ast_local::AstLocal, ast_name::AstName, ast_type::AstType, location::Location,
};

impl AstLocal {
  /// 无槽合成局部变量：`shadow`（父作用域）与 `annotation`（类型标注）均无目标
  /// 节点。cpp 版两枚 `nullptr` 哨兵收口到这一个具名入口（全仓唯一写入 null 的
  /// 构造点），调用方（如 Compiler 的 `__EXP` 局部）不再自写 `null_mut()`。
  /// 槽位读取统一经 `slot_opt`/`slot_ref` 契约把关（functions/optional_node.rs）。
  pub const fn synthetic(
    name: AstName,
    location: Location,
    function_depth: usize,
    loop_depth: usize,
    is_const: bool,
  ) -> Self {
    Self {
      name,
      location,
      shadow: null_mut(),
      function_depth,
      loop_depth,
      is_const,
      is_exported: false,
      annotation: null_mut(),
    }
  }

  pub fn new(
    name: AstName,
    location: Location,
    shadow: *mut AstLocal,
    function_depth: usize,
    loop_depth: usize,
    annotation: *mut AstType,
    is_const: bool,
  ) -> Self {
    Self {
      name,
      location,
      shadow,
      function_depth,
      loop_depth,
      is_const,
      is_exported: false,
      annotation,
    }
  }
}
