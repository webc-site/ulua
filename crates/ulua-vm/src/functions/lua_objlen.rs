use crate::{
  enums::lua_type::LuaType,
  functions::{index_2_addr::index_2_addr, lua_h_getn::lua_h_getn},
  macros::ttype::ttype,
  records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_objlen(l: *mut LuaState, idx: i32) -> i32 {
  unsafe {
    let o: StkId = index_2_addr(l, idx);
    let tt = ttype!(o);

    // tag 判别经 `LuaType::from_c_int`（const fn）取判别式：原 `tt == LuaType::X as u32`
    // 级联比较消除，非上述四型（含 nil/boolean/number 等）仍落 `_ => 0`
    match LuaType::from_c_int(tt as i32) {
      Some(LuaType::String) => (*o).as_string().len as i32,
      Some(LuaType::UserData) => (*o).as_userdata().len as i32,
      Some(LuaType::Buffer) => (*o).as_buffer().len as i32,
      Some(LuaType::Table) => lua_h_getn((*o).as_table_ptr()),
      _ => 0,
    }
  }
}
