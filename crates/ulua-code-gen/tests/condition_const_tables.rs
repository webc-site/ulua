//! IrCondition / IrBlockKind 常量映射定表的行为锁定回归（对齐 cpp oracle）。
//!
//! 覆盖本轮把 match/if 链收敛为编译期 const 索引表的 5 枚函数：
//! - `get_condition_int_64`      ← cpp `IrLoweringA64.cpp:123` getConditionInt64
//! - `get_condition_int` (A64)   ← cpp `IrLoweringA64.cpp:64`   getConditionInt
//! - `get_condition_fp`          ← cpp `IrLoweringA64.cpp:24`   getConditionFP
//! - `get_condition_int` (X64)   ← cpp `EmitCommonX64.cpp:98`   getConditionInt
//! - `get_negated_condition_ir_condition` ← cpp `IrUtils.h:142` getNegatedCondition
//! - `get_negated_condition` (X64) ← cpp `EmitCommonX64.h` getNegatedCondition（旧 match 逐臂）
//! - 以及 `get_block_kind_priority` ← cpp 块排序优先级。
//!
//! 期望值逐条取自上述 cpp switch，与收敛前的 Rust match 分支一一对应，
//! 用以证明定表改写与旧实现行为全等。

// m4 越界回退臂可观测化：`CODEGEN_ASSERT` 复用 `ulua_common` 的进程级全局单槽
// 断言处理器（与 `assert_handler.rs`、`ulua-bytecode/tests/validate_captures.rs`
// 同一 harness）。安装一个「返回 0（已接管）+ 计数」的处理器即可让回退臂跳过
// `LUAU_DEBUGBREAK`、正常返回各自的 `$fallback`——回退值因此在测试进程内可观测。
use core::{
  ffi::c_char,
  sync::atomic::{AtomicUsize, Ordering},
};
use std::sync::Mutex;

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
    get_negated_condition_condition_x_64::get_negated_condition as get_negated_condition_x64,
    get_negated_condition_ir_utils::get_negated_condition_ir_condition,
  },
};
use ulua_common::functions::assert_handler::{assert_handler, set_assert_handler};

/// 越界回退触发 `CODEGEN_ASSERT` 的次数（证明断言确经处理器接管）。
static CODEGEN_ASSERT_FIRED: AtomicUsize = AtomicUsize::new(0);
/// 处理器是进程级全局槽位；本 binary 内仅本测试接管它。互斥锁只串行化对本槽位
/// 「读原值→安装→恢复」这段读改写（收窄全局单槽被改写的窗口）；全局计数是原子
/// 无锁累加，不在本锁护栏内——其不受踩是「其余测试不触发断言」的事实，而非锁的保证。
static HANDLER_LOCK: Mutex<()> = Mutex::new(());

