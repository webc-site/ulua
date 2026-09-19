use ulua_ast::records::ast_expr::AstExpr;

use crate::{
  enums::type_constant_folding::Type,
  records::{
    compile_error::{CompileError, ERR_EXCEEDED_CONSTANT_LIMIT},
    compiler::Compiler,
  },
};

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn get_constant_number(&mut self, node: *mut AstExpr) -> i32 {
    unsafe {
      let c = self.constants.find(&node);

      if let Some(constant) = c
        && constant.r#type == Type::Number
      {
        let cid = (*self.bytecode).add_constant_number(constant.data.value_number);
        if cid < 0 {
          CompileError::raise(
            &(*node).base.location,
            core::format_args!("{ERR_EXCEEDED_CONSTANT_LIMIT}"),
          );
        }
        return cid;
      }

      -1
    }
  }
}
