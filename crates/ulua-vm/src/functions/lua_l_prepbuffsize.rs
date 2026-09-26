use core::ffi::c_char;

use crate::{functions::extendstrbuf::extendstrbuf, records::lua_l_strbuf::LuaLStrbuf};

/// # Safety
///
/// `b`/`L` 必须相互一致且 `b` 处于 buffinitsize 之后、pushresultsize 之前的有效状态；size 为本次需要的字节数。
pub(crate) unsafe fn lua_l_prepbuffsize(b: *mut LuaLStrbuf, size: usize) -> *mut c_char {
  // Safety: 契约保证 `b` 与 `L` 相互一致，扩容后 b->buffer 与暂存区域指针同步更新，旧指针不再解引用
  unsafe {
    let b_ref = &*b;
    let free = b_ref.end.offset_from(b_ref.p) as usize;
    if free < size {
      extendstrbuf(b, size - free, -1)
    } else {
      b_ref.p
    }
  }
}
