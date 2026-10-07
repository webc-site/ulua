use crate::{
  enums::tms::TMS,
  functions::lua_typename::TYPENAMES_STR,
  macros::{lua_s_fix::luaS_fix, lua_s_new::lua_s_new},
  records::lua_state::LuaState,
};

/// 元表方法名，索引即 `TMS` 变体序号（cpp `luaT_init` 的 eventnames）。
const EVENTNAMES: [&[u8]; 21] = [
  b"__index",
  b"__newindex",
  b"__mode",
  b"__namecall",
  b"__call",
  b"__iter",
  b"__len",
  b"__eq",
  b"__add",
  b"__sub",
  b"__mul",
  b"__div",
  b"__idiv",
  b"__mod",
  b"__pow",
  b"__unm",
  b"__lt",
  b"__le",
  b"__concat",
  b"__type",
  b"__metatable",
];

// 名字表与枚举必须同步，否则 tmname 会漏填或多填
const _: () = assert!(EVENTNAMES.len() == TMS::TmN as usize);

/// cpp `luaT_init`：把类型名与元表方法名 intern 成固定字符串（不参与 GC）。
///
/// # Safety
/// `l` 须为处于 open 阶段、可分配/GC 的存活 LuaState，`(*l).global.ttname`/`tmname` 数组分别覆盖 `TYPENAMES_STR.len()`
/// 与 `TMS::TmN`（编译期常量断言二者一致，见 `EVENTNAMES` 的 `const _`），`lua_s_new` intern 后 `luaS_fix` 置永久；
/// 仅在 VM 初始化（尚无并发）调用。cpp/VM/src/ltm.cpp:80 luaT_init。
pub(crate) unsafe fn lua_t_init(l: *mut LuaState) {
  unsafe {
    // r16-b3 收编：原 `addr_of_mut!` 槽裸指针跨 `lua_s_new`（分配/可 GC）持有，
    // 拆为 cpp `ltm.cpp` 同序形态——先 intern 得 `ts`，槽写经 gs_mut 一句一借，
    // `luaS_fix!` 就地消费同一 `ts` 值（与原 `*slot` 读数恒等）
    // 类型名表与 `lua_typename` 共用同一常量，索引即 LUA_T*
    for (i, &name) in TYPENAMES_STR.iter().enumerate() {
      let ts = lua_s_new(l, name.as_bytes());
      (*l).gs_mut().ttname[i] = ts;
      luaS_fix!(ts);
    }

    for (i, &name) in EVENTNAMES.iter().enumerate() {
      let ts = lua_s_new(l, name);
      (*l).gs_mut().tmname[i] = ts;
      luaS_fix!(ts);
    }
  }
}
