use core::ptr::copy;

use crate::{
  functions::{
    buffer_errors::buffer_oob_error, lua_l_checkbuffer::lua_l_checkbuffer,
    lua_l_checkinteger::lua_l_checkinteger, lua_l_optinteger::lua_l_optinteger,
  },
  macros::isoutofbounds::isoutofbounds,
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须指向本次 buffer 库调用的存活 `LuaState`，索引/长度实参按约定可读，栈顶有压入结果的余量。
pub(crate) unsafe extern "C-unwind" fn buffer_copy(l: *mut LuaState) -> i32 {
  // Safety: 契约保证源/目标偏移与长度经 argcheck 落在各自 buffer 界内，memmove 允许区间重叠
  unsafe {
    let mut tlen: usize = 0;
    let tbuf = lua_l_checkbuffer(l, 1, &mut tlen).cast::<u8>();
    let toffset = lua_l_checkinteger(l, 2);

    let mut slen: usize = 0;
    let sbuf = lua_l_checkbuffer(l, 3, &mut slen).cast::<u8>();
    let soffset = lua_l_optinteger(l, 4, 0);

    // C++ evaluates `int(slen) - soffset` as the default eagerly (signed overflow
    // is UB upstream for soffset = INT_MIN); wrapping_sub reproduces the two's-
    // complement value C++ relies on, which the `size < 0` / isoutofbounds checks
    // below then reject. (Upstream UBSan: lbuflib.cpp:257.)
    let size = lua_l_optinteger(l, 5, (slen as i32).wrapping_sub(soffset));

    if size < 0 {
      buffer_oob_error(l);
    }

    if isoutofbounds(soffset, slen, size as usize) {
      buffer_oob_error(l);
    }

    if isoutofbounds(toffset, tlen, size as usize) {
      buffer_oob_error(l);
    }

    copy(
      sbuf.add(soffset as usize),
      tbuf.add(toffset as usize),
      size as usize,
    );

    0
  }
}
