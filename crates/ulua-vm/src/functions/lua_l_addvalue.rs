use core::ptr::copy_nonoverlapping;

use crate::{
  functions::{extendstrbuf::extendstrbuf, lua_tolstring::lua_tolstring_ref},
  macros::lua_pop::lua_pop,
  records::lua_l_strbuf::LuaLStrbuf,
};

/// # Safety
///
/// `b` 必须指向已在存活 `lua_State` 上初始化、尚未 `pushresult` 的 `LuaLStrbuf`，且其
/// 栈顶槽为可转串的值的槽位：`lua_tolstring_ref` 就地转换栈顶（可分配、可触发 GC），成功后
/// 扩容走 `extendstrbuf` 的 boxloc `-2`（串框须落在栈顶值之下）并弹出该临时串；转换失败
/// （`None`，非串非数值）则不追加也不弹栈，与 cpp 一致。cpp laux.cpp:508 `luaL_addvalue`。
pub(crate) unsafe fn lua_l_addvalue(b: &mut LuaLStrbuf) {
  // Safety: 契约保证 b.l 栈顶可转换、b 的 p/end 为界内游标；扩容后 copy/pop 仅触及新缓冲与栈顶
  unsafe {
    let l = b.l;

    if let Some(s) = lua_tolstring_ref(l, -1) {
      let vl = s.len();
      let free = b.end.offset_from(b.p) as usize;
      if free < vl {
        extendstrbuf(b as *mut _, vl - free, -2);
      }

      // GC 不移动对象且串仍被栈槽钉住，`extendstrbuf` 后切片读取仍有效（同旧指针形态）
      copy_nonoverlapping(s.as_ptr(), b.p as *mut u8, vl);
      b.p = b.p.add(vl);

      lua_pop(l, 1);
    }
  }
}
