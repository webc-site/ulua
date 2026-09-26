use crate::{functions::str_find_aux::str_find_aux, macros::lua_lib_arm::lua_lib_arm};

lua_lib_arm! {
  /// # Safety
  /// `l` 须为存活 `lua_State` 且处于受保护帧：直接转调 `str_find_aux(l,0)`，其对索引 1(串)/2(模式) 做
  /// `luaL_checklstring`（缺失/非串抛错回退）、可选索引 3 起始位，并可抛错与触发 GC；结果压栈需 `(*l).top`
  /// 后留有空槽。契约与 `str_find_aux` 一致。
  /// cpp VM/src/lstrlib.cpp:725
  pub fn str_match(l) { unsafe { str_find_aux(l, 0) } }
}
