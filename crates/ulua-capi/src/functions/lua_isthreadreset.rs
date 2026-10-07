//! 本文件对应 `ulua_lua_isthreadreset` 导出符号（源：ulua-vm/src/functions/lua_isthreadreset.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell_l_cint!` 的 `@refshared` 只读引用重建
//! 变体臂一次调用（r16-v36 地基票：本壳曾因 vm 核心前移为 `&LuaState` 只读接收者而退役为显式壳，
//! 见 `lua_status.rs` 先例；该缺位已由 `@refshared` 臂补足，故复归宏模板单源），壳契约见宏模板。
capi_shell_l_cint!(lua_isthreadreset, lua_isthreadreset @refshared);
