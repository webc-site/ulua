use crate::{
  functions::{
    b_and::b_and_arm, b_arshift::b_arshift_arm, b_countlz::b_countlz, b_countrz::b_countrz,
    b_extract::b_extract_arm, b_lrot::b_lrot_arm, b_lshift::b_lshift_arm, b_not::b_not,
    b_or::b_or_arm, b_replace::b_replace_arm, b_rrot::b_rrot_arm, b_rshift::b_rshift_arm,
    b_swap::b_swap_arm, b_test::b_test_arm, b_xor::b_xor_arm, lua_l_register::lua_l_register_bytes,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::{lua_l_reg::LuaLReg, lua_state::LuaState},
};

/// `bitlib[]`（cpp/VM/src/lbitlib.cpp 的 Lua 名 → `b_*` `_arm` 边界臂表）：
/// 全静态条目编译期落 `static`（rodata），开库时零栈构造、零写入。
static BITLIB: [LuaLReg; 15] = [
  LuaLReg::new(b"arshift", b_arshift_arm),
  LuaLReg::new(b"band", b_and_arm),
  LuaLReg::new(b"bnot", b_not),
  LuaLReg::new(b"bor", b_or_arm),
  LuaLReg::new(b"bxor", b_xor_arm),
  LuaLReg::new(b"btest", b_test_arm),
  LuaLReg::new(b"extract", b_extract_arm),
  LuaLReg::new(b"lrotate", b_lrot_arm),
  LuaLReg::new(b"lshift", b_lshift_arm),
  LuaLReg::new(b"replace", b_replace_arm),
  LuaLReg::new(b"rrotate", b_rrot_arm),
  LuaLReg::new(b"rshift", b_rshift_arm),
  LuaLReg::new(b"countlz", b_countlz),
  LuaLReg::new(b"countrz", b_countrz),
  LuaLReg::new(b"byteswap", b_swap_arm),
];

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；本票收形后
/// 建库表/注册所经 `lua_l_register_bytes` 已降为安全 `fn`（r12-w6d，裸 C 函数指针与
/// `lua_s_new` 转手屏障下沉被调内部窄块），故本体为安全 `fn`）：`l` 须为可分配、可抛错的
/// 受保护帧且栈顶之上留 1 空槽（`lua_l_register_bytes` push 库表并作为返回值）；
/// `BITLIB` 为编译期静态表，每项 `name` 为静态字节切片。cpp/VM/src/lbitlib.cpp:241 luaopen_bit32。
pub fn luaopen_bit32(l: &mut LuaState) -> i32 {
  // `BITLIB` 为本文件同卫生域生成的合法 `unsafe extern "C-unwind"` 臂静态表，名字为
  // 不含尾部 `\0` 的静态字节切片，满足 `lua_l_register_bytes` 切片契约
  lua_l_register_bytes(l, Some(b"bit32"), &BITLIB);

  1
}

lua_lib_fn!(pub fn luaopen_bit32 @ref, luaopen_bit32_arm);
