use crate::records::{ast_type::AstType, ast_type_pack::AstTypePack, node_handle::Node};

#[repr(C)]
#[derive(Debug)]
pub struct AstTypePackVariadic {
  pub base: AstTypePack,
  /// cpp `AstType* variadicType`（`Ast.h:1437`）：两处构造（`Parser.cpp:3467/3479`）
  /// 均以 `parseType()` 结果接线，`...T` 文法必带元素类型，恒非空。
  pub variadic_type: Node<AstType>,
}
