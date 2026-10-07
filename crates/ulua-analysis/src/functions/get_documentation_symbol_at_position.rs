use alloc::string::String;
use core::ptr;

use ulua_ast::{
  records::{
    ast_expr_call::AstExprCall, ast_expr_function::AstExprFunction,
    ast_expr_index_name::AstExprIndexName, position::Position,
  },
  rtti::ast_node_try_as,
};

use crate::{
  functions::{
    check_overloaded_documentation_symbol::check_overloaded_documentation_symbol,
    find_ast_ancestry_of_position_ast_query::find_ast_ancestry_of_position_source_module_position_bool,
    find_binding_at_position::find_binding_at_position,
    find_type_at_position::find_type_at_position, follow_type,
    get_metatable_documentation::get_metatable_documentation, get_type,
  },
  records::{
    arena_handle::{alias, alias_ref},
    extern_type::ExternType,
    module::Module,
    primitive_type::PrimitiveType,
    source_module::SourceModule,
    table_type::TableType,
  },
};
pub fn get_documentation_symbol_at_position(
  source: &SourceModule,
  module: &Module,
  position: Position,
) -> Option<String> {
  let ancestry = find_ast_ancestry_of_position_source_module_position_bool(source, position, false);

  // C++: ancestry[...]->asExpr() — base-class downcast, not concrete RTTI.
  // §2：可空下转直接收口为 `Option<&AstExpr>` 安全视图（const 门面判型+下转，
  // 不再经 `null_mut` 哨兵与 `NonNull::as_ptr` 舞步）。
  let target_expr = ancestry
    .last()
    .copied()
    .and_then(|node| alias(node).as_expr_const());

  let parent_expr = if ancestry.len() >= 2 {
    alias(ancestry[ancestry.len() - 2]).as_expr_const()
  } else {
    None
  };

  if let Some(target_expr) = target_expr {
    if let Some(index_name) = ast_node_try_as::<AstExprIndexName>(target_expr) {
      // C++: module.ast_types.find(indexName->expr) — key is *const AstExpr.
      // index_name.expr 已句柄化恒非空；这里仅经 as_ptr 取指针值作身份键、不解引用。
      let it = module
        .ast_types
        .find(&(index_name.expr.as_ptr().cast_const()));

      if let Some(parent_ty) = it {
        let follow_ty = follow_type::follow(*parent_ty);

        if let Some(ttv) = get_type::get::<TableType>(follow_ty) {
          // C++: ttv->props.find(indexName->index.value) — props keyed by std::string.
          let index_key = index_name.index.as_str_or_empty();
          if let Some(prop_it) = ttv.props.get(index_key)
            && let Some(ty) = { prop_it.read_ty }
          {
            return check_overloaded_documentation_symbol(
              module,
              ty,
              parent_expr,
              prop_it.documentation_symbol.clone(),
            );
          }
        } else if let Some(mut etv) = get_type::get::<ExternType>(follow_ty) {
          loop {
            // C++: etv->props.find(indexName->index.value) — props keyed by std::string.
            let index_key = index_name.index.as_str_or_empty();
            if let Some(prop_it) = etv.props.get(index_key)
              && let Some(ty) = { prop_it.read_ty }
            {
              return check_overloaded_documentation_symbol(
                module,
                ty,
                parent_expr,
                prop_it.documentation_symbol.clone(),
              );
            }

            etv = match etv
              .parent
              .and_then(|parent_ty| get_type::get::<ExternType>(follow_type::follow(parent_ty)))
            {
              Some(next) => next,
              None => break,
            };
          }
        } else if let Some(ptv) = get_type::get::<PrimitiveType>(follow_ty)
          && let Some(metatable_ty) = ptv.metatable
          && let Some(mtable) = get_type::get::<TableType>(metatable_ty)
        {
          let index = index_name.index;
          return get_metatable_documentation(module, parent_expr, mtable, &index);
        }
      }
    } else if let (Some(fn_node), Some(call)) = (
      ast_node_try_as::<AstExprFunction>(target_expr),
      parent_expr.and_then(|parent| ast_node_try_as::<AstExprCall>(parent)),
    ) && let Some(parent_symbol) =
      get_documentation_symbol_at_position(source, module, alias_ref(call.func).base.location.begin)
    {
      // 身份比较：args 元素是 arena 节点裸指针（身份键），与 target 视图地址
      // 用 ptr::eq 逐一比较、不解引用（§10 (ptr,len) 收口面门的既有形态）。
      for (i, &call_arg) in call.args.as_slice().iter().enumerate() {
        if ptr::eq(call_arg, target_expr) {
          let fn_symbol = format!("{}/param/{}", parent_symbol, i);

          if let Some(j) = fn_node
            .args
            .iter()
            .position(|fn_arg| fn_arg.location.contains(position))
          {
            return Some(format!("{}/param/{}", fn_symbol, j));
          }
        }
      }
    }
  }

  if let Some(binding) = find_binding_at_position(module, source, position) {
    return check_overloaded_documentation_symbol(
      module,
      binding.type_id,
      parent_expr,
      binding.documentation_symbol,
    );
  }

  if let Some(ty) = find_type_at_position(module, source, position) {
    let ty_ptr = alias_ref(ty);
    if let Some(doc_symbol) = ty_ptr.documentation_symbol.clone() {
      return Some(doc_symbol);
    }
  }

  None
}
