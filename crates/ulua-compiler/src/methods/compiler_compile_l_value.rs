use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_global::AstExprGlobal, ast_expr_index_expr::AstExprIndexExpr,
    ast_expr_index_name::AstExprIndexName, ast_expr_local::AstExprLocal, ast_node::AstNode,
  },
  rtti::ast_node_as,
};
use ulua_bytecode::records::string_ref::StringRef;
use ulua_common::{FFlag, FFlag::DebugLuauUserDefinedClasses, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::kind::Kind,
  functions::sref_compiler::sref_ast_name,
  records::{
    compile_error::CompileError, compiler::Compiler, l_value::LValue, reg_scope::RegScope,
  },
};

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub(crate) unsafe fn compile_l_value(&mut self, node: *mut AstExpr, rs: &mut RegScope) -> LValue {
    unsafe {
      self.set_debug_line_ast_node(node as *mut AstNode);
      let expr = ast_node_as::<AstExprLocal>(node as *mut AstNode);
      if !expr.is_null() {
        if FFlag::LuauExportValueSyntax.get() && (*(*expr).local).is_exported {
          return LValue {
            kind: Kind::IndexName,
            reg: self.get_export_table_reg(node as *mut AstNode),
            upval: 0,
            index: 0,
            number: 0,
            name: sref_ast_name((*(*expr).local).name),
            location: (*node).base.location,
          };
        }
        let reg = self.get_expr_local_reg(node);
        if reg >= 0 {
          LValue {
            kind: Kind::Local,
            reg: reg as u8,
            upval: 0,
            index: 0,
            number: 0,
            name: StringRef::default(),
            location: (*node).base.location,
          }
        } else {
          LUAU_ASSERT!((*expr).upvalue);
          LValue {
            kind: Kind::Upvalue,
            reg: 0,
            upval: self.get_upval((*expr).local),
            index: 0,
            number: 0,
            name: StringRef::default(),
            location: (*node).base.location,
          }
        }
      } else {
        let expr = ast_node_as::<AstExprGlobal>(node as *mut AstNode);
        if !expr.is_null() {
          if DebugLuauUserDefinedClasses.get()
            && let Some(&class_local) = self.class_locals.find(&(*expr).name)
          {
            CompileError::raise(
              &(*expr).base.base.location,
              core::format_args!(
                "'{}' refers to a class and cannot be used as a variable name (defined on line {})",
                sref_ast_name((*expr).name),
                (*class_local).location.begin.line + 1
              ),
            );
          }

          LValue {
            kind: Kind::Global,
            reg: 0,
            upval: 0,
            index: 0,
            number: 0,
            name: sref_ast_name((*expr).name),
            location: (*node).base.location,
          }
        } else {
          let expr = ast_node_as::<AstExprIndexName>(node as *mut AstNode);
          if !expr.is_null() {
            LValue {
              kind: Kind::IndexName,
              reg: self.compile_expr_auto((*expr).expr, rs),
              upval: 0,
              index: 0,
              number: 0,
              name: sref_ast_name((*expr).index),
              location: (*node).base.location,
            }
          } else {
            let expr = ast_node_as::<AstExprIndexExpr>(node as *mut AstNode);
            if !expr.is_null() {
              let reg = self.compile_expr_auto((*expr).expr, rs);
              self.compile_l_value_index(reg, (*expr).index, rs)
            } else {
              LUAU_ASSERT!(false);
              LValue {
                kind: Kind::Local,
                reg: 0,
                upval: 0,
                index: 0,
                number: 0,
                name: StringRef::default(),
                location: (*node).base.location,
              }
            }
          }
        }
      }
    }
  }
}
