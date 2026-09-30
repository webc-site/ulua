use crate::{
  enums::value_view::ValueView,
  functions::findindex::findindex,
  macros::{
    getnodekey::getnodekey, gkey::gval, gnode::gnode, setnvalue::setnvalue, setobj_2_s::setobj_2_s,
    sizenode::sizenode,
  },
  records::{lua_state::LuaState, lua_table::LuaTable},
  type_aliases::stk_id::StkId,
};

/// # Safety
/// `l` 须为存活 `LuaState`；`t` 须为存活 `LuaTable`（其 `array[0..sizearray]` 与 hash 部分
/// `node[0..sizenode]` 均可读）；`key` 须指向栈上可作为前一键读入、并写回键/值两格（`key` 与 `key.add(1)`）
/// 的 StkId。`findindex` 会读 `key` 当前值。cpp `ltable.cpp:379`。
pub(crate) unsafe fn lua_h_next(l: *mut LuaState, t: *mut LuaTable, key: StkId) -> i32 {
  unsafe {
    // cpp ltable.cpp:379 luaH_next：i = findindex(...) + 1 后先扫数组部分。
    // 查询键的 tag 读链（含 dead key 判定）已全部收敛在 findindex（B2a 已 match 化）；
    // 本函数对表槽位仅剩值轴 nil 判定，改用 B1 的 ValueView。
    let i = findindex(l, t, key) + 1;
    let sizearray = (*t).sizearray;

    // try first array part
    for i in i..sizearray {
      let e = (*t).array.add(i as usize);
      if !matches!(ValueView::from_tvalue(&*e), ValueView::Nil) {
        setnvalue!(key, (i + 1) as f64);
        setobj_2_s!(l, key.add(1), e);
        return 1;
      }
    }

    // then hash part；cpp 复用自增后的计数器，数组循环自然结束时其值恰为
    // sizearray，故哈希起点为 max(i - sizearray, 0)
    let size = sizenode!(t);
    for k in (i - sizearray).max(0)..size {
      let n = gnode!(t, k);
      let val = gval!(n);
      if !matches!(ValueView::from_tvalue(&*val), ValueView::Nil) {
        getnodekey!(l, key, n);
        setobj_2_s!(l, key.add(1), val);
        return 1;
      }
    }

    0 // no more elements
  }
}
