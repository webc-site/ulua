use alloc::string::String;

use ulua_analysis::{records::scope::Scope, type_aliases::type_id::TypeId};

/// cpp `linearSearchForBinding(Scope*, const string&, bool)`：沿 `scope` 的父链
/// 顺序查找 `name` 的绑定，命中返回其 `typeId`。
///
/// 形参收 `&Scope`：只读遍历绑定链，不物化 `&mut`（同一 `Arc<Scope>` 在 cpp 侧是多把
/// 别名，独占借用会与之冲突）。查不到返回 `None`，不抛错。
pub fn linear_search_for_binding(scope: &Scope, name: &str) -> Option<TypeId> {
  scope
    .linear_search_for_binding(&String::from(name), true)
    .map(|binding| binding.type_id)
}
