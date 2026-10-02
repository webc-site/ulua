use ulua_ast::records::{ast_stat::AstStat, ast_stat_block::AstStatBlock, location::Location};

use crate::records::arena_handle::Handle;

#[derive(Debug, Clone)]
pub struct FragmentRegion {
  pub fragment_location: Location,
  /// §2：cpp `FragmentRegion::nearestStatement`（`AstStat*`，默认 `nullptr`）句柄化：
  /// 遍历未命中 ≡ `None`，命中为解析 arena 存活节点（`Handle` 模块契约）。
  pub nearest_statement: Option<Handle<AstStat>>,
  /// §2：cpp `FragmentRegion::parentBlock`（`AstStatBlock*`，默认 `nullptr`）句柄化：
  /// `get_fragment_region*` 构造点恒以「nsf 命中块或根块」写入 `Some`，类型保留
  /// cpp 可空形态供消费链（`FragmentAutocompleteAncestryResult`/空结果早退）透传。
  pub parent_block: Option<Handle<AstStatBlock>>,
}
