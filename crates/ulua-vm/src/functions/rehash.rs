use crate::{
  functions::{
    adjustasize::adjustasize, computesizes::computesizes, countint::countint,
    numusearray::numusearray, numusehash::numusehash, resize::resize,
  },
  macros::{maxbits::MAXBITS, nvalue::nvalue, ttisnumber::ttisnumber},
  records::{lua_state::lua_State, lua_table::LuaTable},
  type_aliases::t_value::TValue,
};

pub(crate) unsafe fn rehash(l: *mut lua_State, t: *mut LuaTable, ek: *const TValue) {
  unsafe {
    let mut nums = [0i32; (MAXBITS + 1) as usize];
    let nasize = numusearray(t, nums.as_mut_ptr());
    let mut totaluse = nasize;
    let mut nasize_mut = nasize;
    totaluse += numusehash(t, nums.as_mut_ptr(), &mut nasize_mut);

    if ttisnumber!(ek) {
      nasize_mut += countint(nvalue!(ek), &mut nums);
    }
    totaluse += 1;

    let na = computesizes(nums.as_ptr(), &mut nasize_mut);
    let mut nh = totaluse - na;

    let nadjusted = adjustasize(t, nasize_mut, ek);
    let aextra = nadjusted - nasize_mut;

    if aextra != 0 {
      nh -= aextra;
      nasize_mut = nadjusted + aextra;
      nasize_mut = adjustasize(t, nasize_mut, ek);
    }

    resize(l, t, nasize_mut, nh);
  }
}
