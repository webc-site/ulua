use core::{ptr::null, slice::from_raw_parts};

use crate::{
  enums::lua_type::LuaType,
  functions::{
    addfield::addfield, lua_l_addlstring::lua_l_addlstring, lua_l_buffinit::lua_l_buffinit,
    lua_l_checktype::lua_l_checktype, lua_l_optinteger::lua_l_optinteger,
    lua_l_optlstring::lua_l_optlstring, lua_l_pushresult::lua_l_pushresult, lua_objlen::lua_objlen,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::{lua_l_strbuf::LuaLStrbuf, lua_state::LuaState},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn tconcat(l: *mut LuaState) -> i32 {
  unsafe {
    let mut lsep: usize = 0;
    let sep = lua_l_optlstring(l, 2, null(), &mut lsep);
    lua_l_checktype(l, 1, LuaType::Table as i32);
    let i = lua_l_optinteger(l, 3, 1);
    let last = lua_objlen(l, 1);
    let last = lua_l_optinteger(l, 4, last);

    let t = (*(*l).base).as_table_ptr();

    let mut b = LuaLStrbuf::new();
    lua_l_buffinit(l, &mut b);
    // 尾元素前的每字段后随分隔符（cpp `while current_i < last` 游走收为区间
    // 迭代）；收尾判定 i <= last 与原循环退出时 `current_i == last` 等价
    //（i > last 时区间为空且两判定同假，字段一个不输出）
    for current_i in i..last {
      addfield(l, &mut b, current_i, t);
      if lsep != 0 {
        lua_l_addlstring(&mut b, from_raw_parts(sep as *const u8, lsep));
      }
    }
    if i <= last {
      addfield(l, &mut b, last, t);
    }
    lua_l_pushresult(&mut b);
    1
  }
}

lua_lib_fn!(pub fn tconcat, tconcat_arm);
