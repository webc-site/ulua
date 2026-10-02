use crate::{
  functions::{
    buffer_window::{buffer_at_ref, buffer_data_ref},
    lua_pushlstring::lua_pushlstring_bytes,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` must point to a valid, properly initialized `LuaState`.
pub(crate) unsafe fn buffer_readstring(l: *mut LuaState) -> i32 {
  // SAFETY: 契约保证 `l` 指向本次 buffer 库调用的存活 LuaState；窗口切片核心
  // （buffer_data_ref/buffer_at_ref）自带 typeerror→oob 抛错序，与旧出参形逐位一致，
  // 派生点保持在取参之前故观察序不变
  unsafe {
    let buf = buffer_data_ref(l, 1);
    let offset = (*l).check_integer(2);
    let size = (*l).check_integer(3);

    (*l).arg_check(size >= 0, 3, "size");

    // buffer_at_ref 的 isoutofbounds 已保证 [offset, offset+size) 落在 buffer 数据界内
    let region = buffer_at_ref(l, buf, offset, size as usize);
    lua_pushlstring_bytes(&mut *l, region);

    1
  }
}

lua_lib_fn!(pub(crate) fn buffer_readstring, buffer_readstring_arm);
