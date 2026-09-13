//! Node: `cxx:Method:Luau.Analysis:Analysis/src/Subtyping.cpp:1909:subtyping_is_covariant_with`
//! Source: `Analysis/src/Subtyping.cpp:1909-2052` (hand-ported)
//!
//! C++ `SubtypingResult Subtyping::isCovariantWith(env, subTable, superTable,
//! forceCovariantTest, scope)` — the "better error suppression" table-vs-table
//! covariance state machine selected when
//! `FFlag::LuauSubtypingTablesHasBetterErrorSuppression` is on. The legacy
//! branch lives in `is_covariant_with_deprecated`.

use alloc::string::ToString;

use ulua_common::FFlag;

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

/// C++ local `auto record = [&](SubtypingResult subResult) { ... }`
/// (Subtyping.cpp:1951-1956). Tracks the error-suppression bookkeeping and
/// folds `subResult` into `result` using the default `andAlso` policy (`Any`).
fn record(
  result: &mut SubtypingResult,
  has_error_suppression: &mut bool,
  should_suppress_errors: &mut bool,
  sub_result: SubtypingResult,
) {
  *has_error_suppression |= sub_result.is_error_suppressing;
  *should_suppress_errors &= sub_result.is_subtype || sub_result.is_error_suppressing;
  result.and_also(sub_result, SubtypingSuppressionPolicy::Any);
}

impl Subtyping {
  pub fn is_covariant_with_subtyping_environment_table_type_table_type_bool_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_table: &TableType,
    super_table: &TableType,
    force_covariant_test: bool,
    scope: *mut Scope,
  ) -> SubtypingResult {
    let mut result = SubtypingResult {
      is_subtype: true,
      ..Default::default()
    };

    if sub_table.props.is_empty()
      && sub_table.indexer.is_none()
      && sub_table.state == TableState::Sealed
      && super_table.indexer.is_some()
    {
      // While it is certainly the case that {} </: {T}, the story is a
      // little bit different for {| |} <: {T}: an unsealed table will
      // later gain the necessary indexer as inference proceeds.
      return SubtypingResult {
        is_subtype: false,
        ..Default::default()
      };
    }

    // This is an unfortunately complicated state machine: an `any`-typed
    // property must be error-suppressing without surfacing a hard error,
    // but a genuinely mismatched property (e.g. `number != boolean`) must
    // still report even when another property suppressed.
    let mut has_error_suppression = false;
    let mut should_suppress_errors = true;

    for (name, super_prop) in &super_table.props {
      // If the sub table has the property with the specific name: check
      // whether the two are invariant subtypes.
      if let Some(sub_prop) = sub_table.props.get(name) {
        let sub_result = self
          .is_covariant_with_subtyping_environment_property_property_string_bool_not_null_scope(
            env,
            sub_prop,
            super_prop,
            name,
            force_covariant_test,
            scope,
          );
        record(
          &mut result,
          &mut has_error_suppression,
          &mut should_suppress_errors,
          sub_result,
        );
      }
      // Otherwise, if the sub table has an indexer whose key type is a
      // super type of string, then use that as the "property" type, e.g.
      // `{ [string]: number } <: { foo: number }`.
      else if let Some(sub_indexer) = sub_table.indexer.as_ref()
        && unsafe {
          self
            .is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
              env,
              (*self.builtin_types).string_type,
              sub_indexer.index_type,
              scope,
            )
            .is_subtype
        }
      {
        if super_prop.is_shared() {
          if FFlag::LuauReadOnlyIndexers.get() && sub_indexer.is_read_only {
            // A read-only indexer cannot satisfy a read-write
            // property requirement.
            let mut sr = SubtypingResult {
              is_subtype: false,
              ..Default::default()
            };
            sr.with_sub_component(index_result_component());
            sr.with_super_component(path_property(name, true));
            record(
              &mut result,
              &mut has_error_suppression,
              &mut should_suppress_errors,
              sr,
            );
          } else {
            let mut sr = self
              .is_invariant_with_subtyping_environment_sub_ty_super_ty_not_null_scope(
                env,
                sub_indexer.index_result_type,
                super_prop.read_ty.unwrap(),
                scope,
              );
            sr.with_sub_component(index_result_component());
            sr.with_super_component(path_property(name, true));
            record(
              &mut result,
              &mut has_error_suppression,
              &mut should_suppress_errors,
              sr,
            );
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
            record(
              &mut result,
              &mut has_error_suppression,
              &mut should_suppress_errors,
              sr,
            );
          }
          if let Some(super_write_ty) = super_prop.write_ty {
            if FFlag::LuauReadOnlyIndexers.get() && sub_indexer.is_read_only {
              let mut sr = SubtypingResult {
                is_subtype: false,
                ..Default::default()
              };
              sr.with_sub_component(index_result_component());
              sr.with_super_component(path_property(name, false));
              record(
                &mut result,
                &mut has_error_suppression,
                &mut should_suppress_errors,
                sr,
              );
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
              record(
                &mut result,
                &mut has_error_suppression,
                &mut should_suppress_errors,
                sr,
              );
            }
          }
        }
      } else if FFlag::LuauSubtypingMissingPropertiesAsNil.get() {
        let nil_prop = unsafe { Property::readonly((*self.builtin_types).nil_type) };
        let mut sr = self
          .is_covariant_with_subtyping_environment_property_property_string_bool_not_null_scope(
            env,
            &nil_prop,
            super_prop,
            name,
            force_covariant_test,
            scope,
          );
        // We must ignore the reasoning from here because the subtype
        // doesn't have a property to traverse into later.
        sr.reasoning.clear();
        record(
          &mut result,
          &mut has_error_suppression,
          &mut should_suppress_errors,
          sr,
        );
      }
      // If the subtable doesn't have a string indexer and the required
      // property does not exist, we can exit early.
      else {
        return SubtypingResult {
          is_subtype: false,
          ..Default::default()
        };
      }
    }

    if let Some(super_indexer) = &super_table.indexer {
      if let Some(sub_indexer) = &sub_table.indexer {
        let sub_result = if FFlag::LuauReadOnlyIndexers.get() {
          // isCovariantWith() properly handles variance of the index
          // result type.
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
        record(
          &mut result,
          &mut has_error_suppression,
          &mut should_suppress_errors,
          sub_result,
        );
      } else if sub_table.state != TableState::Sealed {
        // As above, we assume that {| |} <: {T} because the unsealed
        // table on the left will eventually gain the necessary indexer.
        return SubtypingResult {
          is_subtype: true,
          ..Default::default()
        };
      } else {
        return SubtypingResult {
          is_subtype: false,
          ..Default::default()
        };
      }
    }

    result.is_error_suppressing = has_error_suppression && should_suppress_errors;
    result
  }
}
