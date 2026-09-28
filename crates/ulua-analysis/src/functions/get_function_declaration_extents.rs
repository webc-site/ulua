use ulua_ast::records::{
  ast_expr::AstExpr, ast_expr_function::AstExprFunction, ast_local::AstLocal, location::Location,
};

/// 对照 cpp `getFunctionDeclarationExtents`（`FragmentAutocomplete.cpp:48-73`）。
/// 求函数声明头部范围：优先 return 注解末尾，其次最后一个参数（或其注解）末尾，
/// 再次最后一个 generic pack / generic 末尾，最后落到名字表达式 / local 名末尾。
///
/// # Safety
/// - `expr_fn` 必须是非空、指向存活 arena `AstExprFunction` 的指针：本函数无条件
///   解引用它读取 base.location / return_annotation / args / generic_packs / generics。
///   两个调用点（get_fragment_location.rs）均从 RTTI 甄别出的
///   `AstStatFunction.func`/`AstStatLocalFunction.func` 字段取该指针，parser 对函数
///   语句必生成非空函数体表达式（cpp:50-51 亦直接解引用 `exprFn->location`）。当
///   `args.size != 0` 时 `args.data` 指向连续的 `*mut AstLocal` 数组且元素非空
///   （每个形参必绑定 local，cpp:58 直接 `last->` 解引用）；`generic_packs`/`generics`
///   的数组同理。
/// - `expr_name`/`local_name` 对应 cpp 的默认实参（均默认 nullptr）：允许为 null；
///   非空时必须指向 arena 存活 `AstExpr`/`AstLocal`。else-if 链保证至多其一被解引用，
///   且判空先于解引用，只读其 `location.end`。本函数全程只读借用、无任何写回，
///   函数体内不存在并存可变借用。
pub unsafe fn get_function_declaration_extents(
  expr_fn: *mut AstExprFunction,
  expr_name: *mut AstExpr,
  local_name: *mut AstLocal,
) -> Location {
  // Safety: `expr_fn` 非空且指向 arena 存活的函数表达式节点（见函数级契约）；
  // 再借用为只读引用，后续仅读取分析期不再变化的 AST 字段。
  let f = unsafe { &*expr_fn };
  let fn_begin = f.base.base.location.begin;
  let mut fn_end = f.base.base.location.end;

  if let Some(return_annotation) = f.return_annotation.get() {
    fn_end = return_annotation.base.location.end;
  } else if let Some(last) = f.args.iter().last() {
    if !last.annotation.is_null() {
      fn_end = unsafe { (*last.annotation).base.location.end };
    } else {
      fn_end = last.location.end;
    }
  } else if let Some(last) = f.generic_packs.iter().last() {
    fn_end = last.base.location.end;
  } else if let Some(last) = f.generics.iter().last() {
    fn_end = last.base.location.end;
  } else if !expr_name.is_null() {
    fn_end = unsafe { (*expr_name).base.location.end };
  } else if !local_name.is_null() {
    fn_end = unsafe { (*local_name).location.end };
  }

  Location::new(fn_begin, fn_end)
}
