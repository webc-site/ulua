use crate::{
  enums::lua_type::LUA_T_COUNT, macros::markobject::markobject,
  type_aliases::global_state::global_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn markmt(g: *mut global_State) {
  let mut i = 0;
  while i < (LUA_T_COUNT as i32) {
    unsafe {
      if !(*g).mt[i as usize].is_null() {
        markobject!(g, (*g).mt[i as usize]);
      }
    }
    i += 1;
  }
}
