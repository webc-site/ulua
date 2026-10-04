//! 本文件对应 `ulua_coresumefinish` 导出符号（源：ulua-vm/src/functions/coresumefinish.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的 `l refstate` 独占引用重建参数形一次
//! 调用（本票把 vm 核心 `coresumefinish` 收形为 `&mut LuaState` 接收者，`r` 仍按值 `c_int` 透传，
//! 导出签名逐字不变：首参裸 `*mut LuaState`、返回 `c_int`）。壳契约见宏模板。
capi_shell!(coresumefinish, "ulua_coresumefinish", coresumefinish, [
  l refstate,
  r val c_int,
  => c_int,
]);
