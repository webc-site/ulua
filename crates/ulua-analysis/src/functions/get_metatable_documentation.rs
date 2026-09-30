use alloc::string::String;

use ulua_ast::records::{ast_expr::AstExpr, ast_name::AstName};

use crate::{
  functions::{
    check_overloaded_documentation_symbol::check_overloaded_documentation_symbol, follow_type,
    get_type,
  },
  records::{module::Module, table_type::TableType},
};
pub(crate) fn get_metatable_documentation(
  module: &Module,
  parent_expr: *const AstExpr,
  mtable: &TableType,
  index: &AstName,
) -> Option<String> {
  // C++: auto indexIt = mtable->props.find("__index");
  let index_prop = mtable.props.get("__index")?;

  let followed = if let Some(read_ty) = index_prop.read_ty {
    follow_type::follow(read_ty)
  } else {
    let write_ty = index_prop.write_ty?;
    follow_type::follow(write_ty)
  };

  let ttv = get_type::get::<TableType>(followed)?;

  // C++: auto propIt = ttv->props.find(index.value); — props keyed by std::string.
  let index_key = index.as_str_or_empty();
  let prop = ttv.props.get(index_key)?;

  if let Some(ty) = prop.read_ty {
    return check_overloaded_documentation_symbol(
      module,
      ty,
      parent_expr,
      prop.documentation_symbol.clone(),
    );
  }

  None
}
