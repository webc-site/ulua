use ulua_ast::records::{
  ast_expr::AstExpr, ast_expr_function::AstExprFunction, ast_local::AstLocal, location::Location,
};

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn get_function_declaration_extents(
  expr_fn: *mut AstExprFunction,
  expr_name: *mut AstExpr,
  local_name: *mut AstLocal,
) -> Location {
  let fn_begin = unsafe { (*expr_fn).base.base.location.begin };
  let mut fn_end = unsafe { (*expr_fn).base.base.location.end };

  let return_annotation = unsafe { (*expr_fn).return_annotation };
  if !return_annotation.is_null() {
    fn_end = unsafe { (*return_annotation).base.location.end };
  } else {
    let args = unsafe { (*expr_fn).args };
    if args.size != 0 {
      let last = unsafe { *args.data.add(args.size - 1) };
      let annotation = unsafe { (*last).annotation };
      if !annotation.is_null() {
        fn_end = unsafe { (*annotation).base.location.end };
      } else {
        fn_end = unsafe { (*last).location.end };
      }
    } else {
      let generic_packs = unsafe { (*expr_fn).generic_packs };
      if generic_packs.size != 0 {
        let last = unsafe { *generic_packs.data.add(generic_packs.size - 1) };
        fn_end = unsafe { (*last).base.location.end };
      } else {
        let generics = unsafe { (*expr_fn).generics };
        if generics.size != 0 {
          let last = unsafe { *generics.data.add(generics.size - 1) };
          fn_end = unsafe { (*last).base.location.end };
        } else if !expr_name.is_null() {
          fn_end = unsafe { (*expr_name).base.location.end };
        } else if !local_name.is_null() {
          fn_end = unsafe { (*local_name).location.end };
        }
      }
    }
  }

  Location::new(fn_begin, fn_end)
}
