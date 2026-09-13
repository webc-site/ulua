use ulua_ast::{
  records::{
    ast_stat::AstStat, ast_stat_assign::AstStatAssign, ast_stat_function::AstStatFunction,
    ast_stat_local::AstStatLocal, ast_stat_local_function::AstStatLocalFunction,
    ast_stat_type_alias::AstStatTypeAlias,
  },
  rtti::AstNodeClass,
};

use crate::{
  functions::{
    mk_name_topo_sort_statements_alt_h::mk_name_ast_stat_function,
    mk_name_topo_sort_statements_alt_i::mk_name_ast_stat_local_function,
    mk_name_topo_sort_statements_alt_j::mk_name_ast_stat_assign,
    mk_name_topo_sort_statements_alt_k::mk_name_ast_stat_local,
    mk_name_topo_sort_statements_alt_l::mk_name_ast_stat_type_alias,
  },
  records::identifier::Identifier,
};

/// # Safety
/// 调用方须保证 `el` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn mk_name_ast_stat(el: *mut AstStat) -> Option<Identifier> {
  if el.is_null() {
    return None;
  }

  let node = unsafe { &*el };
  let class_index = node.base.class_index;

  if class_index == AstStatFunction::CLASS_INDEX {
    let function = unsafe { &*(el as *mut AstStatFunction) };
    Some(mk_name_ast_stat_function(function))
  } else if class_index == AstStatLocalFunction::CLASS_INDEX {
    let function = unsafe { &*(el as *mut AstStatLocalFunction) };
    Some(mk_name_ast_stat_local_function(function))
  } else if class_index == AstStatAssign::CLASS_INDEX {
    let assign = unsafe { &*(el as *mut AstStatAssign) };
    mk_name_ast_stat_assign(assign)
  } else if class_index == AstStatLocal::CLASS_INDEX {
    let local = unsafe { &*(el as *mut AstStatLocal) };
    mk_name_ast_stat_local(local)
  } else if class_index == AstStatTypeAlias::CLASS_INDEX {
    let typealias = unsafe { &*(el as *mut AstStatTypeAlias) };
    Some(mk_name_ast_stat_type_alias(typealias))
  } else {
    None
  }
}
