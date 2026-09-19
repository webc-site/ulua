//! 常量池去重语义测试（审计 r1-bytecode 条目 1、2）。
//!
//! 对齐 cpp `BytecodeBuilder::addConstant*`（`Bytecode/src/BytecodeBuilder.cpp`）：
//! `ConstantKey` 是去重键，必须逐字段参与判等，且不能与 `DenseHashMap` 的空键哨兵相撞。

use ulua_bytecode::records::{
  bytecode_builder::BytecodeBuilder, string_ref::StringRef, table_shape::TableShape,
};

/// 条目 1 的回归点：`impl Default for BytecodeBuilder` 若不复用 `new(None)`，
/// `constant_map` 的空键哨兵就等于 `{Nil,0,0,0,0}`，于是 nil 常量命中"已存在"
/// 却永远拿不到缓存 id（每次都新增），同时 `insert_unsafe` 虚增元素计数。
#[test]
fn add_constant_nil_dedupes_across_calls() {
  let mut bcb = BytecodeBuilder::new(None);

  let first = bcb.add_constant_nil();
  let second = bcb.add_constant_nil();
  let third = bcb.add_constant_nil();

  assert_eq!(first, 0, "首条常量应落在 id 0");
  assert_eq!(first, second, "nil 必须去重");
  assert_eq!(second, third, "nil 必须去重");
}

/// `Default` 与 `new(None)` 必须给出同一份去重状态。
#[test]
fn default_builder_matches_new_none() {
  let mut by_default = BytecodeBuilder::default();
  let mut by_new = BytecodeBuilder::new(None);

  let a = by_default.add_constant_nil();
  let b = by_new.add_constant_nil();
  assert_eq!(a, b);
  // 第二次仍要命中同一条目：哨兵碰撞会让 Default 版本每次新增
  assert_eq!(a, by_default.add_constant_nil());
  assert_eq!(b, by_new.add_constant_nil());
}

/// `ConstantKey.r#type` 必须参与判等：`{Nil,0,0,0,0}` 与 `{Boolean,false}` 的
/// 数值字段全同，只有类型位能区分它们。
#[test]
fn nil_and_boolean_constants_do_not_collapse() {
  let mut bcb = BytecodeBuilder::new(None);

  let nil = bcb.add_constant_nil();
  let fals = bcb.add_constant_boolean(false);
  let tru = bcb.add_constant_boolean(true);

  assert_ne!(nil, fals, "Nil 与 Boolean(false) 不得共用一条常量");
  assert_ne!(fals, tru, "Boolean 的 value 位必须参与判等");
  assert_eq!(nil, bcb.add_constant_nil());
  assert_eq!(fals, bcb.add_constant_boolean(false));
  assert_eq!(tru, bcb.add_constant_boolean(true));
}

/// cpp `addConstantVectorf`：x/y 打进 `value`、z/w 打进 `extra1`，四个分量都必须在键上。
#[test]
fn vectorf_key_covers_all_four_components() {
  let mut bcb = BytecodeBuilder::new(None);

  let base = bcb.add_constant_vector(1.0, 2.0, 3.0, 4.0);
  assert_eq!(base, bcb.add_constant_vector(1.0, 2.0, 3.0, 4.0));

  assert_ne!(base, bcb.add_constant_vector(1.5, 2.0, 3.0, 4.0), "x");
  assert_ne!(base, bcb.add_constant_vector(1.0, 2.5, 3.0, 4.0), "y");
  assert_ne!(base, bcb.add_constant_vector(1.0, 2.0, 3.5, 4.0), "z");
  assert_ne!(base, bcb.add_constant_vector(1.0, 2.0, 3.0, 4.5), "w");
}

