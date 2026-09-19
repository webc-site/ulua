use ulua_ast::records::ast_local::AstLocal;
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  records::constant::Constant, type_aliases::local_constant_change_log::LocalConstantChangeLog,
};

pub fn undo_changes_local(
  locals: &mut DenseHashMap<*mut AstLocal, Constant>,
  changes: &LocalConstantChangeLog,
) {
  for it in changes.iter().rev() {
    if it.was_absent {
      if let Some(old) = locals.find_mut(&it.key) {
        *old = Constant::Unknown;
      }
    } else {
      let old_value = it.old_value;
      *locals.get_or_insert(it.key) = old_value;
    }
  }
}
