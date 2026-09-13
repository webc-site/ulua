use core::ptr::null_mut;

use ulua_ast::records::{ast_node::AstNode, ast_stat_assign::AstStatAssign};
use ulua_common::{enums::luau_opcode::LuauOpcode, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::kind::Kind,
  records::{assignment::Assignment, compiler::Compiler},
};

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub(crate) unsafe fn compile_stat_assign(&mut self, stat: *mut AstStatAssign) {
    unsafe {
      let stat_ref = &*stat;
      let mut rs = self.reg_scope_compiler();

      if stat_ref.vars.size == 1 && stat_ref.values.size == 1 {
        let var = self.compile_l_value(*stat_ref.vars.data, &mut rs);
        if var.kind == Kind::Local {
          self.compile_expr(*stat_ref.values.data, var.reg, false);
        } else {
          let reg = self.compile_expr_auto(*stat_ref.values.data, &mut rs);
          self.set_debug_line_ast_node(*stat_ref.vars.data as *mut AstNode);
          self.compile_assign(&var, reg, *stat_ref.vars.data);
        }
        return;
      }

      let mut vars = Vec::with_capacity(stat_ref.vars.size);
      for &var_expr in stat_ref.vars.iter() {
        vars.push(Assignment {
          lvalue: self.compile_l_value(var_expr, &mut rs),
          conflict_reg: Assignment::K_INVALID_REG,
          value_reg: Assignment::K_INVALID_REG,
        });
      }

      self.resolve_assign_conflicts(stat as *mut _, &mut vars, &stat_ref.values);

      // take 取较短一侧，等价于 C++ 的 `min(vars.size, values.size)` 循环
      for (i, &value) in stat_ref.values.iter().take(vars.len()).enumerate() {
        if i + 1 == stat_ref.values.size && stat_ref.vars.size > stat_ref.values.size {
          let rest = (stat_ref.vars.size - stat_ref.values.size + 1) as u32;
          let temp = self.alloc_reg(stat as *mut _, rest);
          self.compile_expr_temp_n(value, temp, rest as u8, true);
          for j in i..stat_ref.vars.size {
            vars[j].value_reg = temp + (j - i) as u8;
          }
        } else {
          let var = &mut vars[i];
          if var.lvalue.kind == Kind::Local {
            var.value_reg = if var.conflict_reg == Assignment::K_INVALID_REG {
              var.lvalue.reg
            } else {
              var.conflict_reg
            };
            self.compile_expr(value, var.value_reg, false);
          } else {
            var.value_reg = self.compile_expr_auto(value, &mut rs);
          }
        }
      }

      for &value in stat_ref.values.iter().skip(stat_ref.vars.size) {
        self.compile_expr_side(value);
      }

      for (i, var) in vars.iter().enumerate() {
        LUAU_ASSERT!(var.value_reg != Assignment::K_INVALID_REG);
        if var.lvalue.kind != Kind::Local {
          self.set_debug_line_location(&var.lvalue.location);
          let target_expr = if i < stat_ref.vars.size {
            *stat_ref.vars.data.add(i)
          } else {
            null_mut()
          };
          self.compile_assign(&var.lvalue, var.value_reg, target_expr);
        }
      }

      for var in vars {
        if var.lvalue.kind == Kind::Local && var.value_reg != var.lvalue.reg {
          (*self.bytecode).emit_abc(LuauOpcode::LOP_MOVE, var.lvalue.reg, var.value_reg, 0);
        }
      }
    }
  }
}
