use alloc::{collections::BTreeMap, string::String, vec::Vec};

use ulua_ast::{
  records::{
    ast_expr_constant_number::AstExprConstantNumber,
    ast_expr_constant_string::AstExprConstantString,
    ast_expr_table::{AstExprTable, ItemKind},
    ast_node::AstNode,
  },
  rtti,
};
use ulua_config::enums::code::Code;

use crate::{functions::emit_warning::emit_warning, records::lint_table_literal::LintTableLiteral};
impl LintTableLiteral {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_expr_table(&mut self, node: *mut AstExprTable) -> bool {
    let node = unsafe { &*node };
    let mut count = 0;
    for item in node.items.iter() {
      if item.kind == ItemKind::List {
        count += 1;
      }
    }

    let mut names: BTreeMap<Vec<u8>, u32> = BTreeMap::new();
    let mut indices: BTreeMap<i32, u32> = BTreeMap::new();

    for item in node.items.iter() {
      if item.key.is_null() {
        continue;
      }

      let key_node = item.key as *mut AstNode;
      let string_expr = unsafe { rtti::ast_node_as::<AstExprConstantString>(key_node) };
      if !string_expr.is_null() {
        let expr = unsafe { &*string_expr };
        let key = expr.value.as_bytes().to_vec();
        let field = String::from_utf8_lossy(&key);

        if let Some(line) = names.get(&key).copied() {
          emit_warning(
            unsafe { &mut *self.context },
            Code::TableLiteral,
            expr.base.base.location,
            format_args!(
              "Table field '{}' is a duplicate; previously defined at line {}",
              field, line
            ),
          );
        } else {
          names.insert(key, expr.base.base.location.begin.line + 1);
        }

        continue;
      }

      let number_expr = unsafe { rtti::ast_node_as::<AstExprConstantNumber>(key_node) };
      if !number_expr.is_null() {
        let expr = unsafe { &*number_expr };
        let value = expr.value;

        if value >= 1.0 && value <= f64::from(count) && f64::from(value as i32) == value {
          emit_warning(
            unsafe { &mut *self.context },
            Code::TableLiteral,
            expr.base.base.location,
            format_args!(
              "Table index {} is a duplicate; previously defined as a list entry",
              value as i32
            ),
          );
        } else if value >= 0.0 && value <= f64::from(i32::MAX) && f64::from(value as i32) == value {
          let index = value as i32;

          if let Some(line) = indices.get(&index).copied() {
            emit_warning(
              unsafe { &mut *self.context },
              Code::TableLiteral,
              expr.base.base.location,
              format_args!(
                "Table index {} is a duplicate; previously defined at line {}",
                index, line
              ),
            );
          } else {
            indices.insert(index, expr.base.base.location.begin.line + 1);
          }
        }
      }
    }

    true
  }
}
