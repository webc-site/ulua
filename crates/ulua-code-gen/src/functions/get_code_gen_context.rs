use core::ptr::null_mut;

use ulua_vm::records::lua_state::LuaState;

use crate::records::base_code_gen_context::BaseCodeGenContext;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[inline]
pub unsafe fn get_code_gen_context(l: *mut LuaState) -> *mut BaseCodeGenContext {
  // Safety: 镜像 cpp `static_cast<BaseCodeGenContext*>(l->global->ecb.context)`——依 `# Safety`
  // 契约，l 存活且 ecb.context 为 null 或合法 BaseCodeGenContext*；此处逐层判空并做裸指针
  // 转手，全程不解引用可疑指针。以下窄块统一援引本契约。
  if l.is_null() {
    return null_mut();
  }

  let global = unsafe { (*l).global };
  if global.is_null() {
    return null_mut();
  }

  unsafe { (*global).ecb.context as *mut BaseCodeGenContext }
}
