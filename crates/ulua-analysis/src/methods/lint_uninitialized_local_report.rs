//! C++ `LintUninitializedLocal::report` (`Analysis/src/Linter.cpp:2118`).
//!
//! The `LintUninitializedLocal` record carries a placeholder no-op `report`
//! method, so the faithful reporting logic lives here as a free function over
//! `&mut LintUninitializedLocal` and is invoked from `process`.

use ulua_config::enums::code::Code;

use crate::{
  functions::emit_warning::emit_warning, records::lint_uninitialized_local::LintUninitializedLocal,
};

pub fn lint_uninitialized_local_report(pass: &mut LintUninitializedLocal) {
  let context = pass.context;

  for (local, l) in pass.locals.iter() {
    let local = *local;

    if l.defined && !l.initialized && !l.assigned && !l.first_use.is_null() {
      unsafe {
        emit_warning(
          &mut *context,
          Code::UninitializedLocal,
          (*l.first_use).base.base.location,
          format_args!(
            "Variable '{}' defined at line {} is never initialized or assigned; initialize with 'nil' to silence",
            (*local).name,
            (*local).location.begin.line + 1
          ),
        );
      }
    }
  }
}
