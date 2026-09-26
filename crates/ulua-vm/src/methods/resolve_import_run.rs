use core::ffi::c_void;

use crate::{
  functions::lua_v_getimport::lua_v_getimport,
  macros::{lua_d_checkstack::luaD_checkstack, setnilvalue::setnilvalue},
  records::{lua_state::LuaState, resolve_import::ResolveImport},
};

impl ResolveImport {
  /// # Safety
  ///
  /// 仅作为 `lua_d_pcall` 的 `Pfunc` 中转回调使用：`l` 必须指向当前执行协程的存活
  /// `LuaState`；`ud` 必须为空或指向调用期间全程存活的 `ResolveImport` 实例地址。
  pub(crate) unsafe extern "C-unwind" fn run(l: *mut LuaState, ud: *mut c_void) {
    // Safety: 契约保证 ud 为空或活实例地址；checkstack 预留 1 槽后才写 (*l).top 并做表查找
    unsafe {
      // ud 由调用方以本类型实例地址回填；判空一次后按引用读取字段
      let Some(self_) = (ud as *mut ResolveImport).as_ref() else {
        return;
      };

      // note: we call getimport with nil propagation which means that accesses to table chains like A.B.C will resolve in nil
      // this is technically not necessary but it reduces the number of exceptions when loading scripts that rely on getfenv/setfenv for global
      // injection
      // allocate a stack slot so that we can do table lookups
      luaD_checkstack!(l, 1);
      setnilvalue!((*l).top);
      (*l).top = (*l).top.add(1);

      lua_v_getimport(
        l,
        (*l).gt,
        self_.k,
        (*l).top.sub(1),
        self_.id,
        true, /* propagatenil= */
      );
    }
  }
}
