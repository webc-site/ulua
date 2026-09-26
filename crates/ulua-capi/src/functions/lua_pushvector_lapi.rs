//! 本文件由 `crates/ulua-capi/tools/gen_capi.py` 自动生成（源：ulua-vm/src/functions/lua_pushvector_lapi.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell_push!` 的一次调用，壳契约见宏模板。
capi_shell_push!(
  lua_pushvector_lapi,
  lua_pushvector_lua_state_f32_f32_f32_f32,
  x: f32,
  y: f32,
  z: f32,
  w: f32
);
capi_shell_push!(lua_pushvector_lapi, lua_pushvector_lua_state_f32_f32_f32, x: f32, y: f32, z: f32);
