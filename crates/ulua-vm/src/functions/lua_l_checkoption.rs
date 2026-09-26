use core::{ffi::c_char, ptr::null_mut};

use crate::{
  functions::{
    cstr_bytes, cstr_cow, lua_l_argerror_l::lua_l_argerror_l,
    lua_l_checklstring::lua_l_checklstring, lua_l_optlstring::lua_l_optlstring,
    lua_pushfstring_l::lua_pushfstring_l,
  },
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 必须指向存活 `LuaState`（失配路径经 argerror 抛错不返回）；第 `narg` 栈槽须为可读串值或可取默认；
/// `lst` 须为以 NULL 元素收尾的可读 C 串指针数组（越界读会踩数组尾）；`def` 为空或指向可读 NUL 结尾 C 串。
/// 对应 cpp laux.cpp:99。
pub unsafe fn lua_l_checkoption(
  l: *mut LuaState,
  narg: i32,
  def: *const c_char,
  lst: *const *const c_char,
) -> i32 {
  // Safety: 契约保证 `l` 为存活调用帧、实参 narg 可读且 lst 以 NULL 结尾覆盖 def 索引，失配路径抛错不返回
  unsafe {
    let name: *const c_char = if !def.is_null() {
      lua_l_optlstring(l, narg, def, null_mut())
    } else {
      lua_l_checklstring(l, narg, null_mut())
    };

    // 选项表为 NUL 结尾指针数组：字节序列比较等价 cpp `strcmp == 0`
    let name_bytes = cstr_bytes(name);
    let mut i: i32 = 0;
    loop {
      let opt = *lst.add(i as usize);
      if opt.is_null() {
        break;
      }
      if cstr_bytes(opt) == name_bytes {
        return i;
      }
      i += 1;
    }

    let msg = lua_pushfstring_l(l, format_args!("invalid option '{}'", cstr_cow(name)));
    let msg_str = cstr_cow(msg);
    lua_l_argerror_l(l, narg, msg_str.as_ref())
  }
}
