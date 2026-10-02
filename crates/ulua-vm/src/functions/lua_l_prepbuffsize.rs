use crate::{functions::extendstrbuf::extendstrbuf, records::lua_l_strbuf::LuaLStrbuf};

/// 确保 `b` 的可写游标后至少还有 `size` 字节余量；不足经 `extendstrbuf` 扩容（GC 分配、
/// 可抛错并重挂 `b.p`），本函数只保留扩容副作用、不返回游标。
///
/// 收形决策（r12-w6c）：cpp `luaL_prepbuffsize` 的「返回当前写游标」在本仓直接消费面
/// 实测为零——四处直接调用点（`add_s`/`addquoted`/`lua_l_addchar` 函数形与宏形）全部
/// 丢弃返回游标、纯取扩容副作用；唯一需要游标的下游 `lua_l_buffinitsize` 改为就地复读
/// `b.p` 获得同值游标。刻意不返回 `&mut [u8]` 余量窗：`extendstrbuf` 走 frealloc 搬移
/// 语义，扩容后旧缓冲指针失效，任何以旧基址构造、跨调用存活的切片窗都是悬垂承诺；
/// 无返回形即无窗，寿命契约随之消解。
///
/// # Safety
///
/// `b` 必须指向已在存活 `lua_State` 上经 `lua_l_buffinit`/`lua_l_buffinitsize` 初始化、
/// 尚未 `pushresult` 的 `LuaLStrbuf`（`p`/`end` 为其缓冲内可写游标）；size 为本次需要的
/// 字节数。cpp laux.cpp:441 `luaL_prepbuffsize`。
pub(crate) unsafe fn lua_l_prepbuffsize(b: &mut LuaLStrbuf, size: usize) {
  // SAFETY: 契约保证 b 的 p/end 为其缓冲界内游标；扩容走 `b as *mut _` 转呈 extendstrbuf
  // （家族 addlstring/addvalue 同形先例：借用窗口在契约边界重建为裸指针、调用即结束），
  // extendstrbuf 扩容后已把新游标同步写回 b.p，本函数不留任何跨调用存活的旧指针引用。
  unsafe {
    let free = b.end.offset_from(b.p) as usize;
    if free < size {
      extendstrbuf(b as *mut _, size - free, -1);
    }
  }
}
