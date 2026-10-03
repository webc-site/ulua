use crate::{
  enums::{lua_type::LuaType, t_key_view::TKeyView, value_view::ValueView},
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 的存活/独占前提已由 `&mut` 接收者类型承载（r16-v41 收形）；屏障仍保留是因为体内有真实
/// 裸操作：`(*l.base)` 解引用栈基址 `StkId` 取 index 1 槽、再经 `as_table_ptr` 得表对象裸指针 `t`，
/// 随后 `(*t).array_window()`/`(*t).node_window()` 依该裸句柄读取数组段/哈希段窗口（cpp 按 `sizenode`
/// 走查同形），这些 `*mut Table`/`StkId` 解引用均非安全门面可达。余下取参前提：`l` 须处于 `maxn`
/// 调用的受保护帧，1..=n 实参栈槽可读，数值探测与比较不越过帧栈界，`push_number` 需 `top` 后留结果余量。
/// cpp `ltablib.cpp:53`。
pub(crate) unsafe fn maxn(l: &mut LuaState) -> i32 {
  let mut max: f64 = 0.0;
  // SAFETY: 上方 `&mut l` 保证存活帧，`l.base`/`t` 裸解引用只读 1..=n 实参栈槽与表对象，不越过帧栈界
  unsafe {
    l.check_type(1, LuaType::Table);

    let t = (*l.base).as_table_ptr();

    // 数组尾部：最后一个非 nil 元素决定 max，rposition 从后往前提前终止；
    // 走 array_window 共享窗（窗长 max(sizearray,0)，null 数组归空窗，免手工 c_slice）
    let arr = (*t).array_window();
    if let Some(i) = arr
      .iter()
      .rposition(|v| !matches!(ValueView::from_tvalue(v), ValueView::Nil))
    {
      max = (i + 1) as f64;
    }

    // 哈希段走 node_window 共享窗，只读迭代不写回；哨兵表窗长恒 1（dummy 单格
    // val 恒 nil ⇒ 空桶被跳过），与 cpp 按 sizenode 走查逐位一致
    for n in (*t).node_window() {
      if matches!(ValueView::from_tvalue(&n.val), ValueView::Nil) {
        continue;
      }

      // 键轴（B2a TKeyView）：数值键候选链收敛为变体 match
      if let TKeyView::Number(v) = TKeyView::from_tkey(&n.key)
        && v > max
      {
        max = v;
      }
    }

    l.push_number(max);
  }
  1
}

lua_lib_fn!(pub(crate) fn maxn @ref, maxn_arm);
