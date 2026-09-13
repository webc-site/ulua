//! 编译器核心语义测试，对齐 C++ `ConstantFolding.cpp` / `BuiltinFolding.cpp` /
//! `CostModel.cpp` / `Compiler.cpp` 的 import id 编码。

use alloc::vec::Vec;

use ulua_ast::records::{
  allocator::Allocator, ast_expr_binary::AstExprBinaryOp, ast_expr_unary::AstExprUnaryOp,
  ast_name_table::AstNameTable,
};
use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;

use crate::{
  enums::type_constant_folding::Type,
  functions::{
    bit_32::bit32, cbool::cbool, cnum::cnum, compute_cost::compute_cost,
    cstring_builtin_folding::cstring_str, cvar::cvar, cvector::cvector,
    escape_and_append::escape_and_append, fold_binary::fold_binary, fold_unary::fold_unary,
    parallel_add_sat::parallel_add_sat, parallel_mul_sat::parallel_mul_sat,
  },
  records::{compiler::Compiler, constant::Constant},
};

/// 构造 (Allocator, AstNameTable)：表持有 allocator 裸指针，二者须同生命周期存活
macro_rules! string_table {
  () => {{
    let mut allocator = Allocator::new();
    let names = AstNameTable::new(&mut allocator);
    (allocator, names)
  }};
}

/// C++ `BytecodeBuilder::getImportId` 重载：各 id 占 10 位，前缀 1/2/3
#[test]
fn import_id_bit_encoding() {
  assert_eq!(
    BytecodeBuilder::get_import_id2(1, 2),
    (2u32 << 30) | (1 << 20) | (2 << 10)
  );
  assert_eq!(
    BytecodeBuilder::get_import_id3(1, 2, 3),
    (3u32 << 30) | (1 << 20) | (2 << 10) | 3
  );
  assert_eq!(BytecodeBuilder::get_import_id(7), (1u32 << 30) | (7 << 20));
}

/// C++ computeCost：低 7 位已饱和（0x7f）时直接返回，不应用折扣
#[test]
fn compute_cost_saturated_skips_discounts() {
  unsafe {
    assert_eq!(compute_cost(0x7f, [true, true].as_ptr(), 2), 0x7f);
    // 基础成本 5，第 0 个变量折扣 1 且为常量 → 5 - 1 = 4；
    // 第 1 个变量折扣为 0（model 位 16..23）→ 不变
    let model = 5 | (1 << 8);
    let vars = [true, false];
    assert_eq!(compute_cost(model, vars.as_ptr(), 2), 4);
    // 非常量变量不产生折扣
    let vars_not_const = [false, false];
    assert_eq!(compute_cost(model, vars_not_const.as_ptr(), 2), 5);
  }
}

/// C++ bit32：经 i64 转换，负数与大数优雅截断
#[test]
fn bit32_truncates_through_i64() {
  assert_eq!(bit32(7.0), 7);
  assert_eq!(bit32(-1.0), 0xffff_ffff);
  assert_eq!(bit32(4294967296.0 + 5.0), 5);
}

/// C++ parallel_add_sat：7 位通道逐字节饱和
#[test]
fn parallel_add_sat_saturates_7bit_lanes() {
  assert_eq!(parallel_add_sat(0x40, 0x40), 0x7f); // 64+64 → 饱和 127
  assert_eq!(parallel_add_sat(0x10, 0x20), 0x30); // 无饱和
  assert_eq!(parallel_add_sat(0, 0), 0);
}

/// C++ parallel_mul_sat：7 位通道乘 b，乘积 >= 128 时饱和
#[test]
fn parallel_mul_sat_saturates_products() {
  assert_eq!(parallel_mul_sat(0x10, 4), 0x40); // 16*4 = 64
  assert_eq!(parallel_mul_sat(0x10, 16), 0x7f); // 16*16 = 256 → 饱和
  assert_eq!(parallel_mul_sat(0x10, 1000), 0x7f); // b 上限 127 → 饱和
}

/// C++ escapeAndAppend：'%' 复制转义
#[test]
fn escape_and_append_doubles_percent() {
  let mut buf = Vec::new();
  escape_and_append(&mut buf, b"100%s");
  assert_eq!(buf, b"100%%s".to_vec());

  let mut plain = Vec::new();
  escape_and_append(&mut plain, b"abc");
  assert_eq!(plain, b"abc".to_vec());
}

