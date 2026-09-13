use alloc::vec::Vec;
use core::{ffi::CStr, ptr::null_mut};

use ulua_ast::records::{
  ast_array::AstArray, ast_generic_type::AstGenericType, ast_generic_type_pack::AstGenericTypePack,
  ast_node::AstNode,
};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::polarity::Polarity,
  records::{
    duplicate_generic_parameter::DuplicateGenericParameter, generic_type::GenericType,
    generic_type_definition::GenericTypeDefinition,
    generic_type_definitions::GenericTypeDefinitions, generic_type_pack::GenericTypePack,
    generic_type_pack_definition::GenericTypePackDefinition, scope::Scope,
    type_checker::TypeChecker, type_error::TypeError, type_fun::TypeFun, type_level::TypeLevel,
    type_pack_var::TypePackVar,
  },
  type_aliases::{scope_ptr_type::ScopePtr, type_error_data::TypeErrorData},
};
impl TypeChecker {
  pub fn create_generic_types(
    &mut self,
    scope: &ScopePtr,
    level_opt: Option<TypeLevel>,
    node: &AstNode,
    generic_names: &AstArray<*mut AstGenericType>,
    generic_pack_names: &AstArray<*mut AstGenericTypePack>,
    use_cache: bool,
  ) -> GenericTypeDefinitions {
    let scope_ptr = scope.as_ref() as *const Scope as *mut Scope;
    LUAU_ASSERT!(unsafe { (*scope_ptr).parent.is_some() });

    let level = level_opt.unwrap_or(unsafe { (*scope_ptr).level });

    let mut generics = Vec::new();

    for generic in generic_names.iter() {
      let generic = unsafe { &**generic };
      let mut default_value = None;

      if !generic.default_value.is_null() {
        default_value = Some(self.resolve_type(scope.clone(), unsafe { &*generic.default_value }));
      }

      let n = unsafe {
        CStr::from_ptr(generic.name.value)
          .to_string_lossy()
          .into_owned()
      };

      if unsafe {
        (*scope_ptr).private_type_bindings.contains_key(&n)
          || (*scope_ptr).private_type_pack_bindings.contains_key(&n)
      } {
        self.report_error_type_error(&TypeError::type_error_location_type_error_data(
          node.location,
          TypeErrorData::DuplicateGenericParameter(DuplicateGenericParameter::new(n.clone())),
        ));
      }

      let g = if use_cache {
        let parent_ptr =
          unsafe { (*scope_ptr).parent.as_ref().unwrap().as_ref() as *const Scope as *mut Scope };
        let existing = unsafe { (*parent_ptr).type_alias_type_parameters.get(&n).copied() };

        match existing {
          Some(c) if !c.is_null() => c,
          _ => {
            let new_ty = self.add_type(&GenericType::generic_type_type_level_name(level, &n));
            unsafe {
              (*parent_ptr)
                .type_alias_type_parameters
                .insert(n.clone(), new_ty);
            }
            new_ty
          }
        }
      } else {
        self.add_type(&GenericType::generic_type_type_level_name(level, &n))
      };

      generics.push(GenericTypeDefinition {
        ty: g,
        default_value,
      });
      unsafe {
        (*scope_ptr)
          .private_type_bindings
          .insert(n, TypeFun::type_fun_type_id(g));
      }
    }

    let mut generic_packs = Vec::new();

    for generic_pack in generic_pack_names.iter() {
      let generic_pack = unsafe { &**generic_pack };
      let mut default_value = None;

      if !generic_pack.default_value.is_null() {
        default_value = Some(
          self.resolve_type_pack_scope_ptr_ast_type_pack(scope.clone(), unsafe {
            &*generic_pack.default_value
          }),
        );
      }

      let n = unsafe {
        CStr::from_ptr(generic_pack.name.value)
          .to_string_lossy()
          .into_owned()
      };

      if unsafe {
        (*scope_ptr).private_type_pack_bindings.contains_key(&n)
          || (*scope_ptr).private_type_bindings.contains_key(&n)
      } {
        self.report_error_type_error(&TypeError::type_error_location_type_error_data(
          node.location,
          TypeErrorData::DuplicateGenericParameter(DuplicateGenericParameter::new(n.clone())),
        ));
      }

      let parent_ptr =
        unsafe { (*scope_ptr).parent.as_ref().unwrap().as_ref() as *const Scope as *mut Scope };
      let existing = unsafe {
        (*parent_ptr)
          .type_alias_type_pack_parameters
          .get(&n)
          .copied()
      };
      let cached = match existing {
        Some(c) if !c.is_null() => c,
        _ => {
          let mut gtp = GenericTypePack {
            index: 0,
            level,
            scope: null_mut(),
            name: n.clone(),
            explicit_name: false,
            polarity: Polarity::Unknown,
          };
          gtp.generic_type_pack_type_level_name(level, &n);
          let new_tp = self.add_type_pack_type_pack_var(TypePackVar::from(gtp));
          unsafe {
            (*parent_ptr)
              .type_alias_type_pack_parameters
              .insert(n.clone(), new_tp);
          }
          new_tp
        }
      };

      generic_packs.push(GenericTypePackDefinition {
        tp: cached,
        default_value,
      });
      unsafe {
        (*scope_ptr).private_type_pack_bindings.insert(n, cached);
      }
    }

    GenericTypeDefinitions {
      generic_types: generics,
      generic_packs,
    }
  }
}
