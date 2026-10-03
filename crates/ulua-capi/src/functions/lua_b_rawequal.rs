//! 本文件对应 `ulua_lua_b_rawequal` 导出符号（源：ulua-vm/src/functions/lua_b_rawequal.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell_l_cint!` 的 `@ref` 引用重建变体臂一次调用
//! （r16-v32 地基回迁：本壳曾因 vm 核心前移为 `&mut LuaState` 接收者而退役为显式壳，见
//! `lua_status.rs` 先例；该缺位已由 `@ref` 臂补足，故复归宏模板单源），壳契约见宏模板。
capi_shell_l_cint!(lua_b_rawequal, lua_b_rawequal @ref);
