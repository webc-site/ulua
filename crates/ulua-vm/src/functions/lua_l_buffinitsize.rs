use crate::{
  functions::{lua_l_buffinit::lua_l_buffinit, lua_l_prepbuffsize::lua_l_prepbuffsize},
  records::{lua_l_strbuf::LuaLStrbuf, lua_state::LuaState},
};

/// # Safety
/// `b`/`L` 必须相互一致且 `b` 处于 buffinitsize 之后、pushresultsize 之前的有效状态；size 为本次需要的字节数。
pub(crate) unsafe fn lua_l_buffinitsize(
  l: *mut LuaState,
  b: *mut LuaLStrbuf,
  size: usize,
) -> *mut u8 {
  // SAFETY: 契约保证 `L`/`b` 一致且 size 经溢出防护，块内分配的缓冲挂回栈值并同步 L->top 引用；
  // prep 扩容走 frealloc 搬移后已把新游标同步写回 `(*b).p`（extendstrbuf 尾行返回的即该字段值），
  // 故扩容副作用后就地复读 `(*b).p` 与旧「prep 返回游标」逐位同值，不留跨扩容存活的旧指针窗。
  unsafe {
    lua_l_buffinit(&mut *l, &mut *b);
    lua_l_prepbuffsize(&mut *b, size);
    (*b).p
  }
}
