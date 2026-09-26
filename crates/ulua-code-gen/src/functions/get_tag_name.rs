use core::ffi::c_int;

use ulua_vm::enums::lua_type::LuaType;

/// cpp `IrDump` 的 tag 名查表（"tnil"/"tboolean"...）。
/// 判别式与名字的单一真相在 [`LuaType::tag_name`]。
pub(crate) fn get_tag_name(tag: u8) -> &'static str {
  match LuaType::from_c_int(tag as c_int) {
    Some(ty) => ty.tag_name(),
    None => unreachable!("Unknown type tag"),
  }
}
