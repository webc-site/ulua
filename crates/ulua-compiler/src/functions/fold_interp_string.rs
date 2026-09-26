use alloc::vec::Vec;

use ulua_ast::records::{
  ast_expr::AstExpr, ast_expr_interp_string::AstExprInterpString, ast_name_table::AstNameTable,
};
use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::dense_hash_map::DenseHashMap};

use crate::records::{constant::Constant, node::Node};

/// cpp `result.valueString = ""`：静态空 C 串（NUL 结尾字节串）。
const EMPTY_CSTR: &[u8] = b"\0";

/// 常量字符串折叠上限（C++ `kConstantFoldStringLimit`）
const K_CONSTANT_FOLD_STRING_LIMIT: usize = 4096;

/// C++ `foldInterpString`：拼接插值字符串常量；超限或不含常量时返回 `Constant::Unknown`
pub(crate) fn fold_interp_string(
  expr: &AstExprInterpString,
  constants: &DenseHashMap<Node<AstExpr>, Constant>,
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
      // Safety: 唯一调用点 ConstantVisitor::analyze 先对 expressions 逐个 analyze()
      // 并在结果为 Str 时 record_expr_constant 写入本 map（键即同一 expr_ptr 指针），
      // 仅当 only_constant_sub_expr 为真才进入本函数，故 find 必命中；expr_ptr 为
      // arena 存活表达式指针，仅作查表键。
      let c = unsafe { constants.find(&Node::from(expr_ptr)).unwrap_unchecked() };
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
    return Constant::string(EMPTY_CSTR.as_ptr(), 0);
  }

  // 第二遍：拼接源串片段与已折叠的表达式字符串
  let mut tmp = Vec::with_capacity(result_length);
  for (index, string) in strings.iter().enumerate() {
    tmp.extend_from_slice(string.as_bytes());
    if let Some(&expr_ptr) = expressions.get(index) {
      // Safety: 第二遍与第一遍共用同一 map 与同一表达式集，前提不变：
      // 每个 expr_ptr 均有已录入的 Str 常量，find 必命中。
      let c = unsafe { constants.find(&Node::from(expr_ptr)).unwrap_unchecked() };
      tmp.extend_from_slice(c.get_string_bytes());
    }
  }

  let name = string_table.get_or_add_slice(&tmp);
  Constant::string(name.value, result_length as u32)
}
