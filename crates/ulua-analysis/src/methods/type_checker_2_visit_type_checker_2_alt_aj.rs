use ulua_ast::records::ast_expr_call::AstExprCall;

use crate::{
  enums::{type_context::TypeContext, value_context::ValueContext},
  functions::{match_assert::match_assert, match_type_of::match_type_of},
  records::{in_conditional_context::InConditionalContext, type_checker_2::TypeChecker2},
};

impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证 `call` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_expr_call(&mut self, call: *mut AstExprCall) {
    let mut _flipper: Option<InConditionalContext> = None;

    unsafe {
      // We want to preserve the existing conditional context if we are in a `typeof` call.
      if !match_type_of(&*call) {
        _flipper = Some(InConditionalContext::new(
          &mut self.type_context as *mut TypeContext,
          TypeContext::Default,
        ));
      }

      self.visit_ast_expr_value_context((*call).func, ValueContext::RValue);

      if match_assert(&*call) && (*call).args.size > 0 {
        {
          // C++: `InConditionalContext flipper(&typeContext);` (TypeChecker2.cpp:1843)
          // uses the default `newValue = TypeContext::Condition` so that the first
          // argument of `assert(...)` is checked in a conditional context (refinements
          // like `assert(typeof(x) == "table")` apply to property accesses inside it).
          let _flipper = InConditionalContext::new(
            &mut self.type_context as *mut TypeContext,
            TypeContext::Condition,
          );
          self.visit_ast_expr_value_context(*(*call).args.data, ValueContext::RValue);
        }

        for i in 1..(*call).args.size {
          let arg = *(*call).args.data.add(i);
          self.visit_ast_expr_value_context(arg, ValueContext::RValue);
        }
      } else {
        for &arg in (*call).args.as_slice() {
          self.visit_ast_expr_value_context(arg, ValueContext::RValue);
        }
      }

      // SAFETY: call 指向 AST arena 节点。
      self.visit_call(&*call);
    }
  }
}
