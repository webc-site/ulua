use core::ffi::c_void;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  macros::{black_2_gray::black2gray, isblack::isblack, isdead::isdead},
  records::{gc_object::GCObject, lua_state::LuaState},
};

/// # Safety
/// `l` 须存活且 GC 处于增量阶段（`gcstate != GCSpause`）；`o` 必须是当前黑色存活对象；
/// `gclist` 必须指向 `o` 自身的 gclist 链接字段（可写）。违反（如对灰对象调用、gclist 指向他处）
/// 会污染 `grayagain` 链，导致活对象被提前清扫。cpp lgc.cpp:1473。
pub unsafe fn lua_c_barrierback(l: *mut LuaState, o: *mut GCObject, gclist: *mut *mut GCObject) {
  unsafe {
    let g = (*l).global;

    LUAU_ASSERT!(isblack!(o) && !isdead!(g, o));
    LUAU_ASSERT!((*g).gcstate as i32 != 0); // GCSpause 为 0

    // black2gray(o)：重新标灰，但保留其 gclist
    black2gray!(o);

    *gclist = (*g).grayagain;
    (*g).grayagain = o;
  }
}

/// # Safety
/// C ABI 导出壳：`o`/`gclist` 必须分别是由 `luaC_barrierback` 协议擦除为 `c_void` 的真实
/// `GCObject*` 与其 gclist 字段地址，`l` 及 GC 阶段满足内层 `lua_c_barrierback` 契约。
/// 传入任意指针会在还原类型后按错误布局改写 GC 头。cpp lgc.cpp:1473。
pub unsafe extern "C-unwind" fn lua_c_barrierback_export(
  l: *mut LuaState,
  o: *mut c_void,
  gclist: *mut *mut c_void,
) {
  // Safety: 契约保证 `l` 存活且 `t`/`o` 相互一致（o 被 t 引用），屏障仅按协议改灰白标签并入 remark 队列，不越出对象头写界
  unsafe {
    lua_c_barrierback(l, o as *mut GCObject, gclist as *mut *mut GCObject);
  }
}
