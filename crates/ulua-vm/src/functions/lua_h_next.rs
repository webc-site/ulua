use crate::{
  enums::value_view::ValueView,
  functions::findindex::findindex,
  macros::{getnodekey::getnodekey, setnvalue::setnvalue, setobj_2_s::setobj_2_s},
  records::{lua_state::LuaState, lua_table::LuaTable},
  type_aliases::stk_id::StkId,
};

/// # Safety
/// `l` 须为存活 `LuaState`；`t` 须为存活 `LuaTable` 的共享只读借用（经 `array_window`/
/// `node_window` 共享窗读其数组与哈希两段，窗内借用期内表不得重排 `array`/`node` 指针，
/// 本函数不写表）；`key` 须指向栈上可作为前一键读入、并写回键/值两格（`key` 与 `key.add(1)`）
/// 的 StkId（出参写点保留裸指针形态，不借降级隐藏）。`findindex` 会读 `key` 当前值。cpp `ltable.cpp:379`。
pub(crate) unsafe fn lua_h_next(l: *mut LuaState, t: &LuaTable, key: StkId) -> i32 {
  unsafe {
    // cpp ltable.cpp:379 luaH_next：i = findindex(...) + 1 后先扫数组部分。
    // 查询键的 tag 读链（含 dead key 判定）已全部收敛在 findindex（B2a 已 match 化）；
    // 本函数对表槽位仅剩值轴 nil 判定，改用 B1 的 ValueView。
    // findindex 只读查询键，从出参槽即时借用（写回发生在其后）。
    let i = findindex(&mut *l, t, &*key) + 1;
    let sizearray = t.sizearray;

    // try first array part：切 array_window 共享窗，窗[i] ⇔ cpp `array[i]`（E1 契约逐位
    // 一致）；i = findindex + 1 ≥ 0，起点即 i，越上界自然止。
    for (idx, e) in t.array_window().iter().enumerate().skip(i.max(0) as usize) {
      if !matches!(ValueView::from_tvalue(e), ValueView::Nil) {
        setnvalue!(key, (idx + 1) as f64);
        setobj_2_s!(l, key.add(1), e);
        return 1;
      }
    }

    // then hash part；cpp 复用自增后的计数器，数组循环自然结束时其值恰为
    // sizearray，故哈希起点为 max(i - sizearray, 0)。切 node_window 共享窗：
    // 哨兵表窗长恒 1（单格 dummy，val 恒 nil ⇒ 空桶），读出与 cpp
    // `gnode!(t, 0)` 走查逐位一致；实向量窗长 twoto(lsizenode) ⇔ sizenode!(t)。
    // r12-w6d 逐点复核定性：本文件唯一 gnode 字样为 cpp 口径引用，代码点位已收编
    // node_window 共享窗形（窗长 1 哨兵折叠语义引 E1 契约），无余量。
    let nodes = t.node_window();
    for n in nodes.iter().skip((i - sizearray).max(0) as usize) {
      if !matches!(ValueView::from_tvalue(&n.val), ValueView::Nil) {
        getnodekey!(l, key, n);
        setobj_2_s!(l, key.add(1), &n.val);
        return 1;
      }
    }

    0 // no more elements
  }
}
