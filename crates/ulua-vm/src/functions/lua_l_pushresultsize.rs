use crate::{functions::lua_l_pushresult::lua_l_pushresult, records::lua_l_strbuf::LuaLStrbuf};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn lua_l_pushresultsize(b: *mut LuaLStrbuf, size: usize) {
  unsafe {
    (*b).p = (*b).p.wrapping_add(size);
    lua_l_pushresult(b);
  }
}
