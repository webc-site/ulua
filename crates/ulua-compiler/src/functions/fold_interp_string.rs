use alloc::vec::Vec;

use ulua_ast::records::{
  ast_expr::AstExpr, ast_expr_interp_string::AstExprInterpString, ast_name_table::AstNameTable,
};
use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::dense_hash_map::DenseHashMap};

use crate::records::constant::Constant;

/// 常量字符串折叠上限（C++ `kConstantFoldStringLimit`）
const K_CONSTANT_FOLD_STRING_LIMIT: usize = 4096;

/// C++ `foldInterpString`：拼接插值字符串常量；超限或不含常量时返回 `Constant::Unknown`
pub fn fold_interp_string(
  expr: &AstExprInterpString,
  constants: &DenseHashMap<*mut AstExpr, Constant>,
  string_table: &mut AstNameTable,
) -> Constant {
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
      LUAU_ASSERT!(matches!(c, Constant::Str(_)));
      result_length += c.string_len() as usize;
    }
  }

  if result_length > K_CONSTANT_FOLD_STRING_LIMIT {
    return Constant::Unknown;
  }

  if result_length == 0 {
    // C++ `result.valueString = ""` — 非空指针指向静态空 C 串。
    // 若为 null，空插值（如 `{empty}`）作为字符串常量下发时会触发 sref() 的
    // `LUAU_ASSERT(data.begin())`
    return Constant::string(c"".as_ptr(), 0);
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
  Constant::string(name.value, result_length as u32)
}
