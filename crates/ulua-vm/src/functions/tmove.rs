use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_g_readonlyerror::check_writable, lua_h_resizearray::lua_h_resizearray,
    moveelements::moveelements,
  },
  macros::{lua_lib_fn::lua_lib_fn, sizenode::sizenode},
  records::lua_state::LuaState,
};

/// cpp `shouldsparsemove` 的稀疏搬移阈值（ltablib.cpp:95，上游注记待 autotuning）。
const MIN_SPARSE_MOVE_ELEMS: i32 = 32;

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；r16-v41 收形后
/// 判型/取整/`arg_check`/压栈全经安全门面，体内裸操作仅余「两帧槽 hvalue 式取表句柄 + 句柄交
/// `check_writable`/`lua_h_resizearray`/`moveelements`」：句柄由紧邻的 `check_type` 与 `slot` 正索引
/// 界内换算就地建立（`tt∈{1,5}` 已在取值点与判型两处确认非 nil 表，`off=tt-1` 恒落已用窗内，与旧
/// 基址偏移读数逐位同址），表头不随数组段重分配搬移，且全程无用户代码可入，故无「合法输入令内部
/// 不变量失守即 UB」通路，本体降为安全 `fn`）：`l` 须处于可抛错受保护帧且帧顶窗深 ≥4——栈 1 号位为
/// table、2/3/4 号位为整数、5 号位可选 table，界校验失手经 `arg_check` 发散，只读目表经
/// `check_writable` 发散（cpp 于 `moveelements` 外预检一次以阻止只读表 resize）。
/// cpp/VM/src/ltablib.cpp:272 tmove。
pub(crate) fn tmove(l: &mut LuaState) -> i32 {
  l.check_type(1, LuaType::Table);
  let f = l.check_integer(2);
  let e = l.check_integer(3);
  let t = l.check_integer(4);
  let tt = if l.is_none_or_nil(5) { 1 } else { 5 };

  l.check_type(tt, LuaType::Table);

  if e >= f {
    l.arg_check(f > 0 || e < i32::MAX + f, 3, "too many elements to move");
    let n = e - f + 1;
    l.arg_check(t <= i32::MAX - n + 1, 4, "destination wrap around");

    // SAFETY: `src`/`dst` 借自上方两处 `check_type` 刚钉住的存活表槽；`check_writable` 的 `l`
    // 前提由 `as_mut_ptr` 的存活独占借用承载；字段裸读与 resize 均作用于该两存活表头。
    unsafe {
      let src = l.slot(1).get().as_table_ptr();
      let dst = l.slot(tt).get().as_table_ptr();

      check_writable(l.as_mut_ptr(), dst);

      let srcelems = (*src).sizearray + sizenode!(src);
      let dstelems = (*dst).sizearray + sizenode!(dst);
      let maxelems = srcelems.max(dstelems);
      // DELIBERATE DEVIATION: cpp `ltablib.cpp:292` 以 `DFFlag::LuauTableMoveTimeoutFix` 门控
      // 稀疏搬移（旗关即恒 false），本 port 无 DF 旗设施、恒按启发式取值——仅性能路径超集，
      // 搬移结果逐位同。
      let sparsemove = n > MIN_SPARSE_MOVE_ELEMS && n / 2 > maxelems;

      if t > 0 && (t - 1) <= (*dst).sizearray && (t - 1 + n) > (*dst).sizearray {
        // resize 先行扩目表数组段，`moveelements` 其后越段写才界内（cpp 同序）。
        lua_h_resizearray(l.as_mut_ptr(), dst, t - 1 + n);
      }

      moveelements(l, 1, tt, f, e, t, sparsemove);
    }
  }

  l.push_value(tt);
  1
}

lua_lib_fn!(pub(crate) fn tmove @ref, tmove_arm);
