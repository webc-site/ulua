use alloc::{string::ToString, vec::Vec};

use ulua_common::fflag;

use crate::{
  enums::{
    subtyping_suppression_policy::SubtypingSuppressionPolicy, table_state::TableState,
    type_field::TypeField,
  },
  records::{
    property_type::Property, property_type_path::Property as PathProperty, scope::Scope,
    subtyping::Subtyping, subtyping_environment::SubtypingEnvironment,
    subtyping_result::SubtypingResult, table_type::TableType,
  },
  type_aliases::component::Component,
};

fn path_property(name: &str, is_read: bool) -> Component {
  Component::Property(PathProperty {
    name: name.to_string(),
    is_read,
  })
}

fn index_result_component() -> Component {
  Component::TypeField(TypeField::IndexResult)
}

impl Subtyping {
  pub fn is_covariant_with_deprecated(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_table: &TableType,
    super_table: &TableType,
    force_covariant_test: bool,
    scope: *mut Scope,
  ) -> SubtypingResult {
    let mut result = SubtypingResult::ok();

    if sub_table.props.is_empty()
      && sub_table.indexer.is_none()
      && sub_table.state == TableState::Sealed
      && super_table.indexer.is_some()
    {
      return SubtypingResult::fail();
    }

    for (name, super_prop) in &super_table.props {
      let mut results = Vec::new();

      if let Some(sub_prop) = sub_table.props.get(name) {
        results.push(
          self
            .is_covariant_with_subtyping_environment_property_property_string_bool_not_null_scope(
              env,
              sub_prop,
              super_prop,
              name,
              force_covariant_test,
              scope,
            ),
        );
      } else if let Some(sub_indexer) = &sub_table.indexer {
        // Safety: self.builtin_types.as_ptr() 为 Subtyping 构造期接线的 BuiltinTypes 单例
        // 指针（直译 C++ NotNull 引用），非空且活过整次子类型检查；此处仅读出
        // string_type 这一 Copy 句柄并提前拷贝，随后对 safe fn 的调用与求值顺序
        // 不变（unsafe 窗收敛到该次读取）。
        let string_type = self.builtin_types.get().string_type;
        let can_index_by_string = self
          .is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
            env,
            string_type,
            sub_indexer.index_type,
            scope,
          )
          .is_subtype;

        if can_index_by_string {
          if super_prop.is_shared() {
            if fflag::LuauReadOnlyIndexers.get() && sub_indexer.is_read_only {
              let mut sr = SubtypingResult::fail();
              sr.with_sub_component(index_result_component());
              sr.with_super_component(path_property(name, true));
              results.push(sr);
            } else {
              let mut sr = self
                .is_invariant_with_subtyping_environment_sub_ty_super_ty_not_null_scope(
                  env,
                  sub_indexer.index_result_type,
                  // 旧求解器路径：indexer 转 Property 时 read_ty 恒为 Some
                  // （cpp `*superProp.readTy` 同前提直解）。
                  super_prop
                    .read_ty
                    .expect("旧求解器 indexer Property 的 read_ty 恒 Some"),
                  scope,
                );
              sr.with_sub_component(index_result_component());
              sr.with_super_component(path_property(name, true));
              results.push(sr);
            }
          } else {
            if let Some(super_read_ty) = super_prop.read_ty {
              let mut sr = self
                .is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
                  env,
                  sub_indexer.index_result_type,
                  super_read_ty,
                  scope,
                );
              sr.with_sub_component(index_result_component());
              sr.with_super_component(path_property(name, true));
              results.push(sr);
            }

            if let Some(super_write_ty) = super_prop.write_ty {
              if fflag::LuauReadOnlyIndexers.get() && sub_indexer.is_read_only {
                let mut sr = SubtypingResult::fail();
                sr.with_sub_component(index_result_component());
                sr.with_super_component(path_property(name, false));
                results.push(sr);
              } else {
                let mut sr = self
                  .is_contravariant_with_subtyping_environment_sub_ty_super_ty_not_null_scope(
                    env,
                    sub_indexer.index_result_type,
                    super_write_ty,
                    scope,
                  );
                sr.with_sub_component(index_result_component());
                sr.with_super_component(path_property(name, false));
                results.push(sr);
              }
            }
          }
        }
      } else if fflag::LuauSubtypingMissingPropertiesAsNil.get() {
        // Safety: 同一 builtin_types 构造不变量（非空、指向存活只读单例），
        // 此处仅读取 nil_type Copy 句柄构造局部 Property，无并存可变借用。
        let nil_prop = { Property::readonly(self.builtin_types.get().nil_type) };
        let mut sr = self
          .is_covariant_with_subtyping_environment_property_property_string_bool_not_null_scope(
            env,
            &nil_prop,
            super_prop,
            name,
            force_covariant_test,
            scope,
          );
        sr.reasoning.clear();
        results.push(sr);
      }

      if results.is_empty() {
        return SubtypingResult::fail();
      }

      let is_subtype = results.iter().all(|sr| sr.is_subtype);

      if result.is_subtype && !is_subtype {
        for sr in results {
          result.and_also(sr, SubtypingSuppressionPolicy::Any);
        }
      } else {
        for sr in results {
          result.and_also(sr, SubtypingSuppressionPolicy::All);
        }
      }
    }

    if let Some(super_indexer) = &super_table.indexer {
      if let Some(sub_indexer) = &sub_table.indexer {
        let indexer_result = if fflag::LuauReadOnlyIndexers.get() {
          self.is_covariant_with_subtyping_environment_table_indexer_table_indexer_not_null_scope(
            env,
            sub_indexer,
            super_indexer,
            scope,
          )
        } else {
          self.is_invariant_with_subtyping_environment_sub_ty_super_ty_not_null_scope(
            env,
            *sub_indexer,
            *super_indexer,
            scope,
          )
        };
        result.and_also(indexer_result, SubtypingSuppressionPolicy::All);
      } else if sub_table.state != TableState::Sealed {
        return SubtypingResult::ok();
      } else {
        return SubtypingResult::fail();
      }
    }

    result
  }
}
