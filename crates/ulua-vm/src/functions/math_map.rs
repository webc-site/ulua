use crate::{macros::lua_lib_fn::lua_lib_fn, records::lua_state::LuaState};

/// # Safety
/// `l` 须为存活 LuaState 并处于本 C 函数受保护帧：栈 1..=5 号位逐个经 `lua_l_checknumber` 强制为数字
/// （非数字抛错），结果经 `lua_pushnumber` 写回并可分配/GC。cpp/VM/src/lmathlib.cpp:425 math_map。
pub unsafe fn math_map(l: *mut LuaState) -> i32 {
  unsafe {
    let x = (*l).check_number(1);
    let inmin = (*l).check_number(2);
    let inmax = (*l).check_number(3);
    let outmin = (*l).check_number(4);
    let outmax = (*l).check_number(5);

    let result = outmin + (x - inmin) * (outmax - outmin) / (inmax - inmin);
    (*l).push_number(result);
    1
  }
}

lua_lib_fn!(pub fn math_map, math_map_arm);
