use crate::{
  functions::{
    ensure_stack::ensure_stack, index_2_addr::index_2_addr, lapi_barrier::lua_c_threadbarrier_lapi,
  },
  macros::{
    api_check::api_check, api_update_top::api_update_top, getnodekey::getnodekey, gkey::gval,
    setnvalue::setnvalue, setobj_2_s::setobj_2_s,
  },
  records::{lua_node::LuaNode, lua_state::LuaState, lua_table::LuaTable},
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// # Safety
/// `l` 须为存活 `LuaState`；`idx` 经 `index_2_addr` 解析出的槽须为 table（`api_check ttistable`），
/// `iter` 须 `>=0`（`api_check`）作为下次遍历起点；`ensure_stack(l,2)` 预留两槽写回键/值，命中时按
/// `array[0..sizearray]` 与 `node[0..(1<<lsizenode)]` 区间遍历。cpp `lapi.cpp:1570`。
pub unsafe fn lua_rawiter(l: *mut LuaState, idx: i32, mut iter: i32) -> i32 {
  unsafe {
    lua_c_threadbarrier_lapi(l);
    // cpp `ensure_stack(L, 2)`：命中时一次写入 top+0/top+1 两格
    ensure_stack(l, 2);

    let t: StkId = index_2_addr(l, idx);
    api_check!(l, (*t).is_table());
    api_check!(l, iter >= 0);

    let h: *mut LuaTable = (*t).as_table_ptr();
    let sizearray = (*h).sizearray;

    // first we advance iter through the array portion
    while (iter as u32) < (sizearray as u32) {
      let e: *mut TValue = (*h).array.add(iter as usize);
      if !(*e).is_nil() {
        let top: StkId = (*l).top;
        setnvalue!(top.add(0), (iter + 1) as f64);
        setobj_2_s!(l, top.add(1), e);
        api_update_top!(l, top.add(2));
        return iter + 1;
      }
      iter += 1;
    }

    let sizenode = 1 << (*h).lsizenode;

    // then we advance iter through the hash portion
    while ((iter - sizearray) as u32) < (sizenode as u32) {
      let n: *mut LuaNode = (*h).node.add((iter - sizearray) as usize);
      let val = gval!(n);
      if !(*val).is_nil() {
        let top: StkId = (*l).top;
        getnodekey!(l, top.add(0), n);
        setobj_2_s!(l, top.add(1), val);
        api_update_top!(l, top.add(2));
        return iter + 1;
      }
      iter += 1;
    }

    // traversal finished
    -1
  }
}
