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

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；本票收形后
/// 建库表/注册与建 metatable 仍经 `lua_l_register_bytes`/`createmetatable_mut` 两处不安全被调而
/// 落窄块（被调方自身保留 `# Safety`：裸 C 函数指针与 `lua_s_new` 转手未清零），故本体降为安全
/// `fn`）：`l` 须为可分配、可抛错的受保护帧且栈顶之上留足空槽；`createmetatable_mut` 要求调用前
/// 栈顶即字符串库表（相对 -2）。cpp/VM/src/lstrlib.cpp:1746 luaopen_string。
pub fn luaopen_string(l: &mut LuaState) -> i32 {
  // SAFETY: `STRLIB` 为本文件同卫生域生成的合法 `unsafe extern "C-unwind"` 臂静态表，
  // 名字为不含尾部 `\0` 的静态字节切片，满足 `lua_l_register_bytes` 切片契约
  unsafe { lua_l_register_bytes(l, Some(b"string"), &STRLIB) };

  // SAFETY: `l.as_mut_ptr()` 为当前独占借用重建的裸句柄，借用窗止于本次调用；
  // 上一步已把字符串库表置于栈顶，满足 `createmetatable_mut` 的 -2 栈位前提
  unsafe { createmetatable_mut(l.as_mut_ptr()) };

  1
}

lua_lib_fn!(pub fn luaopen_string @ref, luaopen_string_arm);
