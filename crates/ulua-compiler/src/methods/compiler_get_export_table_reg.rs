use ulua_ast::records::ast_node::AstNode;
use ulua_common::enums::luau_opcode::LuauOpcode;

use crate::records::compiler::Compiler;

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn get_export_table_reg(&mut self, node: *mut AstNode) -> u8 {
    let local_ptr = &mut self.export_table_local as *mut _;
    let reg = self.get_local_reg(local_ptr);
    if reg >= 0 {
      return reg as u8;
    }

    let upval = unsafe { self.get_upval(local_ptr) };
    let reg = unsafe { self.alloc_reg(node, 1) };
    unsafe {
      (*self.bytecode).emit_abc(LuauOpcode::LOP_GETUPVAL, reg, upval, 0);
    }
    reg
  }
}
