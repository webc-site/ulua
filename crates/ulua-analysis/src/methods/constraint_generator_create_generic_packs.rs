use alloc::vec::Vec;
use core::{ffi::CStr, ptr::null_mut};

use ulua_ast::records::{ast_array::AstArray, ast_generic_type_pack::AstGenericTypePack};

use crate::{
  enums::polarity::Polarity,
  records::{
    blocked_type_pack::BlockedTypePack, constraint_generator::ConstraintGenerator,
    generic_type_pack::GenericTypePack, generic_type_pack_definition::GenericTypePackDefinition,
    scope::Scope,
  },
  type_aliases::{name_type::Name, scope_ptr_type::ScopePtr, type_pack_id::TypePackId},
};
impl ConstraintGenerator {
  pub fn create_generic_packs(
    &mut self,
    scope: &ScopePtr,
    generics: AstArray<*mut AstGenericTypePack>,
    use_cache: bool,
    add_types: bool,
  ) -> Vec<(Name, GenericTypePackDefinition)> {
    let mut result: Vec<(Name, GenericTypePackDefinition)> = Vec::new();
    let scope_ptr = scope.as_ref() as *const Scope as *mut Scope;

    for &generic in generics.as_slice() {
      let generic_name_ptr = unsafe { (*generic).name.value };
      let generic_name_str = unsafe {
        CStr::from_ptr(generic_name_ptr)
          .to_string_lossy()
          .into_owned()
      };

      let mut generic_ty: Option<TypePackId> = None;

      if use_cache
        && let Some(parent_scope) = scope.parent.as_ref()
        && let Some(type_pack_id) = parent_scope
          .type_alias_type_pack_parameters
          .get(&generic_name_str)
      {
        generic_ty = Some(*type_pack_id);
      }

      if generic_ty.is_none() {
        let mut gtp = GenericTypePack {
          index: 0,
          level: Default::default(),
          scope: null_mut(),
          name: Default::default(),
          explicit_name: false,
          polarity: Polarity::None,
        };
        gtp.generic_type_pack_scope_name_polarity(
          scope_ptr,
          generic_name_str.clone(),
          Polarity::None,
        );
        generic_ty = Some(unsafe { (*self.arena).add_type_pack_t(gtp) });

        if let Some(parent_scope) = scope.parent.as_ref() {
          let parent_scope_ptr = parent_scope.as_ref() as *const Scope as *mut Scope;
          unsafe {
            (*parent_scope_ptr)
              .type_alias_type_pack_parameters
              .insert(generic_name_str.clone(), generic_ty.unwrap());
          }
        }
      }

      let default_ty: Option<TypePackId> = if unsafe { (*generic).default_value }.is_null() {
        None
      } else {
        let mut btp = BlockedTypePack {
          index: 0,
          owner: null_mut(),
        };
        btp.blocked_type_pack_blocked_type_pack();
        Some(unsafe { (*self.arena).add_type_pack_t(btp) })
      };

      if add_types {
        unsafe {
          (*scope_ptr)
            .private_type_pack_bindings
            .insert(generic_name_str.clone(), generic_ty.unwrap());
        }
      }

      result.push((
        generic_name_str,
        GenericTypePackDefinition {
          tp: generic_ty.unwrap(),
          default_value: default_ty,
        },
      ));
    }

    result
  }
}
