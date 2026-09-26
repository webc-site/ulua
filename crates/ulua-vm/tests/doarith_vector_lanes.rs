//! `LUA_VECTOR_SIZE == 3` 的向量臂回归护栏（对照 cpp/VM/src/lvmutils.cpp:451-558
//! 与 `lobject.h:141-151` 的 `condvector4`）：向量算术只参与 3 个分量，
//! 第 4 分量既不参与运算、也绝不写进结果 TValue。
//!
//! 3-lane 布局下 `value(8B) + extra(4B)` 正好是 3 个 f32，第 4 个 f32 槽与
//! `tt` 重叠：`vvalue!` 只给出 `&[f32; 3]`，`setvvalue!` 用
//! `LUA_VECTOR_SIZE == 4` 门控跳过 lane-3 写。这里对结果做
//! `[f32; 3] + tt` 的逐字节比对，把这条契约钉死（调用点里那个
//! `*vb.add(3)` 形式的 lane-3 读出门即越界读，须由 `cargo +nightly miri test`
//! 捕获，本用例保证正常路径的可见结果不被污染）。
//!
//! 驱动面为 C ABI 注册表导出的 8 个 `lua_v_doarithimpl_tm_*` 入口
//! （cpp 对应 `lobject.h` 的 `luauF_*` tm 导出族），与内部
//! `lua_v_doarithimpl` 一一对应。

use core::{mem::size_of, ptr::null_mut, slice::from_raw_parts};

use ulua_vm::{
  enums::{lua_type::LuaType, tms::TMS},
  functions::lua_v_doarithimpl::{
    lua_v_doarithimpl_tm_add, lua_v_doarithimpl_tm_div, lua_v_doarithimpl_tm_idiv,
    lua_v_doarithimpl_tm_mod, lua_v_doarithimpl_tm_mul, lua_v_doarithimpl_tm_pow,
    lua_v_doarithimpl_tm_sub, lua_v_doarithimpl_tm_unm,
  },
  macros::{setnvalue::setnvalue, setvvalue::setvvalue},
  records::lua_state::LuaState,
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// 按 `op` 分派到对应的 C ABI 导出入口（与 `lua_v_doarithimpl` 的转发一一对应）。
///
/// # Safety
/// 指针须满足 `lua_v_doarithimpl` 的前置条件（与源用例一致）。
unsafe fn lua_v_doarithimpl(
  l: *mut LuaState,
  ra: StkId,
  rb: *const TValue,
  rc: *const TValue,
  op: TMS,
) {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    match op {
      TMS::TmAdd => lua_v_doarithimpl_tm_add(l, ra, rb, rc),
      TMS::TmSub => lua_v_doarithimpl_tm_sub(l, ra, rb, rc),
      TMS::TmMul => lua_v_doarithimpl_tm_mul(l, ra, rb, rc),
      TMS::TmDiv => lua_v_doarithimpl_tm_div(l, ra, rb, rc),
      TMS::TmIDiv => lua_v_doarithimpl_tm_idiv(l, ra, rb, rc),
      TMS::TmMod => lua_v_doarithimpl_tm_mod(l, ra, rb, rc),
      TMS::TmPow => lua_v_doarithimpl_tm_pow(l, ra, rb, rc),
      TMS::TmUnm => lua_v_doarithimpl_tm_unm(l, ra, rb, rc),
      _ => unreachable!("本用例只驱动 8 个算术 tm"),
    }
  }
}

/// 构造向量 TValue；第 4 实参是给 `setvvalue!` 的哨兵，3-lane 构建下必须被丢弃。
fn vector(x: f32, y: f32, z: f32) -> TValue {
  let mut t = TValue::default();
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`t` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe { setvvalue!(&mut t, x, y, z, f32::from_bits(0xDEAD_BEEF)) };
  t
}

/// 构造数值 TValue（`rc`/`rb` 的标量侧）。
fn number(n: f64) -> TValue {
  let mut t = TValue::default();
  // `setnvalue!`/`set_nvalue` 已收口为安全实现：宏体对 `&mut t` 的解引用非 unsafe。
  setnvalue!(&mut t, n);
  t
}

