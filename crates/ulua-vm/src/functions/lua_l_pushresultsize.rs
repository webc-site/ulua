use crate::{functions::lua_l_pushresult::lua_l_pushresult, records::lua_l_strbuf::LuaLStrbuf};

/// cpp `luaL_pushresultsize`（laux.cpp:604）：先把预写游标按本次写入字节数 `size` 提交，
/// 再交 [`lua_l_pushresult`] 终局压栈。
///
/// `b` 以独占引用传入（D 族句柄形收口）：本函数是状态机的消费端，要求对缓冲的独占写权，
/// 与 [`lua_l_pushresult`] 同形；`b.p` 的推进仍为同分配区内的裸地址算式（`p` 字段保留
/// `*mut u8` 的理由见 `records/lua_l_strbuf.rs` 的 DELIBERATE DEVIATION 注记）。
///
/// # Safety
///
/// `b`/`L` 必须相互一致且 `b` 处于 buffinitsize 之后、pushresultsize 之前的有效状态：
/// `b.p` 落在当前缓冲界内、`b.p + size` 不越已写入区（即 `lua_l_prepbuffsize` 预留的
/// `size` 字节已由调用方写满）；余下前提见 [`lua_l_pushresult`] 契约。
pub(crate) unsafe fn lua_l_pushresultsize(b: &mut LuaLStrbuf, size: usize) {
  // 游标提交：同分配区内的裸地址算式（`p` 字段保留 `*mut u8` 的理由见 record 注记）
  b.p = b.p.wrapping_add(size);
  // SAFETY: 契约保证 `b` 与 `L` 相互一致、终态串可读，压入后释放临时缓冲不再回写
  unsafe { lua_l_pushresult(b) };
}
