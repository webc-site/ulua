use alloc::vec::Vec;

use ulua_ast::records::{
  ast_expr::AstExpr, ast_expr_interp_string::AstExprInterpString, ast_name_table::AstNameTable,
};
use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::dense_hash_map::DenseHashMap};

use crate::{enums::type_constant_folding::Type, records::constant::Constant};

/// 常量字符串折叠上限（C++ `kConstantFoldStringLimit`）
const K_CONSTANT_FOLD_STRING_LIMIT: usize = 4096;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn fold_interp_string(
  result: &mut Constant,
  expr: *mut AstExprInterpString,
  constants: &mut DenseHashMap<*mut AstExpr, Constant>,
  string_table: &mut AstNameTable,
) {
  let expr = unsafe { &*expr };
  LUAU_ASSERT!(expr.strings.len() == expr.expressions.len() + 1);

  let strings = expr.strings.as_slice();
  let expressions = expr.expressions.as_slice();

  // 第一遍：计算折叠后总长度
  let mut result_length: usize = 0;
  for (index, string) in strings.iter().enumerate() {
    result_length += string.len();
    if let Some(&expr_ptr) = expressions.get(index) {
      // C++：LUAU_ASSERT(c) 后直接解引用，调用方保证表达式已折叠为字符串常量
      let c = unsafe { constants.find(&expr_ptr).unwrap_unchecked() };
      LUAU_ASSERT!(c.r#type == Type::String);
      result_length += c.string_length as usize;
    }
  }

  if result_length > K_CONSTANT_FOLD_STRING_LIMIT {
    return;
  }

  result.r#type = Type::String;
  result.string_length = result_length as u32;

  if result_length == 0 {
    // C++ `result.valueString = ""` — a non-null pointer to a static empty C-string.
    // A null here later trips sref()'s `LUAU_ASSERT(data.begin())` when the folded
    // empty interpolation (e.g. `{empty}`) is emitted as a string constant.
    result.data.value_string = c"".as_ptr();
    return;
  }

  // 第二遍：拼接源串片段与已折叠的表达式字符串
  let mut tmp = Vec::with_capacity(result_length);
  for (index, string) in strings.iter().enumerate() {
    tmp.extend_from_slice(string.as_bytes());
    if let Some(&expr_ptr) = expressions.get(index) {
      let c = unsafe { constants.find(&expr_ptr).unwrap_unchecked() };
      tmp.extend_from_slice(c.get_string_bytes());
    }
  }

  let name = string_table.get_or_add_slice(&tmp);
  result.data.value_string = name.value;
}
