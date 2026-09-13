use alloc::vec::Vec;
use core::ffi::CStr;

use ulua_ast::records::ast_stat_type_alias::AstStatTypeAlias;

use crate::{
  enums::control_flow::ControlFlow,
  functions::{
    follow_type::follow_type_id, get_mutable_type::get_mutable_type_id,
    get_type_alt_j::get_type_id, get_type_pack::get_type_pack_id,
  },
  methods::type_checker_check_function_signature::scope_mut,
  records::{
    generic_type::GenericType, generic_type_pack::GenericTypePack, metatable_type::MetatableType,
    table_type::TableType, type_checker::TypeChecker, type_fun::TypeFun,
  },
  type_aliases::{name_type::Name, scope_ptr_type::ScopePtr},
};
impl TypeChecker {
  pub fn check_scope_ptr_ast_stat_type_alias(
    &mut self,
    scope: &ScopePtr,
    typealias: &AstStatTypeAlias,
  ) -> ControlFlow {
    // SAFETY: name.value 为 NUL 结尾 C 字符串（AST arena 持有）。
    let name_cstr = unsafe { CStr::from_ptr(typealias.name.value) };

    if name_cstr.to_bytes() == b"%error-id%" || name_cstr.to_bytes() == b"typeof" {
      return ControlFlow::None;
    }

    let name: Name = name_cstr.to_string_lossy().into_owned();

    if self
      .duplicate_type_aliases
      .contains(&(typealias.exported, name.clone()))
    {
      return ControlFlow::None;
    }

    let binding = if typealias.exported {
      scope.exported_type_bindings.get(&name).cloned()
    } else {
      scope.private_type_bindings.get(&name).cloned()
    };

    let Some(binding) = binding else {
      return ControlFlow::None;
    };

    let alias_scope = self.child_scope(scope, &typealias.base.base.location);
    {
      // SAFETY: 见 scope_mut 契约。
      let alias_scope_mut = unsafe { &mut *scope_mut(&alias_scope) };
      alias_scope_mut.level = scope.level.incr();

      for param in binding.type_params() {
        if let Some(generic) = get_type_id::<GenericType>(param.ty) {
          alias_scope_mut
            .private_type_bindings
            .insert(generic.name.clone(), TypeFun::type_fun_type_id(param.ty));
        }
      }

      for param in binding.type_pack_params() {
        if let Some(generic) = get_type_pack_id::<GenericTypePack>(param.tp) {
          alias_scope_mut
            .private_type_pack_bindings
            .insert(generic.name.clone(), param.tp);
        }
      }
    }

    // SAFETY: type_ptr 指向 AST arena 节点（parser 保证非空）。
    let mut ty = self.resolve_type(alias_scope.clone(), unsafe { &*typealias.type_ptr });

    // `get_mutable` requires a followed type (it asserts the arg is not a
    // BoundType). `ty` here is the raw result of `resolve_type`, which for
    // a self-referential / chained alias (e.g. `type A = A`, or `type T =
    // Pt; type Pt = ... T ...`) is a Bound — so follow before inspecting it,
    // matching the sibling `check_scope_ptr_ast_stat_local`. (C++ Luau's
    // assert is compiled out in release, masking this; our fuzz build arms
    // it, where it aborted.)
    ty = follow_type_id(ty);
    if let Some(table) = get_mutable_type_id::<TableType>(ty) {
      let type_params: Vec<_> = binding.type_params().iter().map(|param| param.ty).collect();
      let type_pack_params: Vec<_> = binding
        .type_pack_params()
        .iter()
        .map(|param| param.tp)
        .collect();

      let same_tys = table.instantiated_type_params == type_params;
      let same_tps = table.instantiated_type_pack_params == type_pack_params;

      if table.name.is_some() && (table.name.as_ref() != Some(&name) || !same_tys || !same_tps) {
        let mut clone = table.clone();
        clone.name = Some(name.clone());
        clone.instantiated_type_params = type_params;
        clone.instantiated_type_pack_params = type_pack_params;
        ty = self.add_type(&clone);
      } else {
        table.name = Some(name.clone());
        table.instantiated_type_params = type_params;
        table.instantiated_type_pack_params = type_pack_params;
      }
    } else if let Some(metatable) = get_mutable_type_id::<MetatableType>(ty) {
      metatable.synthetic_name = Some(name.clone());
    }

    // SAFETY: 见 scope_mut 契约。
    let scope_ref = unsafe { &mut *scope_mut(scope) };
    let bindings = if typealias.exported {
      &mut scope_ref.exported_type_bindings
    } else {
      &mut scope_ref.private_type_bindings
    };

    if let Some(binding) = bindings.get_mut(&name) {
      self.unify_type_id_type_id_scope_ptr_location(
        ty,
        binding.r#type,
        &alias_scope,
        &typealias.base.base.location,
      );

      // C++：follow 后仍为 Free 时覆写，否则也覆写（两分支同值，保持原翻译）。
      binding.r#type = ty;
    }

    ControlFlow::None
  }
}
