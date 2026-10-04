use crate::{
  functions::{
    lua_l_addchar::lua_l_addchar, lua_l_addlstring::lua_l_addlstring,
    lua_l_addvalue::lua_l_addvalue, lua_l_prepbuffsize::lua_l_prepbuffsize,
    lua_tolstring::lua_tolstring_ref, push_onecapture::push_onecapture,
  },
  macros::{l_esc::L_ESC, lua_l_error::luaL_error},
  records::{lua_l_strbuf::LuaLStrbuf, match_state::MatchState},
};

/// cpp `lstrlib.cpp add_s`（lstrlib.cpp:767）：按替换串模板把 `%m` 捕获展开进累加器。
/// `s`/`e` 为整窗匹配的源偏移对（`%0` 用），偏移==cpp 指针同界。
///
/// w6e 诚实降级：形参已全为引用形（`ms.l` 句柄字段本轮保留裸形，见 `MatchState`
/// 注记），真实裸操作（`lua_tolstring_ref` 转读、缓冲扩容/写入、抛错发散、
/// `lua_l_addvalue` 栈消费）落逐句窄 `unsafe` 块。
///
/// 调用序契约（正确性，非内存安全）：`ms.l` 为存活调用帧（槽 3 为替换串、
/// `lua_l_addvalue` 读栈顶）、`b` 为其上登记的累加器；`s <= e <= ms.src.len()`
/// （`%0` 分支的源窗口由 `MatchState` 门面取切片）。
pub(crate) fn add_s(ms: &mut MatchState, b: &mut LuaLStrbuf, s: usize, e: usize) {
  // SAFETY: 契约保证 `ms.l` 指向存活调用帧，3 号槽恒为替换串；`lua_tolstring_ref`
  // 自身 `# Safety` 转呈——返回借用只在栈下次操作前有效，本次逐字节读出后当场消费；
  // `None` 即 cpp 解引用 NULL 的不可达路径，收敛为空串（不追加任何内容），
  // 不改变正常路径行为
  let repl = unsafe { lua_tolstring_ref(ms.l, 3) }.unwrap_or(&[]);
  // SAFETY: `b` 为同帧登记的累加器，先行预留 repl.len() 字节
  unsafe { lua_l_prepbuffsize(b, repl.len()) };

  // cpp `for (i = 0; i < l; i++)`：`%` 分支吞掉其后的序号/转义字节，正对应迭代器
  // 多取一次 `next`；`%` 为末字节时 cpp 读串尾终止 NUL（值为 0，既非数字也非
  // `L_ESC` → 报错），`next` 取 None 折算成 0 即同点位
  let mut bytes = repl.iter().copied();
  while let Some(c) = bytes.next() {
    if c != L_ESC as u8 {
      // SAFETY: 预留窗内逐字节写游标（prepbuffsize 已按整串长度扩界）
      unsafe { lua_l_addchar(b, c) };
      continue;
    }

    let next = bytes.next().unwrap_or(0); // skip ESC
    if !next.is_ascii_digit() {
      if next != L_ESC as u8 {
        // SAFETY: `ms.l` 存活帧句柄；`lua_l_error_l` raise 后发散
        unsafe {
          luaL_error!(
            ms.l,
            "invalid use of '{}' in replacement string",
            L_ESC as u8 as char
          )
        }
      }
      // SAFETY: 同 addchar 窗——扩容判据失败时其内部先经 prepbuffsize 扩界
      unsafe { lua_l_addchar(b, next) };
    } else if next == b'0' {
      // cpp: lua_addlstring(b, s, e - s); —— %0 整窗，按源偏移切片直接追加
      // SAFETY: src_slice 按契约（s <= e <= src.len()）返回界内切片，追加不越预留窗
      unsafe { lua_l_addlstring(b, ms.src_slice(s, e - s)) };
    } else {
      push_onecapture(ms, (next - b'1') as i32, Some(s), Some(e));
      // SAFETY: 栈顶即上句压入的捕获串（push_onecapture 契约），b 为同帧累加器
      unsafe { lua_l_addvalue(b) }; // add capture to accumulated result
    }
  }
}
