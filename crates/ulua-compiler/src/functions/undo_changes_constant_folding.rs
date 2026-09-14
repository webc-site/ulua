use ulua_ast::records::ast_expr::AstExpr;
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  enums::type_constant_folding::Type::Unknown, records::constant::Constant,
  type_aliases::expr_constant_change_log::ExprConstantChangeLog,
};

pub fn undo_changes_expr(
  constants: &mut DenseHashMap<*mut AstExpr, Constant>,
  changes: &ExprConstantChangeLog,
) {
  for it in changes.iter().rev() {
    if it.was_absent {
      if let Some(old) = constants.find_mut(&it.key) {
        old.r#type = Unknown;
      }
    } else {
      let old = it.old_value;
      *constants.get_or_insert(it.key) = old;
    }
  }
}
