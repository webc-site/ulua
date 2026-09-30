use core::ptr::eq;

use crate::records::{scope::Scope, scope_registry::resolve_scope};

/// 沿 `right` 的 parent 句柄链上溯，判断链上是否命中 `left`（cpp
/// `subsumesStrict(NotNull<Scope> left, NotNull<Scope> right)` 的严格包含版）。
///
/// 引用化说明（原 `# Safety` 前提改由类型承担）：`left`/`right` 均为 scope 树
/// 内非空存活节点（cpp NotNull），此处仅沿 `parent` 句柄还原注册表引用并比较地址。
pub fn subsumes_strict(left: &Scope, right: &Scope) -> bool {
  let mut current = Some(right);
  while let Some(scope) = current {
    let parent = scope.parent.and_then(resolve_scope);
    if let Some(parent) = parent
      && eq(parent, left)
    {
      return true;
    }
    current = parent;
  }

  false
}
