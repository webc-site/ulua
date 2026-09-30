//! 判别值→variant 直接位型转换（`transmute`）枚举的前置不变量守卫。
//!
//! `LuauOpcode::from(u8)`（`crates/ulua-common/src/enums/luau_opcode.rs`）与
//! `LuauBuiltinFunction::from_id(i32)`
//! （`crates/ulua-common/src/enums/luau_builtin_function.rs`）各自保留一处
//! `transmute`：它只在「判别值从 0 起连续、区间内每个值都是合法 variant」时才
//! 有效。给枚举插入显式判别值（或留下编号空洞）会静默破坏该前提，因此这里用
//! 往返用例把守 —— 前提破时先在这里失败，而不是变成 UB。

use ulua_common::enums::{luau_builtin_function::LuauBuiltinFunction, luau_opcode::LuauOpcode};

/// cpp `LOP__COUNT` 之前的每个字节都映射回自身；`LOP__COUNT` 及之后钳到
/// `LOP_NOP`（对齐 cpp `switch` 的 default 分支）。
#[test]
fn luau_opcode_discriminants_are_contiguous() {
  let count = LuauOpcode::LopCount as u8;

  for v in 0..count {
    assert_eq!(
      LuauOpcode::from(v) as u8,
      v,
      "判别值 {v} 未映射回自身：枚举判别值不再连续"
    );
  }

  for v in count..=u8::MAX {
    assert_eq!(
      LuauOpcode::from(v),
      LuauOpcode::LopNop,
      "越界判别值 {v} 应钳到 LopNop"
    );
  }
}

/// `LbfNone`(=0) 到最大 LBF id 的每个 id 都映射回自身，区间外返回 `None`。
#[test]
fn luau_builtin_function_ids_are_contiguous() {
  let max = LuauBuiltinFunction::LbfBufferWriteinteger as i32;

  for id in 0..=max {
    let converted = LuauBuiltinFunction::from_id(id);
    assert_eq!(
      converted.map(|f| f as i32),
      Some(id),
      "builtin id {id} 未映射回自身：枚举判别值不再连续"
    );
  }

  for id in [max + 1, i32::MAX, -1, i32::MIN] {
    assert_eq!(LuauBuiltinFunction::from_id(id), None, "id {id} 应越界");
  }
}
