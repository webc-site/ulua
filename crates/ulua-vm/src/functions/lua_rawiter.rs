use core::ffi::c_int;

use crate::{
  functions::{index_2_addr::index2addr, lua_concat::lua_c_threadbarrier_lapi},
  macros::{
    api_check::api_check, api_update_top::api_update_top, getnodekey::getnodekey, hvalue::hvalue,
    setnvalue::setnvalue, setobj_2_s::setobj2s, ttisnil::ttisnil, ttistable::ttistable,
  },
  records::{lua_node::LuaNode, lua_state::lua_State, lua_table::LuaTable},
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_rawiter(l: *mut lua_State, idx: c_int, iter: c_int) -> c_int {
  unsafe {
    lua_c_threadbarrier_lapi(l);

    let t: StkId = index2addr(l, idx);
    api_check!(l, ttistable!(t));
    api_check!(l, iter >= 0);

    let h: *mut LuaTable = hvalue!(t);
    let sizearray = (*h).sizearray;

    // first we advance iter through the array portion
    let mut iter = iter;
    while (iter as u32) < (sizearray as u32) {
      let e: *mut TValue = (*h).array.add(iter as usize);
      if !ttisnil!(e) {
        let top: StkId = (*l).top;
        setnvalue!(top.add(0), (iter + 1) as f64);
        setobj2s!(l, top.add(1), e);
        api_update_top!(l, top.add(2));
        return iter + 1;
      }
      iter += 1;
    }

    let sizenode = 1 << (*h).lsizenode;

    // then we advance iter through the hash portion
    while ((iter - sizearray) as u32) < (sizenode as u32) {
      let n: *mut LuaNode = (*h).node.add((iter - sizearray) as usize);
      let val = core::ptr::addr_of_mut!((*n).val);
      if !ttisnil!(val) {
        let top: StkId = (*l).top;
        getnodekey!(l, top.add(0), n);
        setobj2s!(l, top.add(1), val);
        api_update_top!(l, top.add(2));
        return iter + 1;
      }
      iter += 1;
    }

    // traversal finished
    -1
  }
}
