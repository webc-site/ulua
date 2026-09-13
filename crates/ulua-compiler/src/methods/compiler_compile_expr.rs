use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_binary::AstExprBinary, ast_expr_call::AstExprCall,
    ast_expr_constant_bool::AstExprConstantBool, ast_expr_constant_integer::AstExprConstantInteger,
    ast_expr_constant_nil::AstExprConstantNil, ast_expr_constant_number::AstExprConstantNumber,
    ast_expr_constant_string::AstExprConstantString, ast_expr_function::AstExprFunction,
    ast_expr_global::AstExprGlobal, ast_expr_group::AstExprGroup, ast_expr_if_else::AstExprIfElse,
    ast_expr_index_expr::AstExprIndexExpr, ast_expr_index_name::AstExprIndexName,
    ast_expr_instantiate::AstExprInstantiate, ast_expr_interp_string::AstExprInterpString,
    ast_expr_local::AstExprLocal, ast_expr_table::AstExprTable,
    ast_expr_type_assertion::AstExprTypeAssertion, ast_expr_unary::AstExprUnary,
    ast_expr_varargs::AstExprVarargs, ast_node::AstNode,
  },
  rtti::{ast_node_as, ast_node_is},
};
use ulua_bytecode::methods::bytecode_builder_get_string_hash::bytecode_builder_get_string_hash;
use ulua_common::{FFlag, enums::luau_opcode::LuauOpcode, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::type_constant_folding::Type,
  functions::{sref_compiler::sref_ast_name, sref_compiler_alt_c::sref_ast_array_c_char},
  records::{
    compile_error::{CompileError, ERR_EXCEEDED_CONSTANT_LIMIT},
    compiler::Compiler,
  },
};

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn compile_expr(&mut self, node: *mut AstExpr, target: u8, target_temp: bool) {
    unsafe { self.set_debug_line_ast_node(node as *mut AstNode) };

    unsafe {
      if self.options.coverage_level >= 2 && self.needs_coverage(node as *mut AstNode) {
        (*self.bytecode).emit_abc(LuauOpcode::LOP_COVERAGE, 0, 0, 0);
      }

      if let Some(cv) = self.constants.find(&node)
        && cv.r#type != Type::Unknown
      {
        let cv = *cv;
        self.compile_expr_constant(node, &cv, target);
        return;
      }

      let expr_group = ast_node_as::<AstExprGroup>(node as *mut AstNode);
      if !expr_group.is_null() {
        self.compile_expr((*expr_group).expr, target, target_temp);
        return;
      }

      if ast_node_is::<AstExprConstantNil>(&*(node as *mut AstNode)) {
        (*self.bytecode).emit_abc(LuauOpcode::LOP_LOADNIL, target, 0, 0);
        return;
      }

      let expr_bool = ast_node_as::<AstExprConstantBool>(node as *mut AstNode);
      if !expr_bool.is_null() {
        (*self.bytecode).emit_abc(LuauOpcode::LOP_LOADB, target, (*expr_bool).value as u8, 0);
        return;
      }

      let expr_number = ast_node_as::<AstExprConstantNumber>(node as *mut AstNode);
      if !expr_number.is_null() {
        let cid = (*self.bytecode).add_constant_number((*expr_number).value);
        if cid < 0 {
          CompileError::raise(
            &(*expr_number).base.base.location,
            format_args!("{ERR_EXCEEDED_CONSTANT_LIMIT}"),
          );
        }
        self.emit_load_k(target, cid);
        return;
      }

      let expr_integer = ast_node_as::<AstExprConstantInteger>(node as *mut AstNode);
      if !expr_integer.is_null() {
        let cid = (*self.bytecode).add_constant_integer((*expr_integer).value);
        if cid < 0 {
          CompileError::raise(
            &(*expr_integer).base.base.location,
            format_args!("{ERR_EXCEEDED_CONSTANT_LIMIT}"),
          );
        }
        self.emit_load_k(target, cid);
        return;
      }

      let expr_string = ast_node_as::<AstExprConstantString>(node as *mut AstNode);
      if !expr_string.is_null() {
        let cid = (*self.bytecode).add_constant_string(sref_ast_array_c_char((*expr_string).value));
        if cid < 0 {
          CompileError::raise(
            &(*expr_string).base.base.location,
            format_args!("{ERR_EXCEEDED_CONSTANT_LIMIT}"),
          );
        }
        self.emit_load_k(target, cid);
        return;
      }

      let expr_local = ast_node_as::<AstExprLocal>(node as *mut AstNode);
      if !expr_local.is_null() {
        let is_exported_class = self
          .exported_classes
          .iter()
          .any(|(name, _)| *name == (*(*expr_local).local).name);
        if FFlag::LuauExportValueSyntax.get()
          && (*(*expr_local).local).is_exported
          && !is_exported_class
        {
          let name = sref_ast_name((*(*expr_local).local).name);
          let cid = (*self.bytecode).add_constant_string(name);
          if cid < 0 {
            CompileError::raise(
              &(*expr_local).base.base.location,
              format_args!("{ERR_EXCEEDED_CONSTANT_LIMIT}"),
            );
          }
          let local_ptr = &mut self.export_table_local as *mut _;
          let table_reg = self.get_local_reg(local_ptr);
          if table_reg >= 0 {
            (*self.bytecode).emit_abc(
              LuauOpcode::LOP_GETTABLEKS,
              target,
              table_reg as u8,
              bytecode_builder_get_string_hash(name) as u8,
            );
            (*self.bytecode).emit_aux(cid as u32);
          } else {
            let upval = self.get_upval(local_ptr);
            (*self.bytecode).emit_abc(LuauOpcode::LOP_GETUPVAL, target, upval, 0);
            (*self.bytecode).emit_abc(
              LuauOpcode::LOP_GETTABLEKS,
              target,
              target,
              bytecode_builder_get_string_hash(name) as u8,
            );
            (*self.bytecode).emit_aux(cid as u32);
          }
        } else {
          let reg = self.get_expr_local_reg(node);
          if reg >= 0 {
            if self.options.optimization_level == 0 || target != reg as u8 {
              (*self.bytecode).emit_abc(LuauOpcode::LOP_MOVE, target, reg as u8, 0);
            }
          } else {
            LUAU_ASSERT!((*expr_local).upvalue);
            let uid = self.get_upval((*expr_local).local);
            (*self.bytecode).emit_abc(LuauOpcode::LOP_GETUPVAL, target, uid, 0);
          }
        }
        return;
      }

      let expr_global = ast_node_as::<AstExprGlobal>(node as *mut AstNode);
      if !expr_global.is_null() {
        self.compile_expr_global(expr_global, target);
        return;
      }

      let expr_varargs = ast_node_as::<AstExprVarargs>(node as *mut AstNode);
      if !expr_varargs.is_null() {
        self.compile_expr_varargs(expr_varargs, target, 1, false);
        return;
      }

      let expr_call = ast_node_as::<AstExprCall>(node as *mut AstNode);
      if !expr_call.is_null() {
        if target_temp && self.reg_top != 0 && u32::from(target) == self.reg_top - 1 {
          self.compile_expr_call(expr_call, target, 1, true, false);
        } else {
          self.compile_expr_call(expr_call, target, 1, false, false);
        }
        return;
      }

      let expr_index_name = ast_node_as::<AstExprIndexName>(node as *mut AstNode);
      if !expr_index_name.is_null() {
        self.compile_expr_index_name(expr_index_name, target, target_temp);
        return;
      }

      let expr_index_expr = ast_node_as::<AstExprIndexExpr>(node as *mut AstNode);
      if !expr_index_expr.is_null() {
        self.compile_expr_index_expr(expr_index_expr, target);
        return;
      }

      let expr_function = ast_node_as::<AstExprFunction>(node as *mut AstNode);
      if !expr_function.is_null() {
        self.compile_expr_function(expr_function, target);
        return;
      }

      let expr_table = ast_node_as::<AstExprTable>(node as *mut AstNode);
      if !expr_table.is_null() {
        self.compile_expr_table(expr_table, target, target_temp);
        return;
      }

      let expr_unary = ast_node_as::<AstExprUnary>(node as *mut AstNode);
      if !expr_unary.is_null() {
        self.compile_expr_unary(expr_unary, target);
        return;
      }

      let expr_binary = ast_node_as::<AstExprBinary>(node as *mut AstNode);
      if !expr_binary.is_null() {
        self.compile_expr_binary(expr_binary, target, target_temp);
        return;
      }

      let expr_assertion = ast_node_as::<AstExprTypeAssertion>(node as *mut AstNode);
      if !expr_assertion.is_null() {
        self.compile_expr((*expr_assertion).expr, target, target_temp);
        return;
      }

      let expr_if_else = ast_node_as::<AstExprIfElse>(node as *mut AstNode);
      if !expr_if_else.is_null() {
        self.compile_expr_if_else(expr_if_else, target, target_temp);
        return;
      }

      let interp_string = ast_node_as::<AstExprInterpString>(node as *mut AstNode);
      if !interp_string.is_null() {
        self.compile_expr_interp_string(interp_string, target, target_temp);
        return;
      }

      let expr_instantiate = ast_node_as::<AstExprInstantiate>(node as *mut AstNode);
      if !expr_instantiate.is_null() {
        self.compile_expr((*expr_instantiate).expr, target, target_temp);
        return;
      }
    }

    LUAU_ASSERT!(false);
  }
}
