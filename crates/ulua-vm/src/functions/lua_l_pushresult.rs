use crate::{
  functions::{
    c_slice, lua_pushlstring::lua_pushlstring_bytes, lua_s_buffinish::lua_s_buffinish,
    lua_s_newlstr::lua_s_newlstr,
  },
  macros::{lua_c_check_gc::lua_c_check_gc, setsvalue::setsvalue},
  records::lua_l_strbuf::LuaLStrbuf,
};

/// LBuf 状态机终局（D 族句柄直读 + 已写窗切片形）：`b.l`/`b.storage` 两个
/// `Option<NonNull>` 句柄就地消费——宿主态由句柄取址、溢出缓冲的有/无由 `Some`/`None`
/// 分支表达，不再折回 `null_mut()` 哨兵后 `is_null()` 判空；已写字节一律收口为
/// `&[u8]` 切片窗交下游（`lua_s_newlstr`/`lua_pushlstring_bytes` 的切片入口）。
///
/// 分支与 cpp `laux.cpp:580` 逐点对应：
/// - 已换入 GC 缓冲：`luaC_checkGC` → 游标恰抵 `end` 时 `luaS_buffinish` 就地收尾
///   （零拷贝），否则按 `[data, p)` 窗新建串；
/// - 未溢出：按内联 `buffer` 的 `[buffer, p)` 窗走 push 族切片入口。
///
/// 结果落点均为预留结果槽 `(*l).top - 1`（与 cpp `L->top - 1` 同形）。
///
/// w6e 诚实降级：`b` 已是 `&mut LuaLStrbuf` 引用形参，`b.l`/`b.storage` 为
/// `Option<NonNull>` 句柄字段（record 自持不变量），真实裸操作全部收在下方两处逐句
/// 窄 `unsafe` 块内（review.md §2；判例同 `lua_pushlstring_bytes`）。
///
/// 调用序契约（正确性，非内存安全）：`b` 须处于 `lua_l_buffinit`/`lua_l_buffinitsize`
/// 之后、`pushresult` 之前的有效状态：`b.l` 为 `Some` 且指向处于可 GC 受保护帧的
/// lua_State（未接线误用暴露为 `expect` 级契约违约 panic，相对旧形 null 解引用的 UB
/// 是严格改善面）、`(*l).top - 1` 为预留结果槽；游标 `b.p` 落在当前缓冲（内联
/// `buffer` 或 `storage` 数据区）同一分配内，故 `offset_from` 求长度合法。
/// `lua_c_check_gc`/`lua_s_newlstr`/`lua_pushlstring_bytes` 可分配、可 GC。
/// cpp/VM/src/laux.cpp:580 luaL_pushresult。
///
/// r12-w4b 消费面实测（T9 形裁决）：本函数与 `lua_l_pushresultsize` 同为 `pub(crate)`，
/// 消费点 10 处全在 ulua-vm 的 strlib/os 族——pushresult：`os_date`/`str_format`/
/// `utfchar`/`str_gsub`/`tconcat`/`lua_l_traceback`/`str_pack`，pushresultsize：
/// `str_rep`/`str_char`/`str_shared`；`ulua-capi` 导出表与 ulua-rt/ulua-conformance/tests
/// 面实测零消费（符号未导出，跨 crate 不可见），故本轮**不添加 C 导出垫**（§7 零死代码）。
/// 若将来按 cpp `luaL_pushresult` 开 `ulua_lua_l_pushresult` 导出，循 `lua_l_buffinit`
/// 的显式壳先例在 capi 侧一行折形（`&mut *b`）即可。
pub(crate) fn lua_l_pushresult(b: &mut LuaLStrbuf) {
  // 句柄直读：`Option::expect`/`NonNull::as_ptr` 均安全，无 null 折回
  let l = b
    .l
    .expect("lua_l_pushresult: b.l 由 buffinit 接线，契约保证非空")
    .as_ptr();

  match b.storage {
    // 已换入 GC 溢出缓冲
    Some(storage) => {
      // SAFETY: 契约保证 `l` 为存活帧且 `(*l).top - 1` 为可写结果槽；`storage` 句柄指向
      // buffinit/extendstrbuf 登记的存活 TString，其 `data` 区起至游标 `b.p` 为已写窗
      // （同数组内 `offset_from` 合法）
      unsafe {
        lua_c_check_gc!(l);

        // 结果槽经 `top_slot(-1)` 读数原语预绑定：绑定位置即原 setsvalue 实参序
        // 中目的槽的求值位（buffinish/newlstr 只分配 TString、不挪栈不写栈顶，
        // strbuf 窄腰的 realloc 重读时序在 extendstrbuf 侧自持，无跨 realloc 悬窗）
        let res = (*l).top_slot(-1);
        if b.p == b.end {
          // 恰写满：GC 缓冲就地收尾，免整块复制
          setsvalue!(l, res, lua_s_buffinish(l, storage.as_ptr()));
        } else {
          // TString 载荷声明为 c_char（GC 头布局线格式），字节宽度一致，cast 仅换元素类型
          let data = (*storage.as_ptr()).data.as_ptr().cast::<u8>();
          // 已写窗（切片形）：`[data, p)`
          let written = c_slice(data, b.p.offset_from(data) as usize);
          setsvalue!(l, res, lua_s_newlstr(&mut *l, written));
        }
      }
    }
    // 未溢出：仍写内联 `buffer`（cpp `B->storage == nullptr`）
    None => {
      // SAFETY: 契约保证 `b.p` 落在内联 `buffer` 区内（同数组 `offset_from` 合法），
      // `[buffer, p)` 窗可读；`l` 为存活帧
      unsafe {
        let base = b.buffer.as_ptr();
        lua_pushlstring_bytes(&mut *l, c_slice(base, b.p.offset_from(base) as usize));
      }
    }
  }
}
