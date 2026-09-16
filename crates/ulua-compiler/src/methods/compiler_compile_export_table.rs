use ulua_bytecode::{
  methods::bytecode_builder_get_string_hash::bytecode_builder_get_string_hash,
  records::bytecode_builder::BytecodeBuilder,
};
use ulua_common::{FFlag, enums::luau_opcode::LuauOpcode, macros::luau_assert::LUAU_ASSERT};

use crate::{
  functions::sref_compiler::sref_ast_name,
  records::{
    compile_error::{CompileError, ERR_EXCEEDED_CONSTANT_LIMIT},
    compiler::{Compiler, K_MAX_AD_INDEX},
  },
};

const K_DEFAULT_ALLOC_PC: u32 = !0u32;

impl Compiler {
  pub fn compile_export_table(&mut self) {
    LUAU_ASSERT!(!self.exported_locals.is_empty() || !self.exported_classes.is_empty());
    LUAU_ASSERT!(!self.current_function.is_null());

    let export_local = &mut self.export_table_local as *mut _;

    if !self.locals.contains(&export_local) {
      let table_reg = unsafe { self.alloc_reg(self.current_function as *mut _, 1) };
      let hash_size = Compiler::encode_hash_size(
        (self.exported_locals.len() + self.exported_classes.len()) as u32,
      );

      unsafe {
        (*self.bytecode).emit_abc(LuauOpcode::LOP_NEWTABLE, table_reg, hash_size, 0);
        (*self.bytecode).emit_aux(0);
      }

      unsafe { self.push_local(export_local, table_reg, K_DEFAULT_ALLOC_PC) };
    }

    let loc_node = self.current_function;
    let table_reg = self.get_local_reg(export_local);
    LUAU_ASSERT!(table_reg >= 0);
    let table_reg = table_reg as u8;

    if FFlag::DebugLuauUserDefinedClasses.get() {
      let exported_classes = self.exported_classes.clone();
      for (class_name, class_reg) in exported_classes {
        let class_name_ref = sref_ast_name(class_name);
        let class_name_cid = unsafe { (*self.bytecode).add_constant_string(class_name_ref) };
        if class_name_cid < 0 {
          unsafe {
            CompileError::raise(
              &(*loc_node).base.base.location,
              format_args!("{ERR_EXCEEDED_CONSTANT_LIMIT}"),
            );
          }
        }

        unsafe {
          (*self.bytecode).emit_abc(
            LuauOpcode::LOP_SETTABLEKS,
            class_reg,
            table_reg,
            bytecode_builder_get_string_hash(class_name_ref) as u8,
          );
          (*self.bytecode).emit_aux(class_name_cid as u32);
        }
      }
    }

    let freeze_reg = unsafe { self.alloc_reg(loc_node as *mut _, 2) };
    let freeze_name = unsafe { (*self.names).get_or_add_c_str(c"freeze".as_ptr()) };
    let freeze_cid = unsafe { (*self.bytecode).add_constant_string(sref_ast_name(freeze_name)) };
    if freeze_cid < 0 {
      unsafe {
        CompileError::raise(
          &(*loc_node).base.base.location,
          format_args!("{ERR_EXCEEDED_CONSTANT_LIMIT}"),
        );
      }
    }

    let table_name = unsafe { (*self.names).get_or_add_c_str(c"table".as_ptr()) };
    let table_cid = unsafe { (*self.bytecode).add_constant_string(sref_ast_name(table_name)) };
    if table_cid < 0 {
      unsafe {
        CompileError::raise(
          &(*loc_node).base.base.location,
          format_args!("{ERR_EXCEEDED_CONSTANT_LIMIT}"),
        );
      }
    }

    let iid = BytecodeBuilder::get_import_id2(table_cid, freeze_cid);
    let cid = unsafe { (*self.bytecode).add_import(iid) };

    if (0..K_MAX_AD_INDEX).contains(&cid) {
      unsafe {
        (*self.bytecode).emit_ad(LuauOpcode::LOP_GETIMPORT, freeze_reg, cid as i16);
        (*self.bytecode).emit_aux(iid);
      }
    } else {
      unsafe {
        CompileError::raise(
          &(*loc_node).base.base.location,
          format_args!("{ERR_EXCEEDED_CONSTANT_LIMIT}"),
        );
      }
    }

    unsafe {
      (*self.bytecode).emit_abc(LuauOpcode::LOP_MOVE, freeze_reg + 1, table_reg, 0);
      (*self.bytecode).emit_abc(LuauOpcode::LOP_CALL, freeze_reg, 2, 2);
    }

    self.close_locals(0);
    unsafe {
      (*self.bytecode).emit_abc(LuauOpcode::LOP_RETURN, freeze_reg, 2, 0);
    }
  }
}
