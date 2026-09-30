use ulua_ast::records::{
  ast_expr::AstExpr, ast_expr_function::AstExprFunction, ast_local::AstLocal, location::Location,
};

/// 对照 cpp `getFunctionDeclarationExtents`（`FragmentAutocomplete.cpp:48-73`）。
/// 求函数声明头部范围：优先 return 注解末尾，其次最后一个参数（或其注解）末尾，
/// 再次最后一个 generic pack / generic 末尾，最后落到名字表达式 / local 名末尾。
pub fn get_function_declaration_extents(
  expr_fn: &AstExprFunction,
  expr_name: Option<&AstExpr>,
  local_name: Option<&AstLocal>,
) -> Location {
  let fn_begin = expr_fn.base.base.location.begin;
  let mut fn_end = expr_fn.base.base.location.end;

  if let Some(return_annotation) = expr_fn.return_annotation.get() {
    fn_end = return_annotation.base.location.end;
  } else if let Some(last) = expr_fn.args.iter().last() {
    if let Some(ann) = unsafe { last.annotation.as_ref() } {
      fn_end = ann.base.location.end;
    } else {
      fn_end = last.location.end;
    }
  } else if let Some(last) = expr_fn.generic_packs.iter().last() {
    fn_end = last.base.location.end;
  } else if let Some(last) = expr_fn.generics.iter().last() {
    fn_end = last.base.location.end;
  } else if let Some(expr_name) = expr_name {
    fn_end = expr_name.base.location.end;
  } else if let Some(local_name) = local_name {
    fn_end = local_name.location.end;
  }

  Location::new(fn_begin, fn_end)
}
