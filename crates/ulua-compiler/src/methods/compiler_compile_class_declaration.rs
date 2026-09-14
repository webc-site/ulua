use ulua_ast::records::{ast_node::AstNode, ast_stat_class::AstStatClass};
use ulua_bytecode::records::class_shape::ClassShape;
use ulua_common::{
  FFlag, enums::luau_opcode::LuauOpcode, macros::luau_assert::LUAU_ASSERT,
  records::variant::Variant2,
};

use crate::{functions::sref_compiler::sref_ast_name, records::compiler::Compiler};

const INIT_NAME: &str = "__init";
const NEW_NAME: &str = "new";
const DUMMY_AUX: u32 = 0xDEAD_BEEF;
const INVALID_SUPER_REG: u8 = 0xFF;

impl Compiler {
  /// 编译类声明语句
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn compile_class_declaration(&mut self, decl: *mut AstStatClass) {
    LUAU_ASSERT!(FFlag::DebugLuauUserDefinedClasses.get());

    let decl_ref = unsafe { &*decl };
    let class_name = unsafe { (*decl_ref.name).name };

    let class_local = self.class_locals.find(&class_name).copied();
    let dest = match class_local.map(|l| self.get_local_reg(l)) {
      Some(r) if r >= 0 => r as u8,
      _ => {
        let d = unsafe { self.alloc_reg(decl as *mut _, 1) };
        unsafe { self.push_local(decl_ref.name, d, u32::MAX) };
        d
      }
    };

    if FFlag::LuauExportValueSyntax.get() && decl_ref.exported {
      unsafe { self.ensure_export_table(decl as *mut AstNode) };
      if let Some(entry) = self
        .exported_classes
        .iter_mut()
        .find(|(n, _)| *n == class_name)
      {
        entry.1 = dest;
      } else {
        self.exported_classes.push((class_name, dest));
      }
    }

    let _rs = self.reg_scope_compiler();

    let instr_c_val: u8 = u8::from(decl_ref.open);
    if !decl_ref.super_.is_null() {
      let super_reg = self.get_expr_local_reg(decl_ref.super_);
      if super_reg >= 0 {
        unsafe {
          (*self.bytecode).emit_abc(LuauOpcode::LOP_NEWCLASS, dest, super_reg as u8, instr_c_val);
        }
      } else {
        let super_dest = unsafe { self.alloc_reg(decl as *mut _, 1) };
        unsafe { self.compile_expr(decl_ref.super_, super_dest, false) };
        unsafe {
          (*self.bytecode).emit_abc(LuauOpcode::LOP_NEWCLASS, dest, super_dest, instr_c_val);
        }
      }
    } else {
      unsafe {
        (*self.bytecode).emit_abc(
          LuauOpcode::LOP_NEWCLASS,
          dest,
          INVALID_SUPER_REG,
          instr_c_val,
        );
      }
    }

    let aux_offset = unsafe {
      let offset = (*self.bytecode).emit_label();
      (*self.bytecode).emit_aux(DUMMY_AUX);
      offset
    };

    let class_name_cid = unsafe { (*self.bytecode).add_constant_string(sref_ast_name(class_name)) };
    self.check_constant(class_name_cid, unsafe { &(*decl_ref.name).location });

    let mut shape = ClassShape {
      class_name: class_name_cid,
      ..Default::default()
    };

    let temp = unsafe { self.alloc_reg(decl as *mut _, 1) };
    let mut has_explicit_constructor = false;

    for member in decl_ref.members.as_slice() {
      match member {
        Variant2::V0(prop) => {
          let cid = unsafe { (*self.bytecode).add_constant_string(sref_ast_name(prop.name)) };
          self.check_constant(cid, &prop.name_location);
          shape.property_names.push(cid);
        }
        Variant2::V1(method) => {
          unsafe { self.compile_expr_function(method.function, temp) };
          let cid =
            unsafe { (*self.bytecode).add_constant_string(sref_ast_name(method.function_name)) };
          self.check_constant(cid, unsafe { &(*method.function).base.base.location });
          shape.method_names.push(cid);
          unsafe {
            (*self.bytecode).emit_abc(LuauOpcode::LOP_NEWCLASSMEMBER, dest, 0, temp);
            (*self.bytecode).emit_aux(cid as u32);
          }
          if method.function_name.as_str() == Some(INIT_NAME) {
            has_explicit_constructor = true;
          }
        }
      }
    }

    // 类默认具备 new 与 __init 方法
    let new_cid = unsafe { (*self.bytecode).add_constant_string(NEW_NAME) };
    self.check_constant(new_cid, &decl_ref.base.base.location);
    shape.method_names.push(new_cid);

    if !has_explicit_constructor {
      let init_cid = unsafe { (*self.bytecode).add_constant_string(INIT_NAME) };
      self.check_constant(init_cid, &decl_ref.base.base.location);
      shape.method_names.push(init_cid);
    }

    let class_const = unsafe { (*self.bytecode).add_class_shape(shape) };
    self.check_constant(class_const, &decl_ref.base.base.location);
    unsafe {
      (*self.bytecode).patch_aux(aux_offset, class_const);
    }
  }
}
