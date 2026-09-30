use core::ptr::eq;

use crate::{functions::subsumes_strict::subsumes_strict, records::scope::Scope};

/// cpp `subsumes(Scope* left, Scope* right)`（Scope.h:121-124）：nullptr 视为最外层
/// 可能作用域，可空以 `Option<&Scope>` 表达（非空侧经引用/`alias_opt` 构造点保证）。
pub fn subsumes(left: Option<&Scope>, right: Option<&Scope>) -> bool {
  let (Some(left), Some(right)) = (left, right) else {
    return false;
  };

  eq(left, right) || subsumes_strict(left, right)
}
