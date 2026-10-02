use crate::{
  enums::{lua_type::LuaType, t_key_view::TKeyView, value_view::ValueView},
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 必须指向本次 strlib 调用的存活 `LuaState`，所需实参按索引可读且栈顶有结果余量。
pub(crate) unsafe fn maxn(l: *mut LuaState) -> i32 {
  let mut max: f64 = 0.0;
  // SAFETY: 契约保证 `l` 为存活调用帧、1..=n 实参栈槽可读，数值探测与比较不越过帧栈界
  unsafe {
    (*l).check_type(1, LuaType::Table);

    let t = (*(*l).base).as_table_ptr();

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

    (*l).push_number(max);
  }
  1
}

lua_lib_fn!(pub(crate) fn maxn, maxn_arm);
