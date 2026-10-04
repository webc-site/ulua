use crate::{
  functions::{
    coclose::coclose_arm, cocreate::cocreate_arm, coresumecont::coresumecont_arm,
    coresumey::coresumey_arm, corunning::corunning_arm, costatus::costatus_arm, cowrap::cowrap_arm,
    coyield::coyield_arm, coyieldable::coyieldable_arm, cstr, lua_l_register::lua_l_register_bytes,
    lua_pushcclosurek::lua_pushcclosurek_ref,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::{lua_l_reg::LuaLReg, lua_state::LuaState},
};

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；本票把首参收形为
/// 引用形后，注册/压闭包/落字段全经 `lua_l_register_bytes`/`lua_pushcclosurek_ref`/`set_field_bytes`
/// 门面，仅 `lua_l_register_bytes` 一处不安全被调（裸 C 臂静态表转手）而落窄块，故本体降为安全
/// `fn`）：`l` 须为可分配、可抛错的受保护帧，`CO_FUNCS` 为本文件静态的合法 C 臂表；
/// `set_field_bytes(-2, ...)` 要求 resume 闭包已压栈、coroutine 模块表在 -2。
/// cpp/VM/src/lcorolib.cpp `luaopen_coroutine`。
pub(crate) fn luaopen_coroutine(l: &mut LuaState) -> i32 {
  // SAFETY: `CO_FUNCS` 为本文件同卫生域生成的合法 `unsafe extern "C-unwind"` 臂静态表，
  // 名字为不含尾部 `\0` 的静态字节切片，满足 `lua_l_register_bytes` 切片契约
  unsafe { lua_l_register_bytes(l, Some(b"coroutine"), &CO_FUNCS) };

  lua_pushcclosurek_ref(
    l,
    Some(coresumey_arm),
    cstr(b"resume\0"),
    0,
    Some(coresumecont_arm),
  );
  l.set_field_bytes(-2, b"resume");

  1
}

lua_lib_fn!(pub(crate) fn luaopen_coroutine @ref, luaopen_coroutine_arm);
static CO_FUNCS: [LuaLReg; 7] = [
  LuaLReg::new(b"create", cocreate_arm),
  LuaLReg::new(b"running", corunning_arm),
  LuaLReg::new(b"status", costatus_arm),
  LuaLReg::new(b"wrap", cowrap_arm),
  LuaLReg::new(b"yield", coyield_arm),
  LuaLReg::new(b"isyieldable", coyieldable_arm),
  LuaLReg::new(b"close", coclose_arm),
];
