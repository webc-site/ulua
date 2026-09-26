use crate::{functions::lua_l_pushresult::lua_l_pushresult, records::lua_l_strbuf::LuaLStrbuf};

/// # Safety
///
/// `b`/`L` 必须相互一致且 `b` 处于 buffinitsize 之后、pushresultsize 之前的有效状态；size 为本次需要的字节数。
pub(crate) unsafe fn lua_l_pushresultsize(b: *mut LuaLStrbuf, size: usize) {
  // Safety: 契约保证 b 缓冲终态串可读且与 `L` 同栈帧，压入后释放临时缓冲不再回写
  unsafe {
    let b_ref = &mut *b;
    b_ref.p = b_ref.p.wrapping_add(size);
    lua_l_pushresult(b);
  }
}
