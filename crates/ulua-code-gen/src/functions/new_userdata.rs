use ulua_common::macros::luau_assert::LUAU_ASSERT;
use ulua_vm::{
  functions::lua_u_newudata::lua_u_newudata,
  macros::isblack::isblack,
  records::{gc_object::GCObject, lua_state::LuaState, udata::Udata},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn new_userdata(l: *mut LuaState, s: usize, tag: i32) -> *mut Udata {
  // Safety: l 为活 LuaState, (*l).global 活; lua_u_newudata 成功返回非空活 Udata 否则抛错。
  // udatamt 按 tag 索引, tag 由调用方约束在 [0, utags) 内, 故 udatamt[tag] 在数组界内;
  // h 非空时 (*u).metatable=h 写入的是刚分配的活对象。以下窄块统一援引本契约。
  let u = unsafe { lua_u_newudata(l, s, tag) };

  // Safety: 依上契约——(*l).global 活、udatamt[tag] 界内。
  let h = unsafe { (*(*l).global).udatamt[tag as usize] };
  if !h.is_null() {
    // 目前总是分配未标记对象，因此可跳过 forward barrier
    // Safety: 依上契约——u 为刚分配活对象，isblack! 只读其 GC 头。
    unsafe { LUAU_ASSERT!(!isblack!(u as *mut GCObject)) };

    // Safety: 依上契约——写入刚分配活对象的 metatable 槽。
    unsafe { (*u).metatable = h };
  }

  u
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn new_userdata_export(
  l: *mut LuaState,
  s: usize,
  tag: i32,
) -> *mut Udata {
  // Safety: 导出 C ABI 入口原样转发 l/s/tag 给同契约 unsafe fn new_userdata; 调用方按 ABI
  // 保证 l 为活 LuaState 且 tag∈[0,utags), 满足被调前置条件。
  unsafe { new_userdata(l, s, tag) }
}
