//! Source: `tests/Fixture.h:58-74`（`TestRequireNode`）

use alloc::rc::Rc;

use ulua_analysis::type_aliases::module_name_type::ModuleName;

use crate::records::source_table::SourceTable;

/// cpp `TestRequireNode`：`moduleName` + 指向 `TestFileResolver::source` 的裸指针。
/// Rust 侧以 `Rc<SourceTable>` 共享同一张表（见 [`crate::records::source_table`]），
/// 子节点与父节点持有同一个 `Rc`，故不再有悬垂指针。
#[derive(Debug, Clone)]
pub struct TestRequireNode {
  pub module_name: ModuleName,
  pub sources: Rc<SourceTable>,
}
