use core::ptr::null;

use ulua_ast::records::{ast_node::AstNode, ast_stat_for::AstStatFor};

use crate::{
  enums::type_constant_folding::Type,
  functions::{
    cnum::cnum, compute_cost::compute_cost, cost_model::model_cost, get_trip_count::get_trip_count,
  },
  records::compiler::Compiler,
};

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

    let trip_count =
      if fromc.r#type == Type::Number && toc.r#type == Type::Number && stepc.r#type == Type::Number
      {
        get_trip_count(
          unsafe { fromc.data.value_number },
          unsafe { toc.data.value_number },
          unsafe { stepc.data.value_number },
        )
      } else {
        -1
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
        &*self.builtins_fold,
        &self.constants,
      )
    };

    let varc = true;
    let unrolled_cost = unsafe { compute_cost(cost_model, &varc as *const bool, 1) } * trip_count;
    let baseline_cost = (unsafe { compute_cost(cost_model, null(), 0) } + 1) * trip_count;
    let unroll_profit = if unrolled_cost == 0 {
      threshold_max_boost
    } else {
      threshold_max_boost.min(100 * baseline_cost / unrolled_cost)
    };

    let threshold = threshold_base * unroll_profit / 100;

    if unrolled_cost > threshold {
      unsafe {
        (*self.bytecode).add_debug_remark(format_args!(
          "loop unroll failed: too expensive (iterations {}, cost {}, profit {:.2}x)",
          trip_count,
          unrolled_cost,
          unroll_profit as f64 / 100.0
        ));
      }
      return false;
    }

    unsafe {
      (*self.bytecode).add_debug_remark(format_args!(
        "loop unroll succeeded (iterations {}, cost {}, profit {:.2}x)",
        trip_count,
        unrolled_cost,
        unroll_profit as f64 / 100.0
      ));
    }

    unsafe {
      self.compile_unrolled_for(
        stat,
        trip_count,
        fromc.data.value_number,
        stepc.data.value_number,
      );
    };
    true
  }
}
