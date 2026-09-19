use ulua_ast::records::ast_expr::AstExpr;
use ulua_bytecode::records::string_ref::StringRef;

use crate::{
  enums::kind::Kind,
  functions::sref_compiler_alt_c::sref_ast_array_c_char,
  records::{compiler::Compiler, constant::Constant, l_value::LValue, reg_scope::RegScope},
};

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub(crate) unsafe fn compile_l_value_index(
    &mut self,
    reg: u8,
    index: *mut AstExpr,
    rs: &mut RegScope,
  ) -> LValue {
    unsafe {
      let cv = self.get_constant(index);
      match cv {
        // 整数下标 1..=256 走 IndexNumber 快路径
        Constant::Number(value_number)
          if (1.0..=256.0).contains(&value_number)
            && (value_number as i32) as f64 == value_number =>
        {
          LValue {
            kind: Kind::IndexNumber,
            reg,
            upval: 0,
            index: 0,
            number: (value_number as i32 - 1) as u8,
            name: StringRef::default(),
            location: (*index).base.location,
          }
        }
        Constant::Str(_) => LValue {
          kind: Kind::IndexName,
          reg,
          upval: 0,
          index: 0,
          number: 0,
          name: sref_ast_array_c_char(cv.get_string()),
          location: (*index).base.location,
        },
        _ => LValue {
          kind: Kind::IndexExpr,
          reg,
          upval: 0,
          index: self.compile_expr_auto(index, rs),
          number: 0,
          name: StringRef::default(),
          location: (*index).base.location,
        },
      }
    }
  }
}
