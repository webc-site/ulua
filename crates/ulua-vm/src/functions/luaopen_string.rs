use crate::{
  functions::{
    createmetatable_lstrlib::createmetatable_mut, gmatch::gmatch, lua_l_register::lua_l_register,
    str_byte::str_byte, str_char::str_char, str_find::str_find, str_format::str_format,
    str_gsub::str_gsub, str_len::str_len, str_lower::str_lower, str_match::str_match,
    str_pack::str_pack, str_packsize::str_packsize, str_rep::str_rep, str_reverse::str_reverse,
    str_split::str_split, str_sub::str_sub, str_unpack::str_unpack, str_upper::str_upper,
  },
  records::{lua_l_reg::LuaLReg, lua_state::LuaState},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn luaopen_string(l: *mut LuaState) -> i32 {
  unsafe {
    // Faithful port of the `strlib[]` registration array in lstrlib.cpp:
    // {name, func} pairs ending in a {NULL, NULL} sentinel; lua_l_register
    // copies each into the `string` table.
    let strlib: [LuaLReg; 17] = [
      LuaLReg::new(b"byte", str_byte),
      LuaLReg::new(b"char", str_char),
      LuaLReg::new(b"find", str_find),
      LuaLReg::new(b"format", str_format),
      LuaLReg::new(b"gmatch", gmatch),
      LuaLReg::new(b"gsub", str_gsub),
      LuaLReg::new(b"len", str_len),
      LuaLReg::new(b"lower", str_lower),
      LuaLReg::new(b"match", str_match),
      LuaLReg::new(b"rep", str_rep),
      LuaLReg::new(b"reverse", str_reverse),
      LuaLReg::new(b"sub", str_sub),
      LuaLReg::new(b"upper", str_upper),
      LuaLReg::new(b"split", str_split),
      LuaLReg::new(b"pack", str_pack),
      LuaLReg::new(b"packsize", str_packsize),
      LuaLReg::new(b"unpack", str_unpack),
    ];

    lua_l_register(l, c"string".as_ptr(), &strlib);
    createmetatable_mut(l);

    1
  }
}
