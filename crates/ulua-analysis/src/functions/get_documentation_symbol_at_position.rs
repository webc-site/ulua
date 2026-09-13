use core::ffi::CStr;
use std::ptr::null_mut;

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_function::AstExprFunction,
    ast_expr_index_name::AstExprIndexName, ast_node::AstNode, position::Position,
  },
  rtti::ast_node_as,
};

use crate::{
  functions::{
    check_overloaded_documentation_symbol::check_overloaded_documentation_symbol,
    find_ast_ancestry_of_position_ast_query::find_ast_ancestry_of_position_source_module_position_bool,
    find_binding_at_position::find_binding_at_position,
    find_type_at_position::find_type_at_position, follow_type::follow_type_id,
    get_metatable_documentation::get_metatable_documentation, get_type_alt_j::get_type_id,
  },
  records::{
    extern_type::ExternType, module::Module, primitive_type::PrimitiveType,
    source_module::SourceModule, table_type::TableType,
  },
  type_aliases::documentation_symbol::DocumentationSymbol,
};
pub fn get_documentation_symbol_at_position(
  source: &SourceModule,
  module: &Module,
  position: Position,
) -> Option<DocumentationSymbol> {
  let ancestry = find_ast_ancestry_of_position_source_module_position_bool(source, position, false);

  // C++: ancestry[...]->asExpr() — base-class downcast, not concrete RTTI.
  let target_expr = if !ancestry.is_empty() {
    unsafe { (*ancestry[ancestry.len() - 1]).as_expr() }
  } else {
    null_mut()
  };

  let parent_expr = if ancestry.len() >= 2 {
    unsafe { (*ancestry[ancestry.len() - 2]).as_expr() }
  } else {
    null_mut()
  };

  if !target_expr.is_null() {
    if !unsafe { ast_node_as::<AstExprIndexName>(target_expr as *mut AstNode) }.is_null() {
      let index_name = unsafe { ast_node_as::<AstExprIndexName>(target_expr as *mut AstNode) };
      // C++: module.ast_types.find(indexName->expr) — key is *const AstExpr.
      let it = module
        .ast_types
        .find(&(unsafe { (*index_name).expr } as *const AstExpr));

      if let Some(parent_ty) = it {
        let follow_ty = follow_type_id(*parent_ty);

        if let Some(ttv) = get_type_id::<TableType>(follow_ty) {
          let index_value = unsafe { (*index_name).index.value };
          // C++: ttv->props.find(indexName->index.value) — props keyed by std::string.
          let index_key = unsafe { CStr::from_ptr(index_value).to_string_lossy().into_owned() };
          if let Some(prop_it) = ttv.props.get(&index_key)
            && let Some(ty) = { prop_it.read_ty }
          {
            return unsafe {
              check_overloaded_documentation_symbol(
                module,
                ty,
                parent_expr as *const AstExpr,
                prop_it.documentation_symbol.clone(),
              )
            };
          }
        } else if let Some(mut etv) = get_type_id::<ExternType>(follow_ty) {
          loop {
            let index_value = unsafe { (*index_name).index.value };
            // C++: etv->props.find(indexName->index.value) — props keyed by std::string.
            let index_key = unsafe { CStr::from_ptr(index_value).to_string_lossy().into_owned() };
            if let Some(prop_it) = etv.props.get(&index_key)
              && let Some(ty) = { prop_it.read_ty }
            {
              return unsafe {
                check_overloaded_documentation_symbol(
                  module,
                  ty,
                  parent_expr as *const AstExpr,
                  prop_it.documentation_symbol.clone(),
                )
              };
            }

            match etv.parent {
              Some(parent_ty) => match get_type_id::<ExternType>(follow_type_id(parent_ty)) {
                Some(next) => etv = next,
                None => break,
              },
              None => break,
            }
          }
        } else if let Some(ptv) = get_type_id::<PrimitiveType>(follow_ty)
          && let Some(metatable_ty) = ptv.metatable
          && let Some(mtable) = get_type_id::<TableType>(metatable_ty)
        {
          let index = unsafe { (*index_name).index };
          return get_metatable_documentation(
            module,
            parent_expr as *const AstExpr,
            mtable,
            &index,
          );
        }
      }
    } else if !unsafe { ast_node_as::<AstExprFunction>(target_expr as *mut AstNode) }.is_null()
      && !parent_expr.is_null()
      && !unsafe { ast_node_as::<AstExprCall>(parent_expr as *mut AstNode) }.is_null()
    {
      let call = unsafe { ast_node_as::<AstExprCall>(parent_expr as *mut AstNode) };

      if let Some(parent_symbol) = get_documentation_symbol_at_position(
        source,
        module,
        unsafe { (*(*call).func).base.location }.begin,
      ) {
        for i in 0..unsafe { (*call).args.size } as usize {
          let call_arg = unsafe { *((*call).args.data.add(i)) };

          if call_arg == target_expr {
            let fn_symbol = format!("{}/param/{}", parent_symbol, i);

            let fn_node = unsafe { ast_node_as::<AstExprFunction>(target_expr as *mut AstNode) };
            for j in 0..unsafe { (*fn_node).args.size } as usize {
              let fn_arg = unsafe { *((*fn_node).args.data.add(j)) };

              if unsafe { (*fn_arg).location }.contains(position) {
                return Some(format!("{}/param/{}", fn_symbol, j));
              }
            }
          }
        }
      }
    }
  }

  if let Some(binding) = find_binding_at_position(module, source, position) {
    return unsafe {
      check_overloaded_documentation_symbol(
        module,
        binding.type_id,
        parent_expr as *const AstExpr,
        binding.documentation_symbol,
      )
    };
  }

  if let Some(ty) = find_type_at_position(module, source, position) {
    let ty_ptr = unsafe { &*ty };
    if let Some(doc_symbol) = ty_ptr.documentation_symbol.clone() {
      return Some(doc_symbol);
    }
  }

  None
}
