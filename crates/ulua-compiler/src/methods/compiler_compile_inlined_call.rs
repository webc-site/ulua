use core::{ptr::null_mut, slice::from_raw_parts};

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_function::AstExprFunction,
    ast_expr_varargs::AstExprVarargs, ast_node::AstNode,
  },
  rtti::ast_node_as,
};
use ulua_common::{FFlag, enums::luau_opcode::LuauOpcode};

use crate::{
  enums::type_constant_folding::Type,
  functions::{
    analyze_builtins::analyze_builtins, undo_changes_constant_folding::undo_changes_expr,
    undo_changes_constant_folding_alt_b::undo_changes_local,
  },
  records::{
    compiler::Compiler,
    constant::{Constant, ConstantData},
    inline_arg::InlineArg,
    inline_frame::InlineFrame,
  },
};

const K_INVALID_REG: u8 = 255;
const K_DEFAULT_ALLOC_PC: u32 = !0u32;

/// Type::Unknown 占位常量（值域无意义，对齐 C++ `{Constant::Type_Unknown}`）
fn unknown_constant() -> Constant {
  Constant {
    r#type: Type::Unknown,
    string_length: 0,
    data: ConstantData { value_number: 0.0 },
  }
}

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn compile_inlined_call(
    &mut self,
    expr: *mut AstExprCall,
    func: *mut AstExprFunction,
    target: u8,
    target_count: u8,
  ) {
    unsafe {
      let _rs = self.reg_scope_compiler();
      let old_locals = self.local_stack.len();

      let func_args_size = (*func).args.size;
      let expr_args_size = (*expr).args.size;
      // size == 0 时 data 指针可能为 null，from_raw_parts(null, 0) 属 UB，需守卫
      let func_args = if func_args_size > 0 {
        from_raw_parts((*func).args.data, func_args_size)
      } else {
        &[]
      };
      let expr_args = if expr_args_size > 0 {
        from_raw_parts((*expr).args.data, expr_args_size)
      } else {
        &[]
      };

      let mut args: Vec<InlineArg> = Vec::with_capacity(func_args_size);

      // evaluate all arguments; note that we don't emit code for constant arguments (relying on constant folding)
      // note that compiler state (variable registers/values) does not change here - we defer that to a separate loop below to handle nested calls
      for (i, &var) in func_args.iter().enumerate() {
        let arg: *mut AstExpr = if i < expr_args_size {
          expr_args[i]
        } else {
          null_mut()
        };

        if i + 1 == expr_args_size && func_args_size > expr_args_size && self.is_expr_mult_ret(arg)
        {
          // if the last argument can return multiple values, we need to compute all of them into the remaining arguments
          let tail: u32 = (func_args_size - expr_args_size) as u32 + 1;
          let reg = self.alloc_reg(arg as *mut AstNode, tail);
          let allocpc = (*self.bytecode).get_debug_pc();

          let call = ast_node_as::<AstExprCall>(arg as *mut AstNode);
          if !call.is_null() {
            self.compile_expr_call(call, reg, tail as u8, true, false);
          } else {
            let va = ast_node_as::<AstExprVarargs>(arg as *mut AstNode);
            if !va.is_null() {
              self.compile_expr_varargs(va, reg, tail as u8, false);
            } else {
              ulua_common::macros::luau_assert::LUAU_ASSERT!(false, "Unexpected expression type");
            }
          }

          // all remaining function arguments have been allocated and assigned to
          for (offset, &local) in func_args[i..].iter().enumerate() {
            args.push(InlineArg {
              local,
              reg: reg + offset as u8,
              value: unknown_constant(),
              allocpc,
              init: null_mut(),
            });
          }
          break;
        } else if self.variables.find(&var).is_some_and(|vv| vv.written) {
          // if the argument is mutated, we need to allocate a fresh register even if it's a constant
          let reg = self.alloc_reg(arg as *mut AstNode, 1u32);
          let allocpc = (*self.bytecode).get_debug_pc();
          if !arg.is_null() {
            self.compile_expr_temp(arg, reg);
          } else {
            (*self.bytecode).emit_abc(LuauOpcode::LOP_LOADNIL, reg, 0, 0);
          }
          args.push(InlineArg {
            local: var,
            reg,
            value: unknown_constant(),
            allocpc,
            init: null_mut(),
          });
        } else if arg.is_null() {
          // since the argument is not mutated, we can simply fold the value into the expressions that need it
          args.push(InlineArg {
            local: var,
            reg: K_INVALID_REG,
            value: Constant {
              r#type: Type::Nil,
              string_length: 0,
              data: ConstantData { value_number: 0.0 },
            },
            allocpc: K_DEFAULT_ALLOC_PC,
            init: null_mut(),
          });
        } else if let Some(cv) = self
          .constants
          .find(&arg)
          .filter(|cv| cv.r#type != Type::Unknown)
        {
          // since the argument is not mutated, we can simply fold the value into the expressions that need it
          args.push(InlineArg {
            local: var,
            reg: K_INVALID_REG,
            value: *cv,
            allocpc: K_DEFAULT_ALLOC_PC,
            init: null_mut(),
          });
        } else {
          let le = self.get_expr_local(arg);
          // 一次性取出 written/init，避免与 get_expr_local_reg 的可变借用冲突
          let lv = if !le.is_null() {
            self
              .variables
              .find(&(*le).local)
              .map(|v| (v.written, v.init))
          } else {
            None
          };

          // if the argument is a local that isn't mutated, we will simply reuse the existing register
          let reg = if !le.is_null() {
            self.get_expr_local_reg(le as *mut AstExpr)
          } else {
            -1
          };
          if reg >= 0 && lv.as_ref().is_none_or(|(written, _)| !written) {
            args.push(InlineArg {
              local: var,
              reg: reg as u8,
              value: unknown_constant(),
              allocpc: K_DEFAULT_ALLOC_PC,
              init: lv.map_or(null_mut(), |(_, init)| init),
            });
          } else {
            let temp = self.alloc_reg(arg as *mut AstNode, 1u32);
            let allocpc = (*self.bytecode).get_debug_pc();
            self.compile_expr_temp(arg, temp);
            args.push(InlineArg {
              local: var,
              reg: temp,
              value: unknown_constant(),
              allocpc,
              init: arg,
            });
          }
        }
      }

      // evaluate extra expressions for side effects
      if let Some(extra) = expr_args.get(func_args_size..) {
        for &side in extra {
          self.compile_expr_side(side);
        }
      }

      // apply all evaluated arguments to the compiler state
      // note: locals use current startpc for debug info, although some of them have been computed earlier; this is similar to compileStatLocal
      for arg in &args {
        if arg.value.r#type == Type::Unknown {
          self.push_local(arg.local, arg.reg, arg.allocpc);
          if !arg.init.is_null()
            && let Some(lv) = self.variables.find_mut(&arg.local)
          {
            lv.init = arg.init;
          }
        } else {
          *self.locstants.get_or_insert(arg.local) = arg.value;
        }
      }

      self.inline_frames.push(InlineFrame {
        func,
        local_offset: old_locals,
        target,
        target_count,
        return_jumps: Vec::new(),
      });

      let func_body = (*func).body;

      {
        let ib = &mut self.inline_builtins as *mut _;
        analyze_builtins(
          &mut *ib,
          &self.globals,
          &self.variables,
          &self.options,
          func_body as *mut AstNode,
          &*self.names,
        );
      }

      // If we found new builtins, apply them, but record which expressions we changed so we can undo later
      if !self.inline_builtins.is_empty() {
        let entries: Vec<(*mut AstExprCall, i32)> =
          self.inline_builtins.iter().map(|(k, v)| (*k, *v)).collect();
        for (call_expr, bfid) in entries {
          let builtin = *self.builtins.get_or_insert(call_expr);
          if bfid != builtin {
            *self.inline_builtins_backup.get_or_insert(call_expr) = builtin;
            *self.builtins.get_or_insert(call_expr) = bfid;
          }
        }
        self.inline_builtins.clear();
      }

      let record_changes =
        FFlag::LuauCompilePropagateTableProps2.get() && FFlag::LuauCompileFoldOptimize.get();

      if record_changes {
        self.expr_changes.clear();
        self.local_changes.clear();
      }

      self.fold_constants(func_body as *mut AstNode, record_changes);

      let mut terminates_early = false;
      let body_size = (*func_body).body.size;
      let body = if body_size > 0 {
        from_raw_parts((*func_body).body.data, body_size)
      } else {
        &[]
      };
      for &stat in body {
        self.compile_stat(stat);
        if self.always_terminates(stat) {
          terminates_early = true;
          let curr_frame = self.inline_frames.last_mut().unwrap();
          if !curr_frame.return_jumps.is_empty() {
            let last_jump = *curr_frame.return_jumps.last().unwrap();
            if last_jump == (*self.bytecode).emit_label() - 1 {
              (*self.bytecode).undo_emit(LuauOpcode::LOP_JUMP);
              curr_frame.return_jumps.pop();
            }
          }
          break;
        }
      }

      if !terminates_early {
        for t in 0..target_count {
          (*self.bytecode).emit_abc(LuauOpcode::LOP_LOADNIL, target + t, 0, 0);
        }
        self.close_locals(old_locals);
      }

      self.pop_locals(old_locals);
      let return_label = (*self.bytecode).emit_label();
      let rj = &mut self.inline_frames.last_mut().unwrap().return_jumps as *mut Vec<usize>;
      self.patch_jumps(expr as *mut AstNode, &mut *rj, return_label);
      self.inline_frames.pop();

      // clean up constant state for future inlining attempts
      for &local in func_args {
        if let Some(var) = self.locstants.find_mut(&local) {
          var.r#type = Type::Unknown;
        }
        if let Some(lv) = self.variables.find_mut(&local) {
          lv.init = null_mut();
        }
      }

      if !self.inline_builtins_backup.is_empty() {
        let entries: Vec<(*mut AstExprCall, i32)> = self
          .inline_builtins_backup
          .iter()
          .map(|(k, v)| (*k, *v))
          .collect();
        for (call_expr, bfid) in entries {
          *self.builtins.get_or_insert(call_expr) = bfid;
        }
        self.inline_builtins_backup.clear();
      }

      if record_changes {
        undo_changes_expr(&mut self.constants, &self.expr_changes);
        undo_changes_local(&mut self.locstants, &self.local_changes);
      } else {
        self.fold_constants(func_body as *mut AstNode, false);
      }
    }
  }
}
