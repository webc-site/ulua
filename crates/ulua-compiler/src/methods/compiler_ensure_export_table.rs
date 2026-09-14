use ulua_ast::records::ast_node::AstNode;
use ulua_common::{enums::luau_opcode::LuauOpcode, macros::luau_assert::LUAU_ASSERT};

use crate::records::compiler::Compiler;

const K_DEFAULT_ALLOC_PC: u32 = !0u32;

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn ensure_export_table(&mut self, node: *mut AstNode) {
    let export_local = &mut self.export_table_local as *mut _;
    if self.locals.contains(&export_local) {
      return;
    }

    LUAU_ASSERT!(self.at_top_level());

    let table_reg = unsafe { self.alloc_reg(node, 1) };
    unsafe {
      (*self.bytecode).emit_abc(
        LuauOpcode::LOP_NEWTABLE,
        table_reg,
        Compiler::encode_hash_size(0),
        0,
      );
      (*self.bytecode).emit_aux(0);
    }

    unsafe { self.push_local(export_local, table_reg, K_DEFAULT_ALLOC_PC) };
  }
}
