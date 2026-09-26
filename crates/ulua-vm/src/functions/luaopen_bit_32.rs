use crate::{
  functions::{
    b_and::b_and_arm, b_arshift::b_arshift_arm, b_countlz::b_countlz, b_countrz::b_countrz,
    b_extract::b_extract_arm, b_lrot::b_lrot_arm, b_lshift::b_lshift_arm, b_not::b_not,
    b_or::b_or_arm, b_replace::b_replace_arm, b_rrot::b_rrot_arm, b_rshift::b_rshift_arm,
    b_swap::b_swap_arm, b_test::b_test_arm, b_xor::b_xor_arm, lua_l_register::lua_l_register,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::{lua_l_reg::LuaLReg, lua_state::LuaState},
};

/// # Safety
/// `l` 须为存活 LuaState 且栈顶之上留 1 空槽（`lua_l_register` push 库表并作为返回值），处于可分配/GC 的受保护帧；
/// `bitlib` 是本地栈数组，含 NUL 名终止项，`.as_ptr()` 在 `lua_l_register` 调用期内存活。cpp/VM/src/lbitlib.cpp:241 luaopen_bit32。
pub unsafe fn luaopen_bit32(l: *mut LuaState) -> i32 {
  unsafe {
    // Faithful port of bitlib[] in lbitlib.cpp (Lua name -> b_* `_arm` 边界臂).
    let bitlib: [LuaLReg; 15] = [
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

    lua_l_register(l, c"bit32".as_ptr(), &bitlib);

    1
  }
}

lua_lib_fn!(pub fn luaopen_bit32, luaopen_bit32_arm);
