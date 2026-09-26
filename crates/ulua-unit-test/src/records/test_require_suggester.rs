//! Source: `tests/Fixture.h:76-86`、`tests/Fixture.cpp:141-144`

use alloc::rc::{Rc, Weak};

use ulua_analysis::{
  records::{require_node::RequireNode, require_suggester::RequireSuggester},
  type_aliases::module_name_type::ModuleName,
};

use crate::records::{source_table::SourceTable, test_require_node::TestRequireNode};

/// cpp `TestRequireSuggester`：只持一个 `TestFileResolver*`，`getNode` 里读
/// `&resolver->source`。Rust 侧以 `Weak<SourceTable>` 表达这条"回指宿主"的弱关联：
/// resolver 拥有唯一强引用，suggester 被调用时 `upgrade` 失败即表示宿主已不存在，
/// 无从给出候选（cpp 里这种情况是悬垂指针解引用）。补全在单线程测试内运行，
/// 故用 `Rc`/`Weak` 而非 `Arc`。
#[derive(Debug, Clone)]
pub struct TestRequireSuggester {
  sources: Weak<SourceTable>,
}

impl TestRequireSuggester {
  /// cpp `TestRequireSuggester(TestFileResolver* resolver)`：挂上宿主的 source 表。
  pub fn new(sources: &Rc<SourceTable>) -> Self {
    Self {
      sources: Rc::downgrade(sources),
    }
  }
}

impl RequireSuggester for TestRequireSuggester {
  /// C++ `TestRequireSuggester::getNode`：给出持有全量 source 表的测试节点。
  /// 节点是 `(模块名, Rc<SourceTable>)` 的轻结构，栈上构造后以 `&dyn` 交给
  /// `visit`，省掉 cpp/Rust 两侧的一次节点装箱。
  ///
  /// `&mut dyn FnMut(&dyn RequireNode)` 为 ulua-analysis 的 `RequireSuggester`
  /// trait 签名字段类型所定（该 trait 面向运行期开放的实现方集合），测试替身
  /// 只能照签实现。
  fn with_node(&self, name: &ModuleName, visit: &mut dyn FnMut(&dyn RequireNode)) {
    let Some(sources) = self.sources.upgrade() else {
      return;
    };
    let node = TestRequireNode {
      module_name: name.clone(),
      sources,
    };
    visit(&node);
  }
}
