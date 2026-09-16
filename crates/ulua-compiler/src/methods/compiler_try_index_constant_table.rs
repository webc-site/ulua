use core::ptr::null_mut;

use ulua_ast::records::{
  ast_expr::AstExpr, ast_expr_index_name::AstExprIndexName, ast_expr_local::AstExprLocal,
  ast_expr_table::AstExprTable, item_ast::ItemKind,
};

use crate::{
  enums::{table_constant_kind::TableConstantKind, type_constant_folding::Type},
  functions::unwrap_expr_of_type::unwrap_expr_of_type,
  records::compiler::Compiler,
};

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn try_index_constant_table(&mut self, expr: *mut AstExprIndexName) -> *mut AstExpr {
    unsafe {
      if expr.is_null() {
        return null_mut();
      }

      let table_expr = (*expr).expr;
      let table_local = unwrap_expr_of_type::<AstExprLocal>(table_expr);
      if table_local.is_null() {
        return null_mut();
      }

      let lv = self.variables.find(&(*table_local).local);
      if lv.is_none() {
        return null_mut();
      }
      let lv = *lv.unwrap();
      if lv.written || lv.init.is_null() {
        return null_mut();
      }

      let table_kind = self.table_constants.find(&(*table_local).local);
      if table_kind.is_none() {
        return null_mut();
      }
      let table_kind = *table_kind.unwrap();
      if table_kind != TableConstantKind::ConstantTable {
        return null_mut();
      }

      let table = unwrap_expr_of_type::<AstExprTable>(lv.init);
      if table.is_null() {
        return null_mut();
      }

      let mut match_value: *mut AstExpr = null_mut();

      for item in (*table).items.as_slice() {
        if item.kind == ItemKind::Record || item.kind == ItemKind::General {
          match self.constants.find(&item.key) {
            Some(key_constant) => {
              if key_constant.r#type == Type::String && key_constant.string_length != 0 {
                let arr = key_constant.get_string();
                let key_name = (*self.names).get_or_add(arr.data, arr.size);

                if key_name.operator_eq_ast_name(&(*expr).index) {
                  match_value = item.value;
                }
              }
            }
            None => match_value = null_mut(),
          }
        }
      }

      match_value
    }
  }
}
