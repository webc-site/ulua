use crate::{records::type_checker::TypeChecker, type_aliases::module_ptr_module::ModulePtr};

/// `current_module` 非空不变式的单点表述（原 40 余处逐字重复的 expect 文案
/// 收敛于此）：cpp `TypeChecker::currentModule` 是直接持有的模块引用、不存在
/// 空态；Rust 以 `Option<ModulePtr>` 建模，`check_without_recursion_check`
/// 入口置入 `Some`、末尾才 `take()`，整个 check 调用树内恒为 `Some`。
pub(crate) const CURRENT_MODULE_INVARIANT: &str = "current_module 由 check_without_recursion_check 入口置入 Some、末尾才 take()，check 调用树内恒为 Some";

impl TypeChecker {
  /// `current_module` 的非空读取收口：等价 cpp 直接解引用 `currentModule`。
  pub fn expect_current_module(&self) -> &ModulePtr {
    self
      .current_module
      .as_ref()
      .expect(CURRENT_MODULE_INVARIANT)
  }
}
