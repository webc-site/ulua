use core::ptr::null;

use ulua_ast::{
  records::{
    ast_expr_call::AstExprCall, ast_expr_constant_bool::AstExprConstantBool, ast_node::AstNode,
    ast_stat::AstStat, ast_stat_block::AstStatBlock, ast_stat_expr::AstStatExpr,
    ast_stat_if::AstStatIf, ast_stat_repeat::AstStatRepeat, ast_stat_return::AstStatReturn,
    ast_stat_while::AstStatWhile,
  },
  rtti::{ast_node_is, ast_node_try_as, ast_node_try_as_ptr},
};

use crate::records::type_checker_2::TypeChecker2;

impl TypeChecker2 {
  // cpp TypeChecker2.cpp:411
  pub fn type_checker_2_get_fallthrough(&mut self, node: *const AstStat) -> *const AstStat {
    if node.is_null() {
      return null();
    }

    // SAFETY: 上方已判空非 null；AST 由解析器 arena 持有，本函数全程只读。
    // 仅此处一次裸解引用换得 &AstNode，后续下转与字段访问全部走安全引用。
    let node_ref: &AstNode = unsafe { &*(node as *const AstNode) };

    if let Some(stat) = ast_node_try_as::<AstStatBlock>(node_ref) {
      if stat.body.is_empty() {
        return node;
      }

      // 除最后一条外的所有语句都必须能"穿透"（不含 return/break）
      let body = stat.body.as_slice();
      if body[..body.len() - 1].iter().any(|child| {
        self
          .type_checker_2_get_fallthrough(child.as_ptr().cast_const())
          .is_null()
      }) {
        return null();
      }

      return self.type_checker_2_get_fallthrough(body[body.len() - 1].as_ptr().cast_const());
    }

    if let Some(stat) = ast_node_try_as::<AstStatIf>(node_ref) {
      let thenf =
        self.type_checker_2_get_fallthrough(stat.thenbody.cast::<AstStat>().as_ptr().cast_const());
      if !thenf.is_null() {
        return thenf;
      }

      if let Some(else_stat) = stat.elsebody.get() {
        return self.type_checker_2_get_fallthrough(else_stat as *const AstStat);
      }
      return node;
    }

    if ast_node_is::<AstStatReturn>(node_ref) {
      return null();
    }

    if let Some(stat) = ast_node_try_as::<AstStatExpr>(node_ref) {
      if let Some(call) = (unsafe {
        // Safety: stat.expr 为 parser 保证的存活只读子表达式；ast_node_try_as_ptr 先判空、
        // 再按 RTTI class index 命中才返回 Some(&T)，未命中为 None，不凭空构造引用。
        ast_node_try_as_ptr::<AstExprCall>(stat.expr)
      }) && unsafe {
        // Safety: call 由上方 RTTI 命中得到，指向 arena 中存活节点且本函数全程只读；
        // is_error_call 仅只读遍历该调用表达式，单线程串行无并发别名。
        self.is_error_call(call)
      } {
        return null();
      }
      return node;
    }

    if let Some(stat) = ast_node_try_as::<AstStatWhile>(node_ref) {
      if let Some(expr) = ast_node_try_as::<AstExprConstantBool>(stat.condition.get())
        && expr.value
        && unsafe {
          // Safety: stat.body 为 parser 保证的非空 AstStatBlock，repr(C) 首字段基址重合
          // 可转 *mut AstStat 视图传入；has_break 只读遍历该 AST，单线程无并发别名。
          !self.type_checker_2_has_break(stat.body.as_ptr().cast::<AstStat>())
        }
      {
        return null();
      }
      return node;
    }

    if let Some(stat) = ast_node_try_as::<AstStatRepeat>(node_ref) {
      // condition/body 已句柄化为 Node（非空由类型层承载）：判型走安全引用门面，
      // has_break/get_fallthrough 以 as_ptr+cast 桥交指针形态（同上方 while 分支）。
      if let Some(expr) = ast_node_try_as::<AstExprConstantBool>(stat.condition.get())
        && !expr.value
        && !unsafe {
          // Safety: repr(C) 基址重合上转 AstStat 视图；has_break 只读遍历该
          // AST，单线程无并发别名。
          self.type_checker_2_has_break(stat.body.as_ptr().cast::<AstStat>())
        }
      {
        return null();
      }
      if self
        .type_checker_2_get_fallthrough(stat.body.as_ptr().cast::<AstStat>())
        .is_null()
      {
        return null();
      }
      return node;
    }

    node
  }
}
