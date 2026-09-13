//! `TypeChecker2::checkIndexTypeFromType`（TypeChecker2.cpp 对照）。
use alloc::{string::String, vec::Vec};
use core::ptr::null;

use ulua_ast::records::location::Location;
use ulua_common::{FFlag, records::dense_hash_set::DenseHashSet};

use crate::{
  enums::{normalization_result::NormalizationResult, value_context::ValueContext},
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{
    cannot_extend_table::{CannotExtendTable, Context as CannotExtendTableContext},
    extern_type::ExternType,
    function_type::FunctionType,
    missing_union_property::MissingUnionProperty,
    normalization_too_complex::NormalizationTooComplex,
    not_a_table::NotATable,
    primitive_type::PrimitiveType,
    property_access_violation::{
      Context as PropertyAccessViolationContext, PropertyAccessViolation,
    },
    type_checker_2::TypeChecker2,
    type_iterator::TypeIterator,
    union_type::UnionType,
    unknown_property::UnknownProperty,
  },
  type_aliases::{type_error_data::TypeErrorData, type_id::TypeId},
};
impl TypeChecker2 {
  pub fn check_index_type_from_type(
    &mut self,
    table_ty: TypeId,
    prop: &str,
    context: ValueContext,
    location: Location,
    ast_index_expr_type: TypeId,
  ) {
    let table_ty = follow_type_id(table_ty);

    let mut has_cyclic_union = false;
    let mut union_seen: DenseHashSet<TypeId> = DenseHashSet::new(null());
    let mut union_stack = Vec::from([table_ty]);
    while let Some(candidate) = union_stack.pop() {
      let candidate = follow_type_id(candidate);
      let Some(union) = get_type_id::<UnionType>(candidate) else {
        continue;
      };

      if union_seen.contains(&candidate) {
        has_cyclic_union = true;
        break;
      }

      union_seen.insert(candidate);

      for &option in &union.options {
        if get_type_id::<UnionType>(follow_type_id(option)).is_some() {
          union_stack.push(option);
        }
      }
    }

    if !has_cyclic_union {
      let norm = self.normalizer.normalize(table_ty);
      if norm.should_suppress_errors() {
        return;
      }
    }

    let prop = String::from(prop);
    let mut seen = DenseHashSet::new(null());
    let mut errors = Vec::new();
    let prop_type = self.has_index_type_from_type(
      table_ty,
      &prop,
      context,
      &location,
      &mut seen,
      ast_index_expr_type,
      &mut errors,
    );
    self.report_errors(errors);

    if prop_type.present == NormalizationResult::HitLimits {
      self.report_error_type_error_data_location(
        TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
        &location,
      );
      return;
    }

    if prop_type.present != NormalizationResult::False {
      return;
    }

    if let Some(union) = get_type_id::<UnionType>(table_ty) {
      let mut found_prop = false;
      let mut missing_prop = Vec::new();
      let mut hit_limits = false;

      let mut it = unsafe { TypeIterator::<UnionType>::type_iterator_type(union) };
      let end_it = TypeIterator::<UnionType>::type_iterator_default();
      while it.operator_ne(&end_it) {
        let option = it.operator_deref();
        it.operator_inc();

        let mut option_seen = DenseHashSet::new(null());
        let mut option_errors = Vec::new();
        let option_prop = self.has_index_type_from_type(
          option,
          &prop,
          context,
          &location,
          &mut option_seen,
          ast_index_expr_type,
          &mut option_errors,
        );
        self.report_errors(option_errors);

        if option_prop.present == NormalizationResult::HitLimits {
          hit_limits = true;
          break;
        } else if option_prop.present == NormalizationResult::True && option_prop.result.is_some() {
          found_prop = true;
        } else if option_prop.present == NormalizationResult::False {
          missing_prop.push(option);
        }
      }

      if hit_limits {
        self.report_error_type_error_data_location(
          TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
          &location,
        );
        return;
      }

      if found_prop && !missing_prop.is_empty() {
        self.report_error_type_error_data_location(
          TypeErrorData::MissingUnionProperty(MissingUnionProperty {
            r#type: table_ty,
            missing: missing_prop,
            key: prop,
          }),
          &location,
        );
        return;
      }
    }

    if context == ValueContext::LValue {
      let mut seen_read = DenseHashSet::new(null());
      let mut dummy = Vec::new();
      let read_prop = self.has_index_type_from_type(
        table_ty,
        &prop,
        ValueContext::RValue,
        &location,
        &mut seen_read,
        ast_index_expr_type,
        &mut dummy,
      );

      if read_prop.present == NormalizationResult::True && read_prop.result.is_some() {
        self.report_error_type_error_data_location(
          TypeErrorData::PropertyAccessViolation(PropertyAccessViolation {
            table: table_ty,
            key: prop,
            context: PropertyAccessViolationContext::CannotWrite,
          }),
          &location,
        );
      } else if get_type_id::<PrimitiveType>(table_ty).is_some()
        || get_type_id::<FunctionType>(table_ty).is_some()
      {
        self.report_error_type_error_data_location(
          TypeErrorData::NotATable(NotATable { ty: table_ty }),
          &location,
        );
      } else if let Some(extern_type) = get_type_id::<ExternType>(table_ty) {
        if !FFlag::LuauTweakAccessViolationReporting.get() || extern_type.indexer.is_some() {
          self.report_error_type_error_data_location(
            TypeErrorData::UnknownProperty(UnknownProperty {
              table: table_ty,
              key: prop,
            }),
            &location,
          );
        } else {
          self.report_error_type_error_data_location(
            TypeErrorData::PropertyAccessViolation(PropertyAccessViolation {
              table: table_ty,
              key: prop,
              context: PropertyAccessViolationContext::CannotWrite,
            }),
            &location,
          );
        }
      } else {
        self.report_error_type_error_data_location(
          TypeErrorData::CannotExtendTable(CannotExtendTable {
            table_type: table_ty,
            context: CannotExtendTableContext::Property,
            prop,
          }),
          &location,
        );
      }
    } else {
      let mut seen_write = DenseHashSet::new(null());
      let mut dummy = Vec::new();
      let write_prop = self.has_index_type_from_type(
        table_ty,
        &prop,
        ValueContext::LValue,
        &location,
        &mut seen_write,
        ast_index_expr_type,
        &mut dummy,
      );

      if write_prop.present == NormalizationResult::True && write_prop.result.is_some() {
        self.report_error_type_error_data_location(
          TypeErrorData::PropertyAccessViolation(PropertyAccessViolation {
            table: table_ty,
            key: prop,
            context: PropertyAccessViolationContext::CannotRead,
          }),
          &location,
        );
      } else {
        self.report_error_type_error_data_location(
          TypeErrorData::UnknownProperty(UnknownProperty {
            table: table_ty,
            key: prop,
          }),
          &location,
        );
      }
    }
  }
}
