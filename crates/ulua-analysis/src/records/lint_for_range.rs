use core::ptr::from_mut;

use ulua_ast::{
  records::{
    ast_expr_constant_number::AstExprConstantNumber,
    ast_expr_unary::{AstExprUnary, AstExprUnaryOp},
    ast_stat_for::AstStatFor,
    ast_visitor::AstVisitor,
    location::Location,
  },
  rtti::ast_node_try_as_ptr,
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
      let fc = unsafe { ast_node_try_as_ptr::<AstExprConstantNumber>(node_ref.from.as_ptr()) };
      let fu = unsafe { ast_node_try_as_ptr::<AstExprUnary>(node_ref.from.as_ptr()) };
      let tc = unsafe { ast_node_try_as_ptr::<AstExprConstantNumber>(node_ref.to.as_ptr()) };
      let tu = unsafe { ast_node_try_as_ptr::<AstExprUnary>(node_ref.to.as_ptr()) };
      let range_location = Location::new(
        node_ref.from.get().base.location.begin,
        node_ref.to.get().base.location.end,
      );
      let fu_len = fu.is_some_and(|u| u.op == AstExprUnaryOp::Len);
      let tu_len = tu.is_some_and(|u| u.op == AstExprUnaryOp::Len);
      // 句柄为 Copy，先局部化，使 emit_warning 的 `&mut` 重建不与下方
      // `self.get_loop_end` 的只读借用相互冲突。
      let mut ctx = self.context;
      if (fu_len && tc.is_some_and(|c| c.value == 1.0))
        // 双写合一：is_some 判定+unwrap 比较并为 is_some_and 短路（与
        // `fc.is_some() && tc.is_some() && fc.unwrap()...` 逐分支等价）。
        || fc.is_some_and(|fc| tc.is_some_and(|tc| fc.value > tc.value))
      {
        emit_warning(
          ctx.get(),
          Code::ForRange,
          range_location,
          format_args!("For loop should iterate backwards; did you forget to specify -1 as step?"),
        );
      } else if let (Some(fc), Some(tc)) = (fc, tc)
        && self.get_loop_end(fc.value, tc.value) != tc.value
      {
        // 上游为 C `printf("%g", …)`，6 位有效数字并自动切换科学计数法
        emit_warning(
          ctx.get(),
          Code::ForRange,
          range_location,
          format_args!(
            "For loop ends at {} instead of {}; did you forget to specify step?",
            format_g(self.get_loop_end(fc.value, tc.value)),
            format_g(tc.value)
          ),
        );
      } else if fc.is_some_and(|c| c.value == 0.0) && tu_len {
        emit_warning(
          ctx.get(),
          Code::ForRange,
          range_location,
          format_args!("For loop starts at 0, but arrays start at 1"),
        );
      } else if fu_len && tc.is_some_and(|c| c.value == 0.0) {
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
