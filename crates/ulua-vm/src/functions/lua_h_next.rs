use crate::{
  functions::findindex::findindex,
  macros::{
    cast_num::cast_num, getnodekey::getnodekey, gnode::gnode, setnvalue::setnvalue,
    setobj_2_s::setobj_2_s, sizenode::sizenode, ttisnil::ttisnil,
  },
  type_aliases::{lua_state::lua_State, lua_table::LuaTable, stk_id::StkId},
};

// Required by getnodekey macro expansion

pub(crate) unsafe fn lua_h_next(l: *mut lua_State, t: *mut LuaTable, key: StkId) -> i32 {
  unsafe {
    let mut i = findindex(l, t, key);

    i += 1;

    // try first array part
    while i < (*t).sizearray {
      let e = (*t).array.add(i as usize);
      if !ttisnil!(e) {
        setnvalue!(key, cast_num!(i + 1));
        setobj_2_s!(l, key.add(1), e);
        return 1;
      }
      i += 1;
    }

    // then hash part
    let mut k = i - (*t).sizearray;
    let size = sizenode!(t);
    while k < size {
      let n = gnode!(t, k);
      // gval(n) in C++ is (&(n)->val). In our Rust macros, gval is not provided as a standalone macro
      // but the logic is usually just the address of the val field.
      let val = core::ptr::addr_of_mut!((*n).val);
      if !ttisnil!(val) {
        getnodekey!(l, key, n);
        setobj_2_s!(l, key.add(1), val);
        return 1;
      }
      k += 1;
    }

    0 // no more elements
  }
}
