//! Faithful port of `TypeChecker2::visit(AstStatLocal*)` (TypeChecker2.cpp:791-859).
use alloc::string::String;
use core::{cmp::max, ptr::null_mut};

use ulua_ast::records::{
  ast_expr_call::AstExprCall, ast_expr_varargs::AstExprVarargs, ast_node::AstNode,
  ast_stat_local::AstStatLocal, location::Location,
};

use crate::{
  enums::value_context::ValueContext,
  functions::{extend_type_pack::extend_type_pack, follow_type::follow_type_id},
  records::{count_mismatch::CountMismatch, type_checker_2::TypeChecker2, type_pack::TypePack},
  type_aliases::type_error_data::TypeErrorData,
};
impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证 `local` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_local(&mut self, local: *mut AstStatLocal) {
    unsafe {
      let values = (*local).values;
      let vars = (*local).vars;

      let count = max(values.size, vars.size);
      for i in 0..count {
        let value = if i < values.size {
          *values.data.add(i)
        } else {
          null_mut()
        };
        let is_pack = !value.is_null()
          && ((*(value as *mut AstNode)).is::<AstExprCall>()
            || (*(value as *mut AstNode)).is::<AstExprVarargs>());

        if !value.is_null() {
          self.visit_ast_expr_value_context(value, ValueContext::RValue);
        }

        // values.size is usize; `values.size - 1` matches C++ where size is
        // never 0 in this branch when i could equal size-1.
        if i != values.size.wrapping_sub(1) || !is_pack {
          let var = if i < vars.size {
            *vars.data.add(i)
          } else {
            null_mut()
          };

          if !var.is_null() && !(*var).annotation.is_null() {
            // SAFETY: annotation/value 指向 AST arena 节点。
            let annotation_type = self.lookup_annotation(&*(*var).annotation);
            let value_type = if !value.is_null() {
              Some(self.lookup_type(&*value))
            } else {
              None
            };
            if let Some(value_type) = value_type {
              let duplicate_solver_mismatch = (*self.module).errors.iter().position(|error| {
                if let TypeErrorData::TypeMismatch(mismatch) = &error.data {
                  follow_type_id(mismatch.wanted_type) == follow_type_id(annotation_type)
                    && follow_type_id(mismatch.given_type) == follow_type_id(value_type)
                } else {
                  false
                }
              });

              if let Some(index) = duplicate_solver_mismatch {
                (*self.module).errors.remove(index);
              }
              // SAFETY: value 指向 AST arena 节点。
              self.test_potential_literal_is_subtype(&*value, annotation_type);
            }

            self.visit_ast_type((*var).annotation);
          }
        } else if !value.is_null() {
          let value_pack = self.lookup_pack(value);
          let mut value_types = TypePack {
            head: Vec::new(),
            tail: None,
          };
          if i < vars.size {
            let builtin_types = self.builtin_types;
            value_types = extend_type_pack(
              &mut (*self.module).internal_types,
              builtin_types,
              value_pack,
              vars.size - i,
              Vec::new(),
            );
          }

          let mut error_location = Location::default();
          let mut j = i;
          while j < vars.size {
            if j - i >= value_types.head.len() {
              error_location = (*(*vars.data.add(j))).location;
              break;
            }

            let var = *vars.data.add(j);
            if !(*var).annotation.is_null() {
              // SAFETY: annotation 指向 AST arena 节点。
              let var_type = self.lookup_annotation(&*(*var).annotation);
              self.test_is_subtype_type_id_type_id_location(
                value_types.head[j - i],
                var_type,
                (*value).base.location,
              );

              self.visit_ast_type((*var).annotation);
            }
            j += 1;
          }

          let remaining_vars = vars.size.saturating_sub(i);
          if value_types.head.len() < remaining_vars {
            let last_value = *values.data.add(values.size - 1);
            let kind = if (*(last_value as *mut AstNode)).is::<AstExprCall>() {
              CountMismatch::FUNCTION_RESULT
            } else {
              CountMismatch::EXPR_LIST_RESULT
            };
            self.report_error_type_error_data_location(
              CountMismatch {
                // We subtract 1 here because the final AST expression is
                // not worth one value.  It is worth 0 or more depending on
                // valueTypes.head
                expected: values.size - 1 + value_types.head.len(),
                maximum: None,
                actual: vars.size,
                context: kind,
                is_variadic: false,
                function: String::new(),
              }
              .into(),
              &error_location,
            );
          }
        }
      }
    }
  }
}
