use std::{ffi::CStr, ptr::null};

use ulua_ast::records::ast_expr_global::AstExprGlobal;

use crate::records::identifier::Identifier;
pub fn mk_name_ast_expr_global(global: &AstExprGlobal) -> Identifier {
  Identifier::new(
    unsafe {
      CStr::from_ptr(global.name.value)
        .to_string_lossy()
        .into_owned()
    },
    null(),
  )
}
