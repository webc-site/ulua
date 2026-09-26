//! 本文件由 `crates/ulua-capi/tools/gen_capi.py` 自动生成（源：ulua-vm/src/functions/lua_pushboolean.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell_push!` 的一次调用，壳契约见宏模板。
capi_shell_push!(lua_pushboolean, lua_pushboolean, b: c_int);
