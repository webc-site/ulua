use crate::records::{ast_type::AstType, name::Name, position::Position};

/// cpp `Parser::Binding`（`Ast/include/Luau/Parser.h:57`）：形参/局部绑定。
///
/// `annotation` 可空是 cpp 语义（`parseOptionalType()` 在无 `:` 时返回 `nullptr`，
/// `Parser.cpp:2424`），表示「未标注」。原先的 `Default` 用 null 哨兵 + 零跨度伪造
/// 「空绑定」，全仓无调用点（构造一律走 `Binding::new`），故删除——空槽不再需要一个
/// 由哨兵指针表达的第三种状态。
#[derive(Debug, Clone)]
pub struct Binding {
  pub name: Name,
  pub annotation: *mut AstType,
  pub colon_position: Position,
  pub is_const: bool,
}
