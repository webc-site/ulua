use core::ffi::CStr;

use ulua_ast::records::{ast_expr::AstExpr, ast_name::AstName};

use crate::{
  functions::{
    check_overloaded_documentation_symbol::check_overloaded_documentation_symbol,
    follow_type::follow_type_id, get_type_alt_j::get_type_id,
  },
  records::{module::Module, table_type::TableType},
  type_aliases::documentation_symbol::DocumentationSymbol,
};
pub(crate) fn get_metatable_documentation(
  module: &Module,
  parent_expr: *const AstExpr,
  mtable: &TableType,
  index: &AstName,
) -> Option<DocumentationSymbol> {
  // C++: auto indexIt = mtable->props.find("__index");
  let index_prop = mtable.props.get("__index")?;

  let followed = if let Some(read_ty) = index_prop.read_ty {
    follow_type_id(read_ty)
  } else {
    let write_ty = index_prop.write_ty?;
    follow_type_id(write_ty)
  };

  let ttv = get_type_id::<TableType>(followed)?;

  // C++: auto propIt = ttv->props.find(index.value); — props keyed by std::string.
  let index_key = unsafe { CStr::from_ptr(index.value).to_string_lossy().into_owned() };
  let prop = ttv.props.get(&index_key)?;

  if let Some(ty) = prop.read_ty {
    return unsafe {
      check_overloaded_documentation_symbol(
        module,
        ty,
        parent_expr,
        prop.documentation_symbol.clone(),
      )
    };
  }

  None
}
