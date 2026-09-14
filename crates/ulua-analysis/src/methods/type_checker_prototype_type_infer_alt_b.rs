use alloc::sync::Arc;
use core::ffi::CStr;

use ulua_ast::records::ast_stat_declare_extern_type::AstStatDeclareExternType;
use ulua_common::{functions::format::format, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::table_state::TableState,
  functions::{
    follow_type::follow_type_id, get_mutable_type::get_mutable_type_id, get_type_alt_j::get_type_id,
  },
  records::{
    extern_type::ExternType,
    generic_error::GenericError,
    module::Module,
    scope::Scope,
    table_type::TableType,
    type_checker::TypeChecker,
    type_fun::TypeFun,
    unknown_symbol::{Context, UnknownSymbol},
  },
  type_aliases::{
    name_type::Name, scope_ptr_type::ScopePtr, type_error_data::TypeErrorData, type_id::TypeId,
  },
};
impl TypeChecker {
  pub fn prototype_scope_ptr_ast_stat_declare_extern_type(
    &mut self,
    scope: ScopePtr,
    declared_extern_type: &AstStatDeclareExternType,
  ) {
    let mut super_ty: Option<TypeId> = Some(unsafe { (*self.builtin_types).extern_type });

    if let Some(super_name_astname) = declared_extern_type.super_name {
      let super_name: Name = unsafe {
        CStr::from_ptr(super_name_astname.value)
          .to_string_lossy()
          .into_owned()
      };
      let lookup_type = scope.lookup_type(&super_name);

      if lookup_type.is_none() {
        self.report_error_location_type_error_data(
          &declared_extern_type.base.base.location,
          TypeErrorData::UnknownSymbol(UnknownSymbol::new(super_name, Context::Type)),
        );
        self
          .incorrect_extern_type_definitions
          .insert(declared_extern_type as *const AstStatDeclareExternType);
        return;
      }

      let lookup_type = lookup_type.unwrap();

      // We don't have generic extern type_arguments, so this assertion _should_ never be hit.
      LUAU_ASSERT!(
        lookup_type.type_params().is_empty() && lookup_type.type_pack_params().is_empty()
      );
      super_ty = Some(lookup_type.r#type());

      if get_type_id::<ExternType>(follow_type_id(super_ty.unwrap())).is_none() {
        let class_name =
          unsafe { CStr::from_ptr(declared_extern_type.name.value).to_string_lossy() };
        self.report_error_location_type_error_data(
          &declared_extern_type.base.base.location,
          TypeErrorData::GenericError(GenericError::new(format(format_args!(
            "Cannot use non-class type '{}' as a superclass of class '{}'",
            unsafe { CStr::from_ptr(super_name_astname.value).to_string_lossy() },
            class_name
          )))),
        );
        self
          .incorrect_extern_type_definitions
          .insert(declared_extern_type as *const AstStatDeclareExternType);
        return;
      }
    }

    let class_name: Name = unsafe {
      CStr::from_ptr(declared_extern_type.name.value)
        .to_string_lossy()
        .into_owned()
    };

    let module_name = { self.current_module.as_ref().unwrap().name.clone() };

    let scope_level = scope.level;
    let scope_raw = scope.as_ref() as *const Scope as *mut Scope;

    let class_ty: TypeId = unsafe {
      (*(Arc::as_ptr(self.current_module.as_ref().unwrap()) as *mut Module))
        .internal_types
        .add_type(ExternType {
          name: class_name.clone(),
          props: Default::default(),
          parent: super_ty,
          metatable: None,
          tags: Default::default(),
          user_data: None,
          definition_module_name: module_name,
          definition_location: Some(declared_extern_type.base.base.location),
          indexer: None,
          relation: None,
        })
    };
    let meta_ty: TypeId = unsafe {
      (*(Arc::as_ptr(self.current_module.as_ref().unwrap()) as *mut Module))
        .internal_types
        .add_type(TableType::table_type_table_state_type_level_scope(
          TableState::Sealed,
          scope_level,
          scope_raw,
        ))
    };

    // class_ty 刚以 ExternType 变体分配（TypeInfer.cpp:1711-1718），下转必然成功。
    get_mutable_type_id::<ExternType>(class_ty)
      .unwrap()
      .metatable = Some(meta_ty);

    unsafe {
      (*scope_raw).exported_type_bindings.insert(
        class_name,
        TypeFun {
          type_params: Default::default(),
          type_pack_params: Default::default(),
          r#type: class_ty,
          definition_location: Some(declared_extern_type.base.base.location),
        },
      );
    }
  }
}
