//! Source: `tests/Fixture.h`

use std::{cell::RefCell, rc::Rc, sync::Arc};

use hashbrown::HashMap;
use ulua_analysis::{
  enums::type_file_resolver::Type as SourceCodeType,
  records::{
    file_resolver::FileResolver, module_info::ModuleInfo, require_suggester::RequireSuggester,
    source_code::SourceCode, type_check_limits::TypeCheckLimits,
  },
  type_aliases::module_name_type::ModuleName,
};
use ulua_ast::records::ast_expr::AstExpr;

use crate::records::{source_table::SourceTable, test_require_suggester::TestRequireSuggester};

/// 测试解析器的可变状态全部经 `Rc` 共享：`Frontend` 独占持有本类型的克隆
/// （`fixture_get_frontend` 移交所有权）后，夹具句柄与 frontend 内实例读写
/// 的是同一份表，与 cpp「Frontend 存裸指针、宿主直改字段」的可见性等价。
#[derive(Debug, Default, Clone)]
pub struct TestFileResolver {
  /// cpp `TestFileResolver::source`。`Rc` 是为了让 `TestRequireSuggester` 及其
  /// 派生的 [`RequireNode`](ulua_analysis::records::require_node::RequireNode)
  /// 数据共享同一张表（cpp 用 `&resolver->source` 裸指针）。
  pub source: Rc<SourceTable>,
  /// 内部可变的共享表：见结构体注（宿主与 frontend 各持一份句柄仍可互通）。
  pub source_types: Rc<RefCell<HashMap<ModuleName, SourceCodeType>>>,
  /// 同 `source_types`。
  pub environments: Rc<RefCell<HashMap<ModuleName, String>>>,
  /// C++ `FileResolver::requireSuggester` 成员；经 trait 的
  /// `require_suggester` 读取。
  ///
  /// `Arc<dyn RequireSuggester>` 为 ulua-analysis 的 `FileResolver` trait
  /// 签名（`fn require_suggester(&self) -> Option<&Arc<dyn RequireSuggester>>`）
  /// 所定，字段类型必须与 trait 一致；trait 处保留 `dyn` 的理由是
  /// `RequireSuggester` 是宿主注入点、实现方在 ulua-analysis 之外，无法
  /// enum_dispatch 穷举（详见该 trait 注）。
  pub require_suggester: Option<Arc<dyn RequireSuggester>>,
}

impl TestFileResolver {
  /// cpp `TestFileResolver()` 里 `FileResolver(std::make_shared<TestRequireSuggester>(this))`
  /// 的等价布线：suggester 持宿主 source 表的 `Weak`，宿主析构后 `upgrade` 失败，
  /// 不会再读到已释放的表。
  pub fn enable_require_suggester(&mut self) {
    if self.require_suggester.is_none() {
      // 存储位是 `Arc<dyn RequireSuggester>`（对应 cpp `shared_ptr<RequireSuggester>`）。
      // `Box<dyn _>` 中转不可避免：suggester 经 `Weak<SourceTable>` 回指宿主、
      // 非 Send + Sync，直接 `Arc::new` 具体类型触发 clippy::arc_with_non_send_sync；
      // 而 `Arc::from(Box<_>)` / `into()` 均无法把 `Box<具体>` 与 `Arc<dyn _>`
      // 两段 unsized 转换在推导链上叠加，故先显式装箱为 trait object 再转 `Arc`。
      let suggester: Box<dyn RequireSuggester> = Box::new(TestRequireSuggester::new(&self.source));
      self.require_suggester = Some(Arc::from(suggester));
    }
  }
}

impl FileResolver for TestFileResolver {
  /// C++ `TestFileResolver::readSource`
  fn read_source(&mut self, name: &ModuleName) -> Option<SourceCode> {
    Self::read_source(self, name)
  }

  /// C++ `TestFileResolver::resolveModule`
  fn resolve_module(
    &mut self,
    context: Option<&ModuleInfo>,
    expr: &AstExpr,
    limits: &TypeCheckLimits,
  ) -> Option<ModuleInfo> {
    Self::resolve_module(self, context, expr, limits)
  }

  /// C++ `TestFileResolver::getHumanReadableModuleName`
  fn get_human_readable_module_name(&self, name: &ModuleName) -> String {
    Self::get_human_readable_module_name(self, name)
  }

  /// C++ `TestFileResolver::getEnvironmentForModule`
  fn get_environment_for_module(&self, name: &ModuleName) -> Option<String> {
    Self::get_environment_for_module(self, name)
  }

  fn require_suggester(&self) -> Option<&Arc<dyn RequireSuggester>> {
    self.require_suggester.as_ref()
  }

  fn enable_require_suggester(&mut self) {
    Self::enable_require_suggester(self)
  }
}
