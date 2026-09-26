use alloc::{string::String, sync::Arc, vec::Vec};

use ulua_ast::records::location::Location;
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  functions::{
    arc_as_mut::arc_as_mut,
    clone_clone::{clone_type_fun, clone_type_id as clone_type},
    generate_documentation_symbols::generate_documentation_symbols,
    persist_type::persist,
  },
  records::{
    arena_handle::Handle, binding::Binding, clone_state::CloneState, global_types::GlobalTypes,
    module::Module, symbol::Symbol,
  },
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};
pub fn persist_checked_types(
  checked_module: Arc<Module>,
  globals: &mut GlobalTypes,
  target_scope: ScopePtr,
  package_name: String,
) {
  let mut clone_state = CloneState {
    // 经 `GlobalTypes::builtin_types_ref` chokepoint 取内建单例共享引用，再由
    // `Handle::from_ref` 安全转铸为 clone 期只读句柄，调用点免触碰裸字段。
    builtin_types: Handle::from_ref(globals.builtin_types_ref()),
    seen_types: DenseHashMap::default(),
    seen_type_packs: DenseHashMap::default(),
  };

  let mut types_to_persist: Vec<TypeId> = Vec::with_capacity(
    checked_module.declared_globals.len() + checked_module.exported_type_bindings.len(),
  );

  let target_scope_ptr = arc_as_mut(&target_scope);
  let names_ptr = arc_as_mut(&globals.global_names.names);

  for (name, ty) in &checked_module.declared_globals {
    // `builtin_types` 取自 `globals.builtin_types_ref()` chokepoint 的共享引用（指向存活
    // 单例），`*ty` 为 arena 存活 `TypeId`，全程单线程串行、无别名冲突。
    let global_ty = clone_type(*ty, &mut globals.global_types, &mut clone_state);

    const INFIX: &str = "/global/";
    let mut documentation_symbol =
      String::with_capacity(package_name.len() + INFIX.len() + name.len());
    documentation_symbol.push_str(&package_name);
    documentation_symbol.push_str(INFIX);
    documentation_symbol.push_str(name);

    // Safety: `generate_documentation_symbols` 只沿 `global_ty`（刚由 clone 写入
    // arena、存活且块地址稳定的 `TypeId`）递归读取类型节点，无写别名。
    unsafe { generate_documentation_symbols(global_ty, documentation_symbol.clone()) };

    // Safety: `names_ptr = arc_as_mut(&globals.global_names.names)`，该 `Arc` 在本函数
    // 存活且单线程独占；据 `arc_as_mut` 约定重建可变借用向名字表追加字符串，无并发写。
    let ast_name = unsafe { (*names_ptr).get_or_add_str(name) };
    let binding = Binding {
      type_id: global_ty,
      location: Location::default(),
      deprecated: false,
      deprecated_suggestion: String::new(),
      documentation_symbol: Some(documentation_symbol),
    };
    // Safety: `target_scope_ptr = arc_as_mut(&target_scope)` 指向存活的 `Arc<Scope>`
    // 目标，本函数单线程独占写其 `bindings`，无其它活动借用冲突。
    unsafe {
      (*target_scope_ptr)
        .bindings
        .insert(Symbol::from_global(ast_name), binding);
    }

    types_to_persist.push(global_ty);
  }

  for (name, ty) in &checked_module.exported_type_bindings {
    let global_ty = clone_type_fun(ty, &mut globals.global_types, &mut clone_state);

    const INFIX: &str = "/globaltype/";
    let mut documentation_symbol =
      String::with_capacity(package_name.len() + INFIX.len() + name.len());
    documentation_symbol.push_str(&package_name);
    documentation_symbol.push_str(INFIX);
    documentation_symbol.push_str(name);

    // Safety: 同 declared_globals 分支——`global_ty.r#type` 为 clone 写入 arena 的
    // 存活 `TypeId`，`generate_documentation_symbols` 仅沿其递归只读遍历。
    unsafe { generate_documentation_symbols(global_ty.r#type, documentation_symbol) };

    let global_ty_type = global_ty.r#type;
    // Safety: `target_scope_ptr` 指向存活的 `Arc<Scope>` 目标，本函数单线程独占写其
    // `exported_type_bindings`，无其它活动借用冲突。
    unsafe {
      (*target_scope_ptr)
        .exported_type_bindings
        .insert(name.clone(), global_ty);
    }

    types_to_persist.push(global_ty_type);
  }

  for ty in types_to_persist {
    persist(ty);
  }
}
