use ulua_ast::records::ast_stat_function::AstStatFunction;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::{identifier::Identifier, internal_compiler_error::InternalCompilerError};
pub fn mk_name_ast_stat_function(function: &AstStatFunction) -> Identifier {
  // Import the overload that handles AstExpr (the type of function->name)
  use crate::functions::mk_name_topo_sort_statements_alt_g::mk_name_ast_expr;

  let name = unsafe { mk_name_ast_expr(&*function.name) };
  LUAU_ASSERT!(name.is_some());

  match name {
    Some(id) => id,
    None => {
      let err = InternalCompilerError::new(
        "Internal error: Function declaration has a bad name".to_string(),
        None,
        None,
      );
      // message 为合法 UTF-8，Display 输出即原文；此前经 what() 的 CStr 往返纯属多余 unsafe
      panic!("{err}");
    }
  }
}
