use alloc::string::ToString;
use core::ptr::null;

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_error::AstExprError, ast_expr_global::AstExprGlobal,
    ast_expr_index_name::AstExprIndexName, ast_expr_local::AstExprLocal, ast_local::AstLocal,
    ast_name::AstName, ast_stat::AstStat, ast_stat_assign::AstStatAssign,
    ast_stat_function::AstStatFunction, ast_stat_local::AstStatLocal,
    ast_stat_local_function::AstStatLocalFunction, ast_stat_type_alias::AstStatTypeAlias,
  },
  rtti::ast_node_try_as,
};
use ulua_common::{functions::format::format, macros::luau_assert::LUAU_ASSERT};

use crate::records::{
  arena_handle::alias_ref, identifier::Identifier, internal_compiler_error::InternalCompilerError,
};

fn mk_name_ast_local(local: &AstLocal) -> Identifier {
  Identifier::new(
    local.name.as_str_or_empty().to_string(),
    local as *const AstLocal,
  )
}

pub fn mk_name_ast_expr_local(local: &AstExprLocal) -> Identifier {
  // local 槽已句柄化恒非空：.get() 安全借用取 name 字段。
  mk_name_ast_local(local.local.get())
}

pub fn mk_name_ast_expr_global(global: &AstExprGlobal) -> Identifier {
  Identifier::new(global.name.as_str_or_empty().to_string(), null())
}

pub fn mk_name_ast_name(name: &AstName) -> Identifier {
  Identifier::new(name.as_str_or_empty().to_string(), null())
}

pub fn mk_name_ast_expr_index_name(expr: &AstExprIndexName) -> Option<Identifier> {
  // expr 已句柄化恒非空：.get() 安全只读引用传给递归的 `mk_name_ast_expr`。
  let lhs = mk_name_ast_expr(expr.expr.get());
  if let Some(lhs) = lhs {
    // AstName 由词法器 intern，必为合法 UTF-8；空名（null）按 "" 处理。
    let index_str = expr.index.as_str_or_empty().to_string();

    let mut s = lhs.name().to_string();
    s.push('.');
    s.push_str(&index_str);

    Some(Identifier::new(s, lhs.ctx()))
  } else {
    None
  }
}

fn mk_name_ast_expr_error(expr: &AstExprError) -> Identifier {
  Identifier::new(format(format_args!("error#{}", expr.message_index)), null())
}

pub fn mk_name_ast_expr(expr: &AstExpr) -> Option<Identifier> {
  // 对照 C++ `mkName(const AstExpr&)` 的 `expr.as<AstExprXxx>()` 分派：全部改用
  // 安全下转 `ast_node_try_as`（`AstExpr::base` 即 repr(C) 首字段 AstNode，类判别
  // 与基址重合论证收敛在该函数内部），故本函数无需 unsafe。
  if let Some(local) = ast_node_try_as::<AstExprLocal>(&expr.base) {
    return Some(mk_name_ast_expr_local(local));
  }

  if let Some(global) = ast_node_try_as::<AstExprGlobal>(&expr.base) {
    return Some(mk_name_ast_expr_global(global));
  }

  if let Some(index_name) = ast_node_try_as::<AstExprIndexName>(&expr.base) {
    return mk_name_ast_expr_index_name(index_name);
  }

  if let Some(error) = ast_node_try_as::<AstExprError>(&expr.base) {
    return Some(mk_name_ast_expr_error(error));
  }

  None
}

pub fn mk_name_ast_stat_function(function: &AstStatFunction) -> Identifier {
  // Import the overload that handles AstExpr (the type of function->name)
  use crate::functions::mk_name_topo_sort_statements::mk_name_ast_expr;

  // `function.name` 已句柄化为 Node<AstExpr>（parser 保证非空——函数声明必有名字
  // 表达式，arena 存活由句柄契约承载），`.get()` 直出安全引用给 `mk_name_ast_expr`。
  let name = mk_name_ast_expr(function.name.get());
  LUAU_ASSERT!(name.is_some());

  match name {
    Some(id) => id,
    None => {
      let err = InternalCompilerError::new(
        "Internal error: Function declaration has a bad name".to_string(),
        None,
        None,
      );
      // message 为合法 UTF-8，Display 输出即原文；此前经 what() 的 C 串往返纯属多余 unsafe
      panic!("{err}");
    }
  }
}

pub fn mk_name_ast_stat_local_function(function: &AstStatLocalFunction) -> Identifier {
  // `function.name` 已句柄化为 Node<AstLocal>（非空 + arena 存活由句柄契约承载），
  // `.get()` 直出安全引用交给 `mk_name_ast_local`。
  mk_name_ast_local(function.name.get())
}

fn mk_name_ast_stat_assign(assign: &AstStatAssign) -> Option<Identifier> {
  if assign.vars.len() != 1 {
    return None;
  }

  // size==1 时 `as_slice()` 恰含一个元素，安全索引取代原先的 `*vars.data` 裸读。
  let var_ptr = assign.vars.as_slice()[0];
  if var_ptr.is_null() {
    return None;
  }

  mk_name_ast_expr(alias_ref(var_ptr)).map(|id| Identifier::new(id.name().to_string(), id.ctx()))
}

fn mk_name_ast_stat_local(local: &AstStatLocal) -> Option<Identifier> {
  if local.vars.len() != 1 {
    return None;
  }

  // size==1 时经 `as_slice()` 安全取首元素，替代原 `*vars.data` 裸读。
  let var_ptr = local.vars.as_slice()[0];
  if var_ptr.is_null() {
    return None;
  }

  Some(mk_name_ast_local(alias_ref(var_ptr)))
}

pub fn mk_name_ast_stat_type_alias(typealias: &AstStatTypeAlias) -> Identifier {
  mk_name_ast_name(&typealias.name)
}

/// 判空与类型断言走 Rust 安全引用，消除 `unsafe` 与裸指针。
/// 对应 C++ `mkName(AstStat* const el)` (TopoSortStatements.cpp)。
pub fn mk_name_ast_stat(el: Option<&AstStat>) -> Option<Identifier> {
  let node = el?;

  if let Some(function) = ast_node_try_as::<AstStatFunction>(&node.base) {
    return Some(mk_name_ast_stat_function(function));
  }
  if let Some(function) = ast_node_try_as::<AstStatLocalFunction>(&node.base) {
    return Some(mk_name_ast_stat_local_function(function));
  }
  if let Some(assign) = ast_node_try_as::<AstStatAssign>(&node.base) {
    return mk_name_ast_stat_assign(assign);
  }
  if let Some(local) = ast_node_try_as::<AstStatLocal>(&node.base) {
    return mk_name_ast_stat_local(local);
  }
  if let Some(typealias) = ast_node_try_as::<AstStatTypeAlias>(&node.base) {
    return Some(mk_name_ast_stat_type_alias(typealias));
  }

  None
}
