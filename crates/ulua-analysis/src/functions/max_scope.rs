use crate::{
  functions::subsumes_scope::subsumes,
  records::{arena_handle::alias_opt, scope::Scope},
};

/// `inline Scope* max(Scope* left, Scope* right)` (Scope.h:126-132).
/// Returns the inner (more-specific) of two scopes.
/// 形参/返回保持可空裸指针：两侧均来自 `TableType::scope` 等可空字段，结果
/// 原样回存字段（字段布局不改）；判定本身经 `subsumes` 的 `Option<&Scope>` 口。
pub fn max(left: *mut Scope, right: *mut Scope) -> *mut Scope {
  if subsumes(alias_opt(left), alias_opt(right)) {
    right
  } else {
    left
  }
}
