use core::ptr::addr_of_mut;

use crate::{
  functions::{
    lua_c_barrierback::lua_c_barrierback, luau_execute::luau_execute, luau_precall::luau_precall,
  },
  macros::{
    isblack::isblack, lua_callinfo_return::LUA_CALLINFO_RETURN, pcrlua::PCRLUA,
    scheduled_reentry::SCHEDULED_REENTRY,
  },
  records::{gc_object::GCObject, lua_state::LuaState},
  type_aliases::stk_id::StkId,
};

/// # Safety
/// 调用方须保证：`l` 处于可承接抛错的受保护调用入口，`func..(*l).top` 为可读参数窗口
/// （非函数值时 precall 内 tryfuncTM 抛错），`nresults` 与已预留结果槽一致，CallInfo/栈尾有
/// 新帧余量；Lua 分支会置 isactive 并经 gclist 写屏障恢复黑化线程，`preparereentry` 时不进入
/// luau_execute、由外层调度重入。cpp ldo.cpp:257 `performcall`
pub(crate) unsafe fn performcall(
  l: *mut LuaState,
  func: StkId,
  nresults: i32,
  preparereentry: bool,
) {
  // Safety: 契约保证 `l` 存活、func 可读且 nresults 与调用方协议一致，Lua/C 分支的 isactive 与错误处理链保存恢复成对
  unsafe {
    if luau_precall(l, func, nresults) == PCRLUA {
      (*(*l).ci).flags |= LUA_CALLINFO_RETURN as u32;

      let oldactive = (*l).isactive;
      (*l).isactive = true;

      let o = l as *mut GCObject;
      if isblack!(o) {
        lua_c_barrierback(l, o, addr_of_mut!((*l).gclist));
      }

      if preparereentry {
        (*l).status = SCHEDULED_REENTRY as u8;
      } else {
        luau_execute(l);
      }

      if !oldactive {
        (*l).isactive = false;
      }
    }
  }
}
