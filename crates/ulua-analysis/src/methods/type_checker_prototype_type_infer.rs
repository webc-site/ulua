use core::ffi::CStr;

use ulua_ast::records::ast_stat_type_alias::AstStatTypeAlias;

use crate::{
  functions::get_mutable_type::get_mutable_type_id,
  records::{
    duplicate_type_definition::DuplicateTypeDefinition, free_type::FreeType, scope::Scope,
    type_checker::TypeChecker, type_error::TypeError, type_fun::TypeFun,
  },
  type_aliases::{name_type::Name, scope_ptr_type::ScopePtr, type_error_data::TypeErrorData},
};
impl TypeChecker {
  pub fn prototype_scope_ptr_ast_stat_type_alias_i32(
    &mut self,
    scope: ScopePtr,
    typealias: &AstStatTypeAlias,
    sub_level: i32,
  ) {
    let name_cstr = unsafe { CStr::from_ptr(typealias.name.value) };

    // If the alias is missing a name, we can't do anything with it.  Ignore it.
    // Also, typeof is not a valid type alias name.  We will report an error for
    // this in check()
    if name_cstr.to_bytes() == b"%error-id%" || name_cstr.to_bytes() == b"typeof" {
      return;
    }

    let name: Name = name_cstr.to_string_lossy().into_owned();

    let mut binding: Option<TypeFun> = None;
    if let Some(it) = scope.exported_type_bindings.get(&name) {
      binding = Some(it.clone());
    } else if let Some(it) = scope.private_type_bindings.get(&name) {
      binding = Some(it.clone());
    }

    if binding.is_some() {
      let location = *scope.type_alias_locations.get(&name).unwrap();
      self.report_error_type_error(&TypeError::type_error_location_type_error_data(
        typealias.base.base.location,
        TypeErrorData::DuplicateTypeDefinition(DuplicateTypeDefinition::new(
          name.clone(),
          Some(location),
        )),
      ));

      self
        .duplicate_type_aliases
        .insert((typealias.exported, name));
    } else {
      let global_scope = unsafe { &*self.global_scope };
      let is_builtin = global_scope.builtin_type_names.contains(&name);
      if is_builtin {
        self.report_error_location_type_error_data(
          &typealias.base.base.location,
          TypeErrorData::DuplicateTypeDefinition(DuplicateTypeDefinition::new(name.clone(), None)),
        );
        self
          .duplicate_type_aliases
          .insert((typealias.exported, name));
      } else {
        let alias_scope = self.child_scope(&scope, &typealias.base.base.location);
        {
          let alias_scope_raw = alias_scope.as_ref() as *const Scope as *mut Scope;
          unsafe {
            (*alias_scope_raw).level = scope.level.incr();
            (*alias_scope_raw).level.sub_level = sub_level;
          }
        }

        let defs = self.create_generic_types(
          &alias_scope,
          Some(scope.level),
          &typealias.base.base,
          &typealias.generics,
          &typealias.generic_packs,
          true,
        );

        let ty = self.fresh_type_scope_ptr(alias_scope.clone());
        // C++: FreeType* ftv = getMutable<FreeType>(ty); LUAU_ASSERT(ftv);
        // fresh_type_scope_ptr 刚分配的必是 FreeType。
        get_mutable_type_id::<FreeType>(ty)
          .unwrap()
          .forwarded_type_alias = true;

        let type_fun = TypeFun {
          type_params: defs.generic_types,
          type_pack_params: defs.generic_packs,
          r#type: ty,
          definition_location: Some(typealias.base.base.location),
        };

        let scope_raw = scope.as_ref() as *const Scope as *mut Scope;
        unsafe {
          if typealias.exported {
            (*scope_raw)
              .exported_type_bindings
              .insert(name.clone(), type_fun);
          } else {
            (*scope_raw)
              .private_type_bindings
              .insert(name.clone(), type_fun);
          }

          (*scope_raw)
            .type_alias_locations
            .insert(name.clone(), typealias.base.base.location);
          (*scope_raw)
            .type_alias_name_locations
            .insert(name, typealias.name_location);
        }
      }
    }
  }
}
