use core::ffi::c_void;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_status::LuaStatus,
  functions::lua_d_pcall::lua_d_pcall,
  macros::{savestack::savestack, setnilvalue::setnilvalue},
  records::{lua_state::LuaState, resolve_import::ResolveImport},
  type_aliases::t_value::TValue,
};

/// # Safety
/// `l` 必须指向存活 `LuaState`，操作数 `*const TValue`/`StkId` 指针可读/可写且对齐，TM 调用协议（res 槽、栈余量）满足。
pub(crate) unsafe fn resolve_import_safe(l: *mut LuaState, k: *mut TValue, id: u32) {
  // SAFETY: 契约保证 `l` 为存活调用帧且对应 pc 的导入空间已建立，解析出的模块表压入栈顶且表可读
  unsafe {
    let mut ri = ResolveImport { k, id };

    if (*(*l).gt).safeenv != 0 {
      let old_top = (*l).get_top();
      let status = lua_d_pcall(
        l,
        Some(ResolveImport::run),
        &mut ri as *mut _ as *mut c_void,
        savestack!(l, (*l).top),
        0,
      );

      LUAU_ASSERT!(old_top + 1 == (*l).get_top());

      if status != LuaStatus::Ok as i32 {
        // r12-w9b 收编：恢复点（lua_d_pcall）后错误位写 nil 的裸偏移读数改经
        // `top_slot(-1)` 边界原语——原语即调用现读 `self.top`，与替代前
        // `(*l).top.sub(1)` 同位点现读，跨搬栈不预绑定（correctstack 悬窗教训）
        setnilvalue!((*l).top_slot(-1));
      }
    } else {
      // r12-w7a2 收编（同形单点·protected 路径 else 分支）：写 nil 到保留顶槽后经
      // raise_top 原语抬顶——setnilvalue 不触场不搬栈，与被替代的裸抬顶式同址同宽；
      // cpp 同形 `setnilvalue(s2v(L->top++));`
      setnilvalue!((*l).top);
      (*l).raise_top(1);
    }
  }
}