/// C++ encodeHashSize：ceil(log2(hashSize)) + 1
#[test]
fn encode_hash_size_is_log2_plus_one() {
  assert_eq!(Compiler::encode_hash_size(0), 0);
  assert_eq!(Compiler::encode_hash_size(1), 1);
  assert_eq!(Compiler::encode_hash_size(2), 2);
  assert_eq!(Compiler::encode_hash_size(3), 3);
  assert_eq!(Compiler::encode_hash_size(4), 3);
}

/// C++ foldBinary：数值算术（Add/Sub/Mul/Div/FloorDiv/Mod/Pow）
#[test]
fn folds_number_arithmetic() {
  let (_alloc, mut names) = string_table!();

  let mut fold = |op, la: Constant, ra: Constant| {
    let mut result = cvar();
    fold_binary(&mut result, op, &la, &ra, &mut names);
    unsafe { result.data.value_number }
  };

  assert_eq!(fold(AstExprBinaryOp::Add, cnum(1.5), cnum(2.25)), 3.75);
  assert_eq!(fold(AstExprBinaryOp::Sub, cnum(1.5), cnum(2.25)), -0.75);
  assert_eq!(fold(AstExprBinaryOp::Mul, cnum(1.5), cnum(2.0)), 3.0);
  assert_eq!(fold(AstExprBinaryOp::Div, cnum(7.0), cnum(2.0)), 3.5);
  assert_eq!(fold(AstExprBinaryOp::FloorDiv, cnum(7.0), cnum(2.0)), 3.0);
  assert_eq!(fold(AstExprBinaryOp::Mod, cnum(7.0), cnum(3.0)), 1.0);
  assert_eq!(fold(AstExprBinaryOp::Pow, cnum(2.0), cnum(10.0)), 1024.0);
}

