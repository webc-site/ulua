use core::ffi::c_void;

use crate::{
  enums::tms::TMS,
  functions::lua_h_getstr::lua_h_getstr,
  macros::{
    lightuserdatatag::lightuserdatatag, lua_lutag_limit::LUA_LUTAG_LIMIT, ttype::ttype,
    utag_proxy::UTAG_PROXY,
  },
  records::{lua_state::LuaState, t_string::tstring},
  type_aliases::t_value::TValue,
};

/// # Safety
/// 调用方须保证：`l` 存活且 global 的 ttname/mt/tmname/lightuserdataname 数组已按类型标签初始化、
/// `o` 须为存活 `TValue` 的共享只读借用（本函数只读其 tag 与 gc/指针位，体内 `lua_h_getstr`
/// 为只读表查找、不搬 Lua 栈；userdata 的 metatable/`name` 读取沿 cpp 契约）；返回存活 TString，
/// 其有效性止于后续 GC 或 lightuserdataname 被改写。cpp ltm.cpp:140 `luaT_objtypenamestr`
pub(crate) unsafe fn lua_t_objtypenamestr(l: *mut LuaState, o: &TValue) -> *const tstring {
  unsafe {
    // Userdata created by the environment can have a custom type name set in the individual metatable
    // If there is no custom name, 'userdata' is returned
    if o.is_userdata() {
      let u = o.as_userdata_ptr();
      let mt = (*u).metatable;
      if (*u).tag as i32 != UTAG_PROXY && !mt.is_null() {
        // B2-2a 任务B：同借用窗口内即时读判定——Option<Slot> 原生收口，
        // miss(None)/非串值均如原 sentinel-nil 路径落空继续下行
        if let Some(type_) = lua_h_getstr(&*mt, (*l).gs_ref().tmname[TMS::TmType as usize])
          && type_.get().is_string()
        {
          return type_.get().as_string_ptr();
        }

        return (*l).gs_ref().ttname[ttype!(o) as usize];
      }
    }

    // Tagged lightuserdata can be named using lua_setlightuserdataname
    if o.is_lightuserdata() {
      let tag = lightuserdatatag!(o);

      if (tag as u32) < LUA_LUTAG_LIMIT as u32 {
        let name = (*l).gs_ref().lightuserdataname[tag as usize];
        if !name.is_null() {
          return name;
        }
      }
    }

    // For all types except userdata and table, a global metatable can be set with a global name override
    let mt = (*l).gs_ref().mt[ttype!(o) as usize];
    if !mt.is_null() {
      // 同上：同窗口即时读判定，Option<Slot> 原生收口
      if let Some(type_) = lua_h_getstr(&*mt, (*l).gs_ref().tmname[TMS::TmType as usize])
        && type_.get().is_string()
      {
        return type_.get().as_string_ptr();
      }
    }

    (*l).gs_ref().ttname[ttype!(o) as usize]
  }
}

/// # Safety
/// 与 [`lua_t_objtypenamestr`] 同契约：`l` 存活、global 名称数组完整、`o` 为可读 TValue；
/// 返回的 TString 指针以 `c_void` 宽化，有效性随 GC 结束。cpp ltm.cpp:140
pub unsafe extern "C-unwind" fn lua_t_objtypenamestr_export(
  l: *mut LuaState,
  o: *const TValue,
) -> *const c_void {
  // SAFETY: 导出壳原样转发同契约 `lua_t_objtypenamestr`，裸指针在此转为共享只读借用
  // （契约要求 `o` 非空存活），返回 C 字符串指针按 c_void 宽化
  unsafe { lua_t_objtypenamestr(l, &*o).cast() }
}
