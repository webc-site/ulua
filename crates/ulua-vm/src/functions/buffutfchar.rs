use crate::{
  functions::lua_o_utf_8_esc::lua_o_utf_8_esc,
  macros::{maxunicode::MAXUNICODE, utf_8_buffsz::UTF8BUFFSZ},
  records::lua_state::LuaState,
};

/// cpp `lutf8lib.cpp:146 buffutfchar`：把 `arg` 栈槽码点 UTF-8 编码进 `buff` 尾部，
/// C++ 的 `charstr`/长度双出参折叠为返回的编码字节切片（借用 `buff`，与 `l` 无关，
/// 显式 `'a` 标注令 `l` 的借用随调用即止，调用方可继续经 `l` 压栈）。
///
/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；
/// 本票收形后取参/校验/编码全经 `check_integer`/`arg_check` 安全门面与纯安全编码函数
/// `lua_o_utf_8_esc`，体内已无裸操作，故本体降为安全 `fn`）：`l` 须处于可抛错受保护帧，
/// `arg` 为其合法栈索引；码点不在 `[0, MAXUNICODE]` 时经 `arg_check` 抛错、不返回。
/// `buff` 为调用方自有的 UTF8BUFFSZ 栈缓冲（对应 cpp 局部 `char buff[UTF8BUFFSZ]`），
/// 写入只回退占用其尾部。cpp lutf8lib.cpp:146。
pub(crate) fn buffutfchar<'a>(
  l: &mut LuaState,
  arg: i32,
  buff: &'a mut [u8; UTF8BUFFSZ],
) -> &'a [u8] {
  let code = l.check_integer(arg);
  l.arg_check((0..=MAXUNICODE).contains(&code), arg, "value out of range");

  let n = lua_o_utf_8_esc(buff, (code as i64) as u32);
  // 编码字节数 n ∈ [1, UTF8BUFFSZ]，尾部切片由下标检查自证界内
  &buff[UTF8BUFFSZ - n as usize..]
}
