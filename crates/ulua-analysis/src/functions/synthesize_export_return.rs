//! `void synthesizeExportReturn(NotNull<BuiltinTypes> builtinTypes, NotNull<Module> module)`.
//! Reference: `Module.cpp:361-467`.

use alloc::{string::String, sync::Arc, vec};
use core::{ffi::CStr, ptr::null_mut};

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_local::AstExprLocal, ast_local::AstLocal, ast_name::AstName,
    ast_node::AstNode, ast_stat_assign::AstStatAssign, ast_stat_class::AstStatClass,
    ast_stat_function::AstStatFunction, ast_stat_local::AstStatLocal,
    ast_stat_local_function::AstStatLocalFunction,
  },
  rtti::ast_node_as,
};
use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT, records::dense_hash_set::DenseHashSet};

use crate::{
  enums::table_state::TableState,
  functions::follow_type::follow_type_id,
  records::{
    builtin_types::BuiltinTypes, module::Module, property_type::Property, scope::Scope,
    symbol::Symbol, table_type::TableType, type_pack::TypePack,
  },
  type_aliases::{props_type::Props, type_id::TypeId},
};
fn key_of(name: AstName) -> String {
  if name.value.is_null() {
    String::new()
  } else {
    unsafe { CStr::from_ptr(name.value).to_string_lossy().into_owned() }
  }
}

/// C++ `Property(TypeId readTy)` — the single-argument constructor sets
/// `readTy == writeTy` (a read-write property). Reference: `Type.h` Property ctor.
fn prop_from_ty(ty: TypeId) -> Property {
  Property {
    read_ty: Some(ty),
    write_ty: Some(ty),
    ..Property::default()
  }
}

/// # Safety
/// 调用方须保证 `builtin_types、`module` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn synthesize_export_return(builtin_types: *mut BuiltinTypes, module: *mut Module) {
  let module_ref = unsafe { &mut *module };
  LUAU_ASSERT!(!module_ref.root.is_null());

  let module_scope = module_ref.get_module_scope();
  let module_scope_ptr = Arc::as_ptr(&module_scope) as *mut Scope;
  let mut props: Props = Props::new();

  let lookup_exported_binding_type = |local: *mut AstLocal| -> TypeId {
    let scope = unsafe { (*module_scope_ptr).find_narrowest_scope_containing((*local).location) };

    if let Some((binding, _scope)) = unsafe { (*scope).lookup_ex_symbol(Symbol::from_local(local)) }
    {
      return unsafe { follow_type_id((*binding).type_id) };
    }

    unsafe { (*builtin_types).error_type }
  };

  let lookup_expr_type = |expr: *mut AstExpr| -> TypeId {
    if let Some(ty) = unsafe { (*module).ast_types.find(&(expr as *const AstExpr)) } {
      return follow_type_id(*ty);
    }

    unsafe { (*builtin_types).error_type }
  };

  let mut exported_locals: DenseHashSet<*mut AstLocal> = DenseHashSet::new(null_mut());

  let body = unsafe { &(*module_ref.root).body };
  for &statement in body.as_slice() {
    let node = statement as *mut AstNode;

    let local_stat = unsafe { ast_node_as::<AstStatLocal>(node) };
    let local_function = unsafe { ast_node_as::<AstStatLocalFunction>(node) };
    let assign = unsafe { ast_node_as::<AstStatAssign>(node) };
    let func_stat = unsafe { ast_node_as::<AstStatFunction>(node) };

    if !local_stat.is_null() {
      let local_stat = unsafe { &*local_stat };
      if !local_stat.is_exported {
        continue;
      }

      // vars/values 一一对应；长度不等时全部按 binding 处理
      for (i, &local) in local_stat.vars.as_slice().iter().enumerate() {
        exported_locals.insert(local);

        let key = key_of(unsafe { (*local).name });

        if local_stat.vars.size != local_stat.values.size || i >= local_stat.values.size {
          props.insert(
            key.clone(),
            prop_from_ty(lookup_exported_binding_type(local)),
          );
        } else {
          let value = local_stat.values.as_slice()[i];
          props.insert(key.clone(), Property::readonly(lookup_expr_type(value)));
        }

        props.get_mut(&key).unwrap().location = Some(unsafe { (*local).location });
      }
    } else if !local_function.is_null() {
      let local_function = unsafe { &*local_function };
      if unsafe { !(*local_function.name).is_exported } {
        continue;
      }

      let key = key_of(unsafe { (*local_function.name).name });
      props.insert(
        key.clone(),
        Property::readonly(lookup_exported_binding_type(local_function.name)),
      );
      props.get_mut(&key).unwrap().location = Some(unsafe { (*local_function.name).location });
    } else if !assign.is_null() {
      let assign = unsafe { &*assign };
      // vars/values 一一对应；长度不等时全部按 binding 处理
      for (i, &local) in assign.vars.as_slice().iter().enumerate() {
        let expr_local = unsafe { ast_node_as::<AstExprLocal>(local as *mut AstNode) };
        if expr_local.is_null() || !exported_locals.contains(&unsafe { (*expr_local).local }) {
          continue;
        }

        let local = unsafe { (*expr_local).local };
        let key = key_of(unsafe { (*local).name });

        if assign.vars.size != assign.values.size || i >= assign.values.size {
          props.insert(
            key.clone(),
            prop_from_ty(lookup_exported_binding_type(local)),
          );
        } else {
          let value = assign.values.as_slice()[i];
          props.insert(key.clone(), Property::readonly(lookup_expr_type(value)));
        }

        props.get_mut(&key).unwrap().location = Some(unsafe { (*local).location });
      }
    } else if !func_stat.is_null() {
      let func_stat = unsafe { &*func_stat };
      let expr_local = unsafe { ast_node_as::<AstExprLocal>(func_stat.name as *mut AstNode) };
      if !expr_local.is_null() && exported_locals.contains(&unsafe { (*expr_local).local }) {
        let local = unsafe { (*expr_local).local };
        let key = key_of(unsafe { (*local).name });
        props.insert(
          key.clone(),
          Property::readonly(lookup_expr_type(func_stat.func as *mut AstExpr)),
        );
        props.get_mut(&key).unwrap().location = Some(unsafe { (*local).location });
      }
    } else if FFlag::DebugLuauUserDefinedClasses.get() {
      let class_stat = unsafe { ast_node_as::<AstStatClass>(node) };
      if !class_stat.is_null() {
        let class_stat = unsafe { &*class_stat };
        if !class_stat.exported {
          continue;
        }

        let key = key_of(unsafe { (*class_stat.name).name });
        props.insert(
          key.clone(),
          Property::readonly(lookup_exported_binding_type(class_stat.name)),
        );
        props.get_mut(&key).unwrap().location = Some(unsafe { (*class_stat.name).location });
      }
    }
  }

  if props.is_empty() {
    return;
  }

  let level = unsafe { (*module_scope_ptr).level };
  let exports = module_ref.internal_types.add_type(
    TableType::table_type_props_optional_table_indexer_type_level_table_state(
      &props,
      None,
      level,
      TableState::Sealed,
    ),
  );
  let exports_pack = module_ref.internal_types.add_type_pack_t(TypePack {
    head: vec![exports],
    tail: None,
  });
  unsafe { (*module_scope_ptr).return_type = exports_pack };
}
