use crate::{
  functions::{
    createmetatable_lstrlib::createmetatable_mut, gmatch::gmatch_arm,
    lua_l_register::lua_l_register_bytes, str_byte::str_byte_arm, str_char::str_char_arm,
    str_find::str_find, str_format::str_format_arm, str_gsub::str_gsub_arm, str_len::str_len_arm,
    str_lower::str_lower_arm, str_match::str_match, str_pack::str_pack_arm,
    str_packsize::str_packsize_arm, str_rep::str_rep_arm, str_reverse::str_reverse_arm,
    str_split::str_split_arm, str_sub::str_sub_arm, str_unpack::str_unpack_arm,
    str_upper::str_upper_arm,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::{lua_l_reg::LuaLReg, lua_state::LuaState},
};

/// `strlib[]`（cpp/VM/src/lstrlib.cpp 的 Lua 名 → `str_*` `_arm` 边界臂表）：
/// 全静态条目编译期落 `static`（rodata），开库时零栈构造、零写入。
static STRLIB: [LuaLReg; 17] = [
  LuaLReg::new(b"byte", str_byte_arm),
  LuaLReg::new(b"char", str_char_arm),
  LuaLReg::new(b"find", str_find),
  LuaLReg::new(b"format", str_format_arm),
  LuaLReg::new(b"gmatch", gmatch_arm),
  LuaLReg::new(b"gsub", str_gsub_arm),
  LuaLReg::new(b"len", str_len_arm),
  LuaLReg::new(b"lower", str_lower_arm),
  LuaLReg::new(b"match", str_match),
  LuaLReg::new(b"rep", str_rep_arm),
  LuaLReg::new(b"reverse", str_reverse_arm),
  LuaLReg::new(b"sub", str_sub_arm),
  LuaLReg::new(b"upper", str_upper_arm),
  LuaLReg::new(b"split", str_split_arm),
  LuaLReg::new(b"pack", str_pack_arm),
  LuaLReg::new(b"packsize", str_packsize_arm),
  LuaLReg::new(b"unpack", str_unpack_arm),
];

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn luaopen_string(l: *mut LuaState) -> i32 {
  unsafe {
    lua_l_register_bytes(l, Some(b"string"), &STRLIB);
    createmetatable_mut(l);

    1
  }
}

lua_lib_fn!(pub fn luaopen_string, luaopen_string_arm);
