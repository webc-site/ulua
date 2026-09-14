use alloc::vec::Vec;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  enums::{polarity::Polarity, table_state::TableState, value_context::ValueContext},
  functions::{
    extend_type_pack::extend_type_pack, fast_is_subtype::fast_is_subtype,
    follow_type::follow_type_id, fresh_type::fresh_type, get_mutable_type::get_mutable,
    get_table_type::get_table_type, get_type_alt_j::get_type_id,
    lookup_extern_type_prop::lookup_extern_type_prop,
    track_interior_free_type::track_interior_free_type,
  },
  records::{
    any_type::AnyType, constraint::Constraint, constraint_solver::ConstraintSolver,
    extern_type::ExternType, free_type::FreeType, function_type::FunctionType,
    intersection_type::IntersectionType, metatable_type::MetatableType, never_type::NeverType,
    primitive_type::PrimitiveType, property_type::Property, singleton_type::SingletonType,
    string_singleton::StringSingleton, table_prop_lookup_result::TablePropLookupResult,
    table_type::TableType, type_level::TypeLevel, union_type::UnionType,
  },
  type_aliases::{singleton_variant::SingletonVariant, type_id::TypeId},
};

impl ConstraintSolver {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool_set_type_id(
    &mut self,
    constraint: *const Constraint,
    subject_type: TypeId,
    prop_name: &str,
    context: ValueContext,
    in_conditional: bool,
    suppress_simplification: bool,
    seen: &mut DenseHashSet<TypeId>,
  ) -> TablePropLookupResult {
    if seen.contains(&subject_type) {
      return TablePropLookupResult {
        blocked_types: Vec::new(),
        prop_type: None,
        is_index: false,
      };
    }

    let mut seen = seen.clone();
    seen.insert(subject_type);

    let subject_type = follow_type_id(subject_type);

    if self.is_blocked_type_id(subject_type) {
      TablePropLookupResult {
        blocked_types: vec![subject_type],
        prop_type: None,
        is_index: false,
      }
    } else if get_type_id::<AnyType>(subject_type).is_some()
      || get_type_id::<NeverType>(subject_type).is_some()
    {
      TablePropLookupResult {
        blocked_types: Vec::new(),
        prop_type: Some(subject_type),
        is_index: false,
      }
    } else if let Some(ttv) = get_mutable::<TableType>(subject_type) {
      if let Some(prop) = ttv.props.get(prop_name) {
        match context {
          ValueContext::RValue => {
            if let Some(read_ty) = prop.read_ty {
              return TablePropLookupResult {
                blocked_types: Vec::new(),
                prop_type: Some(read_ty),
                is_index: false,
              };
            }
          }
          ValueContext::LValue => {
            if let Some(write_ty) = prop.write_ty {
              return TablePropLookupResult {
                blocked_types: Vec::new(),
                prop_type: Some(write_ty),
                is_index: false,
              };
            }
          }
        }
      }

      if let Some(indexer) = ttv.indexer {
        if self.is_blocked_type_id(indexer.index_type) {
          return TablePropLookupResult {
            blocked_types: vec![indexer.index_type],
            prop_type: None,
            is_index: true,
          };
        }

        // CLI-169235: build a faux string-singleton literal from the prop
        // name and reuse subtyping (same logic as `index<_, _>`), so a named
        // access hits the indexer when the name is a subtype of the index
        // key (e.g. "Val1" against a `"Val1"|"Val2"|"Val3"` key), not just
        // when the key is a plain string.
        let faux_literal = unsafe {
          (*self.arena).add_type(SingletonType::new(SingletonVariant::V1(
            StringSingleton::new(prop_name.to_string()),
          )))
        };
        if fast_is_subtype(faux_literal, indexer.index_type) {
          return TablePropLookupResult {
            blocked_types: Vec::new(),
            prop_type: Some(indexer.index_result_type),
            is_index: true,
          };
        }
      }

      if ttv.state == TableState::Free {
        let result = fresh_type(
          // SAFETY: arena 在 solver 存活期内有效。
          unsafe { &mut *self.arena },
          // SAFETY: builtin_types 在 solver 存活期内有效。
          unsafe { &*self.builtin_types },
          ttv.scope,
          Polarity::Mixed,
        );
        track_interior_free_type(ttv.scope, result);

        match context {
          ValueContext::RValue => {
            ttv
              .props
              .insert(prop_name.to_string(), Property::readonly(result));
          }
          ValueContext::LValue => {
            if let Some(prop) = ttv.props.get_mut(prop_name)
              && prop.is_read_only()
            {
              prop.write_ty = prop.read_ty;
              return TablePropLookupResult {
                blocked_types: Vec::new(),
                prop_type: prop.read_ty,
                is_index: false,
              };
            }

            ttv
              .props
              .insert(prop_name.to_string(), Property::rw_type_id(result));
          }
        }

        return TablePropLookupResult {
          blocked_types: Vec::new(),
          prop_type: Some(result),
          is_index: false,
        };
      }

      if in_conditional {
        return TablePropLookupResult {
          blocked_types: Vec::new(),
          prop_type: Some(unsafe { (*self.builtin_types).unknown_type }),
          is_index: false,
        };
      }

      TablePropLookupResult {
        blocked_types: Vec::new(),
        prop_type: None,
        is_index: false,
      }
    } else if let Some(mt) = get_type_id::<MetatableType>(subject_type) {
      if context == ValueContext::LValue {
        return unsafe {
          self
          .lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool_set_type_id(
            constraint,
            mt.table,
            prop_name,
            context,
            in_conditional,
            suppress_simplification,
            &mut seen,
          )
        };
      }

      let result = unsafe {
        self
          .lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool_set_type_id(
            constraint,
            mt.table,
            prop_name,
            context,
            in_conditional,
            suppress_simplification,
            &mut seen,
          )
      };
      if !result.blocked_types.is_empty() || result.prop_type.is_some() {
        return result;
      }

      let metatable = follow_type_id(mt.metatable);
      if self.is_blocked_type_id(metatable) {
        return TablePropLookupResult {
          blocked_types: vec![metatable],
          prop_type: None,
          is_index: false,
        };
      }

      if let Some(mtt) = get_table_type(metatable) {
        let Some(index_prop) = mtt.props.get("__index") else {
          return result;
        };

        if index_prop.is_write_only() {
          return TablePropLookupResult {
            blocked_types: Vec::new(),
            prop_type: Some(unsafe { (*self.builtin_types).error_type }),
            is_index: false,
          };
        }

        if let Some(index_type) = index_prop.read_ty {
          let index_type = follow_type_id(index_type);
          if let Some(ft) = get_type_id::<FunctionType>(index_type) {
            let rets = unsafe {
              extend_type_pack(
                // SAFETY: arena 在 solver 存活期内有效。
                &mut *self.arena,
                self.builtin_types,
                ft.ret_types,
                1,
                Vec::new(),
              )
            };
            return TablePropLookupResult {
              blocked_types: Vec::new(),
              prop_type: if rets.head.len() == 1 {
                Some(rets.head[0])
              } else {
                Some(unsafe { (*self.builtin_types).nil_type })
              },
              is_index: false,
            };
          }

          return unsafe {
            self.lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool_set_type_id(
                        constraint,
                        index_type,
                        prop_name,
                        context,
                        in_conditional,
                        suppress_simplification,
                        &mut seen,
                    )
          };
        }

        result
      } else if get_type_id::<MetatableType>(metatable).is_some() {
        unsafe {
          self
          .lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool_set_type_id(
            constraint,
            metatable,
            prop_name,
            context,
            in_conditional,
            suppress_simplification,
            &mut seen,
          )
        }
      } else {
        result
      }
    } else if let Some(cls) = get_type_id::<ExternType>(subject_type) {
      let prop_name_string = prop_name.to_string();
      if let Some(prop) = lookup_extern_type_prop(cls, &prop_name_string) {
        return TablePropLookupResult {
          blocked_types: Vec::new(),
          prop_type: if context == ValueContext::RValue {
            prop.read_ty
          } else {
            prop.write_ty
          },
          is_index: false,
        };
      }

      if let Some(indexer) = &cls.indexer {
        return TablePropLookupResult {
          blocked_types: Vec::new(),
          prop_type: Some(indexer.index_result_type),
          is_index: true,
        };
      }

      TablePropLookupResult {
        blocked_types: Vec::new(),
        prop_type: None,
        is_index: false,
      }
    } else if let Some(ft) = get_type_id::<FreeType>(subject_type) {
      let upper_bound = follow_type_id(ft.upper_bound);

      if get_type_id::<TableType>(upper_bound).is_some()
        || get_type_id::<PrimitiveType>(upper_bound).is_some()
      {
        let res = unsafe {
          self
          .lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool_set_type_id(
            constraint,
            upper_bound,
            prop_name,
            context,
            in_conditional,
            suppress_simplification,
            &mut seen,
          )
        };

        if res.prop_type.is_some() {
          return res;
        }
      }

      let scope = ft.scope;
      let new_upper_bound = unsafe {
        (*self.arena).add_type(TableType::table_type_table_state_type_level_scope(
          TableState::Free,
          TypeLevel::default(),
          scope,
        ))
      };

      track_interior_free_type(
        // SAFETY: constraint 由调用方保证有效（NotNull 语义）。
        unsafe { (*constraint).scope },
        new_upper_bound,
      );

      // C++ `LUAU_ASSERT(tt)`：刚 add_type 的 TableType 必然可变下转成功。
      let tt = get_mutable::<TableType>(new_upper_bound).expect("fresh table is TableType");

      let prop_type = fresh_type(
        // SAFETY: arena 在 solver 存活期内有效。
        unsafe { &mut *self.arena },
        // SAFETY: builtin_types 在 solver 存活期内有效。
        unsafe { &*self.builtin_types },
        scope,
        Polarity::Mixed,
      );
      track_interior_free_type(scope, prop_type);

      match context {
        ValueContext::RValue => {
          tt.props
            .insert(prop_name.to_string(), Property::readonly(prop_type));
        }
        ValueContext::LValue => {
          tt.props
            .insert(prop_name.to_string(), Property::rw_type_id(prop_type));
        }
      }

      self.constraint_solver_unify(constraint, subject_type, new_upper_bound);

      TablePropLookupResult {
        blocked_types: Vec::new(),
        prop_type: Some(prop_type),
        is_index: false,
      }
    } else if let Some(utv) = get_type_id::<UnionType>(subject_type) {
      let mut blocked = Vec::new();
      let mut options = Vec::new();

      for ty in &utv.options {
        let result = unsafe {
          self
          .lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool_set_type_id(
            constraint,
            *ty,
            prop_name,
            context,
            in_conditional,
            suppress_simplification,
            &mut seen,
          )
        };

        for blocked_ty in result.blocked_types {
          if !blocked.contains(&blocked_ty) {
            blocked.push(blocked_ty);
          }
        }

        if let Some(prop_type) = result.prop_type
          && !options.contains(&prop_type)
        {
          options.push(prop_type);
        }
      }

      if !blocked.is_empty() {
        return TablePropLookupResult {
          blocked_types: blocked,
          prop_type: None,
          is_index: false,
        };
      }

      if options.is_empty() {
        return TablePropLookupResult {
          blocked_types: Vec::new(),
          prop_type: None,
          is_index: false,
        };
      }

      if options.len() == 1 {
        return TablePropLookupResult {
          blocked_types: Vec::new(),
          prop_type: Some(options[0]),
          is_index: false,
        };
      }

      let prop_type = if options.len() == 2 && !suppress_simplification {
        let scope = unsafe { (*constraint).scope };
        let location = unsafe { (*constraint).location };
        if context == ValueContext::LValue {
          self.simplify_intersection_not_null_scope_location_type_id_type_id(
            scope, location, options[0], options[1],
          )
        } else {
          self.simplify_union(scope, location, options[0], options[1])
        }
      } else if context == ValueContext::LValue {
        unsafe { (*self.arena).add_type(IntersectionType { parts: options }) }
      } else {
        unsafe { (*self.arena).add_type(UnionType { options }) }
      };

      TablePropLookupResult {
        blocked_types: Vec::new(),
        prop_type: Some(prop_type),
        is_index: false,
      }
    } else if let Some(itv) = get_type_id::<IntersectionType>(subject_type) {
      let mut blocked = Vec::new();
      let mut options = Vec::new();

      for ty in &itv.parts {
        let result = unsafe {
          self
          .lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool_set_type_id(
            constraint,
            *ty,
            prop_name,
            context,
            in_conditional,
            suppress_simplification,
            &mut seen,
          )
        };

        for blocked_ty in result.blocked_types {
          if !blocked.contains(&blocked_ty) {
            blocked.push(blocked_ty);
          }
        }

        if let Some(prop_type) = result.prop_type
          && !options.contains(&prop_type)
        {
          options.push(prop_type);
        }
      }

      if !blocked.is_empty() {
        return TablePropLookupResult {
          blocked_types: blocked,
          prop_type: None,
          is_index: false,
        };
      }

      if options.is_empty() {
        return TablePropLookupResult {
          blocked_types: Vec::new(),
          prop_type: None,
          is_index: false,
        };
      }

      if options.len() == 1 {
        return TablePropLookupResult {
          blocked_types: Vec::new(),
          prop_type: Some(options[0]),
          is_index: false,
        };
      }

      let prop_type = if options.len() == 2 && !suppress_simplification {
        self.simplify_intersection_not_null_scope_location_type_id_type_id(
          unsafe { (*constraint).scope },
          unsafe { (*constraint).location },
          options[0],
          options[1],
        )
      } else {
        unsafe { (*self.arena).add_type(IntersectionType { parts: options }) }
      };

      TablePropLookupResult {
        blocked_types: Vec::new(),
        prop_type: Some(prop_type),
        is_index: false,
      }
    } else if let Some(pt) = get_type_id::<PrimitiveType>(subject_type) {
      if pt.r#type == PrimitiveType::STRING
        && let Some(metatable_id) = pt.metatable
      {
        let metatable = follow_type_id(metatable_id);
        // C++ `LUAU_ASSERT(metatableTable)`：string 元表必然是 TableType。
        let metatable_table = get_type_id::<TableType>(metatable).expect("metatable is table");
        if let Some(index_prop) = metatable_table.props.get("__index") {
          if index_prop.is_write_only() {
            return TablePropLookupResult {
              blocked_types: Vec::new(),
              prop_type: Some(unsafe { (*self.builtin_types).error_type }),
              is_index: false,
            };
          }

          if let Some(index_type) = index_prop.read_ty {
            return unsafe {
              self.lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool_set_type_id(
                            constraint,
                            index_type,
                            prop_name,
                            context,
                            in_conditional,
                            suppress_simplification,
                            &mut seen,
                        )
            };
          }
        }
      }

      if in_conditional && pt.r#type == PrimitiveType::TABLE {
        return TablePropLookupResult {
          blocked_types: Vec::new(),
          prop_type: Some(unsafe { (*self.builtin_types).unknown_type }),
          is_index: false,
        };
      }

      TablePropLookupResult {
        blocked_types: Vec::new(),
        prop_type: None,
        is_index: false,
      }
    } else {
      TablePropLookupResult {
        blocked_types: Vec::new(),
        prop_type: None,
        is_index: false,
      }
    }
  }
}
