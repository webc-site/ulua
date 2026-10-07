use core::{
  cmp,
  ptr::{from_mut, null_mut},
};

use ulua_ast::{
  enums::ast_stat_ref::AstStatRef,
  records::{
    ast_expr::AstExpr, ast_expr_function::AstExprFunction, ast_stat::AstStat,
    ast_stat_return::AstStatReturn, ast_visitor::AstVisitor, location::Location,
    position::Position,
  },
  visit::ast_stat_visit_ref,
};
use ulua_config::enums::code::Code;

use crate::{
  functions::{emit_warning::emit_warning, get_fallthrough::get_fallthrough},
  macros::lint_stat_process,
  records::{
    arena_handle::alias_ref, lint_context::LintContext, lint_context_handle::LintContextHandle,
  },
};

#[derive(Debug, Clone)]
pub struct LintImplicitReturn<'ctx> {
  pub(crate) context: LintContextHandle<'ctx>,
}

impl<'ctx> LintImplicitReturn<'ctx> {
  pub fn get_end_location(&mut self, node: *const ()) -> Location {
    lint_implicit_return_get_end_location(self, node)
  }

  fn get_value_return(&mut self, block: &mut AstStat) -> *mut AstStatReturn {
    lint_implicit_return_get_value_return(self, block)
  }
}

impl<'ctx> AstVisitor for LintImplicitReturn<'ctx> {
  fn visit_expr_function(&mut self, node: &mut AstExprFunction) -> bool {
    self.visit(node)
  }
}

// —— 原 methods/lint_implicit_return_get_end_location.rs ——
fn lint_implicit_return_get_end_location(
  _this: &mut LintImplicitReturn,
  node: *const (),
) -> Location {
  let node = node as *const AstStat;
  let node_ref = alias_ref(node);
  let loc = node_ref.base.location;
  if matches!(
    node_ref.as_stat_ref(),
    AstStatRef::Expr(_) | AstStatRef::Assign(_) | AstStatRef::Local(_)
  ) {
    return loc;
  }
  if loc.begin.line == loc.end.line {
    return loc;
  }
  let column = cmp::max(0, loc.end.column as i32 - 3) as u32;
  Location::new(Position::new(loc.end.line, column), loc.end)
}

// —— 原 methods/lint_implicit_return_get_value_return.rs ——
fn lint_implicit_return_get_value_return(
  _this: &mut LintImplicitReturn,
  block: &mut AstStat,
) -> *mut AstStatReturn {
  struct Visitor {
    result: *mut AstStatReturn,
  }
  impl AstVisitor for Visitor {
    fn visit_expr(&mut self, _node: &mut AstExpr) -> bool {
      false
    }
    fn visit_stat_return(&mut self, node: &mut AstStatReturn) -> bool {
      let node = from_mut(node);
      if self.result.is_null() && alias_ref(node).list.size > 0 {
        self.result = node;
      }
      false
    }
  }
  // 既有约定（review.md §2）：result 空 = 未命中值返回，指针身份面即下方 `is_null` 判读点，
  // 与 records/visitor.rs `result` 同族保留。
  let mut visitor = Visitor { result: null_mut() };
  // block 由调用方的 `Node::get_mut()` 物化：解析器保证 AstExprFunction.body 为非空
  // AstStatBlock，`cast::<AstStat>` 依 repr(C) 基址重合上转，`&mut` 借用即存活且独占
  // 证明，`_ref` 门面全链路 safe；visitor 为栈上局部值，遍历在语句结束前完成。
  ast_stat_visit_ref(block, &mut visitor);
  visitor.result
}

// —— 原 methods/lint_implicit_return_process.rs ——
impl<'ctx> LintImplicitReturn<'ctx> {
  lint_stat_process!(LintImplicitReturn);
}

// —— 原 methods/lint_implicit_return_visit.rs ——
impl<'ctx> LintImplicitReturn<'ctx> {
  /// cpp `visit(AstExprFunction*)`：`node` 为分析期存活、由 arena 持有的函数节点
  /// 独占借用（cpp 裸指针形参的 Rust 收窄形态）；其 `body` 句柄经 `cast`+`get_mut`
  /// 出借基类视图：`get_fallthrough` 仍按指针身份消费，`get_value_return` 收引用。
  pub fn visit(&mut self, node: &mut AstExprFunction) -> bool {
    let bodyf = get_fallthrough(node.body.cast::<AstStat>().as_ptr());
    let vret = self.get_value_return(node.body.cast::<AstStat>().get_mut());
    if !bodyf.is_null() && !vret.is_null() {
      let location = self.get_end_location(bodyf as *const ());
      let return_line = alias_ref(vret).base.base.location.begin.line + 1;
      let mut handle = self.context;
      let context = handle.get();
      if !node.debugname.is_null() {
        let debugname = node.debugname.as_str_or_empty();
        emit_warning(
          context,
          Code::ImplicitReturn,
          location,
          format_args!(
            "Function '{}' can implicitly return no values even though there's an explicit return at line {}; add explicit return to silence",
            debugname, return_line
          ),
        );
      } else {
        emit_warning(
          context,
          Code::ImplicitReturn,
          location,
          format_args!(
            "Function can implicitly return no values even though there's an explicit return at line {}; add explicit return to silence",
            return_line
          ),
        );
      }
    }
    true
  }
}
