use ulua_ast::{
  records::{
    ast_expr::AstExpr,
    ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
    ast_expr_global::AstExprGlobal,
    ast_expr_index_name::AstExprIndexName,
    ast_expr_local::AstExprLocal,
    ast_local::AstLocal,
    ast_name::AstName,
  },
  rtti::ast_node_try_as,
};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  enums::global::Global,
  functions::{ast_slot_ref::ast_slot_ref, get_global_state::get_global_state},
  records::{builtin::Builtin, node::Node, variable::Variable},
};

/// 仅 crate 内查表用；入参为 arena 存活节点地址句柄（契约见 `Node::borrow`）。
pub(crate) fn get_builtin(
  node: Node<AstExpr>,
  globals: &DenseHashMap<AstName, Global>,
  variables: &DenseHashMap<Node<AstLocal>, Variable>,
) -> Builtin {
  // 顶层判定链走引用形态；嵌套实参（expr.expr 等）保留判空下转（可空槽），
  // v.init 由 Option 命中。
  let node_ref = node.borrow();

  // 局部变量：未写且带初值时沿初值继续追踪
  if let Some(expr) = ast_node_try_as::<AstExprLocal>(&node_ref.base) {
    if let Some(v) = variables.find(&expr.local.into())
      && !v.written
      && let Some(init) = v.init
    {
      return get_builtin(init, globals, variables);
    }
    return Builtin::default();
  }

  if let Some(expr) = ast_node_try_as::<AstExprIndexName>(&node_ref.base) {
    // `<local>.<method>(...)`：local 初值须指向 global（或 `global or ...` 左支）
    // expr.expr 已句柄化恒非空；经 as_ptr 桥接 `ast_slot_ref` 收口判空与只读
    // 解引用，RTTI 未命中返回 None。
    if let Some(object) =
      ast_slot_ref(expr.expr.as_ptr()).and_then(|e| ast_node_try_as::<AstExprLocal>(e))
      && let Some(v) = variables.find(&object.local.into())
      && !v.written
      && let Some(init) = v.init
    {
      let target_global = ast_node_try_as::<AstExprGlobal>(init.borrow()).or_else(|| {
        ast_node_try_as::<AstExprBinary>(init.borrow())
          .filter(|cond| cond.op == AstExprBinaryOp::Or)
          // Or 二元节点的 left 操作数由 parser 保证非空存活。
          .and_then(|cond| ast_slot_ref(cond.left.as_ptr()))
          .and_then(|left| ast_node_try_as::<AstExprGlobal>(left))
      });

      if let Some(global) = target_global
        && get_global_state(globals, global.name) == Global::Default
      {
        return Builtin {
          object: global.name,
          method: expr.index,
        };
      }
    }

    // `<global>.<method>(...)`：同第一处，expr.expr 已句柄化，经 as_ptr 桥接门面只读解析。
    if let Some(object) =
      ast_slot_ref(expr.expr.as_ptr()).and_then(|e| ast_node_try_as::<AstExprGlobal>(e))
      && get_global_state(globals, object.name) == Global::Default
    {
      return Builtin {
        object: object.name,
        method: expr.index,
      };
    }

    return Builtin::default();
  }

  // `<global>(...)`
  if let Some(expr) = ast_node_try_as::<AstExprGlobal>(&node_ref.base)
    && get_global_state(globals, expr.name) == Global::Default
  {
    return Builtin {
      object: AstName::new(),
      method: expr.name,
    };
  }

  Builtin::default()
}
