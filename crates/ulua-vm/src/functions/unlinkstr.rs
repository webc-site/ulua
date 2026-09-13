use crate::{
  macros::lmod::lmod,
  records::{global_state::global_State, lua_state::lua_State, t_string::tstring},
};

pub(crate) unsafe fn unlinkstr(l: *mut lua_State, ts: *mut tstring) -> bool {
  unsafe {
    let g: *mut global_State = (*l).global;

    let hash = (*ts).hash as usize;
    let size = (*g).strt.size as usize;
    let mut p: *mut *mut tstring = (*g).strt.hash.add(lmod!(hash, size) as usize);

    while !(*p).is_null() {
      let curr = *p;
      if curr == ts {
        *p = (*curr).next;
        return true;
      } else {
        p = &mut (*curr).next as *mut *mut tstring;
      }
    }

    false
  }
}
