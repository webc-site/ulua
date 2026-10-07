use ulua_analysis::{records::scope::Scope, type_aliases::type_id::TypeId};

use crate::functions::linear_search_for_binding::linear_search_for_binding;

/// cpp `lookupName(Scope*, const string&)`：旧求解器形态的名字查找，实为
/// `linearSearchForBinding` 的薄封装。
///
/// 形参收 `&Scope`（cpp 的 `Scope*` 只读使用）：调用方从 `Arc<Scope>` 克隆或
/// `Scope` 借用处取引用即可，绑定链遍历不写 scope，查不到返回 `None`。
pub fn lookup_name(scope: &Scope, name: &str) -> Option<TypeId> {
  linear_search_for_binding(scope, name)
}
