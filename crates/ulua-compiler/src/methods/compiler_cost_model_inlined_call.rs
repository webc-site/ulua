use core::ptr::null_mut;

use ulua_ast::records::{
  ast_expr_call::AstExprCall, ast_expr_function::AstExprFunction, ast_node::AstNode,
};
use ulua_common::FFlag;

use crate::{
  enums::type_constant_folding::Type,
  functions::{
    cost_model::model_cost, undo_changes_constant_folding::undo_changes_expr,
    undo_changes_constant_folding_alt_b::undo_changes_local,
  },
  records::{
    compiler::Compiler,
    constant::{Constant, ConstantData},
  },
};

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn cost_model_inlined_call(
    &mut self,
    expr: *mut AstExprCall,
    func: *mut AstExprFunction,
  ) -> u64 {
    unsafe {
      let func_ref = &*func;
      let expr_ref = &*expr;

      for (i, &var) in func_ref.args.iter().enumerate() {
        let arg = if i < expr_ref.args.size {
          *expr_ref.args.data.add(i)
        } else {
          null_mut()
        };

        if i + 1 == expr_ref.args.size
          && func_ref.args.size > expr_ref.args.size
          && self.is_expr_mult_ret(arg)
        {
          break;
        }

        if self.variables.find(&var).is_some_and(|vv| vv.written) {
          continue;
        }

        if arg.is_null() {
          *self.locstants.get_or_insert(var) = Constant {
            r#type: Type::Nil,
            string_length: 0,
            data: ConstantData::default(),
          };
        } else if let Some(cv) = self.constants.find(&arg)
          && cv.r#type != Type::Unknown
        {
          *self.locstants.get_or_insert(var) = *cv;
        }
      }

      let record_changes =
        FFlag::LuauCompilePropagateTableProps2.get() && FFlag::LuauCompileFoldOptimize.get();

      if record_changes {
        self.expr_changes.clear();
        self.local_changes.clear();
      }

      self.fold_constants(func_ref.body as *mut AstNode, record_changes);

      let cost = model_cost(
        func_ref.body as *mut AstNode,
        func_ref.args.data as *const _,
        func_ref.args.size,
        &self.builtins,
        &self.constants,
      );

      for &arg in func_ref.args.iter() {
        if let Some(var) = self.locstants.find_mut(&arg) {
          var.r#type = Type::Unknown;
        }
      }

      if record_changes {
        undo_changes_expr(&mut self.constants, &self.expr_changes);
        undo_changes_local(&mut self.locstants, &self.local_changes);
      } else {
        self.fold_constants(func_ref.body as *mut AstNode, false);
      }

      cost
    }
  }
}