/// 结果 TValue 的全部字节（value + extra + tt）。
fn bytes(t: &TValue) -> &[u8] {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`t` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe { from_raw_parts(t as *const TValue as *const u8, size_of::<TValue>()) }
}

/// 跑一次 `lua_v_doarithimpl` 并与 `want` 逐字节比对。
///
/// `l` 传 `null_mut()`：向量臂与「双数值」臂都在触达唯一使用 `l` 的
/// `call_bin_tm`（元方法慢路径）之前 return，空指针不会被解引用。
fn assert_op(op: TMS, rb: &TValue, rc: &TValue, want: [f32; 3]) {
  let mut res = TValue::default();
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`res` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe { lua_v_doarithimpl(null_mut(), &mut res, rb, rc, op) };

  assert_eq!(res.tt, LuaType::Vector as i32, "{op:?} 结果必须是向量 tag");
  assert_eq!(res.as_vector_ref(), &want, "{op:?} 三个分量必须逐位一致");
  // 期望值经同一个 setvvalue! 生成：字节相等即 extra 承载 lane-2、
  // tt 承载 tag，第 4 分量槽没有被写。
  let expected = vector(want[0], want[1], want[2]);
  assert_eq!(bytes(&res), bytes(&expected), "{op:?} 结果字节布局被改写");
}

/// v⊗v：ADD/SUB/MUL/DIV/IDIV/UNM 六个向量臂（cpp lvmutils.cpp:464-496）。
#[test]
fn vector_with_vector_keeps_three_lanes() {
  let b = vector(6.0, -8.0, 12.0);
  let c = vector(2.0, 4.0, 3.0);

  assert_op(TMS::TmAdd, &b, &c, [8.0, -4.0, 15.0]);
  assert_op(TMS::TmSub, &b, &c, [4.0, -12.0, 9.0]);
  assert_op(TMS::TmMul, &b, &c, [12.0, -32.0, 36.0]);
  assert_op(TMS::TmDiv, &b, &c, [3.0, -2.0, 4.0]);
  assert_op(TMS::TmIDiv, &b, &c, [3.0, -2.0, 4.0]);
  assert_op(TMS::TmUnm, &b, &c, [-6.0, 8.0, -12.0]);
}

/// v⊗s：MUL/DIV/IDIV 只缩放前三个分量（cpp lvmutils.cpp:497-527）。
#[test]
fn vector_with_scalar_keeps_three_lanes() {
  let b = vector(6.0, -8.0, 12.0);
  let s = number(2.0);

  assert_op(TMS::TmMul, &b, &s, [12.0, -16.0, 24.0]);
  assert_op(TMS::TmDiv, &b, &s, [3.0, -4.0, 6.0]);
  assert_op(TMS::TmIDiv, &b, &s, [3.0, -4.0, 6.0]);
}

/// s⊗v：对称分支同样不得写第 4 分量（cpp lvmutils.cpp:528-558）。
#[test]
fn scalar_with_vector_keeps_three_lanes() {
  let s = number(12.0);
  let c = vector(6.0, -8.0, 3.0);

  assert_op(TMS::TmMul, &s, &c, [72.0, -96.0, 36.0]);
  assert_op(TMS::TmDiv, &s, &c, [2.0, -1.5, 4.0]);
  assert_op(TMS::TmIDiv, &s, &c, [2.0, -2.0, 4.0]);
}

/// 双数值回退臂：与向量分支互不干扰，结果 tag 必须是 Number。
#[test]
fn scalar_with_scalar_falls_back_to_number() {
  let b = number(7.0);
  let c = number(2.0);

  for (op, want) in [
    (TMS::TmAdd, 9.0f64),
    (TMS::TmSub, 5.0),
    (TMS::TmMul, 14.0),
    (TMS::TmDiv, 3.5),
    (TMS::TmIDiv, 3.0),
    (TMS::TmMod, 1.0),
    (TMS::TmPow, 49.0),
  ] {
    let mut res = TValue::default();
    // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`res` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
    unsafe { lua_v_doarithimpl(null_mut(), &mut res, &b, &c, op) };

    assert_eq!(res.tt, LuaType::Number as i32, "{op:?} 结果必须是数值");
    // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`res` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
    assert_eq!(unsafe { res.value.n }, want, "{op:?} 数值结果不符");
  }
}
