use ulua_ast::records::ast_expr::AstExpr;
use ulua_config::enums::code::Code;

use crate::{
  functions::{emit_warning::emit_warning, similar::similar},
  records::lint_duplicate_condition::LintDuplicateCondition,
};
impl LintDuplicateCondition {
  pub fn detect_duplicates(&mut self, conditions: &[*mut AstExpr]) {
    const K_MAX_DISTANCE: usize = 5;

    // K_MAX_DISTANCE 窗口内比较是否重复
    for (i, &cur_cond) in conditions.iter().enumerate() {
      let start = i.saturating_sub(K_MAX_DISTANCE);

      for &prev_cond in &conditions[start..i] {
        if unsafe { similar(prev_cond, cur_cond) } {
          let current = unsafe { (*cur_cond).base.location };
          let previous = unsafe { (*prev_cond).base.location };

          if current.begin.line == previous.begin.line {
            emit_warning(
              unsafe { &mut *self.context },
              Code::DuplicateCondition,
              current,
              format_args!(
                "Condition has already been checked on column {}",
                previous.begin.column + 1
              ),
            );
          } else {
            emit_warning(
              unsafe { &mut *self.context },
              Code::DuplicateCondition,
              current,
              format_args!(
                "Condition has already been checked on line {}",
                previous.begin.line + 1
              ),
            );
          }

          break;
        }
      }
    }
  }
}
