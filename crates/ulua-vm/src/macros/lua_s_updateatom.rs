//! Source: `VM/src/lstring.h` (lstring.h:21-25, hand-ported)

// #define lua_s_updateatom(l, ts)
//     { if (ts->atom == ATOM_UNDEF)
//           ts->atom = l->global->cb.useratom ? l->global->cb.useratom(l, ts->data, ts->len) : -1; }
#[macro_export]
macro_rules! lua_s_updateatom {
  ($l:expr, $ts:expr) => {
    if (*$ts).atom as i32 == $crate::macros::atom_undef::ATOM_UNDEF {
      (*$ts).atom = match (*(*$l).global).cb.useratom {
        Some(useratom) => useratom(
          $l,
          core::ptr::addr_of!((*$ts).data) as *const core::ffi::c_char,
          (*$ts).len as usize,
        ),
        None => -1,
      };
    }
  };
}

pub use lua_s_updateatom;
