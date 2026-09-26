use crate::{
  enums::{lua_type::LuaType, tms::TMS},
  functions::lua_h_getstr::lua_h_getstr,
  macros::{lua_o_nilobject::LUA_O_NILOBJECT, objectvalue::objectvalue, ttype::ttype},
  records::{lua_state::LuaState, lua_table::LuaTable},
  type_aliases::t_value::TValue,
};

/// # Safety
/// 调用方须保证：`l` 存活且 global 的 mt/tmname 数组完整、`o` 指向可读 TValue（其类型标签决定
/// 解引用 hvalue/uvalue/objectvalue 分支）、`event` 为 tmname 界内合法 TMS。返回元表字段或
/// luaO_nilobject（永不为 null），有效性止于该元表被修改或 GC。cpp ltm.cpp:112 `luaT_gettmbyobj`
pub(crate) unsafe fn lua_t_gettmbyobj(
  l: *mut LuaState,
  o: *const TValue,
  event: TMS,
) -> *const TValue {
  // Safety: 契约保证 `l` 存活且 `(*l).global` 的 mt/tmname 数组完整、`o` 指向可读 TValue
  // （tag 与 payload 一致，故 hvalue!/uvalue!/objectvalue! 各按分支解引用）
  unsafe {
    /*
      NB: Tag-methods were replaced by meta-methods in Lua 5.0, but the
      old names are still around (this function, for example).
    */

    // tag 判别经 `LuaType::from_c_int`（const fn）取判别式，与 dumpobj/enumobj 同轨：
    // 原 `t if t == LuaType::X as u32` 魔法数 guard 链消除，且 `ttype!` 只读一次
    // （tag 读出与分支判定间无写入，两次读同值）
    let tag = ttype!(o);
    let mt: *mut LuaTable = match LuaType::from_c_int(tag as i32) {
      Some(LuaType::Table) => (*(*o).as_table_ptr()).metatable,
      Some(LuaType::UserData) => (*(*o).as_userdata_ptr()).metatable,
      Some(LuaType::Object) => (*(*objectvalue!(o)).lclass).instancemetatable,
      _ => (*(*l).global).mt[tag as usize],
    };

    if !mt.is_null() {
      lua_h_getstr(mt, (*(*l).global).tmname[event as usize])
    } else {
      LUA_O_NILOBJECT
    }
  }
}
