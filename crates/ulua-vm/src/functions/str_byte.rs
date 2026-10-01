use crate::{
  functions::{
    lua_l_checklstring::lua_l_checklstring_ref, lua_l_checkstack::lua_l_checkstack,
    lua_l_optinteger::lua_l_optinteger, posrelat::posrelat,
  },
  macros::{lua_l_error::luaL_error, lua_lib_fn::lua_lib_fn, uchar::uchar},
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn str_byte(l: *mut LuaState) -> i32 {
  unsafe {
    // 借用切片形态取源串：len 即切片长度，逐字节读取走切片下标，免指针出参与 from_raw_parts
    let s = lua_l_checklstring_ref(&mut *l, 1);
    let len = s.len();
    let mut posi = posrelat(lua_l_optinteger(&mut *l, 2, 1), len);
    let mut pose = posrelat(lua_l_optinteger(&mut *l, 3, posi), len);

    if posi <= 0 {
      posi = 1;
    }
    if (pose as usize) > len {
      pose = len as i32;
    }

    if posi > pose {
      return 0; // empty interval; return no values
    }

    let n = pose - posi + 1;
    // oracle 同位死守卫（lstrlib.cpp:137 `if (n <= 0)`）：posi/pose 已钳位后
    // 恒假——忠实保留，防 sync-cpp 时漂移
    if posi + n <= pose {
      // overflow?
      luaL_error!(l, "string slice too long");
    }

    lua_l_checkstack(&mut *l, n, "string slice too long");

    // 钳位保证 1 <= posi <= pose <= len，[posi-1, pose) 恒界内
    for &b in &s[(posi - 1) as usize..pose as usize] {
      (*l).push_integer(uchar(b as i32) as i32);
    }

    n
  }
}

lua_lib_fn!(pub fn str_byte, str_byte_arm);
