use core::{cmp::min, ptr::null, slice::from_raw_parts};

use ulua_ast::records::{ast_expr_call::AstExprCall, ast_expr_function::AstExprFunction};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{functions::compute_cost::compute_cost, records::compiler::Compiler};

/// 成本模型位向量最多覆盖的参数数（C++ `bool varc[8]`）
const K_MAX_COST_ARGS: usize = 8;
/// 内联基线成本加成（C++ `computeCost(...) + 3`）
const K_BASELINE_COST_BONUS: i32 = 3;
/// 内联允许的最大寄存器压力（C++ `regTop > 128`）
const K_MAX_INLINE_REG_TOP: u32 = 128;
/// 内联允许的最大函数栈帧大小（C++ `stackSize > 32`）
const K_MAX_INLINE_STACK_SIZE: u32 = 32;

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn try_compile_inlined_call(
    &mut self,
    expr: *mut AstExprCall,
    func: *mut AstExprFunction,
    target: u8,
    target_count: u8,
    mult_ret: bool,
    threshold_base: i32,
    threshold_max_boost: i32,
    depth_limit: i32,
  ) -> bool {
    unsafe {
      let fi = self.functions.find(&func);
      LUAU_ASSERT!(fi.is_some());
      let fi = fi.unwrap_unchecked();
      let fi_stack_size = fi.stack_size;
      let mut call_cost_model = fi.cost_model;
      let fi_cost_model = fi.cost_model;

      if self.reg_top > K_MAX_INLINE_REG_TOP || fi_stack_size > K_MAX_INLINE_STACK_SIZE {
        (*self.bytecode).add_debug_remark(format_args!("inlining failed: high register pressure"));
        return false;
      }

      if self.inline_frames.len() as i32 >= depth_limit {
        (*self.bytecode).add_debug_remark(format_args!("inlining failed: too many inlined frames"));
        return false;
      }

      for frame in &self.inline_frames {
        if frame.func == func {
          (*self.bytecode).add_debug_remark(format_args!(
            "inlining failed: can't inline recursive calls"
          ));
          return false;
        }
      }

      if mult_ret {
        (*self.bytecode).add_debug_remark(format_args!(
          "inlining failed: can't convert fixed returns to multret"
        ));
        return false;
      }

      let func_args_size = (*func).args.size;
      let expr_args_size = (*expr).args.size;

      // compute constant bitvector for all arguments to feed the cost model
      let mut varc = [false; K_MAX_COST_ARGS];
      let mut has_constant = false;

      if expr_args_size > 0 {
        let expr_args = from_raw_parts((*expr).args.data, expr_args_size);
        for (i, &arg) in expr_args
          .iter()
          .take(min(func_args_size, K_MAX_COST_ARGS))
          .enumerate()
        {
          if self.is_constant(arg) {
            varc[i] = true;
            has_constant = true;
          }
        }
      }

      // if the last argument only returns a single value, all following arguments are nil
      if expr_args_size != 0 && !self.is_expr_mult_ret(*(*expr).args.data.add(expr_args_size - 1)) {
        for flag in varc
          .iter_mut()
          .take(min(func_args_size, K_MAX_COST_ARGS))
          .skip(expr_args_size)
        {
          *flag = true;
          has_constant = true;
        }
      }

      // If we had constant arguments that can affect the cost model of this specific call in non-trivial ways
      if has_constant {
        call_cost_model = self.cost_model_inlined_call(expr, func);
      }

      let inlined_cost = compute_cost(
        call_cost_model,
        varc.as_ptr(),
        min(func_args_size, K_MAX_COST_ARGS),
      );
      let baseline_cost = compute_cost(fi_cost_model, null(), 0) + K_BASELINE_COST_BONUS;
      let inline_profit = if inlined_cost == 0 {
        threshold_max_boost
      } else {
        min(threshold_max_boost, 100 * baseline_cost / inlined_cost)
      };

      let threshold = threshold_base * inline_profit / 100;

      if inlined_cost > threshold {
        (*self.bytecode).add_debug_remark(format_args!(
          "inlining failed: too expensive (cost {}, profit {:.2}x)",
          inlined_cost,
          inline_profit as f64 / 100.0
        ));
        return false;
      }

      (*self.bytecode).add_debug_remark(format_args!(
        "inlining succeeded (cost {}, profit {:.2}x, depth {})",
        inlined_cost,
        inline_profit as f64 / 100.0,
        self.inline_frames.len() as i32
      ));

      self.compile_inlined_call(expr, func, target, target_count);
      true
    }
  }
}
