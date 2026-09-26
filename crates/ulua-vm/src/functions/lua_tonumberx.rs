use crate::{
  enums::value_view::ValueView,
  functions::{cstr_bytes, index_2_addr::index_2_addr, lua_o_str_2_d::lua_o_str_2_d},
  macros::getstr::getstr,
  records::{lua_state::LuaState, t_string::tstring},
  type_aliases::t_value::TValue,
};

/// cpp `lua_tonumberx`（`VM/src/lapi.cpp:414`）：`idx` 槽位可作数值时返回该数值。
///
/// cpp 的 `int* isnum` 出参与临时 `TValue n` 收口为 `Option<f64>` 返回值：`None`
/// 即 cpp 的 `*isnum = 0` 失败路径（其返回值 0 只是占位，调用方只消费标志）。
/// 整条 `tonumber!` + `nvalue!` 宏链收敛为 [`ValueView`] match：
/// - `Number` 臂直接带出 f64（对应 `ttisnumber` 快路径 + `nvalue!` 读）；
/// - `String` 臂走 `lua_o_str_2_d`（对应 `luaV_tonumber` 的 `ttisstring` +
///   `luaO_str2d` 解析，NaN/inf 字面量按 C `strtod` 语义放行，与 cpp 一致）；
/// - `Integer`（本 fork 扩展的 i64 tag）不隐式转 Number，与 cpp
///   `luaV_tonumber`（`lvmutils.cpp:25`）只认 Number/String 一致；
/// - 其余 tag 为失败路径。
///
/// # Safety
///
/// `l` 必须是正在执行的 C 函数帧的存活 `LuaState`，`idx` 为其合法栈索引（越界
/// 读错槽甚至悬垂 TValue）；`ValueView::from_tvalue` 仅读栈槽值，不触发 GC 也不
/// 分配。cpp lapi.cpp:414。
pub unsafe fn lua_tonumberx(l: *mut LuaState, idx: i32) -> Option<f64> {
  // Safety: 契约保证 `index_2_addr` 返回可读、对齐的 TValue 槽（越界为可读的
  // LUA_O_NILOBJECT）；字符串臂的 `getstr`/`cstr_bytes` 前提（NUL 结尾完整 payload）
  // 与既有 `lua_v_tonumber` 宏链相同。
  unsafe {
    match ValueView::from_tvalue(&*(index_2_addr(l, idx) as *const TValue)) {
      ValueView::Number(n) => Some(n),
      ValueView::String(ts) => lua_o_str_2_d(cstr_bytes(getstr(ts as *const tstring))),
      ValueView::Integer(_)
      | ValueView::Nil
      | ValueView::Boolean(_)
      | ValueView::Vector(_)
      | ValueView::Table(_)
      | ValueView::Function(_)
      | ValueView::Thread(_)
      | ValueView::Userdata(_)
      | ValueView::LightUserdata { .. }
      | ValueView::IteratorDone
      | ValueView::Buffer(_)
      | ValueView::Class(_)
      | ValueView::Object(_)
      | ValueView::UpVal(_)
      | ValueView::Other(_) => None,
    }
  }
}
