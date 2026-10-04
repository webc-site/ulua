//! JIT 专用 setmetatable 赋值快速通道（本 fork 扩展，cpp 无对应）。
//!
//! `LOP_FASTCALL2(LBF_SETMETATABLE)` 的生成码内联类型守卫后直调本函数完成剩余
//! 语义，免去 `call_fallback → 解释器重派发 → luaB_setmetatable → lua_pushlstring
//! intern` 的全套开销（oop 复测归因：call_fallback ≈19% + 字符串驻留 ≈6%）。
//!
//! 语义对齐（与 `lua_b_setmetatable` + lapi `lua_setmetatable` 逐位一致）：
//! - 前置不满足（obj 非 table、mt 非 nil/table、已有元表带 `__metatable`）→
//!   返回 0，由生成码跳 fallback 块交回解释器路径，错误消息与抛出时机保真；
//! - readonly 表 → `check_writable` 原地抛错（lapi 同款时序，错误不打折）；
//! - 成功 → 写 `metatable` 字段 + `lua_c_objbarrier`（可 GC，仅 mt 非空时，
//!   lapi 同款）。
//!
//! 无栈协议：obj/mt 直接以调用帧栈槽指针传入，不触碰 `l.top`（调用方 FASTCALL
//! 约定已把实参落在栈槽，栈布局由生成码维护）。
//!
//! Source: `VM/src/lbaselib.cpp:100` + `VM/src/lapi.cpp:1067`（hand-ported 组合）

use core::ptr::null_mut;

use crate::{
  functions::{
    lua_g_readonlyerror::check_writable, lua_h_getstr::lua_h_getstr, lua_s_newlstr::lua_s_newlstr,
  },
  macros::lua_c_objbarrier::lua_c_objbarrier,
  records::{lua_state::LuaState, lua_table::LuaTable},
  type_aliases::t_value::TValue,
};

/// `__metatable` 保护键字节串（intern 查找键，与 `lua_b_setmetatable` 同字面量）。
const PROTECTED_METAFIELD: &[u8] = b"__metatable";

/// JIT 快速通道主体。返回 1 = 赋值已完成；0 = 前置不满足，须走解释器 fallback。
///
/// # Safety
/// `l` 须为存活 `LuaState`（受保护帧，允许体内 intern 分配触发 GC 步进）；`obj`/`mt`
/// 须指向调用帧内存活的栈槽 `TValue`（本函数全程只读二者、不搬移 Lua 栈、不重入
/// 解释器，槽地址在调用窗口内稳定）。`obj` 非 table 或 `mt` 非 nil/table 时仅返回 0，
/// 不触碰任何 VM 状态。
pub unsafe extern "C-unwind" fn lua_codegen_setmetatable_export(
  l: *mut LuaState,
  obj: *const TValue,
  mt: *const TValue,
) -> i32 {
  // SAFETY: 上述契约；体内检查失败即早退，任何写面都在 obj=table + mt=nil/table 确认之后。
  unsafe {
    if !(*obj).is_table() {
      return 0; // check_type(1, Table)：fallback 路径抛「table expected」
    }
    let mt_is_nil = (*mt).is_nil();
    if !mt_is_nil && !(*mt).is_table() {
      return 0; // arg_expected(2, "nil or table")：fallback 路径抛原错误
    }

    let h = (*obj).as_table_ptr();

    // __metatable 保护检查（luaL_getmetafield 语义的免栈形态）：仅当已有元表才可能
    // 命中——原始哈希查找（不经 tmcache，与 getmetafield 口径一致）。命中即返回 0，
    // 由解释器路径抛「cannot change a protected metatable」。
    if !(*h).metatable.is_null() {
      let key = lua_s_newlstr(&mut *l, PROTECTED_METAFIELD);
      let protected = lua_h_getstr(&*(*h).metatable, key);
      if let Some(slot) = protected
        && !slot.get().is_nil()
      {
        return 0;
      }
    }

    // readonly 表：lapi 同款原地抛错（luaG_readonlyerror），不走 fallback。
    check_writable(l, h);

    // 赋值段（lapi lua_setmetatable 的 Table 分支逐位同形：字段写 + objbarrier）
    let mt_ptr: *mut LuaTable = if mt_is_nil {
      null_mut()
    } else {
      (*mt).as_table_ptr()
    };
    (*h).metatable = mt_ptr;
    if !mt_ptr.is_null() {
      lua_c_objbarrier!(l, h, mt_ptr);
    }

    1
  }
}
