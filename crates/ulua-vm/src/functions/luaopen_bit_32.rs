use crate::{
  functions::{
    b_and::b_and_arm, b_arshift::b_arshift, b_countlz::b_countlz, b_countrz::b_countrz,
    b_extract::b_extract, b_lrot::b_lrot, b_lshift::b_lshift, b_not::b_not, b_or::b_or_arm,
    b_replace::b_replace, b_rrot::b_rrot, b_rshift::b_rshift, b_swap::b_swap, b_test::b_test,
    b_xor::b_xor, lua_l_register::lua_l_register,
  },
  records::{lua_l_reg::LuaLReg, lua_state::LuaState},
};

/// # Safety
/// `l` 须为存活 LuaState 且栈顶之上留 1 空槽（`lua_l_register` push 库表并作为返回值），处于可分配/GC 的受保护帧；
/// `bitlib` 是本地栈数组，含 NUL 名终止项，`.as_ptr()` 在 `lua_l_register` 调用期内存活。cpp/VM/src/lbitlib.cpp:241 luaopen_bit32。
pub unsafe extern "C-unwind" fn luaopen_bit32(l: *mut LuaState) -> i32 {
  unsafe {
    // Faithful port of bitlib[] in lbitlib.cpp (Lua name -> b_* function).
    let bitlib: [LuaLReg; 15] = [
      LuaLReg::new(b"arshift", b_arshift),
      LuaLReg::new(b"band", b_and_arm),
      LuaLReg::new(b"bnot", b_not),
      LuaLReg::new(b"bor", b_or_arm),
      LuaLReg::new(b"bxor", b_xor),
      LuaLReg::new(b"btest", b_test),
      LuaLReg::new(b"extract", b_extract),
      LuaLReg::new(b"lrotate", b_lrot),
      LuaLReg::new(b"lshift", b_lshift),
      LuaLReg::new(b"replace", b_replace),
      LuaLReg::new(b"rrotate", b_rrot),
      LuaLReg::new(b"rshift", b_rshift),
      LuaLReg::new(b"countlz", b_countlz),
      LuaLReg::new(b"countrz", b_countrz),
      LuaLReg::new(b"byteswap", b_swap),
    ];

    lua_l_register(l, c"bit32".as_ptr(), &bitlib);

    1
  }
}
