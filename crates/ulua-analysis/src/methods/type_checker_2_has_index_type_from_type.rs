//! Faithful port of `TypeChecker2::hasIndexTypeFromType` (TypeChecker2.cpp:3727-3859).
/// Faithful port of the local `struct PropertyType` from TypeChecker2.cpp:116-120
/// (`NormalizationResult present; std::optional<TypeId> result;`).
///
/// The shared `records::property_type::PropertyType` slot was repurposed as an
/// alias of `Property` (read/write types), so the result type of
/// `hasIndexTypeFromType` is defined locally to match the reference exactly.
use alloc::vec::Vec;

use ulua_ast::records::location::Location;
use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT, records::dense_hash_set::DenseHashSet};

use crate::{
  enums::{normalization_result::NormalizationResult, value_context::ValueContext},
  functions::{
    arc_as_mut::arc_as_mut,
    begin_type::{begin_intersection_type, begin_union_type},
    find_metatable_entry::find_metatable_entry,
    find_table_property_respecting_meta_type_utils::find_table_property_respecting_meta,
    follow_type,
    get_table_type::get_table_type,
    get_type,
    in_conditional::in_conditional,
    is_string::is_string,
    lookup_extern_type_prop::lookup_extern_type_prop,
  },
  records::{
    any_type::AnyType, arena_handle::Handle, extern_type::ExternType,
    intersection_type::IntersectionType, never_type::NeverType, primitive_type::PrimitiveType,
    singleton_type::SingletonType, string_singleton::StringSingleton, table_type::TableType,
    type_checker_2::TypeChecker2, type_error::TypeError, union_type::UnionType,
  },
  type_aliases::{error_type::ErrorType, singleton_variant::SingletonVariant, type_id::TypeId},
};
#[derive(Debug, Clone)]
pub struct PropertyType {
  pub present: NormalizationResult,
  pub result: Option<TypeId>,
}

impl PropertyType {
  /// `NormalizationResult::True` + 无结果（seen 循环守卫的返回）。
  const TRUE_NONE: PropertyType = PropertyType {
    present: NormalizationResult::True,
    result: None,
  };
  const FALSE_NONE: PropertyType = PropertyType {
    present: NormalizationResult::False,
    result: None,
  };
}

