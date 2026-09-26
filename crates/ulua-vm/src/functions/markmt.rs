use crate::{
  enums::lua_type::LUA_T_COUNT, macros::markobject::markobject, records::global_state::global_State,
};

/// # Safety
/// 调用方须保证：`g` 为存活 global_State 且处于 GC mark 阶段，`mt[0..LUA_T_COUNT)` 中每个指针
/// 要么为 null、要么指向未被回收的存活 LuaTable；否则 markobject! 对悬垂对象写灰白标签即 UB。
/// cpp lgc.cpp:890 `markmt`
pub(crate) unsafe fn markmt(g: *mut global_State) {
  // Safety: 契约保证 `g` 指向存活 global_State，mtisfrozen 遍历仅触及 bt 数组类型界内的元表指针
  unsafe {
    for mt in (*g).mt.iter_mut().take(LUA_T_COUNT as usize) {
      if !(*mt).is_null() {
        markobject!(g, *mt);
      }
    }
  }
}
