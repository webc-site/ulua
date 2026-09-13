use ulua_ast::{
  records::{ast_node::AstNode, ast_stat_block::AstStatBlock, ast_stat_local::AstStatLocal},
  rtti::ast_node_as,
};
use ulua_common::{FFlag, FInt, macros::luau_assert::LUAU_ASSERT};

use crate::records::{
  non_strict_context::NonStrictContext, non_strict_type_checker::NonStrictTypeChecker,
  recursion_counter::RecursionCounter,
};
impl NonStrictTypeChecker {
  /// # Safety
  /// 调用方须保证 `block` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_block(&mut self, block: *mut AstStatBlock) -> NonStrictContext {
    LUAU_ASSERT!(!block.is_null());

    let mut _rc: Option<RecursionCounter> = None;
    if FFlag::LuauAddRecursionCounterToNonStrictTypeChecker.get() {
      _rc = Some(unsafe {
        RecursionCounter::recursion_counter_i32(&mut self.non_strict_recursion_count)
      });
      if FInt::LuauNonStrictTypeCheckerRecursionLimit.get() > 0
        && self.non_strict_recursion_count >= FInt::LuauNonStrictTypeCheckerRecursionLimit.get()
      {
        return NonStrictContext::new();
      }
    }

    let _stack_pusher = self.push_stack(block as *mut AstNode);

    let mut ctx = NonStrictContext::new();

    unsafe {
      let block = &*block;
      let mut i = block.body.size;
      while i > 0 {
        i -= 1;
        let stat = *block.body.data.add(i);

        let local_ptr = ast_node_as::<AstStatLocal>(stat as *mut AstNode);
        if !local_ptr.is_null() {
          let local = &*local_ptr;
          self.visit_ast_stat(stat);
          let mut j = 0usize;
          while j < local.vars.size {
            let var = *local.vars.data.add(j);
            let def = (*self.dfg).get_def_ast_local(var);
            ctx.remove(&def);
            // C++ `visit(local->annotation)` — `local` here is the loop var (AstLocal).
            self.visit_ast_type((*var).annotation);
            j += 1;
          }
        } else {
          let other_ctx = self.visit_ast_stat(stat);
          ctx = NonStrictContext::disjunction(self.builtin_types, self.arena, &other_ctx, &ctx);
        }
      }
    }

    ctx
  }
}
