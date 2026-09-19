use ulua_ast::records::{ast_node::AstNode, ast_stat_for::AstStatFor};

use crate::{
  functions::{
    cnum::cnum, compute_cost::compute_cost, cost_model::model_cost, get_trip_count::get_trip_count,
  },
  records::{compiler::Compiler, constant::Constant},
};

/// 利润百分比换算基数（C++ `profit = maxBoost * 100 / cost`）
const K_COST_PERCENT_SCALE: i32 = 100;

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn try_compile_unrolled_for(
    &mut self,
    stat: *mut AstStatFor,
    threshold_base: i32,
    threshold_max_boost: i32,
  ) -> bool {
    let stat_ref = unsafe { &*stat };

    let one = cnum(1.0);

    let fromc = self.get_constant(stat_ref.from);
    let toc = self.get_constant(stat_ref.to);
    let stepc = if !stat_ref.step.is_null() {
      self.get_constant(stat_ref.step)
    } else {
      one
    };

    // 三者均为 Number 常量时才可折叠；同时缓存数值供展开编译复用
    let mut nums = (0.0, 0.0, 0.0);
    let trip_count = match (&fromc, &toc, &stepc) {
      (Constant::Number(f), Constant::Number(t), Constant::Number(s)) => {
        nums = (*f, *t, *s);
        get_trip_count(*f, *t, *s)
      }
      _ => -1,
    };

    if trip_count < 0 {
      unsafe {
        (*self.bytecode)
          .add_debug_remark(format_args!("loop unroll failed: invalid iteration count"));
      }
      return false;
    }

    if trip_count > threshold_base {
      unsafe {
        (*self.bytecode).add_debug_remark(format_args!(
          "loop unroll failed: too many iterations ({})",
          trip_count
        ));
      }
      return false;
    }

    if let Some(lv) = self.variables.find(&stat_ref.var)
      && lv.written
    {
      unsafe {
        (*self.bytecode)
          .add_debug_remark(format_args!("loop unroll failed: mutable loop variable"));
      }
      return false;
    }

    let var = stat_ref.var;
    let cost_model = unsafe {
      model_cost(
        stat_ref.body as *mut AstNode,
        &var,
        1,
        // cpp Compiler.cpp:4167 传 `builtins` 成员（O>=1 恒有数据），
        // 而非 O2 才赋值的 builtinsFold 别名指针
        &self.builtins,
        &self.constants,
      )
    };

    let unrolled_cost = compute_cost(cost_model, &[true]) * trip_count;
    let baseline_cost = (compute_cost(cost_model, &[]) + 1) * trip_count;
    let unroll_profit = if unrolled_cost == 0 {
      threshold_max_boost
    } else {
      threshold_max_boost.min(K_COST_PERCENT_SCALE * baseline_cost / unrolled_cost)
    };

    let threshold = threshold_base * unroll_profit / K_COST_PERCENT_SCALE;

    if unrolled_cost > threshold {
      unsafe {
        (*self.bytecode).add_debug_remark(format_args!(
          "loop unroll failed: too expensive (iterations {}, cost {}, profit {:.2}x)",
          trip_count,
          unrolled_cost,
          unroll_profit as f64 / K_COST_PERCENT_SCALE as f64
        ));
      }
      return false;
    }

    unsafe {
      (*self.bytecode).add_debug_remark(format_args!(
        "loop unroll succeeded (iterations {}, cost {}, profit {:.2}x)",
        trip_count,
        unrolled_cost,
        unroll_profit as f64 / K_COST_PERCENT_SCALE as f64
      ));
    }

    unsafe {
      self.compile_unrolled_for(stat, trip_count, nums.0, nums.2);
    };
    true
  }
}
