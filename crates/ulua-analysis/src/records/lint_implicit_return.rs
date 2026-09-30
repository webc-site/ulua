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
  visit::ast_stat_visit,
};
use ulua_config::enums::code::Code;

use crate::{
  functions::{emit_warning::emit_warning, get_fallthrough::get_fallthrough},
  macros::lint_stat_process,
  records::{lint_context::LintContext, lint_context_handle::LintContextHandle},
};

#[derive(Debug, Clone)]
pub struct LintImplicitReturn<'ctx> {
  pub(crate) context: LintContextHandle<'ctx>,
}

impl<'ctx> LintImplicitReturn<'ctx> {
  pub fn get_end_location(&mut self, node: *const ()) -> Location {
    lint_implicit_return_get_end_location(self, node)
  }

  pub fn get_value_return(&mut self, node: *mut ()) -> *mut AstStatReturn {
    lint_implicit_return_get_value_return(self, node)
  }
}

impl<'ctx> AstVisitor for LintImplicitReturn<'ctx> {
  fn visit_expr_function(&mut self, node: &mut AstExprFunction) -> bool {
    self.visit(node)
  }
}

// —— 原 methods/lint_implicit_return_get_end_location.rs ——
pub fn lint_implicit_return_get_end_location(
  _this: &mut LintImplicitReturn,
  node: *const (),
) -> Location {
  let node = node as *const AstStat;
  let node_ref = unsafe { &*node };
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
pub fn lint_implicit_return_get_value_return(
  _this: &mut LintImplicitReturn,
  node: *mut (),
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
      // Safety: node 是 from_mut 从 dispatch 借出的 &mut AstStatReturn 取得的地址，
      // 非空、对齐、指向 traversal 独占的存活 AST 节点；读取 .list.size 与把该地址
      // 存入 self.result（返回给 visit，在调用方持有的同一 AST 寿命内使用）均不
      // 越过借用来源的有效范围。
      unsafe {
        if self.result.is_null() && (*node).list.size > 0 {
          self.result = node;
        }
      }
      false
    }
  }
  let mut visitor = Visitor { result: null_mut() };
  // Safety: node 来自 lint_implicit_return_visit 的 `node.body as *mut ()`，
  // 解析器保证 AstExprFunction.body 为非空 AstStatBlock arena 节点；AstStatBlock
  // 首字段即 AstStat 基座（repr(C) 基址重合），`as *mut AstStat` 是合法基类视图，
  // 满足 ast_stat_visit 的 "null 或存活前缀节点" 契约；visitor 为栈上局部值的
  // 独占可变借用，遍历在语句结束前完成。
  unsafe {
    ast_stat_visit(node.cast::<AstStat>(), &mut visitor);
  }
  visitor.result
}

// —— 原 methods/lint_implicit_return_process.rs ——
impl<'ctx> LintImplicitReturn<'ctx> {
  lint_stat_process!(LintImplicitReturn);
}

// —— 原 methods/lint_implicit_return_visit.rs ——
impl<'ctx> LintImplicitReturn<'ctx> {
  /// cpp `visit(AstExprFunction*)`：`node` 为分析期存活、由 arena 持有的函数节点
  /// 共享借用（cpp 裸指针形参的 Rust 对应）；其 `body` 字段仍是裸指针，交由
  /// `get_fallthrough`/`get_value_return` 按各自契约处理。
  pub fn visit(&mut self, node: &AstExprFunction) -> bool {
    let bodyf = get_fallthrough(node.body.cast::<AstStat>().as_ptr());
    let vret = self.get_value_return(node.body.as_ptr().cast::<()>());
    if !bodyf.is_null() && !vret.is_null() {
      let location = self.get_end_location(bodyf as *const ());
      // Safety: vret 经上方 !vret.is_null() 分支保证非空，指向 arena 存活 AstStat；
      // 仅解引用读取 base.base.location.begin.line，产生临时只读借用无别名。
      let return_line = unsafe { (*vret).base.base.location.begin.line + 1 };
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
