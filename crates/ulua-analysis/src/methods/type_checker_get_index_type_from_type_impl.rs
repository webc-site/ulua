use alloc::{string::String, vec::Vec};

use ulua_ast::records::location::Location;
use ulua_common::{fint, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::table_state::TableState,
  functions::{
    begin_type::begin_union_type, follow_type, get_mutable_table_type::get_mutable_table_type,
    get_type, is_string::is_string, lookup_extern_type_prop::lookup_extern_type_prop,
    reduce_union::reduce_union,
  },
  records::{
    any_type::AnyType, extern_type::ExternType, intersection_type::IntersectionType,
    missing_union_property::MissingUnionProperty, never_type::NeverType, property_type::Property,
    recursion_limiter::RecursionLimiter, type_checker::TypeChecker, union_type::UnionType,
    unknown_property::UnknownProperty,
  },
  type_aliases::{
    error_type::ErrorType, name_type::Name, scope_ptr_type::ScopePtr,
    type_error_data::TypeErrorData, type_id::TypeId,
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
    let mut ty = follow_type::follow(ty);

    if get_type::get::<ErrorType>(ty).is_some()
      || get_type::get::<AnyType>(ty).is_some()
      || get_type::get::<NeverType>(ty).is_some()
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
      // 紧邻 LUAU_ASSERT 蕴含 Some。
      ty = mt_index.expect("紧邻 LUAU_ASSERT(mt_index.is_some()) 蕴含");
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
    } else if let Some(cls) = get_type::get::<ExternType>(ty) {
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
    } else if let Some(utv) = get_type::get::<UnionType>(ty) {
      let mut good_options: Vec<TypeId> = Vec::new();
      let mut bad_options: Vec<TypeId> = Vec::new();

      // C++ `for (TypeId t : utv)`——UnionTypeIterator 展平嵌套 union 并
      // follow，裸遍历 options 会漏掉嵌套成员。
      for t in begin_union_type(utv) {
        // 上游 `RecursionLimiter _rl("TypeInfer::UnionType", &recursionCount, limit)`
        // （TypeInfer.cpp:2168）：构造即 +1，离开本轮迭代由 Drop 回收。
        let _rl = RecursionLimiter::new(
          "TypeInfer::UnionType",
          &mut self.recursion_count,
          fint::LuauTypeInferRecursionLimit.get(),
        );

        // Not needed when we normalize type_arguments.
        if get_type::get::<AnyType>(follow_type::follow(t)).is_some() {
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

      // 上游 `return addType(UnionType{std::move(result)})`（TypeInfer.cpp:2199）。
      return Some(self.add_type(&UnionType { options: result }));
    } else if let Some(itv) = get_type::get::<IntersectionType>(ty) {
      let mut parts: Vec<TypeId> = Vec::new();

      for &t in itv.parts.iter() {
        // 同上：上游在 IntersectionType 分支也建了一个 RecursionLimiter。
        let _rl = RecursionLimiter::new(
          "TypeInfer::IntersectionType",
          &mut self.recursion_count,
          fint::LuauTypeInferRecursionLimit.get(),
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

      // 上游 `addType(IntersectionType{...})`（TypeInfer.cpp:2224），
      // 注释里的 "Not at all correct." 是上游原文。
      return Some(self.add_type(&IntersectionType { parts }));
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
