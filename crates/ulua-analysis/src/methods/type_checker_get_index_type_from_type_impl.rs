use alloc::{string::String, sync::Arc, vec::Vec};
use core::mem::zeroed;

use ulua_ast::records::location::Location;
use ulua_common::{FInt, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::table_state::TableState,
  functions::{
    follow_type::follow_type_id, get_mutable_table_type::get_mutable_table_type,
    get_type_alt_j::get_type_id, is_string::is_string,
    lookup_extern_type_prop::lookup_extern_type_prop, reduce_union::reduce_union,
  },
  records::{
    any_type::AnyType, error_type::ErrorType, extern_type::ExternType,
    intersection_type::IntersectionType, missing_union_property::MissingUnionProperty,
    module::Module, never_type::NeverType, property_type::Property,
    recursion_limiter::RecursionLimiter, type_checker::TypeChecker, union_type::UnionType,
    unknown_property::UnknownProperty,
  },
  type_aliases::{
    name_type::Name, scope_ptr_type::ScopePtr, type_error_data::TypeErrorData, type_id::TypeId,
  },
};
impl TypeChecker {
  pub fn get_index_type_from_type_impl(
    &mut self,
    scope: ScopePtr,
    ty: TypeId,
    name: &Name,
    location: &Location,
    add_errors: bool,
  ) -> Option<TypeId> {
    let mut ty = follow_type_id(ty);

    if get_type_id::<ErrorType>(ty).is_some()
      || get_type_id::<AnyType>(ty).is_some()
      || get_type_id::<NeverType>(ty).is_some()
    {
      return Some(ty);
    }

    self.tablify(ty);

    if is_string(ty) {
      let mt_index = self.find_metatable_entry(
        self.string_type,
        String::from("__index"),
        location,
        add_errors,
      );
      LUAU_ASSERT!(mt_index.is_some());
      ty = mt_index.unwrap();
    }

    if let Some(table_type) = get_mutable_table_type(ty) {
      if let Some(prop) = table_type.props.get(name) {
        return Some(prop.type_deprecated());
      } else if let Some(indexer) = table_type.indexer {
        // TODO: Property lookup should work with string singletons or unions thereof as the indexer key type.
        let errors = self.try_unify(self.string_type, indexer.index_type, &scope, location);

        if errors.is_empty() {
          return Some(indexer.index_result_type);
        }

        if add_errors {
          self.report_error_location_type_error_data(
            location,
            TypeErrorData::UnknownProperty(UnknownProperty {
              table: ty,
              key: name.clone(),
            }),
          );
        }

        return None;
      } else if table_type.state == TableState::Free {
        let result = self.fresh_type_type_level(table_type.level);
        table_type.props.insert(
                    name.clone(),
                    Property::property_type_id_bool_string_optional_location_tags_optional_string_optional_location(
                        result,
                        false,
                        String::new(),
                        None,
                        Default::default(),
                        None,
                        None,
                    ),
                );
        return Some(result);
      }

      if let Some(found) =
        self.find_table_property_respecting_meta(ty, name.clone(), location, add_errors)
      {
        return Some(found);
      }
    } else if let Some(cls) = get_type_id::<ExternType>(ty) {
      if let Some(prop) = lookup_extern_type_prop(cls, name) {
        return Some(prop.type_deprecated());
      }

      if let Some(indexer) = cls.indexer {
        // TODO: Property lookup should work with string singletons or unions thereof as the indexer key type.
        let errors = self.try_unify(self.string_type, indexer.index_type, &scope, location);

        if errors.is_empty() {
          return Some(indexer.index_result_type);
        }

        if add_errors {
          self.report_error_location_type_error_data(
            location,
            TypeErrorData::UnknownProperty(UnknownProperty {
              table: ty,
              key: name.clone(),
            }),
          );
        }

        return None;
      }
    } else if let Some(utv) = get_type_id::<UnionType>(ty) {
      let mut good_options: Vec<TypeId> = Vec::new();
      let mut bad_options: Vec<TypeId> = Vec::new();

      for &t in utv.options.iter() {
        let mut _rl = RecursionLimiter {
          base: unsafe { zeroed() },
          native_stack_guard: unsafe { zeroed() },
        };
        _rl.recursion_limiter_recursion_limiter(
          "TypeInfer::UnionType",
          &mut self.recursion_count,
          FInt::LuauTypeInferRecursionLimit.get(),
        );

        // Not needed when we normalize type_arguments.
        if get_type_id::<AnyType>(follow_type_id(t)).is_some() {
          return Some(t);
        }

        if let Some(ty2) = self.get_index_type_from_type(scope.clone(), t, name, location, false) {
          good_options.push(ty2);
        } else {
          bad_options.push(t);
        }
      }

      if !bad_options.is_empty() {
        if add_errors {
          if good_options.is_empty() {
            self.report_error_location_type_error_data(
              location,
              TypeErrorData::UnknownProperty(UnknownProperty {
                table: ty,
                key: name.clone(),
              }),
            );
          } else {
            self.report_error_location_type_error_data(
              location,
              TypeErrorData::MissingUnionProperty(MissingUnionProperty {
                r#type: ty,
                missing: bad_options,
                key: name.clone(),
              }),
            );
          }
        }
        return None;
      }

      let result = reduce_union(&good_options);
      if result.is_empty() {
        return Some(self.never_type);
      }

      if result.len() == 1 {
        return Some(result[0]);
      }

      return Some(unsafe {
        let module = Arc::as_ptr(self.current_module.as_ref().unwrap()) as *mut Module;
        (*module)
          .internal_types
          .add_type(UnionType { options: result })
      });
    } else if let Some(itv) = get_type_id::<IntersectionType>(ty) {
      let mut parts: Vec<TypeId> = Vec::new();

      for &t in itv.parts.iter() {
        let mut _rl = RecursionLimiter {
          base: unsafe { zeroed() },
          native_stack_guard: unsafe { zeroed() },
        };
        _rl.recursion_limiter_recursion_limiter(
          "TypeInfer::IntersectionType",
          &mut self.recursion_count,
          FInt::LuauTypeInferRecursionLimit.get(),
        );

        if let Some(ty2) = self.get_index_type_from_type(scope.clone(), t, name, location, false) {
          parts.push(ty2);
        }
      }

      // If no parts of the intersection had the property we looked up for, it never existed at all.
      if parts.is_empty() {
        if add_errors {
          self.report_error_location_type_error_data(
            location,
            TypeErrorData::UnknownProperty(UnknownProperty {
              table: ty,
              key: name.clone(),
            }),
          );
        }
        return None;
      }

      if parts.len() == 1 {
        return Some(parts[0]);
      }

      return Some(unsafe {
        let module = Arc::as_ptr(self.current_module.as_ref().unwrap()) as *mut Module;
        (*module)
          .internal_types
          .add_type(IntersectionType { parts })
      }); // Not at all correct.
    }

    if add_errors {
      self.report_error_location_type_error_data(
        location,
        TypeErrorData::UnknownProperty(UnknownProperty {
          table: ty,
          key: name.clone(),
        }),
      );
    }

    None
  }
}
