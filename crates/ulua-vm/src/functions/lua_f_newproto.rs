use core::{
  mem::size_of,
  ptr::{null, null_mut},
};

use crate::{
  enums::lua_type::LuaType,
  functions::lua_m_newgco::lua_m_newgco,
  macros::lua_c_init::luaC_init,
  records::{lua_state::LuaState, proto::Proto},
};

/// # Safety
/// `l` 须为存活 LuaState 且 `(*l).activememcat`/全局分配器有效，处于可分配/GC 的受保护帧（`lua_m_newgco` 可能触发 GC 或 OOM）；
/// 返回的新 Proto 已被 `luaC_init` 挂入 GC 且各数组指针/size 字段自洽归零，仅在 `l` 的 GC 接住前对其独占写入。
/// cpp/VM/src/lfunc.cpp:12 luaF_newproto。
pub unsafe fn lua_f_newproto(l: *mut LuaState) -> *mut Proto {
  unsafe {
    let proto = lua_m_newgco(l, size_of::<Proto>(), (*l).activememcat) as *mut Proto;

    luaC_init!(l, proto, LuaType::Proto as i32);

    let f = &mut *proto;
    f.nups = 0;
    f.numparams = 0;
    f.is_vararg = 0;
    f.maxstacksize = 0;
    f.flags = 0;

    f.k = null_mut();
    f.code = null_mut();
    f.p = null_mut();
    f.codeentry = null();

    f.execdata = null_mut();
    f.exectarget = 0;

    f.lineinfo = null_mut();
    f.abslineinfo = null_mut();
    f.locvars = null_mut();
    f.upvalues = null_mut();
    f.source = null_mut();

    f.debugname = null_mut();
    f.debuginsn = null_mut();

    f.typeinfo = null_mut();
    f.userdata = null_mut();
    f.gclist = null_mut();

    f.sizecode = 0;
    f.sizep = 0;
    f.sizelocvars = 0;
    f.sizeupvalues = 0;
    f.sizek = 0;
    f.sizelineinfo = 0;
    f.linegaplog2 = 0;
    f.linedefined = 0;
    f.bytecodeid = 0;
    f.sizetypeinfo = 0;

    f.feedbackvec = null_mut();
    f.feedbackvecsize = 0;
    f.funid = 0;
    f.cost = 0;

    proto
  }
}
