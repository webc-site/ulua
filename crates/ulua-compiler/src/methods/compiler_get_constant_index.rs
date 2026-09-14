use ulua_ast::records::ast_expr::AstExpr;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::type_constant_folding::Type,
  functions::sref_compiler_alt_c::sref_ast_array_c_char,
  records::{
    compile_error::{CompileError, ERR_EXCEEDED_CONSTANT_LIMIT},
    compiler::Compiler,
  },
};

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn get_constant_index(&mut self, node: *mut AstExpr) -> i32 {
    unsafe {
      // 单次查找 + let-else，替代双重 unwrap
      let constant = match self.constants.find(&node) {
        Some(c) if c.r#type != Type::Unknown => c,
        _ => return -1,
      };
      let cid = match constant.r#type {
        Type::Nil => (*self.bytecode).add_constant_nil(),
        Type::Boolean => (*self.bytecode).add_constant_boolean(constant.data.value_boolean),
        Type::Number => (*self.bytecode).add_constant_number(constant.data.value_number),
        Type::Integer => (*self.bytecode).add_constant_integer(constant.data.value_integer64),
        Type::Vector => (*self.bytecode).add_constant_vector(
          constant.data.value_vector[0],
          constant.data.value_vector[1],
          constant.data.value_vector[2],
          constant.data.value_vector[3],
        ),
        Type::String => {
          let string_data = (*constant).get_string();
          (*self.bytecode).add_constant_string(sref_ast_array_c_char(string_data))
        }
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
