//! `LUAU_F_TABLE` 装表布局契约。
//!
//! 对照 cpp/VM/src/lbuiltins.cpp:2589-2781 的 `extern const luau_FastFunction
//! LUAU_F_TABLE[256]` 字面量：已移植槽必须指向各自的 `luauF_*` 实现（指错函数
//! 会让 FASTCALL 静默执行错误语义，`is_some` 级别的弱断言拦不住；函数指针的
//! `PartialEq` 即地址比较，直接 `assert_eq!` 逐槽钉死）；未移植槽一律
//! `luau_f_missing` 兜底（cpp 尾段 64 个 dummy 槽 + 零初始化 NULL 的安全等价
//! 形态，见 lbuiltins.cpp:2762-2764 注释）；`LBF_NONE` 保持 cpp 的 NULL
//! （lbuiltins.cpp:2591 表首项）。
//!
//! 各 `luau_f_*` 实现体的行为契约在各自测试侧，本文件只钉「哪个 id 装哪个
//! 函数」这张表本身。

use core::ptr::fn_addr_eq;
use std::{vec, vec::Vec};

use ulua_common::enums::luau_builtin_function::LuauBuiltinFunction;
use ulua_vm::{
  functions::{
    luau_f_byte::luau_f_byte,
    luau_f_extract::luau_f_extract,
    luau_f_missing::luau_f_missing,
    luau_f_modf::luau_f_modf,
    luau_f_rawequal::luau_f_rawequal,
    luau_f_readinteger::{luau_f_bufferreadlong, luau_f_readinteger},
    luau_f_vectormin::luau_f_vectormin,
    luau_f_writeinteger::luau_f_writeinteger,
  },
  macros::luau_f_table::{LUAU_F_TABLE, TABLE_LEN, build_table},
  type_aliases::luau_fast_function::LuauFastFunction,
};

/// 已移植进表的 builtin id → 应有的实现函数（与 `build_table` 覆盖项一一对应，
/// 逐项对照 cpp lbuiltins.cpp luauF_table 字面量的下标与函数名）。
fn ported_slots() -> Vec<(LuauBuiltinFunction, LuauFastFunction)> {
  vec![
    (LuauBuiltinFunction::LbfMathModf, Some(luau_f_modf)),
    (LuauBuiltinFunction::LbfBit32Extract, Some(luau_f_extract)),
    (LuauBuiltinFunction::LbfStringByte, Some(luau_f_byte)),
    (LuauBuiltinFunction::LbfRawequal, Some(luau_f_rawequal)),
    (
      LuauBuiltinFunction::LbfBufferReadi8,
      Some(luau_f_readinteger::<i8>),
    ),
    (
      LuauBuiltinFunction::LbfBufferReadu8,
      Some(luau_f_readinteger::<u8>),
    ),
    (
      LuauBuiltinFunction::LbfBufferWriteu8,
      Some(luau_f_writeinteger::<u8>),
    ),
    (
      LuauBuiltinFunction::LbfBufferReadi16,
      Some(luau_f_readinteger::<i16>),
    ),
    (
      LuauBuiltinFunction::LbfBufferReadu16,
      Some(luau_f_readinteger::<u16>),
    ),
    (
      LuauBuiltinFunction::LbfBufferWriteu16,
      Some(luau_f_writeinteger::<u16>),
    ),
    (
      LuauBuiltinFunction::LbfBufferReadi32,
      Some(luau_f_readinteger::<i32>),
    ),
    (
      LuauBuiltinFunction::LbfBufferReadu32,
      Some(luau_f_readinteger::<u32>),
    ),
    (
      LuauBuiltinFunction::LbfBufferWriteu32,
      Some(luau_f_writeinteger::<u32>),
    ),
    (
      LuauBuiltinFunction::LbfBufferReadinteger,
      Some(luau_f_bufferreadlong),
    ),
    (LuauBuiltinFunction::LbfVectorMin, Some(luau_f_vectormin)),
    // writeinteger 的 int64 臂（cpp `luauF_bufferwritelong`）尚未移植，
    // 留在兜底侧——与 macros/luau_f_table.rs 的装表注释同一声明。
  ]
}

/// 两个槽位是否指向同一函数（fn 指针的 `==` 触发
/// `unpredictable_function_pointer_comparisons`，按 std 指引走 `fn_addr_eq`；
/// `None` 只与 `None` 相同）。
fn same_fn(a: LuauFastFunction, b: LuauFastFunction) -> bool {
  match (a, b) {
    (None, None) => true,
    (Some(x), Some(y)) => fn_addr_eq(x, y),
    _ => false,
  }
}

/// 已移植的槽位必须逐地址指向自己的实现；若停留在 `luau_f_missing` 或装错
/// 函数，快速调用面会静默失效/走错语义（`n < 0` 回退慢路径掩盖一切）。
#[test]
fn ported_slots_point_to_their_own_impl() {
  for (id, expected) in ported_slots() {
    assert!(
      same_fn(LUAU_F_TABLE[id as usize], expected),
      "builtin {id:?} 槽位地址与应有实现不符"
    );
  }
}

/// 全表逐槽扫描（256 = `LOP_FASTCALL*` A 字段全域，cpp 表长）：
/// - `LBF_NONE` 是 cpp 的 NULL（唯一允许的空槽，FASTCALL 不会派发它）；
/// - PORTED 名单之外的每一个槽（含上游 dummy 尾部）都必须是
///   `luau_f_missing`，样本名单不再充数；
/// - 非空实现槽总数恰等于 PORTED 名单长度（防表外私加/漏装）。
#[test]
fn whole_table_layout_matches_upstream() {
  let missing: LuauFastFunction = Some(luau_f_missing);
  let ported: Vec<usize> = ported_slots()
    .into_iter()
    .map(|(id, _)| id as usize)
    .collect();
  let none_idx = LuauBuiltinFunction::LbfNone as usize;
  assert_eq!(TABLE_LEN, 256, "cpp LUAU_F_TABLE[256] 表长");
  assert!(
    LUAU_F_TABLE[none_idx].is_none(),
    "LBF_NONE 必须保持 cpp NULL"
  );

  let mut real_impls = 0;
  for (idx, &slot) in LUAU_F_TABLE.iter().enumerate() {
    if idx == none_idx || ported.contains(&idx) {
      real_impls += usize::from(slot.is_some());
      continue;
    }
    assert!(
      same_fn(slot, missing),
      "槽 {idx} 未移植，必须兜底 luau_f_missing"
    );
  }
  assert_eq!(
    real_impls,
    ported.len(),
    "非空实现槽总数 = PORTED（LBF_NONE 保持 NULL 不计入）"
  );
}

/// 静态表与编译期装表函数逐项同址（`LUAU_F_TABLE` 即 `build_table()` 的
/// 常量求值形态，二者漂移说明静态初始化被旁路改写）。
#[test]
fn static_table_equals_build_table() {
  let built = build_table();
  assert_eq!(built.len(), LUAU_F_TABLE.len());
  for (idx, (a, b)) in built.iter().zip(LUAU_F_TABLE.iter()).enumerate() {
    assert!(same_fn(*a, *b), "槽 {idx} 静态表与 build_table() 漂移");
  }
}
