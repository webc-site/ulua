use core::{ffi::CStr, ptr::null};

use ulua_ast::records::ast_name::AstName;

use crate::records::identifier::Identifier;
pub fn mk_name_ast_name(name: &AstName) -> Identifier {
  Identifier::new(
    unsafe { CStr::from_ptr(name.value).to_string_lossy().into_owned() },
    null(),
  )
}
