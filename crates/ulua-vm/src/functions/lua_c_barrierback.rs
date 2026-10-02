use core::ffi::c_void;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  macros::{black_2_gray::black2gray, isblack::isblack, isdead::isdead},
  records::{gc_object::GCObject, lua_state::LuaState},
};

/// `luaC_barrierback` 向后写屏障（cpp `lgc.cpp:1473` 系）。r16-v9b：首参仍收 `&mut LuaState`
/// （仿 v5 rawequal/v7 pusherror 形制），`l` 非空/对齐与存活/独占由接收者引用承载，
/// `l.global` 读数为 safe 位；GC 阶段与对象协议前提归 Safety 节承载（unsafe fn 语义）。
///
/// # Safety
/// GC 须处于增量阶段（`gcstate != GCSpause`）；`o` 必须是当前黑色存活对象；`gclist`
/// 必须指向 `o` 自身的 gclist 链接字段（可写）。违反（如对灰对象调用、gclist 指向他处）
/// 会污染 `grayagain` 链，导致活对象被提前清扫。cpp lgc.cpp:1473。
pub unsafe fn lua_c_barrierback(l: &mut LuaState, o: *mut GCObject, gclist: *mut *mut GCObject) {
  // 接收者引用保证 `l` 有效，字段现读系 safe 位。
  let g = l.global;

  // SAFETY: 接收者保证 `l`（及由其派生的 `g`）非空与对齐；`o`/`gclist` 为 GC 协议裸形
  // （跨 crate 函数指针消费，不引用化），isblack/isdead/black2gray 裸读与 `*gclist`、
  // grayagain 链写在 GC 阶段与对象协议界内，其成立前提系上方 Safety 节。
  unsafe {
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
  // SAFETY: 契约保证 `l` 存活且 `t`/`o` 相互一致（o 被 t 引用），`&mut *l` 引用重建仅收形
  // （非空/对齐由本壳调用侧契约承载），屏障仅按协议改灰白标签并入 remark 队列，
  // 不越出对象头写界
  unsafe {
    lua_c_barrierback(&mut *l, o as *mut GCObject, gclist as *mut *mut GCObject);
  }
}
