//! Source: `tests/Fixture.h:76-86`、`tests/Fixture.cpp:141-150`

use alloc::rc::{Rc, Weak};

use crate::records::source_table::SourceTable;

/// cpp `TestRequireSuggester`：只持一个 `TestFileResolver*`，`getNode` 里读
/// `&resolver->source`。Rust 侧以 `Weak<SourceTable>` 表达这条"回指宿主"的弱关联：
/// resolver 拥有唯一强引用，suggester 被调用时 `upgrade` 失败即表示宿主已不存在，
/// 无从给出候选（cpp 里这种情况是悬垂指针解引用）。补全在单线程测试内运行，
/// 故用 `Rc`/`Weak` 而非 `Arc`。
///
/// trait 实现见 [`crate::methods::test_require_suggester_get_node`]（`get_node` /
/// `get_children` / `resolve_path_to_node` 三个查询）。
#[derive(Debug, Clone)]
pub struct TestRequireSuggester {
  /// 宿主 source 表的弱回指：`upgrade` 失败即宿主已析构，无从给出候选。
  /// 字段直接暴露给本 crate 的三个查询方法（review.md §7「别套 getter」）。
  pub(crate) sources: Weak<SourceTable>,
}

impl TestRequireSuggester {
  /// cpp `TestRequireSuggester(TestFileResolver* resolver)`：挂上宿主的 source 表。
  pub fn new(sources: &Rc<SourceTable>) -> Self {
    Self {
      sources: Rc::downgrade(sources),
    }
  }
}
