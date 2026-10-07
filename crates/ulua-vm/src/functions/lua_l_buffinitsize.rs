use crate::{
  functions::{lua_l_buffinit::lua_l_buffinit, lua_l_prepbuffsize::lua_l_prepbuffsize},
  records::{lua_l_strbuf::LuaLStrbuf, lua_state::LuaState},
};

/// 调用序契约（正确性，非内存安全——`l`/`b` 的存活/独占前提已由 `&mut` 接收者类型承载）：
/// size 为本次需要的字节数；`b` 须是本次调用新构造、或处于 buffinit 之后、pushresultsize
/// 之前的缓冲。返回的裸游标 `*mut u8` 指向 `b` 的缓冲体（内部缓冲或 prep 扩容后的堆块），
/// prep 扩容副作用后 `b.p` 已同步；调用方须自行保证以其构造的切片长度不超过本次申请量。
pub(crate) fn lua_l_buffinitsize(l: &mut LuaState, b: &mut LuaLStrbuf, size: usize) -> *mut u8 {
  lua_l_buffinit(l, b);
  // SAFETY: 契约保证 size 经溢出防护，块内分配的缓冲挂回栈值并同步 top 引用；
  // prep 扩容走 frealloc 搬移后已把新游标同步写回 `b.p`（extendstrbuf 尾行返回的即该字段值），
  // 故扩容副作用后就地复读 `b.p` 与旧「prep 返回游标」逐位同值，不留跨扩容存活的旧指针窗。
  unsafe { lua_l_prepbuffsize(b, size) };
  b.p
}