/// 模拟 host 接管：计一次并返回 0（cpp `assertCallHandler` 语义：0 即无需再断点）。
extern "C-unwind" fn suppress_and_count(
  _expression: *const c_char,
  _file: *const c_char,
  _line: i32,
  _function: *const c_char,
) -> i32 {
  CODEGEN_ASSERT_FIRED.fetch_add(1, Ordering::SeqCst);
  0
}

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
  // Count（越界）走 default 臂：其回退值经 handler 接管后可观测，见
  // `out_of_range_condition_fallback_arms_return_cpp_values`。
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
  // cpp getConditionFP 仅前 10 个有映射。
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
  // 表长 10，故无符号条件（判别式 10..=13）与 `Count` 同走回退臂；该臂带
  // `CODEGEN_ASSERT`（对齐 cpp `default:` 臂的 `CODEGEN_ASSERT(!"Unexpected
  // condition code")`），其回退值 `Always` 的可观测钉在
  // `out_of_range_condition_fallback_arms_return_cpp_values`。
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
  // Count 走 default 臂：其回退值经 handler 接管后可观测，见
  // `out_of_range_condition_fallback_arms_return_cpp_values`。
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
fn negated_condition_x64_matches_old_match() {
  use ConditionX64::*;
  // 期望值逐条取自改写前的 Rust match 臂（其本身对齐 cpp EmitCommonX64.h），
  // 证明 match→定表收敛后 26 个具名条件逐值全等。
  let pairs = [
    (Overflow, NoOverflow),
    (NoOverflow, Overflow),
    (Carry, NoCarry),
    (NoCarry, Carry),
    (Below, NotBelow),
    (BelowEqual, NotBelowEqual),
    (Above, NotAbove),
    (AboveEqual, NotAboveEqual),
    (Equal, NotEqual),
    (Less, NotLess),
    (LessEqual, NotLessEqual),
    (Greater, NotGreater),
    (GreaterEqual, NotGreaterEqual),
    (NotBelow, Below),
    (NotBelowEqual, BelowEqual),
    (NotAbove, Above),
    (NotAboveEqual, AboveEqual),
    (NotEqual, Equal),
    (NotLess, Less),
    (NotLessEqual, LessEqual),
    (NotGreater, Greater),
    (NotGreaterEqual, GreaterEqual),
    (Zero, NotZero),
    (NotZero, Zero),
    (Parity, NotParity),
    (NotParity, Parity),
  ];
  for (cond, want) in pairs {
    assert_eq!(
      get_negated_condition_x64(cond),
      want,
      "x64 negate({cond:?})"
    );
    // 取反为对合
    assert_eq!(
      get_negated_condition_x64(get_negated_condition_x64(cond)),
      cond,
      "x64 negate involution({cond:?})"
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

/// 越界回退臂可观测钉：6 张表的 `default:` 回退臂（均带 `CODEGEN_ASSERT`，
/// 对齐 cpp 各 switch 的 default 臂断言）逐表钉死回退值——
/// `IR_TO_A64_INT64`/`IR_TO_A64_INT` → `Always`，`IR_TO_X64_INT` → `Zero`，
/// `IR_NEGATED` → `Count`，`X64_NEGATED` → `Count`，`IR_TO_A64_FP` → `Always`
/// （表长 10，4 个无符号条件与 `Count` 共 5 例越界）。不接管 handler 时回退臂
/// 会走 `LUAU_DEBUGBREAK` 直接崩掉测试进程（这正是它们此前“不可观测故不断言”的
/// 根因）；此处按 `assert_handler.rs` 先例接管一个返回 0 的处理器，使回退值可
/// 观测、断言其精确取值，并核对 `CODEGEN_ASSERT` 恰好触发 10 次以证明全部走的
/// 越界回退路径而非命中。
#[test]
fn out_of_range_condition_fallback_arms_return_cpp_values() {
  let _guard = HANDLER_LOCK
    .lock()
    .unwrap_or_else(|poisoned| poisoned.into_inner());
  let previous = assert_handler();
  CODEGEN_ASSERT_FIRED.store(0, Ordering::SeqCst);
  set_assert_handler(Some(suppress_and_count));

  // 越界 Count（判别式 14，各表 len 均 ≤ 14）→ 各自回退常量。
  let int64_fallback = get_condition_int_64(IrCondition::Count);
  let a64_int_fallback = get_condition_int_a64(IrCondition::Count);
  let x64_int_fallback = get_condition_int_x64(IrCondition::Count);
  let negated_fallback = get_negated_condition_ir_condition(IrCondition::Count);
  // `get_condition_fp` 表长仅 10：4 个无符号条件（10..=13）与 Count 同为越界。
  let fp_unsigned_fallbacks = [
    IrCondition::UnsignedLess,
    IrCondition::UnsignedLessEqual,
    IrCondition::UnsignedGreater,
    IrCondition::UnsignedGreaterEqual,
    IrCondition::Count,
  ]
  .map(get_condition_fp);
  // X64 取反表：仅 `Count`（判别式 26，表长 26）越界回退。
  let x64_negated_fallback = get_negated_condition_x64(ConditionX64::Count);

  // 立即恢复原处理器，收窄全局单槽被改写的窗口（其余取值/计数断言不再依赖 handler）。
  set_assert_handler(previous);

  assert_eq!(
    int64_fallback,
    ConditionA64::Always,
    "IR_TO_A64_INT64 回退臂 = Always"
  );
  assert_eq!(
    a64_int_fallback,
    ConditionA64::Always,
    "IR_TO_A64_INT 回退臂 = Always"
  );
  assert_eq!(
    x64_int_fallback,
    ConditionX64::Zero,
    "IR_TO_X64_INT 回退臂 = Zero"
  );
  assert_eq!(
    negated_fallback,
    IrCondition::Count,
    "IR_NEGATED 回退臂 = Count（取反哨兵回落为自身）"
  );
  assert_eq!(
    fp_unsigned_fallbacks,
    [ConditionA64::Always; 5],
    "IR_TO_A64_FP 回退臂 = Always（无符号条件与 Count 共 5 例）"
  );
  assert_eq!(
    x64_negated_fallback,
    ConditionX64::Count,
    "X64_NEGATED 回退臂 = Count（cpp `Count:` 臂断言后 return Count）"
  );
  assert_eq!(
    CODEGEN_ASSERT_FIRED.load(Ordering::SeqCst),
    10,
    "六张含 CODEGEN_ASSERT 的回退臂共触发十次断言（4+5+1，证明走的是越界 default 而非命中）"
  );
}
