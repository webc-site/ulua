use core::{ffi::CStr, mem::swap};

use ulua_ast::records::{
  ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
  ast_expr_call::AstExprCall,
  ast_expr_constant_string::AstExprConstantString,
  ast_expr_global::AstExprGlobal,
};

use crate::{enums::type_kind::TypeKind, records::lint_unknown_type::LintUnknownType};

impl LintUnknownType {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit(&mut self, node: *mut AstExprBinary) -> bool {
    let node_ref = unsafe { &*node };
    if node_ref.op != AstExprBinaryOp::CompareNe && node_ref.op != AstExprBinaryOp::CompareEq {
      return true;
    }

    let mut lhs = node_ref.left;
    let mut rhs = node_ref.right;

    // Ensure rhs is the constant string argument
    if !unsafe { (*rhs).base.is::<AstExprConstantString>() } {
      swap(&mut lhs, &mut rhs);
    }

    let call = unsafe { (*lhs).base.as_item::<AstExprCall>() };
    let arg = unsafe { (*rhs).base.as_item::<AstExprConstantString>() };

    if call.is_null() || arg.is_null() {
      return true;
    }

    let g = unsafe { (*(*call).func).base.as_item::<AstExprGlobal>() };
    if g.is_null() {
      return true;
    }

    let g_name = unsafe { CStr::from_ptr((*g).name.value) };
    if g_name.to_bytes() == b"type" {
      unsafe {
        self.validate_type(
          arg as *mut AstExprConstantString,
          &[TypeKind::Primitive, TypeKind::Vector],
          "primitive type",
        )
      };
    } else if g_name.to_bytes() == b"typeof" {
      unsafe {
        self.validate_type(
          arg as *mut AstExprConstantString,
          &[TypeKind::Primitive, TypeKind::Userdata],
          "primitive or userdata type",
        )
      };
    }

    true
  }
}
