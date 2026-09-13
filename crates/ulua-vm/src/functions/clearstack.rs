use crate::{
  macros::setnilvalue::setnilvalue,
  type_aliases::{lua_state::lua_State, stk_id::StkId},
};

pub(crate) unsafe fn clearstack(l: *mut lua_State) {
  unsafe {
    let stack_end: StkId = (*l).stack.wrapping_add((*l).stacksize as usize);
    let mut o: StkId = (*l).top;
    while o < stack_end {
      setnilvalue!(o);
      o = o.wrapping_add(1);
    }
  }
}
