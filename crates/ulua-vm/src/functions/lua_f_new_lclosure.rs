use core::{ffi::c_void, ptr::null_mut};

use crate::{
  enums::lua_type::LuaType,
  functions::{c_slice_mut, lua_m_newgco::lua_m_newgco},
  macros::{lua_c_init::luaC_init, setnilvalue::setnilvalue, size_lclosure::size_lclosure},
  records::{closure::Closure, lua_state::LuaState, lua_table::LuaTable, proto::Proto},
};

/// # Safety
/// `l` 须指向存活 `LuaState`（cpp lfunc.cpp:68）；`p` 须为存活 `Proto`（读 `maxstacksize` 并
/// 存入 `inner.l.p`）；upvalue 数组按 `nelems` 分配并逐槽置 nil，`nelems` 须与实际槽数一致；
/// `e` 仅按指针存入 env、不解引用。
pub unsafe fn lua_f_new_lclosure(
  l: *mut LuaState,
  nelems: i32,
  e: *mut LuaTable,
  p: *mut Proto,
) -> *mut Closure {
  // Safety: 契约保证 `l` 存活、e 为存活环境表、p 为存活 Proto；upvalue 数组按 p->sizeupvalues 分配并与拷贝一一对应
  unsafe {
    let c = lua_m_newgco(l, size_lclosure(nelems as usize), (*l).activememcat) as *mut Closure;

    luaC_init!(l, c, LuaType::Function as i32);
    let cl = &mut *c;
    cl.is_c = 0;
    cl.env = e;
    cl.nupvalues = nelems as u8;
    cl.stacksize = (*p).maxstacksize;
    cl.preload = 0;
    cl.gclist = null_mut();

    let lc = &mut cl.inner.l;
    lc.p = p;

    // cpp lfunc.cpp:46 逐 upvalue 槽置 nil
    for slot in c_slice_mut(lc.uprefs.as_mut_ptr(), nelems as usize) {
      setnilvalue!(slot);
    }

    c
  }
}

/// # Safety
/// C ABI 导出壳，逐参数透传：`l/e/p` 还原为 `LuaState/LuaTable/Proto` 后须满足
/// [`lua_f_new_lclosure`] 的全部契约（e/p 非空且类型正确由 C 侧注册方保证）。
pub unsafe extern "C-unwind" fn lua_f_new_lclosure_export(
  l: *mut LuaState,
  nelems: i32,
  e: *mut c_void,
  p: *mut c_void,
) -> *mut c_void {
  // Safety: 导出壳将 c_void 指针还原为 LuaTable/Proto 后原样转发同契约的 `lua_f_new_lclosure`
  unsafe { lua_f_new_lclosure(l, nelems, e as *mut LuaTable, p as *mut Proto).cast() }
}
