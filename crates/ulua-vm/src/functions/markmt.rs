use crate::{
  enums::lua_type::LUA_T_COUNT, macros::markobject::markobject, records::global_state::global_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn markmt(g: *mut global_State) {
  unsafe {
    for mt in (*g).mt.iter_mut().take(LUA_T_COUNT as usize) {
      if !(*mt).is_null() {
        markobject!(g, *mt);
      }
    }
  }
}
