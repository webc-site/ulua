use ulua_ast::records::ast_expr_global::AstExprGlobal;
use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_common::{FFlag::DebugLuauUserDefinedClasses, enums::luau_opcode::LuauOpcode};

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
  pub unsafe fn compile_expr_global(&mut self, expr: *mut AstExprGlobal, target: u8) {
    unsafe {
      if DebugLuauUserDefinedClasses.get()
        && let Some(&local) = self.class_locals.find(&(*expr).name)
      {
        let reg = self.get_local_reg(local);
        if reg >= 0 {
          if target != reg as u8 {
            (*self.bytecode).emit_abc(LuauOpcode::LOP_MOVE, target, reg as u8, 0);
          }
        } else {
          let uid = self.get_upval(local);
          (*self.bytecode).emit_abc(LuauOpcode::LOP_GETUPVAL, target, uid, 0);
        }
        return;
      }

      if self.can_import(expr) {
        let name = (*expr).name;
        let id0 = (*self.bytecode).add_constant_string(sref_ast_name(name));

        if id0 < 0 {
          let location = (*expr).base.base.location;
          CompileError::raise(
            &location,
            core::format_args!("{ERR_EXCEEDED_CONSTANT_LIMIT}"),
          );
        }

        if id0 < K_MAX_IMPORT_ID {
          let iid = BytecodeBuilder::get_import_id(id0);
          let cid = (*self.bytecode).add_import(iid);

          if (0..K_MAX_AD_INDEX).contains(&cid) {
            (*self.bytecode).emit_ad(LuauOpcode::LOP_GETIMPORT, target, cid as i16);
            (*self.bytecode).emit_aux(iid);
            return;
          }
        }
      }

      let name = (*expr).name;
      let gname = sref_ast_name(name);
      let cid = (*self.bytecode).add_constant_string(gname);

      if cid < 0 {
        let location = (*expr).base.base.location;
        CompileError::raise(
          &location,
          core::format_args!("{ERR_EXCEEDED_CONSTANT_LIMIT}"),
        );
      }

      let hash = BytecodeBuilder::get_string_hash(gname) as u8;
      (*self.bytecode).emit_abc(LuauOpcode::LOP_GETGLOBAL, target, 0, hash);
      (*self.bytecode).emit_aux(cid as u32);
    }
  }
}
