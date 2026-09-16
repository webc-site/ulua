//! Node: `cxx:Method:Luau.Compiler:Compiler/src/ValueTracking.cpp:146:visit`
//!
//! `ValueVisitor::visit(AstStatClass*)` — record class declaration in classLocals
//! and mark its variable written.

use ulua_ast::records::ast_stat_class::AstStatClass;
use ulua_common::FFlag;

use crate::records::value_visitor::ValueVisitor;

impl ValueVisitor {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn visit_ast_stat_class(&mut self, decl: *mut AstStatClass) -> bool {
    if !FFlag::DebugLuauUserDefinedClasses.get() {
      return false;
    }

    unsafe {
      if !decl.is_null() && !(*decl).name.is_null() {
        let name = (*(*decl).name).name;
        self.class_locals.insert(name, (*decl).name);
        self.variables.get_or_insert((*decl).name).written = true;
      }
    }

    true
  }
}
