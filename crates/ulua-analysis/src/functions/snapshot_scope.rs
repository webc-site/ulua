use alloc::{
  string::{String, ToString},
  vec::Vec,
};
/// C++ `static ScopeSnapshot snapshotScope(const Scope* scope, ToStringOptions& opts)`.
use core::ffi::CStr;
use std::collections::HashMap;

use crate::{
  functions::{
    to_string_detailed_to_string::to_string_detailed_type_id_to_string_options,
    to_string_to_string_alt_m::to_string_type_id_to_string_options,
    to_string_to_string_alt_n::to_string_type_pack_id_to_string_options,
  },
  records::{
    binding_snapshot::BindingSnapshot, scope::Scope, scope_snapshot::ScopeSnapshot,
    to_string_options::ToStringOptions, to_string_result::ToStringResult,
    type_binding_snapshot::TypeBindingSnapshot,
  },
};
pub fn snapshot_scope(scope: &Scope, opts: &mut ToStringOptions) -> ScopeSnapshot {
  let mut bindings: HashMap<String, BindingSnapshot> = HashMap::new();
  let mut type_bindings: HashMap<String, TypeBindingSnapshot> = HashMap::new();
  let mut type_pack_bindings: HashMap<String, TypeBindingSnapshot> = HashMap::new();
  let mut children: Vec<ScopeSnapshot> = Vec::new();

  for (symbol, binding) in &scope.bindings {
    let id = binding.type_id as usize;
    let id_str = id.to_string();
    let result: ToStringResult =
      to_string_detailed_type_id_to_string_options(binding.type_id, opts);
    let type_string = result.name.clone();

    // C++ `Symbol::c_str()`: prefer the local's name, else the global AstName.
    let key = unsafe {
      let cstr = if !symbol.local.is_null() {
        (*symbol.local).name.value
      } else {
        symbol.global.value
      };
      CStr::from_ptr(cstr).to_string_lossy().to_string()
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

  for child in &scope.children {
    let child_scope = unsafe { &**child };
    children.push(snapshot_scope(child_scope, opts));
  }

  ScopeSnapshot {
    bindings,
    type_bindings,
    type_pack_bindings,
    children,
  }
}
