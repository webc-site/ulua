use alloc::string::String;
use core::ptr::NonNull;
use std::ptr::null_mut;

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_function::AstExprFunction,
    ast_expr_index_name::AstExprIndexName, position::Position,
  },
  rtti::ast_node_try_as_ptr,
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
    extern_type::ExternType, module::Module, primitive_type::PrimitiveType,
    source_module::SourceModule, table_type::TableType,
  },
};
pub fn get_documentation_symbol_at_position(
  source: &SourceModule,
  module: &Module,
  position: Position,
) -> Option<String> {
  let ancestry = find_ast_ancestry_of_position_source_module_position_bool(source, position, false);

  // C++: ancestry[...]->asExpr() — base-class downcast, not concrete RTTI.
  // 先把裸指针从 ancestry 复制出来，再交给 `as_expr`（`&mut self` 下转需要可变 place）
  let target_expr = match ancestry.last().copied() {
    // Safety: ancestry 的每个 *mut AstNode 均由 find_ast_ancestry 自
    // source.root（解析 arena 的可变根指针）遍历得到，非空、对齐且随
    // `source` 借用存活；arena 节点在本 crate 只经裸指针访问，本函数内
    // 无其他借用/线程与其并存，as_expr 仅读 class_index 并做偏移 0 的
    // 基类指针转换，随后立即结束该临时 &mut。
    Some(node) => unsafe { (*node).as_expr() }.map_or(null_mut(), NonNull::as_ptr),
    None => null_mut(),
  };

  let parent_expr = if ancestry.len() >= 2 {
    let parent_node = ancestry[ancestry.len() - 2];
    // Safety: 同上——ancestry 倒数第二项同为解析 arena 存活节点指针，
    // as_expr 为纯地址视图转换（表达式家族）否则返回 null。
    unsafe { (*parent_node).as_expr() }.map_or(null_mut(), NonNull::as_ptr)
  } else {
    null_mut()
  };

  if !target_expr.is_null() {
    if let Some(index_name) = unsafe { ast_node_try_as_ptr::<AstExprIndexName>(target_expr) } {
      // C++: module.ast_types.find(indexName->expr) — key is *const AstExpr.
      // index_name.expr 已句柄化恒非空；这里仅经 as_ptr 取指针值作身份键、不解引用。
      let it = module
        .ast_types
        .find(&(index_name.expr.as_ptr().cast_const()));

      if let Some(parent_ty) = it {
        let follow_ty = follow_type::follow(*parent_ty);

        if let Some(ttv) = get_type::get::<TableType>(follow_ty) {
          // C++: ttv->props.find(indexName->index.value) — props keyed by std::string.
          // Safety: index_name 存活，index: AstName 内嵌于节点，
          // 其 C 字符串驻留 source 的名字表，随 `source` 借用水久有效。
          let index_key = index_name.index.as_str_or_empty();
          if let Some(prop_it) = ttv.props.get(index_key)
            && let Some(ty) = { prop_it.read_ty }
          {
            return check_overloaded_documentation_symbol(
              module,
              ty,
              parent_expr as *const AstExpr,
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
                parent_expr as *const AstExpr,
                prop_it.documentation_symbol.clone(),
              );
            }

            match etv.parent {
              Some(parent_ty) => {
                match get_type::get::<ExternType>(follow_type::follow(parent_ty)) {
                  Some(next) => etv = next,
                  None => break,
                }
              }
              None => break,
            }
          }
        } else if let Some(ptv) = get_type::get::<PrimitiveType>(follow_ty)
          && let Some(metatable_ty) = ptv.metatable
          && let Some(mtable) = get_type::get::<TableType>(metatable_ty)
        {
          let index = index_name.index;
          return get_metatable_documentation(
            module,
            parent_expr as *const AstExpr,
            mtable,
            &index,
          );
        }
      }
    } else if let (Some(fn_node), Some(call)) = (
      unsafe { ast_node_try_as_ptr::<AstExprFunction>(target_expr) },
      unsafe { ast_node_try_as_ptr::<AstExprCall>(parent_expr) },
    ) && let Some(parent_symbol) = get_documentation_symbol_at_position(
      source,
      module,
      // Safety: call.func 由解析器写为非空 arena 节点，此处只读其 location。
      unsafe { (*call.func).base.location }.begin,
    ) {
      // Safety: call 存活；args 的 data/size 由解析 arena 成对写入，
      // as_slice 给出合法切片，元素皆为 arena 驻留节点指针。
      for (i, &call_arg) in call.args.as_slice().iter().enumerate() {
        if call_arg == target_expr {
          let fn_symbol = format!("{}/param/{}", parent_symbol, i);

          for (j, fn_arg) in fn_node.args.iter().enumerate() {
            if fn_arg.location.contains(position) {
              return Some(format!("{}/param/{}", fn_symbol, j));
            }
          }
        }
      }
    }
  }

  if let Some(binding) = find_binding_at_position(module, source, position) {
    return check_overloaded_documentation_symbol(
      module,
      binding.type_id,
      parent_expr as *const AstExpr,
      binding.documentation_symbol,
    );
  }

  if let Some(ty) = find_type_at_position(module, source, position) {
    // Safety: ty 是 module.ast_types 中登记的 TypeId，指向模块类型 arena
    // 存活节点（follow 后恒非空、对齐），解引用共享引用只读
    // documentation_symbol 字段，全程无写者。
    let ty_ptr = unsafe { &*ty };
    if let Some(doc_symbol) = ty_ptr.documentation_symbol.clone() {
      return Some(doc_symbol);
    }
  }

  None
}
