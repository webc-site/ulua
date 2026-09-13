use core::ptr::null_mut;

use ulua_ast::{
  records::{
    ast_expr_global::AstExprGlobal, ast_expr_index_name::AstExprIndexName, ast_node::AstNode,
  },
  rtti::ast_node_as,
};
use ulua_bytecode::{
  methods::bytecode_builder_get_string_hash::bytecode_builder_get_string_hash,
  records::bytecode_builder::BytecodeBuilder,
};
use ulua_common::{
  FFlag::DebugLuauUserDefinedClasses,
  enums::{luau_bytecode_type::LBC_TYPE_TABLE, luau_opcode::LuauOpcode},
};

use crate::{
  functions::sref_compiler::sref_ast_name,
  records::{
    compile_error::{CompileError, ERR_EXCEEDED_CONSTANT_LIMIT},
    compiler::{Compiler, K_MAX_AD_INDEX, K_MAX_IMPORT_ID},
  },
};

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn compile_expr_index_name(
    &mut self,
    expr: *mut AstExprIndexName,
    target: u8,
    target_temp: bool,
  ) {
    unsafe {
      let expr_ref = &*expr;
      self.set_debug_line_ast_node(expr as *mut _);

      let import_root: *mut AstExprGlobal;
      let import1: *mut AstExprIndexName;
      let import2: *mut AstExprIndexName;

      let index = ast_node_as::<AstExprIndexName>(expr_ref.expr as *mut AstNode);
      if !index.is_null() {
        import_root = ast_node_as::<AstExprGlobal>((*index).expr as *mut AstNode);
        import1 = index;
        import2 = expr;
      } else {
        import_root = ast_node_as::<AstExprGlobal>(expr_ref.expr as *mut AstNode);
        import1 = expr;
        import2 = null_mut();
      }

      if !import_root.is_null()
        && self.can_import_chain(import_root)
        && !(DebugLuauUserDefinedClasses.get()
          && self.class_locals.find(&(*import_root).name).is_some())
      {
        let id0 = (*self.bytecode).add_constant_string(sref_ast_name((*import_root).name));
        let id1 = (*self.bytecode).add_constant_string(sref_ast_name((*import1).index));
        let id2 = if !import2.is_null() {
          (*self.bytecode).add_constant_string(sref_ast_name((*import2).index))
        } else {
          -1
        };

        if id0 < 0 || id1 < 0 || (!import2.is_null() && id2 < 0) {
          CompileError::raise(
            &expr_ref.base.base.location,
            format_args!("{ERR_EXCEEDED_CONSTANT_LIMIT}"),
          );
        }

        // Note: GETIMPORT encoding is limited to 10 bits per object id component
        if id0 < K_MAX_IMPORT_ID && id1 < K_MAX_IMPORT_ID && id2 < K_MAX_IMPORT_ID {
          let iid = if !import2.is_null() {
            BytecodeBuilder::get_import_id3(id0, id1, id2)
          } else {
            BytecodeBuilder::get_import_id2(id0, id1)
          };
          let cid = (*self.bytecode).add_import(iid);
          if (0..K_MAX_AD_INDEX).contains(&cid) {
            (*self.bytecode).emit_ad(LuauOpcode::LOP_GETIMPORT, target, cid as i16);
            (*self.bytecode).emit_aux(iid);
            return;
          }
        }
      }

      let mut rs = self.reg_scope_compiler();
      let local_reg = self.get_expr_local_reg(expr_ref.expr);
      let reg = if local_reg >= 0 {
        local_reg as u8
      } else if target_temp {
        self.compile_expr_temp(expr_ref.expr, target);
        target
      } else {
        self.compile_expr_auto(expr_ref.expr, &mut rs)
      };

      self.set_debug_line_location(&expr_ref.index_location);
      let iname = sref_ast_name(expr_ref.index);
      let cid = (*self.bytecode).add_constant_string(iname);
      if cid < 0 {
        CompileError::raise(
          &expr_ref.base.base.location,
          format_args!("{ERR_EXCEEDED_CONSTANT_LIMIT}"),
        );
      }

      (*self.bytecode).emit_abc(
        LuauOpcode::LOP_GETTABLEKS,
        target,
        reg,
        bytecode_builder_get_string_hash(iname) as u8,
      );
      (*self.bytecode).emit_aux(cid as u32);
      self.hint_temporary_expr_reg_type(expr_ref.expr, reg as i32, LBC_TYPE_TABLE, 2);
    }
  }
}
