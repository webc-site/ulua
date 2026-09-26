//! IrCondition / IrBlockKind 常量映射定表的行为锁定回归（对齐 cpp oracle）。
//!
//! 覆盖本轮把 match/if 链收敛为编译期 const 索引表的 5 枚函数：
//! - `get_condition_int_64`      ← cpp `IrLoweringA64.cpp:123` getConditionInt64
//! - `get_condition_int` (A64)   ← cpp `IrLoweringA64.cpp:64`   getConditionInt
//! - `get_condition_fp`          ← cpp `IrLoweringA64.cpp:24`   getConditionFP
//! - `get_condition_int` (X64)   ← cpp `EmitCommonX64.cpp:98`   getConditionInt
//! - `get_negated_condition_ir_condition` ← cpp `IrUtils.h:142` getNegatedCondition
//! - 以及 `get_block_kind_priority` ← cpp 块排序优先级。
//!
//! 期望值逐条取自上述 cpp switch，与收敛前的 Rust match 分支一一对应，
//! 用以证明定表改写与旧实现行为全等。

use ulua_code_gen::{
  enums::{
    condition_a_64::ConditionA64, condition_x_64::ConditionX64, ir_block_kind::IrBlockKind,
    ir_condition::IrCondition,
  },
  functions::{
    get_block_kind_priority::get_block_kind_priority, get_condition_fp::get_condition_fp,
    get_condition_int_64::get_condition_int_64,
    get_condition_int_emit_common_x_64::get_condition_int as get_condition_int_x64,
    get_condition_int_ir_lowering_a_64::get_condition_int as get_condition_int_a64,
    get_negated_condition_ir_utils::get_negated_condition_ir_condition,
  },
};

/// 全部 14 个具名条件（不含 Count 哨兵），判别式 0..=13。
const CONDITIONS: [IrCondition; 14] = [
  IrCondition::Equal,
  IrCondition::NotEqual,
  IrCondition::Less,
  IrCondition::NotLess,
  IrCondition::LessEqual,
  IrCondition::NotLessEqual,
  IrCondition::Greater,
  IrCondition::NotGreater,
  IrCondition::GreaterEqual,
  IrCondition::NotGreaterEqual,
  IrCondition::UnsignedLess,
  IrCondition::UnsignedLessEqual,
  IrCondition::UnsignedGreater,
  IrCondition::UnsignedGreaterEqual,
];

#[test]
fn condition_int_64_matches_cpp_oracle() {
  use ConditionA64::*;
  let expected = [
    Equal,
    NotEqual,
    Less,
    GreaterEqual,
    LessEqual,
    Greater,
    Greater,
    LessEqual,
    GreaterEqual,
    Less,
    CarryClear,
    UnsignedLessEqual,
    UnsignedGreater,
    CarrySet,
  ];
  for (cond, want) in CONDITIONS.iter().zip(expected) {
    assert_eq!(get_condition_int_64(*cond), want, "int64({cond:?})");
  }
  // Count（越界）走 default 臂：cpp 侧 CODEGEN_ASSERT 会 trap，返回值在测试环境不可观测，故不断言。
}

#[test]
fn condition_int_a64_matches_cpp_oracle() {
  use ConditionA64::*;
  let expected = [
    Equal,
    NotEqual,
    Minus,
    Plus,
    LessEqual,
    Greater,
    Greater,
    LessEqual,
    GreaterEqual,
    Less,
    CarryClear,
    UnsignedLessEqual,
    UnsignedGreater,
    CarrySet,
  ];
  for (cond, want) in CONDITIONS.iter().zip(expected) {
    assert_eq!(get_condition_int_a64(*cond), want, "a64 int({cond:?})");
  }
}

#[test]
fn condition_fp_matches_cpp_oracle() {
  use ConditionA64::*;
  // cpp getConditionFP 仅前 10 个有映射，无符号条件与 Count 一律 Always。
  let expected = [
    Equal,
    NotEqual,
    Minus,
    Plus,
    UnsignedLessEqual,
    UnsignedGreater,
    Greater,
    LessEqual,
    GreaterEqual,
    Less,
  ];
  for (cond, want) in CONDITIONS.iter().zip(expected) {
    assert_eq!(get_condition_fp(*cond), want, "fp({cond:?})");
  }
  for cond in [
    IrCondition::UnsignedLess,
    IrCondition::UnsignedLessEqual,
    IrCondition::UnsignedGreater,
    IrCondition::UnsignedGreaterEqual,
    IrCondition::Count,
  ] {
    assert_eq!(get_condition_fp(cond), Always, "fp default({cond:?})");
  }
}

#[test]
fn condition_int_x64_matches_cpp_oracle() {
  use ConditionX64::*;
  let expected = [
    Equal,
    NotEqual,
    Less,
    NotLess,
    LessEqual,
    NotLessEqual,
    Greater,
    NotGreater,
    GreaterEqual,
    NotGreaterEqual,
    Below,
    BelowEqual,
    Above,
    AboveEqual,
  ];
  for (cond, want) in CONDITIONS.iter().zip(expected) {
    assert_eq!(get_condition_int_x64(*cond), want, "x64 int({cond:?})");
  }
  // Count 走 default 臂：cpp 侧 CODEGEN_ASSERT 会 trap，返回值在测试环境不可观测，故不断言。
}

#[test]
fn negated_condition_matches_cpp_oracle() {
  use IrCondition::*;
  let expected = [
    NotEqual,
    Equal,
    NotLess,
    Less,
    NotLessEqual,
    LessEqual,
    NotGreater,
    Greater,
    NotGreaterEqual,
    GreaterEqual,
    UnsignedGreaterEqual,
    UnsignedGreater,
    UnsignedLessEqual,
    UnsignedLess,
  ];
  for (cond, want) in CONDITIONS.iter().zip(expected) {
    assert_eq!(
      get_negated_condition_ir_condition(*cond),
      want,
      "negate({cond:?})"
    );
  }
  // 对合性：negate(negate(c)) == c
  for cond in CONDITIONS {
    assert_eq!(
      get_negated_condition_ir_condition(get_negated_condition_ir_condition(cond)),
      cond,
      "involution({cond:?})"
    );
  }
}

#[test]
fn block_kind_priority_matches_cpp_oracle() {
  assert_eq!(get_block_kind_priority(IrBlockKind::Fallback), 1);
  assert_eq!(get_block_kind_priority(IrBlockKind::ExitSync), 2);
  assert_eq!(get_block_kind_priority(IrBlockKind::Bytecode), 0);
  assert_eq!(get_block_kind_priority(IrBlockKind::Internal), 0);
  assert_eq!(get_block_kind_priority(IrBlockKind::Linearized), 0);
  assert_eq!(get_block_kind_priority(IrBlockKind::Dead), 0);
}
