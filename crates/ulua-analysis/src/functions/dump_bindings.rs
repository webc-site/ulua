use alloc::string::String;

use crate::{
  functions::to_string_to_string::to_string_type_id_to_string_options,
  records::{scope::Scope, scope_registry::resolve_scope, to_string_options::ToStringOptions},
};

/// 对应 C++ `static void dumpBindings(NotNull<Scope> scope, ToStringOptions& opts)`
/// (`cpp/Analysis/src/ConstraintSolver.cpp:75`)：按 `DebugLuauLogSolver` 调试开关只读
/// 递归打印 scope 绑定。
///
/// `scope` 是 cpp `NotNull<Scope>` 的引用化（同 `snapshot_scope` 的既有形态）：非空与存活
/// 改由引用承担，只剩「整趟递归遍历期内该 scope 树不被并发写」这一条调用方前提。
pub fn dump_bindings(scope: &Scope, opts: &mut ToStringOptions) {
  for (k, v) in &scope.bindings {
    let d: String = to_string_type_id_to_string_options(v.type_id, opts);
    let key_str = k.name();
    println!("\t{} : {}", key_str, d);
  }

  for child_id in &scope.children {
    // `children: Vec<ScopeId>` 句柄恒指向注册表保活的子作用域（scope_registry
    // 契约 1，对应 cpp `NotNull<Scope>` 逐项）；此处只取共享引用递归遍历。
    let Some(child_scope) = resolve_scope(*child_id) else {
      continue;
    };
    dump_bindings(child_scope, opts);
  }
}
