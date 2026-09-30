use ulua_vm::records::lua_state::LuaState;

use crate::records::base_code_gen_context::BaseCodeGenContext;

/// 读取挂在 `l->global->ecb.context` 上的 code-gen 上下文；无上下文时返回 [`None`]
/// （镜像 cpp `getCodeGenContext` 的 null 早退，Rust 侧以 `Option` 取代 null 哨兵）。
///
/// # Safety
/// - `l` 须为 null 或指向存活 `LuaState`，且其 `global` 为 null 或指向存活 `global_State`。
/// - `ecb.context` 依注册契约（`luau_codegen_create`/`create_shared_code_gen_context`）
///   为 null 或指向该 state 独占/共享的存活 `BaseCodeGenContext`。
/// - 返回的可变借用仅在本回调串行窗口内独占消费（VM 执行回调不重入同一上下文的
///   其它可变使用者）；所有权回收点（`on_close_state` 的 `Box::from_raw`）须在借用
///   结束后另行以裸指针配对，不得对返回值做跨窗口缓存。
#[inline]
pub unsafe fn get_code_gen_context<'a>(l: *mut LuaState) -> Option<&'a mut BaseCodeGenContext> {
  if l.is_null() {
    return None;
  }

  // Safety: 依 `# Safety`——l 判空后指向存活 LuaState，其 global 为 null 或活对象；此处仅读字段指针。
  let global = unsafe { (*l).global };
  if global.is_null() {
    return None;
  }

  let ctx = unsafe { (*global).ecb.context }.cast::<BaseCodeGenContext>();
  // Safety: 依 `# Safety`——ecb.context 为 null 或合法 BaseCodeGenContext*；
  // `as_mut` 把 null 映射为 None，其余按注册契约在串行窗口内重建唯一可变借用。
  unsafe { ctx.as_mut() }
}
