use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_index_expr::AstExprIndexExpr,
    ast_expr_index_name::AstExprIndexName, ast_node::AstNode,
  },
  rtti::ast_node_as,
};
use ulua_bytecode::methods::bytecode_builder_get_string_hash::bytecode_builder_get_string_hash;
use ulua_common::enums::{
  luau_bytecode_type::{LBC_TYPE_NUMBER, LBC_TYPE_TABLE},
  luau_opcode::LuauOpcode,
};

use crate::{
  enums::kind::Kind,
  records::{
    compile_error::{CompileError, ERR_EXCEEDED_CONSTANT_LIMIT},
    compiler::Compiler,
    l_value::LValue,
  },
};

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub(crate) unsafe fn compile_l_value_use(
    &mut self,
    lv: &LValue,
    reg: u8,
    set: bool,
    target_expr: *mut AstExpr,
  ) {
    unsafe {
      // C++ `compileLValueUse` opens with `setDebugLine(lv.location)` so the store/load
      // instruction is attributed to the index's own location (e.g. the `["d"]` line of
      // `a["b"]["c"]["d"] = 4`), not whatever line was set while compiling the lvalue base.
      self.set_debug_line_location(&lv.location);
      match lv.kind {
        Kind::Local => {
          if set {
            (*self.bytecode).emit_abc(LuauOpcode::LOP_MOVE, lv.reg, reg, 0);
          } else {
            (*self.bytecode).emit_abc(LuauOpcode::LOP_MOVE, reg, lv.reg, 0);
          }
        }
        Kind::Upvalue => {
          if set {
            (*self.bytecode).emit_abc(LuauOpcode::LOP_SETUPVAL, reg, lv.upval, 0);
          } else {
            (*self.bytecode).emit_abc(LuauOpcode::LOP_GETUPVAL, reg, lv.upval, 0);
          }
        }
        Kind::Global => {
          let cid = (*self.bytecode).add_constant_string(lv.name);
          if cid < 0 {
            CompileError::raise(&lv.location, format_args!("{ERR_EXCEEDED_CONSTANT_LIMIT}"));
          }

          let hash = bytecode_builder_get_string_hash(lv.name) as u8;
          if set {
            (*self.bytecode).emit_abc(LuauOpcode::LOP_SETGLOBAL, reg, 0, hash);
          } else {
            (*self.bytecode).emit_abc(LuauOpcode::LOP_GETGLOBAL, reg, 0, hash);
          }
          (*self.bytecode).emit_aux(cid as u32);
        }
        Kind::IndexName => {
          let cid = (*self.bytecode).add_constant_string(lv.name);
          if cid < 0 {
            CompileError::raise(&lv.location, format_args!("{ERR_EXCEEDED_CONSTANT_LIMIT}"));
          }

          let hash = bytecode_builder_get_string_hash(lv.name) as u8;
          if set {
            (*self.bytecode).emit_abc(LuauOpcode::LOP_SETTABLEKS, reg, lv.reg, hash);
          } else {
            (*self.bytecode).emit_abc(LuauOpcode::LOP_GETTABLEKS, reg, lv.reg, hash);
          }
          (*self.bytecode).emit_aux(cid as u32);

          if !target_expr.is_null() {
            let target_expr_index_name =
              ast_node_as::<AstExprIndexName>(target_expr as *mut AstNode);
            if !target_expr_index_name.is_null() {
              self.hint_temporary_expr_reg_type(
                (*target_expr_index_name).expr,
                lv.reg as i32,
                LBC_TYPE_TABLE,
                2,
              );
            }
          }
        }
        Kind::IndexNumber => {
          if set {
            (*self.bytecode).emit_abc(LuauOpcode::LOP_SETTABLEN, reg, lv.reg, lv.number);
          } else {
            (*self.bytecode).emit_abc(LuauOpcode::LOP_GETTABLEN, reg, lv.reg, lv.number);
          }

          if !target_expr.is_null() {
            let target_expr_index_expr =
              ast_node_as::<AstExprIndexExpr>(target_expr as *mut AstNode);
            if !target_expr_index_expr.is_null() {
              self.hint_temporary_expr_reg_type(
                (*target_expr_index_expr).expr,
                lv.reg as i32,
                LBC_TYPE_TABLE,
                1,
              );
            }
          }
        }
        Kind::IndexExpr => {
          if set {
            (*self.bytecode).emit_abc(LuauOpcode::LOP_SETTABLE, reg, lv.reg, lv.index);
          } else {
            (*self.bytecode).emit_abc(LuauOpcode::LOP_GETTABLE, reg, lv.reg, lv.index);
          }

          if !target_expr.is_null() {
            let target_expr_index_expr =
              ast_node_as::<AstExprIndexExpr>(target_expr as *mut AstNode);
            if !target_expr_index_expr.is_null() {
              self.hint_temporary_expr_reg_type(
                (*target_expr_index_expr).expr,
                lv.reg as i32,
                LBC_TYPE_TABLE,
                1,
              );
              self.hint_temporary_expr_reg_type(
                (*target_expr_index_expr).index,
                lv.index as i32,
                LBC_TYPE_NUMBER,
                1,
              );
            }
          }
        }
      }
    }
  }
}
