use core::{
  ffi::c_int,
  mem::size_of,
  ptr::{null, null_mut},
};

use crate::{
  enums::lua_type::LuaType, functions::lua_m_newgco::luaM_newgco_, macros::lua_c_init::luaC_init,
  records::proto::Proto, type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_f_newproto(l: *mut lua_State) -> *mut Proto {
  unsafe {
    let f = luaM_newgco_(l, size_of::<Proto>(), (*l).activememcat) as *mut Proto;

    luaC_init!(l, f, LuaType::Proto as c_int);

    (*f).nups = 0;
    (*f).numparams = 0;
    (*f).is_vararg = 0;
    (*f).maxstacksize = 0;
    (*f).flags = 0;

    (*f).k = null_mut();
    (*f).code = null_mut();
    (*f).p = null_mut();
    (*f).codeentry = null();

    (*f).execdata = null_mut();
    (*f).exectarget = 0;

    (*f).lineinfo = null_mut();
    (*f).abslineinfo = null_mut();
    (*f).locvars = null_mut();
    (*f).upvalues = null_mut();
    (*f).source = null_mut();

    (*f).debugname = null_mut();
    (*f).debuginsn = null_mut();

    (*f).typeinfo = null_mut();
    (*f).userdata = null_mut();
    (*f).gclist = null_mut();

    (*f).sizecode = 0;
    (*f).sizep = 0;
    (*f).sizelocvars = 0;
    (*f).sizeupvalues = 0;
    (*f).sizek = 0;
    (*f).sizelineinfo = 0;
    (*f).linegaplog2 = 0;
    (*f).linedefined = 0;
    (*f).bytecodeid = 0;
    (*f).sizetypeinfo = 0;

    (*f).feedbackvec = null_mut();
    (*f).feedbackvecsize = 0;
    (*f).funid = 0;

    f
  }
}

pub use lua_f_newproto as luaF_newproto;
