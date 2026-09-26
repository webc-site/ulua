use core::slice::{from_raw_parts, from_raw_parts_mut};

use crate::{
  functions::{
    buffer_errors::buffer_oob_error, lua_l_checkbuffer::lua_l_checkbuffer,
    lua_l_checkinteger::lua_l_checkinteger, lua_l_checklstring::lua_l_checklstring,
    lua_l_optinteger::lua_l_optinteger,
  },
  macros::{isoutofbounds::isoutofbounds, lua_l_argcheck::luaL_argcheck, lua_l_error::luaL_error},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 LuaState 并处于本 C 函数受保护帧：栈 1 号为 buffer（`lua_l_checkbuffer` 取回 `buf`/`len` 并登记 GC），
/// 3 号为字符串（`val`/`size`）；`copy_nonoverlapping` 写入 `buf + offset..buf + offset + count`，
/// 该区间由 `isoutofbounds(offset, len, count)` 前置校验保证落在 buffer 内，且 count ≤ size。cpp/VM/src/lbuflib.cpp:216。
pub(crate) unsafe extern "C-unwind" fn buffer_writestring(l: *mut LuaState) -> i32 {
  unsafe {
    let mut len: usize = 0;
    let buf = lua_l_checkbuffer(l, 1, &mut len).cast::<u8>();
    let offset = lua_l_checkinteger(l, 2);

    let mut size: usize = 0;
    let val = lua_l_checklstring(l, 3, &mut size);
    let count = lua_l_optinteger(l, 4, size as i32);

    luaL_argcheck!(l, count >= 0, 4, "count");

    if count as usize > size {
      luaL_error!(l, "string length overflow");
    }

    if isoutofbounds(offset, len, count as usize) {
      buffer_oob_error(l);
    }

    // 源串与 buffer 不重叠；copy_from_slice 由上方校验保证两侧各 count 字节可读/可写
    let n = count as usize;
    let src = from_raw_parts(val.cast::<u8>(), n);
    let dst = from_raw_parts_mut(buf.add(offset as usize), n);
    dst.copy_from_slice(src);

    0
  }
}
