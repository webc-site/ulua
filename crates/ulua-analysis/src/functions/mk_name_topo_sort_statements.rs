use core::ffi::CStr;

use ulua_ast::records::ast_local::AstLocal;

use crate::records::identifier::Identifier;
pub fn mk_name_ast_local(local: &AstLocal) -> Identifier {
  Identifier::new(
    unsafe {
      CStr::from_ptr(local.name.value)
        .to_string_lossy()
        .into_owned()
    },
    local as *const AstLocal,
  )
}