impl TypeChecker2 {
  pub fn has_index_type_from_type(
    &mut self,
    ty: TypeId,
    prop: &str,
    context: ValueContext,
    location: &Location,
    seen: &mut DenseHashSet<TypeId>,
    ast_index_expr_type: TypeId,
    errors: &mut Vec<TypeError>,
  ) -> PropertyType {
    let mut ty = follow_type::follow(ty);

    // If we have already encountered this type, we must assume that some
    // other codepath will do the right thing and signal false if the
    // property is not present.
    if seen.contains(&ty) {
      return PropertyType::TRUE_NONE;
    }
    seen.insert(ty);

    if get_type::get::<ErrorType>(ty).is_some()
      || get_type::get::<AnyType>(ty).is_some()
      || get_type::get::<NeverType>(ty).is_some()
    {
      return PropertyType {
        present: NormalizationResult::True,
        result: Some(ty),
      };
    }

    if is_string(ty) {
      // SAFETY: builtin_types 由构造方保证有效（C++ 同契约）。
      let string_type = self.builtin_types_ref().string_type;
      let mt_index = find_metatable_entry(
        Handle::from_ptr(self.builtin_types.as_ptr()),
        errors,
        string_type,
        "__index",
        *location,
      );
      LUAU_ASSERT!(mt_index.is_some());
      // 紧邻 LUAU_ASSERT 蕴含 Some。
      ty = mt_index.expect("紧邻 LUAU_ASSERT(mt_index.is_some()) 蕴含");
    }

    if let Some(tt) = get_table_type(ty) {
      if let Some(res_ty) = find_table_property_respecting_meta(
        Handle::from_ptr(self.builtin_types.as_ptr()),
        errors,
        ty,
        prop,
        context,
        *location,
        /* useNewSolver */ true,
      ) {
        return PropertyType {
          present: NormalizationResult::True,
          result: Some(res_ty),
        };
      }

      if let Some(indexer) = &tt.indexer {
        let index_type = follow_type::follow(indexer.index_type);
        // SAFETY: self.module 与会话同寿；&mut self 下经裸指针 place 写
        // internal_types（C++ 直接经 module->internalTypes）。
        let given_type = unsafe {
          (*self.module)
            .internal_types
            .add_type(SingletonType::new(SingletonVariant::V1(
              StringSingleton::new(prop.to_owned()),
            )))
        };
        // SAFETY: 同上。
        let module_scope = unsafe { (*self.module).get_module_scope() };
        let scope = arc_as_mut(&module_scope);
        // subtyping 已句柄化，判空/解引用契约收进 `subtyping_mut` 访问器（C++ 同契约）。
        let key_matches = self
          .subtyping_mut()
          .is_subtype_type_id_type_id_not_null_scope(given_type, index_type, scope)
          .is_subtype;

        if key_matches {
          if fflag::LuauReadOnlyIndexers.get()
            && context == ValueContext::LValue
            && indexer.is_read_only
          {
            return PropertyType::FALSE_NONE;
          }
          return PropertyType {
            present: NormalizationResult::True,
            result: Some(indexer.index_result_type),
          };
        }
      }

      PropertyType {
        present: NormalizationResult::False,
        // SAFETY: builtin_types 由构造方保证有效（C++ 同契约）。
        result: Some(self.builtin_types_ref().unknown_type),
      }
    } else if let Some(cls) = get_type::get::<ExternType>(ty) {
      // If the property doesn't exist on the class, we consult the indexer.
      if let Some(property) = lookup_extern_type_prop(cls, prop) {
        if (context == ValueContext::LValue && property.write_ty.is_none())
          || (context == ValueContext::RValue && property.read_ty.is_none())
        {
          return PropertyType::FALSE_NONE;
        }
        return PropertyType {
          present: NormalizationResult::True,
          result: if context == ValueContext::LValue {
            property.write_ty
          } else {
            property.read_ty
          },
        };
      }
      if let Some(indexer) = &cls.indexer {
        // SAFETY: self.module 同上。
        let inhabited_test_type = unsafe {
          (*self.module).internal_types.add_type(IntersectionType {
            parts: alloc::vec![indexer.index_type, ast_index_expr_type],
          })
        };
        return PropertyType {
          present: self.normalizer.is_inhabited_type_id(inhabited_test_type),
          result: Some(indexer.index_result_type),
        };
      }

      if fflag::DebugLuauUserDefinedClasses.get()
        && let Some(metatable) = cls.metatable
      {
        // For user-defined classes, the object metatable holds metamethods
        // (e.g. __add) directly in its props rather than under an __index table.
        if let Some(mtt) = get_type::get::<TableType>(follow_type::follow(metatable))
          && let Some(mt_prop) = mtt.props.get(prop)
        {
          if (context == ValueContext::LValue && mt_prop.write_ty.is_none())
            || (context == ValueContext::RValue && mt_prop.read_ty.is_none())
          {
            return PropertyType::FALSE_NONE;
          }
          return PropertyType {
            present: NormalizationResult::True,
            result: if context == ValueContext::LValue {
              mt_prop.write_ty
            } else {
              mt_prop.read_ty
            },
          };
        }
      }

      PropertyType::FALSE_NONE
    } else if let Some(utv) = get_type::get::<UnionType>(ty) {
      let mut parts: Vec<TypeId> = Vec::with_capacity(utv.options.len());

      // C++ `for (TypeId part : utv)` — UnionTypeIterator 防环展平并 follow。
      for part in begin_union_type(utv) {
        let result = self.has_index_type_from_type(
          part,
          prop,
          context,
          location,
          seen,
          ast_index_expr_type,
          errors,
        );

        if result.present != NormalizationResult::True {
          return PropertyType {
            present: result.present,
            result: None,
          };
        }
        if let Some(r) = result.result {
          parts.push(r);
        }
      }

      if parts.is_empty() {
        return PropertyType::FALSE_NONE;
      }

      if let [only] = parts.as_slice() {
        return PropertyType {
          present: NormalizationResult::True,
          result: Some(*only),
        };
      }

      // SAFETY: self.module 同上。
      let prop_ty = if context == ValueContext::LValue {
        unsafe {
          (*self.module)
            .internal_types
            .add_type(IntersectionType { parts })
        }
      } else {
        unsafe {
          (*self.module)
            .internal_types
            .add_type(UnionType { options: parts })
        }
      };

      PropertyType {
        present: NormalizationResult::True,
        result: Some(prop_ty),
      }
    } else if let Some(itv) = get_type::get::<IntersectionType>(ty) {
      // C++ `for (TypeId part : itv)` — IntersectionTypeIterator 防环展平并 follow
      // Bound,裸遍历 parts 会漏掉嵌套 intersection。
      for part in begin_intersection_type(itv) {
        let result = self.has_index_type_from_type(
          part,
          prop,
          context,
          location,
          seen,
          ast_index_expr_type,
          errors,
        );
        if result.present != NormalizationResult::False {
          return result;
        }
      }

      PropertyType::FALSE_NONE
    } else if let Some(pt) = get_type::get::<PrimitiveType>(ty) {
      PropertyType {
        present: if in_conditional(self.type_context) && pt.r#type == PrimitiveType::TABLE {
          NormalizationResult::True
        } else {
          NormalizationResult::False
        },
        result: Some(ty),
      }
    } else {
      PropertyType::FALSE_NONE
    }
  }
}
