use ulua_ast::records::ast_expr::AstExpr;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::sref_compiler_alt_c::sref_ast_array_c_char,
  records::{
    compile_error::{CompileError, ERR_EXCEEDED_CONSTANT_LIMIT},
    compiler::Compiler,
    constant::Constant,
  },
};

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn get_constant_index(&mut self, node: *mut AstExpr) -> i32 {
    unsafe {
      // 单次查找 + let-else，替代双重 unwrap
      let constant = match self.constants.find(&node) {
        Some(c) if !c.is_unknown() => c,
        _ => return -1,
      };
      let cid = match *constant {
        Constant::Nil => (*self.bytecode).add_constant_nil(),
        Constant::Boolean(b) => (*self.bytecode).add_constant_boolean(b),
        Constant::Number(n) => (*self.bytecode).add_constant_number(n),
        Constant::Integer(l) => (*self.bytecode).add_constant_integer(l),
        Constant::Vector([x, y, z, w]) => (*self.bytecode).add_constant_vector(x, y, z, w),
        Constant::Str(_) => {
          let string_data = constant.get_string();
          (*self.bytecode).add_constant_string(sref_ast_array_c_char(string_data))
        }
        // 仅 Unknown / Table 会到达
        _ => {
          LUAU_ASSERT!(false);
          return -1;
        }
      };

      if cid < 0 {
        CompileError::raise(
          &(*node).base.location,
          core::format_args!("{ERR_EXCEEDED_CONSTANT_LIMIT}"),
        );
      }

      cid
    }
  }
}
