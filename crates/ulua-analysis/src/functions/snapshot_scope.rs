use alloc::{
  string::{String, ToString},
  vec::Vec,
};

use crate::{
  functions::{
    to_string_detailed_to_string::to_string_detailed,
    to_string_to_string::{
      to_string_type_id_to_string_options, to_string_type_pack_id_to_string_options,
    },
  },
  records::{
    binding_snapshot::BindingSnapshot, scope::Scope, scope_registry::resolve_scope,
    scope_snapshot::ScopeSnapshot, to_string_options::ToStringOptions,
    to_string_result::ToStringResult, type_binding_snapshot::TypeBindingSnapshot,
  },
  type_aliases::collections::HashMap,
};
pub fn snapshot_scope(scope: &Scope, opts: &mut ToStringOptions) -> ScopeSnapshot {
  let mut bindings: HashMap<String, BindingSnapshot> = HashMap::new();
  let mut type_bindings: HashMap<String, TypeBindingSnapshot> = HashMap::new();
  let mut type_pack_bindings: HashMap<String, TypeBindingSnapshot> = HashMap::new();
  let mut children: Vec<ScopeSnapshot> = Vec::new();

  for (symbol, binding) in &scope.bindings {
    let id = binding.type_id as usize;
    let id_str = id.to_string();
    let result: ToStringResult = to_string_detailed(binding.type_id, opts);
    let type_string = result.name.clone();

    // C++ `Symbol::c_str()`: prefer the local's name, else the global AstName.
    let key = {
      // `symbol.local_ref()` chokepoint 把身份句柄物化为 `&AstLocal`（或 None＝全局臂），
      // 只在非空局部时读取 `name`（`AstName` Copy 字段）；`as_str_or_empty` 亦容忍空名。
      let name = match symbol.local_ref() {
        Some(local) => local.name,
        None => symbol.global,
      };
      name.as_str_or_empty().to_string()
    };
    bindings.insert(
      key,
      BindingSnapshot {
        type_id: id_str,
        type_string,
        location: binding.location,
      },
    );
  }

  for (name, tf) in &scope.exported_type_bindings {
    let id = tf.r#type as usize;
    let id_str = id.to_string();
    let type_string = to_string_type_id_to_string_options(tf.r#type, opts);

    type_bindings.insert(
      name.clone(),
      TypeBindingSnapshot {
        type_id: id_str,
        type_string,
      },
    );
  }

  for (name, tf) in &scope.private_type_bindings {
    let id = tf.r#type as usize;
    let id_str = id.to_string();
    let type_string = to_string_type_id_to_string_options(tf.r#type, opts);

    type_bindings.insert(
      name.clone(),
      TypeBindingSnapshot {
        type_id: id_str,
        type_string,
      },
    );
  }

  for (name, tp) in &scope.private_type_pack_bindings {
    let id = *tp as usize;
    let id_str = id.to_string();
    let type_string = to_string_type_pack_id_to_string_options(*tp, opts);

    type_pack_bindings.insert(
      name.clone(),
      TypeBindingSnapshot {
        type_id: id_str,
        type_string,
      },
    );
  }

  for child_id in &scope.children {
    // `children: Vec<ScopeId>` 句柄恒指向注册表保活的子作用域（scope_registry
    // 契约 1）；本次递归快照仅取共享引用读取，单线程无别名。
    let Some(child_scope) = resolve_scope(*child_id) else {
      continue;
    };
    children.push(snapshot_scope(child_scope, opts));
  }

  ScopeSnapshot {
    bindings,
    type_bindings,
    type_pack_bindings,
    children,
  }
}
