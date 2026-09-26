use crate::{
  enums::{lua_type::LuaType, t_key_view::TKeyView, value_view::ValueView},
  functions::{c_slice, lua_l_checktype::lua_l_checktype, lua_pushnumber::lua_pushnumber},
  macros::{lua_lib_fn::lua_lib_fn, sizenode::sizenode},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 必须指向本次 strlib 调用的存活 `LuaState`，所需实参按索引可读且栈顶有结果余量。
pub(crate) unsafe fn maxn(l: *mut LuaState) -> i32 {
  let mut max: f64 = 0.0;
  // Safety: 契约保证 `l` 为存活调用帧、1..=n 实参栈槽可读，数值探测与比较不越过帧栈界
  unsafe {
    lua_l_checktype(l, 1, LuaType::Table as i32);

    let t = (*(*l).base).as_table_ptr();

    // 数组尾部：最后一个非 nil 元素决定 max，rposition 从后往前提前终止
    let arr = c_slice((*t).array, (*t).sizearray as usize);
    if let Some(i) = arr
      .iter()
      .rposition(|v| !matches!(ValueView::from_tvalue(v), ValueView::Nil))
    {
      max = (i + 1) as f64;
    }

    let node_size = sizenode!(t);
    // 节点数组按切片迭代：消除逐次 add 的索引写法，宏只读取不写回
    for n in c_slice((*t).node, node_size as usize) {
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

    lua_pushnumber(l, max);
  }
  1
}

lua_lib_fn!(pub(crate) fn maxn, maxn_arm);