/// C++ foldBinary：数值比较
#[test]
fn folds_number_compare() {
  let (_alloc, mut names) = string_table!();

  let mut fold = |op, la: Constant, ra: Constant| {
    let mut result = cvar();
    fold_binary(&mut result, op, &la, &ra, &mut names);
    assert_eq!(result.r#type, Type::Boolean);
    unsafe { result.data.value_boolean }
  };

  assert!(fold(AstExprBinaryOp::CompareLt, cnum(1.0), cnum(2.0)));
  assert!(!fold(AstExprBinaryOp::CompareLt, cnum(2.0), cnum(1.0)));
  assert!(fold(AstExprBinaryOp::CompareLe, cnum(2.0), cnum(2.0)));
  assert!(fold(AstExprBinaryOp::CompareGt, cnum(3.0), cnum(2.0)));
  assert!(fold(AstExprBinaryOp::CompareGe, cnum(2.0), cnum(2.0)));
  assert!(fold(AstExprBinaryOp::CompareEq, cnum(2.0), cnum(2.0)));
  assert!(fold(AstExprBinaryOp::CompareNe, cnum(2.0), cnum(3.0)));
}

/// C++ foldBinary：And/Or 取操作数
#[test]
fn folds_and_or_operands() {
  let (_alloc, mut names) = string_table!();

  let mut result = cvar();
  fold_binary(
    &mut result,
    AstExprBinaryOp::And,
    &cnum(1.0),
    &cnum(2.0),
    &mut names,
  );
  assert_eq!(unsafe { result.data.value_number }, 2.0); // la 为真 → ra

  let mut result = cvar();
  fold_binary(
    &mut result,
    AstExprBinaryOp::And,
    &cbool(false),
    &cnum(2.0),
    &mut names,
  );
  assert_eq!(result.r#type, Type::Boolean); // la 为假 → la
  assert!(!unsafe { result.data.value_boolean });

  let mut result = cvar();
  fold_binary(
    &mut result,
    AstExprBinaryOp::Or,
    &cnum(0.0),
    &cnum(2.0),
    &mut names,
  );
  assert_eq!(unsafe { result.data.value_number }, 0.0); // la 为真 → la

  let mut result = cvar();
  fold_binary(
    &mut result,
    AstExprBinaryOp::Or,
    &cbool(false),
    &cnum(2.0),
    &mut names,
  );
  assert_eq!(unsafe { result.data.value_number }, 2.0); // la 为假 → ra
}

/// C++ foldBinary：向量四分量加法（含 w 分量）
#[test]
fn folds_vector_add_all_components() {
  let (_alloc, mut names) = string_table!();
  let la = cvector(1.0, 2.0, 3.0, 4.0);
  let ra = cvector(5.0, 6.0, 7.0, 8.0);
  let mut result = cvar();
  fold_binary(&mut result, AstExprBinaryOp::Add, &la, &ra, &mut names);

  assert_eq!(result.r#type, Type::Vector);
  unsafe {
    assert_eq!(result.data.value_vector[0], 6.0);
    assert_eq!(result.data.value_vector[1], 8.0);
    assert_eq!(result.data.value_vector[2], 10.0);
    assert_eq!(result.data.value_vector[3], 12.0);
  }
}

/// C++ foldBinary：标量×向量（num-vec 分支）
#[test]
fn folds_vector_scalar_mul() {
  let (_alloc, mut names) = string_table!();
  let v = cvector(1.0, 2.0, 4.0, 0.0);
  let mut result = cvar();
  fold_binary(
    &mut result,
    AstExprBinaryOp::Mul,
    &cnum(2.0),
    &v,
    &mut names,
  );

  assert_eq!(result.r#type, Type::Vector);
  unsafe {
    assert_eq!(result.data.value_vector[0], 2.0);
    assert_eq!(result.data.value_vector[1], 4.0);
    assert_eq!(result.data.value_vector[2], 8.0);
    assert_eq!(result.data.value_vector[3], 0.0);
  }
}

/// C++ foldBinary：向量×向量乘法，w 分量结果与 had_w 语义
#[test]
fn folds_vector_mul_keeps_w_semantics() {
  let (_alloc, mut names) = string_table!();
  // 任一输入 w 非零 → 折叠并保留乘积 w
  let la = cvector(1.0, 2.0, 3.0, 1.0);
  let ra = cvector(2.0, 3.0, 4.0, 2.0);
  let mut result = cvar();
  fold_binary(&mut result, AstExprBinaryOp::Mul, &la, &ra, &mut names);
  assert_eq!(result.r#type, Type::Vector);
  unsafe {
    assert_eq!(result.data.value_vector[0], 2.0);
    assert_eq!(result.data.value_vector[1], 6.0);
    assert_eq!(result.data.value_vector[2], 12.0);
    assert_eq!(result.data.value_vector[3], 2.0);
  }
}

/// C++ foldBinary：字符串拼接，带长度上限
#[test]
fn folds_string_concat() {
  let (_alloc, mut names) = string_table!();

  let mut result = cvar();
  fold_binary(
    &mut result,
    AstExprBinaryOp::Concat,
    &cstring_str("ab"),
    &cstring_str("cd"),
    &mut names,
  );
  assert_eq!(result.r#type, Type::String);
  assert_eq!(result.string_length, 4);
  assert_eq!(result.get_string_bytes(), b"abcd");

  // 超过 4096 上限不折叠
  let big_bytes = "x".repeat(4097); // 缓冲须存活到断言结束
  let big = cstring_str(&big_bytes);
  let mut result = cvar();
  fold_binary(
    &mut result,
    AstExprBinaryOp::Concat,
    &big,
    &cstring_str("y"),
    &mut names,
  );
  assert_eq!(result.r#type, Type::Unknown);
}

/// C++ foldUnary：Not/Minus/Len
#[test]
fn folds_unary_ops() {
  let mut result = cvar();
  fold_unary(&mut result, AstExprUnaryOp::Not, &cbool(true));
  assert_eq!(result.r#type, Type::Boolean);
  assert!(!unsafe { result.data.value_boolean });

  let mut result = cvar();
  fold_unary(&mut result, AstExprUnaryOp::Not, &cnum(0.0));
  assert_eq!(result.r#type, Type::Boolean);
  assert!(!unsafe { result.data.value_boolean }); // 非nil非false为真

  let mut result = cvar();
  fold_unary(&mut result, AstExprUnaryOp::Minus, &cnum(3.5));
  assert_eq!(result.r#type, Type::Number);
  assert_eq!(unsafe { result.data.value_number }, -3.5);

  let mut result = cvar();
  fold_unary(
    &mut result,
    AstExprUnaryOp::Minus,
    &cvector(1.0, -2.0, 3.0, 0.0),
  );
  assert_eq!(result.r#type, Type::Vector);
  unsafe {
    assert_eq!(result.data.value_vector[0], -1.0);
    assert_eq!(result.data.value_vector[1], 2.0);
    assert_eq!(result.data.value_vector[2], -3.0);
  }

  let mut result = cvar();
  fold_unary(&mut result, AstExprUnaryOp::Len, &cstring_str("abc"));
  assert_eq!(result.r#type, Type::Number);
  assert_eq!(unsafe { result.data.value_number }, 3.0);
}
