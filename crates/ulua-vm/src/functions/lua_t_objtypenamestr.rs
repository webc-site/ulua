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

/// 调用方须保证（safe fn 文档断言，由调用方承载）：`l` 存活且 global 的 ttname/mt/tmname/lightuserdataname
/// 数组已按类型标签初始化、`o` 须为存活 `TValue` 的共享只读借用（本函数只读其 tag 与 gc/指针位，体内
/// `lua_h_getstr` 为只读表查找、不搬 Lua 栈；userdata 的 metatable/`name` 读取沿 cpp 契约）；返回存活
/// TString，其有效性止于后续 GC 或 lightuserdataname 被改写。cpp ltm.cpp:140 `luaT_objtypenamestr`
///
/// r19-w4 收形并降 safe：首参 `*mut LuaState` → `&LuaState`——本函数全程经 `gs_ref` 只读 global 名称表，
/// 从不写穿 `l`；`o` 本就是 `&TValue`。收形后签名不再有调用方传入的裸指针参数，依 §2 假合规防线判例
/// （dev `SubtypingEnvironment::get_mapped_type_bounds` 降 safe；本 crate `lua_tothread` 同款 safe 体含
/// unsafe 读块）由 `unsafe fn` 降为 `fn`。体内对 `o` payload/`(*u).metatable`/`(*mt)`/`gs_ref()` 名称表的
/// 解引用皆源自 `l`/`o` 自有字段而非调用方入参，包于单一 `unsafe` 块并附契约。
pub(crate) fn lua_t_objtypenamestr(l: &LuaState, o: &TValue) -> *const tstring {
  // SAFETY: `o` 为存活 TValue、`l` 存活；`(*u).metatable`、`lua_h_getstr` 表读与 `gs_ref()` 名称表
  // 读数均源自 `l`/`o` 自有字段而非调用方入参，全程只读、不写穿 `l`。
  unsafe {
    // Userdata created by the environment can have a custom type name set in the individual metatable
    // If there is no custom name, 'userdata' is returned
    if o.is_userdata() {
      let u = o.as_userdata_ptr();
      let mt = (*u).metatable;
      if (*u).tag as i32 != UTAG_PROXY && !mt.is_null() {
        // B2-2a 任务B：同借用窗口内即时读判定——Option<Slot> 原生收口，
        // miss(None)/非串值均如原 sentinel-nil 路径落空继续下行
        if let Some(type_) = lua_h_getstr(&*mt, l.gs_ref().tmname[TMS::TmType as usize])
          && type_.get().is_string()
        {
          return type_.get().as_string_ptr();
        }

        return l.gs_ref().ttname[ttype!(o) as usize];
      }
    }

    // Tagged lightuserdata can be named using lua_setlightuserdataname
    if o.is_lightuserdata() {
      let tag = lightuserdatatag!(o);

      if (tag as u32) < LUA_LUTAG_LIMIT as u32 {
        let name = l.gs_ref().lightuserdataname[tag as usize];
        if !name.is_null() {
          return name;
        }
      }
    }

    // For all types except userdata and table, a global metatable can be set with a global name override
    let mt = l.gs_ref().mt[ttype!(o) as usize];
    if !mt.is_null() {
      // 同上：同窗口即时读判定，Option<Slot> 原生收口
      if let Some(type_) = lua_h_getstr(&*mt, l.gs_ref().tmname[TMS::TmType as usize])
        && type_.get().is_string()
      {
        return type_.get().as_string_ptr();
      }
    }

    l.gs_ref().ttname[ttype!(o) as usize]
  }
}

/// # Safety
/// 与 [`lua_t_objtypenamestr`] 同契约：`l` 存活、global 名称数组完整、`o` 非空且指向存活 TValue；
/// 返回的 TString 指针以 `c_void` 宽化，有效性随 GC 结束。cpp ltm.cpp:140
pub unsafe extern "C-unwind" fn lua_t_objtypenamestr_export(
  l: *mut LuaState,
  o: *const TValue,
) -> *const c_void {
  // SAFETY: 导出壳为 FFI 边界，将调用方裸指针转为共享只读借用后原样转发同契约的
  // `lua_t_objtypenamestr`，返回 C 字符串指针按 c_void 宽化（契约要求 `l`/`o` 非空存活）
  unsafe { lua_t_objtypenamestr(&*l, &*o).cast() }
}
