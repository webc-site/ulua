use core::{ffi::c_void, mem::transmute};

use crate::{
  functions::lua_h_getstr::lua_h_getstr,
  macros::{
    lightuserdatatag::lightuserdatatag, lua_lutag_limit::LUA_LUTAG_LIMIT, tsvalue::tsvalue,
    ttislightuserdata::ttislightuserdata, ttisstring::ttisstring, ttisuserdata::ttisuserdata,
    ttype::ttype, utag_proxy::UTAG_PROXY, uvalue::uvalue,
  },
  records::lua_table::LuaTable,
  type_aliases::{lua_state::lua_State, t_string::tstring, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn lua_t_objtypenamestr(l: *mut lua_State, o: *const TValue) -> *const tstring {
  unsafe {
    // Userdata created by the environment can have a custom type name set in the individual metatable
    // If there is no custom name, 'userdata' is returned
    if ttisuserdata!(o) && uvalue!(o).tag as i32 != UTAG_PROXY && !uvalue!(o).metatable.is_null() {
      // TM_TYPE is index 19 in the tag method names array (TMS enum)
      let func: unsafe fn(*mut LuaTable, *mut tstring) -> *const TValue =
        transmute(lua_h_getstr as *const c_void);

      let type_ = func(uvalue!(o).metatable, (*(*l).global).tmname[19]);

      if ttisstring!(type_) {
        return tsvalue!(type_);
      }

      return (*(*l).global).ttname[ttype!(o) as usize];
    }

    // Tagged lightuserdata can be named using lua_setlightuserdataname
    if ttislightuserdata!(o) {
      let tag = lightuserdatatag!(o);

      if (tag as u32) < LUA_LUTAG_LIMIT as u32 {
        let name = (*(*l).global).lightuserdataname[tag as usize];
        if !name.is_null() {
          return name;
        }
      }
    }

    // For all types except userdata and table, a global metatable can be set with a global name override
    let mt = (*(*l).global).mt[ttype!(o) as usize];
    if !mt.is_null() {
      // TM_TYPE is index 19
      let func: unsafe fn(*mut LuaTable, *mut tstring) -> *const TValue =
        transmute(lua_h_getstr as *const c_void);

      let type_ = func(mt, (*(*l).global).tmname[19]);

      if ttisstring!(type_) {
        return tsvalue!(type_);
      }
    }

    (*(*l).global).ttname[ttype!(o) as usize]
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_luaT_objtypenamestr")]
pub unsafe extern "C-unwind" fn lua_t_objtypenamestr_export(
  l: *mut lua_State,
  o: *const TValue,
) -> *const c_void {
  unsafe { lua_t_objtypenamestr(l, o).cast() }
}
