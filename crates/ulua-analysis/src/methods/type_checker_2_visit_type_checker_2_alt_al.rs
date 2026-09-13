//! `TypeChecker2::visit(AstExprIndexExpr*, ValueContext)`（TypeChecker2.cpp 对照）。
use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_constant_string::AstExprConstantString,
    ast_expr_index_expr::AstExprIndexExpr, ast_node::AstNode,
  },
  rtti::ast_node_try_as,
};

use crate::{
  FFlag,
  enums::{value::Value, value_context::ValueContext},
  functions::{
    follow_type::follow_type_id, get_table_type::get_table_type, get_type_alt_j::get_type_id,
    is_optional::is_optional, should_suppress_errors_type_utils::should_suppress_errors,
  },
  records::{
    cannot_extend_table::{self, CannotExtendTable},
    dynamic_property_lookup_on_extern_types_unsafe::DynamicPropertyLookupOnExternTypesUnsafe,
    extern_type::ExternType,
    intersection_type::IntersectionType,
    metatable_type::MetatableType,
    never_type::NeverType,
    normalization_too_complex::NormalizationTooComplex,
    not_a_table::NotATable,
    optional_value_access::OptionalValueAccess,
    property_access_violation::{self, PropertyAccessViolation},
    table_type::TableType,
    type_checker_2::TypeChecker2,
    type_error_data::TypeErrorData,
    type_iterator::TypeIterator,
    union_type::UnionType,
  },
};
impl TypeChecker2 {
  pub fn visit_ast_expr_index_expr_value_context(
    &mut self,
    index_expr: &AstExprIndexExpr,
    context: ValueContext,
  ) {
    let index = index_expr.index;
    // SAFETY: AstExprConstantString 是 #[repr(C)] 单继承，AstNode 子对象在偏移 0。
    if let Some(index_as_constant_string) = ast_node_try_as::<AstExprConstantString>(unsafe {
      &*((index as *const AstExpr).cast::<AstNode>())
    }) {
      // SAFETY: index 指向 AST arena 节点，与 visit 树同寿。
      let ast_index_expr_type = self.lookup_type(unsafe { &*index });
      let string_value = index_as_constant_string
        .value
        .as_bytes()
        .iter()
        .map(|&b| b as char)
        .collect::<String>();

      // SAFETY: index_expr->expr 指向 AST arena 节点。
      unsafe {
        self.visit_expr_name(
          index_expr.expr,
          index_expr.base.base.location,
          &string_value,
          context,
          ast_index_expr_type,
        )
      };
      return;
    }

    self.visit_ast_expr_value_context(index_expr.expr, ValueContext::RValue);
    self.visit_ast_expr_value_context(index_expr.index, ValueContext::RValue);

    // SAFETY: expr/index 均指向 AST arena 节点。
    let expr_type = follow_type_id(self.lookup_type(unsafe { &*index_expr.expr }));
    let index_type = follow_type_id(self.lookup_type(unsafe { &*index_expr.index }));

    if let Some(tt) = get_type_id::<TableType>(expr_type) {
      if let Some(indexer) = &tt.indexer {
        // SAFETY: index 指向 AST arena 节点。
        self.test_is_subtype_type_id_type_id_location(index_type, indexer.index_type, unsafe {
          (*index_expr.index).base.location
        });
        if FFlag::LuauReadOnlyIndexers.get()
          && context == ValueContext::LValue
          && indexer.is_read_only
        {
          let err = PropertyAccessViolation {
            table: expr_type,
            key: "indexer".to_string(),
            context: property_access_violation::Context::CannotWrite,
          };
          self.report_error_type_error_data_location(
            TypeErrorData::PropertyAccessViolation(err),
            &index_expr.base.base.location,
          );
        }
      } else {
        let err = CannotExtendTable {
          table_type: expr_type,
          context: cannot_extend_table::Context::Indexer,
          prop: "indexer??".to_string(),
        };
        self.report_error_type_error_data_location(
          TypeErrorData::CannotExtendTable(err),
          &index_expr.base.base.location,
        );
      }
    } else if let Some(mt) = get_type_id::<MetatableType>(expr_type) {
      self.type_checker_2_index_expr_metatable_helper(index_expr, mt, expr_type, index_type);
    } else if let Some(cls) = get_type_id::<ExternType>(expr_type) {
      if let Some(indexer) = &cls.indexer {
        // SAFETY: index 指向 AST arena 节点。
        self.test_is_subtype_type_id_type_id_location(index_type, indexer.index_type, unsafe {
          (*index_expr.index).base.location
        });
      } else {
        let err = DynamicPropertyLookupOnExternTypesUnsafe { ty: expr_type };
        self.report_error_type_error_data_location(
          TypeErrorData::DynamicPropertyLookupOnExternTypesUnsafe(err),
          &index_expr.base.base.location,
        );
      }
    } else if get_type_id::<UnionType>(expr_type).is_some() && is_optional(expr_type) {
      // &mut → *mut 自动 coerce；should_suppress_errors 为 safe fn。
      let suppression = unsafe { should_suppress_errors(&mut self.normalizer, expr_type) };
      match suppression.value {
        Value::DoNotSuppress | Value::NormalizationFailed => {
          if matches!(suppression.value, Value::NormalizationFailed) {
            self.report_error_type_error_data_location(
              TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
              &index_expr.base.base.location,
            );
          }
          self.report_error_type_error_data_location(
            TypeErrorData::OptionalValueAccess(OptionalValueAccess {
              optional: expr_type,
            }),
            &index_expr.base.base.location,
          );
        }
        Value::Suppress => {}
      }
    } else if let Some(ut) = get_type_id::<UnionType>(expr_type) {
      // if all of the type_arguments are a table type, the union must be
      // a table, and so we shouldn't error.
      let mut all_tables = true;
      let mut union_iter = unsafe { TypeIterator::<UnionType>::type_iterator_type(ut) };
      let union_end = TypeIterator::<UnionType>::type_iterator_default();
      while union_iter.operator_ne(&union_end) {
        let ty = union_iter.operator_deref();
        union_iter.operator_inc();

        if get_table_type(ty).is_none() {
          all_tables = false;
          break;
        }
      }

      if !all_tables {
        let suppression = unsafe { should_suppress_errors(&mut self.normalizer, expr_type) };
        match suppression.value {
          Value::DoNotSuppress | Value::NormalizationFailed => {
            if matches!(suppression.value, Value::NormalizationFailed) {
              self.report_error_type_error_data_location(
                TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
                &index_expr.base.base.location,
              );
            }
            self.report_error_type_error_data_location(
              TypeErrorData::NotATable(NotATable { ty: expr_type }),
              &index_expr.base.base.location,
            );
          }
          Value::Suppress => {}
        }
      }
    } else if let Some(it) = get_type_id::<IntersectionType>(expr_type) {
      let mut any_table = false;
      let mut intersection_iter =
        unsafe { TypeIterator::<IntersectionType>::type_iterator_type(it) };
      let intersection_end = TypeIterator::<IntersectionType>::type_iterator_default();
      while intersection_iter.operator_ne(&intersection_end) {
        let part = intersection_iter.operator_deref();
        intersection_iter.operator_inc();

        if get_table_type(part).is_some() {
          any_table = true;
          break;
        }
      }

      if !any_table {
        self.report_error_type_error_data_location(
          TypeErrorData::NotATable(NotATable { ty: expr_type }),
          &index_expr.base.base.location,
        );
      }
    } else if get_type_id::<NeverType>(expr_type).is_some()
      || self.is_error_suppressing_location_type_id(index_expr.base.base.location, expr_type)
    {
      // Nothing
    } else {
      self.report_error_type_error_data_location(
        TypeErrorData::NotATable(NotATable { ty: expr_type }),
        &index_expr.base.base.location,
      );
    }
  }
}
