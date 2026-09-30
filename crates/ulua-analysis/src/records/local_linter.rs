/// C++ `LintLocalHygiene::Local` (`Analysis/src/Linter.cpp:716`).
///
/// ```cpp
/// struct Local
/// {
///     AstNode* defined = nullptr;
///     bool function;
///     bool import;
///     bool used;
///     bool arg;
/// };
/// ```
use core::ptr::NonNull;

use ulua_ast::records::ast_node::AstNode;
#[derive(Debug, Clone, Default)]
pub struct Local {
  /// §2：cpp `AstNode* defined = nullptr`（可选依赖，null 即「未记录定义点」）
  /// 收口为 `Option<NonNull<AstNode>>`。目标由 parser arena 保活、本结构从不
  /// 解引用它——全仓仅两处判空读取（`.is_some()`）与一处指针同一性比较
  /// （遮蔽检测 `==`），故 `NonNull` 编码的非空 + `Option` 编码的缺席即可，
  /// 无裸指针回渗；`Default` 随之派生（`None` 即 cpp 的 `nullptr` 初值）。
  pub(crate) defined: Option<NonNull<AstNode>>,
  pub(crate) function: bool,
  pub(crate) import: bool,
  pub(crate) used: bool,
  pub(crate) arg: bool,
}
