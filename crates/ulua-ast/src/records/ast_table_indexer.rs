use crate::{
  enums::ast_table_access::AstTableAccess,
  records::{ast_type::AstType, location::Location},
};

/// cpp `AstTableIndexer`（`Ast/include/Luau/Ast.h:1178`）。
///
/// `index_type`/`result_type` 在 cpp 无默认值，且 `Parser::parseTableIndexer` 无条件先
/// `parseType()` 两次再整体写入（`Parser.cpp:2768`/`2776`/`2779`），array-like 形态
/// `{T}` 亦由 `parseTableType` 现造 `AstTypeReference` 补齐索引（`Parser.cpp:2914`）——
/// 文法必建两个子节点。原先的 `Default`（null 哨兵）在全仓无构造点，属伪可空形态，已删除。
#[derive(Debug, Clone)]
pub struct AstTableIndexer {
  pub index_type: *mut AstType,
  pub result_type: *mut AstType,
  pub location: Location,
  pub access: AstTableAccess,
  pub access_location: Option<Location>,
}
