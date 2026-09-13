use ulua_ast::records::ast_expr::AstExpr;
use ulua_config::enums::code::Code;

use crate::{
  enums::table_state::TableState,
  functions::{
    emit_warning::emit_warning, follow_type::follow_type_id, get_type_alt_j::get_type_id,
    is_string::is_string,
  },
  records::{lint_table_operations::LintTableOperations, table_type::TableType},
};

impl LintTableOperations {
  pub fn check_indexer(&mut self, node: &AstExpr, expr: &AstExpr, op: &str) {
    // SAFETY: context 在 linter 存活期内有效；expr 由 lint 遍历分发器保证存活。
    let Some(ty) = (unsafe { (*self.context).get_type(expr as *const AstExpr as *mut AstExpr) })
    else {
      return;
    };
    let followed = follow_type_id(ty);
    let Some(tty_ref) = get_type_id::<TableType>(followed) else {
      return;
    };

    if tty_ref.indexer.is_none()
      && !tty_ref.props.is_empty()
      && tty_ref.state != TableState::Generic
    {
      let msg = format!(
        "Using '{}' on a table without an array part is likely a bug",
        op
      );
      // Pass `format_args!` straight into the call: a `fmt::Arguments`
      // borrows its operands, so storing it in a `let` and using it in a
      // later statement is the shape that dangles if an operand is ever a
      // temporary (the E0716 fixed in ulua-vm's pusherror.rs, issue #3).
      emit_warning(
        unsafe { &mut *self.context },
        Code::TableOperations,
        node.base.location,
        format_args!("{}", msg),
      );
    } else if let Some(indexer) = &tty_ref.indexer
      && is_string(indexer.index_type)
    {
      let msg = format!("Using '{}' on a table with string keys is likely a bug", op);
      // Pass `format_args!` straight into the call: a `fmt::Arguments`
      // borrows its operands, so storing it in a `let` and using it in a
      // later statement is the shape that dangles if an operand is ever a
      // temporary (the E0716 fixed in ulua-vm's pusherror.rs, issue #3).
      emit_warning(
        unsafe { &mut *self.context },
        Code::TableOperations,
        node.base.location,
        format_args!("{}", msg),
      );
    }
  }
}
