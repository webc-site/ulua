use crate::{functions::lua_l_pushresult::lua_l_pushresult, records::lua_l_strbuf::LuaLStrbuf};

/// cpp `luaL_pushresultsize`（laux.cpp:604）：先把预写游标按本次写入字节数 `size` 提交，
/// 再交 [`lua_l_pushresult`] 终局压栈。
///
/// `b` 以独占引用传入（D 族句柄形收口）：本函数是状态机的消费端，要求对缓冲的独占写权，
/// 与 [`lua_l_pushresult`] 同形；`b.p` 的推进仍为同分配区内的裸地址算式（`p` 字段保留
/// `*mut u8` 的理由见 `records/lua_l_strbuf.rs` 的 DELIBERATE DEVIATION 注记）。
///
/// w6e 诚实降级：随 [`lua_l_pushresult`] 同批降级为安全 fn——本函数体内只剩
/// `wrapping_add` 安全算式与安全调用的终局函数，无裸操作点位。
///
/// 调用序契约（正确性，非内存安全）：
/// `b`/`L` 必须相互一致且 `b` 处于 buffinitsize 之后、pushresultsize 之前的有效状态：
/// `b.p` 落在当前缓冲界内、`b.p + size` 不越已写入区（即 `lua_l_prepbuffsize` 预留的
/// `size` 字节已由调用方写满）；余下前提见 [`lua_l_pushresult`] 契约。
///
/// r12-w4b 消费面实测与「不添加 C 导出垫」的 T9 形裁决同见 [`lua_l_pushresult`] 文档。
pub(crate) fn lua_l_pushresultsize(b: &mut LuaLStrbuf, size: usize) {
  // 游标提交：同分配区内的裸地址算式（`p` 字段保留 `*mut u8` 的理由见 record 注记）
  b.p = b.p.wrapping_add(size);
  lua_l_pushresult(b);
}
