use crate::{
  functions::{
    lua_l_checklstring::lua_l_checklstring_ref, lua_l_optinteger::lua_l_optinteger,
    lua_pushlstring::lua_pushlstring_bytes, posrelat::posrelat,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn str_sub(l: *mut LuaState) -> i32 {
  unsafe {
    // 借用切片形态取源串：出参 len 由切片长度承接，后续按下标取子串，免指针游走
    let s = lua_l_checklstring_ref(&mut *l, 1);
    let len = s.len();
    let mut start = posrelat((*l).check_integer(2), len);
    let mut end = posrelat(lua_l_optinteger(&mut *l, 3, -1), len);

    if start < 1 {
      start = 1;
    }
    if end > len as i32 {
      end = len as i32;
    }

    if start <= end {
      // 上方钳位保证 1 <= start <= end <= len，切片区间恒界内
      lua_pushlstring_bytes(l, &s[(start - 1) as usize..end as usize]);
    } else {
      (*l).push_bytes(b"");
    }
    1
  }
}

lua_lib_fn!(pub fn str_sub, str_sub_arm);
