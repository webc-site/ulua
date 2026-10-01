use core::slice::from_raw_parts;

use crate::{
  functions::{
    lua_pushlstring::lua_pushlstring_bytes, lua_s_buffinish::lua_s_buffinish,
    lua_s_newlstr::lua_s_newlstr,
  },
  macros::{lua_c_check_gc::lua_c_check_gc, setsvalue::setsvalue},
  records::lua_l_strbuf::LuaLStrbuf,
};

/// # Safety
/// `b` 须指向存活 `LuaLStrbuf`，其 `l` 为处于可 GC 受保护帧的 lua_State、`(*l).top - 1` 为预留结果槽；
/// `storage` 若非空须为存活缓冲区且写入位置 `b.p` 落在 `storage.data..=end`（`offset_from` 求长度须同数组），
/// 否则走内联 `buffer`。`luaC_checkGC`/`luaS_newlstr`/`lua_pushlstring` 可分配/GC。cpp/VM/src/laux.cpp:580 luaL_pushresult。
pub(crate) unsafe fn lua_l_pushresult(b: *mut LuaLStrbuf) {
  unsafe {
    let b = &*b;
    let l = b.l;
    let storage = b.storage;

    if !storage.is_null() {
      lua_c_check_gc!(l);

      if b.p == b.end {
        setsvalue!(l, (*l).top.offset(-1), lua_s_buffinish(l, storage));
      } else {
        // TString 载荷声明为 c_char（GC 头布局线格式），字节宽度一致，cast 仅换元素类型
        let storage_data = (*storage).data.as_ptr().cast::<u8>();
        let len = b.p.offset_from(storage_data) as usize;
        // SAFETY: storage_data 指向存活缓冲区且 b.p 落在 data..=end（见上方契约），len 字节界内可读
        setsvalue!(
          l,
          (*l).top.offset(-1),
          lua_s_newlstr(&mut *l, from_raw_parts(storage_data, len))
        );
      }
    } else {
      let buffer = b.buffer.as_ptr();
      let len = b.p.offset_from(buffer) as usize;
      // SAFETY: buffer 为内联区基址且 b.p 落在 buffer..=end，len 字节界内可读
      lua_pushlstring_bytes(l, from_raw_parts(buffer, len));
    }
  }
}
