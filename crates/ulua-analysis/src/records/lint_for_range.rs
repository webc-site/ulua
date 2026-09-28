use core::ptr::from_mut;

use ulua_ast::{
  enums::ast_expr_ref::AstExprRef,
  records::{
    ast_expr_unary::AstExprUnaryOp, ast_stat_for::AstStatFor, ast_visitor::AstVisitor,
    location::Location,
  },
  visit::ast_stat_visit,
};
use ulua_config::enums::code::Code;
use ulua_vm::functions::format_directive::format_g;

use crate::{
  functions::emit_warning::emit_warning,
  macros::lint_stat_process,
  records::{lint_context::LintContext, lint_context_handle::LintContextHandle},
};

#[derive(Debug, Clone)]
pub struct LintForRange<'ctx> {
  pub(crate) context: LintContextHandle<'ctx>,
}

impl<'ctx> LintForRange<'ctx> {
  pub fn get_loop_end(&self, from: f64, to: f64) -> f64 {
    from + (to - from).floor()
  }
}

impl<'ctx> AstVisitor for LintForRange<'ctx> {
  fn visit_stat_for(&mut self, node: &mut AstStatFor) -> bool {
    // SAFETY: 遍历入口保证 `node` 指向存活的 AstStatFor，与 `visit_ast_stat_for_linter` 的裸指针契约一致。
    unsafe { self.visit_ast_stat_for_linter(from_mut(node)) }
  }
}

// —— 原 methods/lint_for_range_process.rs ——
impl<'ctx> LintForRange<'ctx> {
  lint_stat_process!(LintForRange);
}

// —— 原 methods/lint_for_range_visit.rs ——
impl<'ctx> LintForRange<'ctx> {
  /// # Safety
  /// `{node}` 须指向本次遍历期间存活的 parse-arena 节点：非空、对齐，地址在该 arena 释放前不
  /// 移动；调用方（AstVisitor 遍历驱动）单线程串行访问，函数体内不产生与之重叠的可变借用。
  /// 对应 C++ `bool LintForRange::visit(AstStatFor* node)` (`cpp/Analysis/src/Linter.cpp:1262`)。
  pub(crate) unsafe fn visit_ast_stat_for_linter(&mut self, node: *mut AstStatFor) -> bool {
    let node_ref = unsafe { &*node };
    // step 落可空 OptNode：cpp `!node->step` 判空以 is_none 承接；from/to 已
    // 句柄化为非空 Node，as_ptr 桥交指针形态判型门面，`.get()` 直出只读视图
    // （原死 unsafe 消解）。
    if node_ref.step.is_none() {
      let fc = match node_ref.from.as_expr_ref() {
        AstExprRef::ConstantNumber(c) => Some(c.value),
        _ => None,
      };
      let fu_len = matches!(
        node_ref.from.as_expr_ref(),
        AstExprRef::Unary(u) if u.op == AstExprUnaryOp::Len
      );
      let tc = match node_ref.to.as_expr_ref() {
        AstExprRef::ConstantNumber(c) => Some(c.value),
        _ => None,
      };
      let tu_len = matches!(
        node_ref.to.as_expr_ref(),
        AstExprRef::Unary(u) if u.op == AstExprUnaryOp::Len
      );
      let range_location = Location::new(
        node_ref.from.get().base.location.begin,
        node_ref.to.get().base.location.end,
      );
      // 句柄为 Copy，先局部化，使 emit_warning 的 `&mut` 重建不与下方
      // `self.get_loop_end` 的只读借用相互冲突。
      let mut ctx = self.context;
      if (fu_len && tc == Some(1.0)) || fc.is_some_and(|fc| tc.is_some_and(|tc| fc > tc)) {
        emit_warning(
          ctx.get(),
          Code::ForRange,
          range_location,
          format_args!("For loop should iterate backwards; did you forget to specify -1 as step?"),
        );
      } else if let (Some(fc), Some(tc)) = (fc, tc)
        && self.get_loop_end(fc, tc) != tc
      {
        // 上游为 C `printf("%g", …)`，6 位有效数字并自动切换科学计数法
        emit_warning(
          ctx.get(),
          Code::ForRange,
          range_location,
          format_args!(
            "For loop ends at {} instead of {}; did you forget to specify step?",
            format_g(self.get_loop_end(fc, tc)),
            format_g(tc)
          ),
        );
      } else if fc == Some(0.0) && tu_len {
        emit_warning(
          ctx.get(),
          Code::ForRange,
          range_location,
          format_args!("For loop starts at 0, but arrays start at 1"),
        );
      } else if fu_len && tc == Some(0.0) {
        emit_warning(
          ctx.get(),
          Code::ForRange,
          range_location,
          format_args!(
            "For loop should iterate backwards; did you forget to specify -1 as step? Also consider changing 0 to 1 since arrays start at 1"
          ),
        );
      }
    }
    true
  }
}
