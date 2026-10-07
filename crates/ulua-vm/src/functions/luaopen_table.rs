//! Source: `VM/src/ltablib.cpp:596-628` (hand-ported)

use crate::{
  functions::{
    foreach::foreach_arm, foreachi::foreachi_arm, getn::getn_arm,
    lua_l_register::lua_l_register_bytes, maxn::maxn_arm, tclear::tclear_arm, tclone::tclone_arm,
    tconcat::tconcat_arm, tcreate::tcreate_arm, tfind::tfind_arm, tfreeze::tfreeze_arm,
    tinsert::tinsert_arm, tisfrozen::tisfrozen_arm, tmove::tmove_arm, tpack::tpack_arm,
    tremove::tremove_arm, tsort::tsort_arm, tunpack::tunpack_arm,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::{lua_l_reg::LuaLReg, lua_state::LuaState},
};

static TAB_FUNCS: [LuaLReg; 17] = [
  LuaLReg::new(b"concat", tconcat_arm),
  LuaLReg::new(b"foreach", foreach_arm),
  LuaLReg::new(b"foreachi", foreachi_arm),
  LuaLReg::new(b"getn", getn_arm),
  LuaLReg::new(b"maxn", maxn_arm),
  LuaLReg::new(b"insert", tinsert_arm),
  LuaLReg::new(b"remove", tremove_arm),
  LuaLReg::new(b"sort", tsort_arm),
  LuaLReg::new(b"pack", tpack_arm),
  LuaLReg::new(b"unpack", tunpack_arm),
  LuaLReg::new(b"move", tmove_arm),
  LuaLReg::new(b"create", tcreate_arm),
  LuaLReg::new(b"find", tfind_arm),
  LuaLReg::new(b"clear", tclear_arm),
  LuaLReg::new(b"freeze", tfreeze_arm),
  LuaLReg::new(b"isfrozen", tisfrozen_arm),
  LuaLReg::new(b"clone", tclone_arm),
];

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；本票收形后
/// 建库表/注册所经 `lua_l_register_bytes` 已降为安全 `fn`（r12-w6d，其窄块下沉承担裸 C
/// 函数指针转手），push cfunction 仍经 `push_c_function` 不安全被调而落
/// 窄块（被调方自身保留 `# Safety`），故本体降为安全
/// `fn`）：`l` 须为可分配、可抛错的受保护帧且栈顶之上留足空槽（`lua_l_register_bytes` push 库表；
/// push cfunction 后 `set_global_bytes` 消费之）。cpp/VM/src/ltablib.cpp:694 luaopen_table。
pub fn luaopen_table(l: &mut LuaState) -> i32 {
  // `TAB_FUNCS` 为本文件同卫生域生成的合法 `unsafe extern "C-unwind"` 臂静态表，名字为
  // 不含尾部 `\0` 的静态字节切片，满足 `lua_l_register_bytes` 切片契约
  lua_l_register_bytes(l, Some(b"table"), &TAB_FUNCS);

  // `tunpack_arm` 为本文件静态表的合法 C 臂（与 TAB_FUNCS 同一注册面）；debugname 经
  // `push_c_function` intern 收口当场复制，静态切片仅调用期借用
  l.push_c_function(Some(tunpack_arm), Some(b"unpack"));
  l.set_global_bytes(b"unpack");

  1
}

lua_lib_fn!(pub fn luaopen_table @ref, luaopen_table_arm);
