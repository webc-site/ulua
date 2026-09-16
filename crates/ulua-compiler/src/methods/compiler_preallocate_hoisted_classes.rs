//! Node: `cxx:Method:Luau.Compiler:Compiler/src/Compiler.cpp:4952:preallocateHoistedClasses`
//!
//! Preallocates local registers for class declarations hoisted at module top level.

use ulua_ast::{
  records::{ast_node::AstNode, ast_stat_block::AstStatBlock, ast_stat_class::AstStatClass},
  rtti::ast_node_as,
};
use ulua_common::{FFlag, enums::luau_opcode::LuauOpcode, macros::luau_assert::LUAU_ASSERT};

use crate::records::compiler::Compiler;

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn preallocate_hoisted_classes(&mut self, body: *mut AstStatBlock) {
    LUAU_ASSERT!(FFlag::DebugLuauUserDefinedClasses.get());

    unsafe {
      if body.is_null() {
        return;
      }
      for &stat in (*body).body.as_slice() {
        let decl = ast_node_as::<AstStatClass>(stat as *mut AstNode);
        if !decl.is_null() {
          let reg = self.alloc_reg(decl as *mut _, 1);
          self.push_local((*decl).name, reg, !0u32);
          (*self.bytecode).emit_abc(LuauOpcode::LOP_LOADNIL, reg, 0, 0);
        }
      }
    }
  }
}
