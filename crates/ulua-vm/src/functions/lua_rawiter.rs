use crate::{
  functions::{
    ensure_stack::ensure_stack, index_2_addr::index_2_addr, lapi_barrier::lua_c_threadbarrier_lapi,
  },
  macros::{
    api_check::api_check, api_update_top::api_update_top, getnodekey::getnodekey,
    setnvalue::setnvalue, setobj_2_s::setobj_2_s,
  },
  records::{lua_state::LuaState, lua_table::LuaTable},
  type_aliases::stk_id::StkId,
};

/// `lua_rawiter` 核心（cpp `lapi.cpp:1570`）。调用序契约（正确性，非内存安全）：
/// `idx` 经 `index_2_addr` 解析出的槽须为 table（`api_check ttistable`），
/// `iter` 须 `>=0`（`api_check`）作为下次遍历起点；`ensure_stack(l,2)` 预留两槽写回键/值，命中时按
/// `array_window()` 与 `node_window()` 共享窗遍历（窗长分别 ⇔ `sizearray`、
/// `1<<lsizenode`，越窗访问由 cpp UB 降级为切片 panic）。
pub fn lua_rawiter(l: &mut LuaState, idx: i32, mut iter: i32) -> i32 {
  unsafe {
    lua_c_threadbarrier_lapi(l);
    // cpp `ensure_stack(L, 2)`：命中时一次写入 top+0/top+1 两格
    ensure_stack(l, 2);

    let t: StkId = index_2_addr(l, idx);
    api_check!(l, (*t).is_table());
    api_check!(l, iter >= 0);

    let h: *mut LuaTable = (*t).as_table_ptr();
    let sizearray = (*h).sizearray;

    // first we advance iter through the array portion：切 array_window 共享窗，
    // 窗[iter] ⇔ 原 `array.add(iter)`（E1 契约逐位一致）；u32 环形判界与 cpp 同构，
    // iter<0 或 iter>=sizearray 时窗体不触达，循环节拍不变。
    let arr = (*h).array_window();
    while (iter as u32) < (sizearray as u32) {
      let e = &arr[iter as usize];
      if !e.is_nil() {
        let top: StkId = l.top;
        setnvalue!(top.add(0), (iter + 1) as f64);
        setobj_2_s!(l, top.add(1), e);
        api_update_top!(l, top.add(2));
        return iter + 1;
      }
      iter += 1;
    }

    // then we advance iter through the hash portion：切 node_window 共享窗。
    // 哨兵表窗长恒 1（单格 dummy，val 恒 nil ⇒ 空桶走查落空），实向量窗长
    // twoto(lsizenode) ⇔ 原 `1 << (*h).lsizenode`，走查序与 cpp 逐位一致。
    let nodes = (*h).node_window();
    while ((iter - sizearray) as u32) < (nodes.len() as u32) {
      let n = &nodes[(iter - sizearray) as usize];
      if !n.val.is_nil() {
        let top: StkId = l.top;
        getnodekey!(l, top.add(0), n);
        setobj_2_s!(l, top.add(1), &n.val);
        api_update_top!(l, top.add(2));
        return iter + 1;
      }
      iter += 1;
    }

    // traversal finished
    -1
  }
}