/// cpp `addConstantVectord`：x 在 `value`，y/z/w 在 `extra1/2/3`。
/// `extra2`/`extra3` 是 Rust 端为 Vectord 补的键位，漏掉任一都会误去重。
#[test]
fn vectord_key_covers_all_four_components() {
  let mut bcb = BytecodeBuilder::new(None);

  let base = bcb.add_constant_vector_d(1.0, 2.0, 3.0, 4.0);
  assert_eq!(base, bcb.add_constant_vector_d(1.0, 2.0, 3.0, 4.0));

  assert_ne!(base, bcb.add_constant_vector_d(1.5, 2.0, 3.0, 4.0), "x");
  assert_ne!(base, bcb.add_constant_vector_d(1.0, 2.5, 3.0, 4.0), "y");
  assert_ne!(
    base,
    bcb.add_constant_vector_d(1.0, 2.0, 3.5, 4.0),
    "extra2=z"
  );
  assert_ne!(
    base,
    bcb.add_constant_vector_d(1.0, 2.0, 3.0, 4.5),
    "extra3=w"
  );

  // 按位建键（与 cpp 的 memcpy 语义一致）：+0.0 与 -0.0 是两条不同常量
  assert_ne!(base, bcb.add_constant_vector_d(1.0, 2.0, 3.0, -0.0));
}

/// 同值不同类型必须各自成条。
#[test]
fn integer_and_number_share_nothing() {
  let mut bcb = BytecodeBuilder::new(None);

  let int = bcb.add_constant_integer(7);
  let num = bcb.add_constant_number(7.0);

  assert_ne!(int, num, "Integer 与 Number 的类型位必须参与判等");
  assert_eq!(int, bcb.add_constant_integer(7));
  assert_eq!(num, bcb.add_constant_number(7.0));
}

/// 字符串按内容去重（cpp `StringRef::operator==` 走 `string_view` 比较）。
#[test]
fn string_constants_dedupe_by_content() {
  let mut bcb = BytecodeBuilder::new(None);

  let buf_a = b"hello".to_vec();
  let buf_b = b"hello".to_vec();

  let first = bcb.add_constant_string(StringRef::from_slice(&buf_a));
  let second = bcb.add_constant_string(StringRef::from_slice(&buf_b));
  assert_eq!(first, second, "等内容字符串必须复用同一条常量");

  let other = bcb.add_constant_string(StringRef::from_slice(b"world"));
  assert_ne!(first, other);
}

/// 哨兵可达性回归：`TableShape::default()` 是解析器接受的零长度 DUPTABLE 形状
/// （`from_function_bytecode` 只拒 `length > K_MAX_LENGTH`），旧实现直接把它当
/// `DenseHashMap` 空键哨兵，于是 `find` 恒 `None`（去重彻底失效），
/// `insert_unsafe` 的 `debug_assert` 在 debug 下直接 panic。
#[test]
fn zero_length_table_shape_dedupes() {
  let mut bcb = BytecodeBuilder::new(None);

  let empty = TableShape::default();
  let first = bcb.add_constant_table(&empty);
  let second = bcb.add_constant_table(&empty);

  assert_eq!(first, 0, "首条 table 常量应落在 id 0");
  assert_eq!(first, second, "零长度 DUPTABLE 必须复用同一条常量");
}

/// 去重键必须逐字段判等：`has_constants` 是 cpp `TableShape::operator==` 的显式
/// 比较项，零长度形状只有它能区分。
#[test]
fn table_shape_has_constants_distinguishes_zero_length() {
  let mut bcb = BytecodeBuilder::new(None);

  let no_consts = TableShape::default();
  let with_consts = TableShape {
    has_constants: true,
    ..TableShape::default()
  };

  let a = bcb.add_constant_table(&no_consts);
  let b = bcb.add_constant_table(&with_consts);
  assert_ne!(a, b, "hasConstants 必须参与判等");
  assert_eq!(a, bcb.add_constant_table(&no_consts));
  assert_eq!(b, bcb.add_constant_table(&with_consts));
}

/// 哨兵本身不可达：一旦长度上界校验放宽，`table_shape.rs` 的编译期 `const` 断言
/// 就会失败，这里再守一条「哨兵 ≠ 任何 default 派生形状」。
#[test]
fn table_shape_sentinel_is_unreachable() {
  assert_ne!(
    TableShape::EMPTY_KEY_SENTINEL,
    TableShape::default(),
    "哨兵不得复用 default 形状"
  );
}
