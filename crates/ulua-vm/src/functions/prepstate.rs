use crate::{
  macros::luai_maxccalls::LUAI_MAXCCALLS,
  records::{lua_state::LuaState, match_state::MatchState},
};

/// cpp `lstrlib.cpp prepstate`（lstrlib.cpp:648）：登记本次匹配的源串/pattern 与深度。
/// 游标协议为偏移制：`ms.src`/`ms.pat` 为实参串 payload 的借用切片，串尾哨兵由
/// `src.len()`/`pat.len()` 表达（原 `src_end`/`p_end` 指针域已消灭）。
///
/// 安全签名：`MatchState` 不再持有裸指针，唯一的 `unsafe` 留在调用点把 C-API
/// `(const char*, size_t)` 收口为切片的入口处（见 `str_find_aux`/`str_gsub`/`gmatch_aux`）。
pub(crate) fn prepstate<'a>(ms: &mut MatchState<'a>, l: *mut LuaState, s: &'a [u8], p: &'a [u8]) {
  ms.l = l;
  ms.matchdepth = LUAI_MAXCCALLS;
  ms.src = s;
  ms.pat = p;
}
