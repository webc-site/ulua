use crate::{
  enums::lua_status::LuaStatus,
  functions::{
    auxresumecont::auxresumecont, auxwrapfinish::auxwrapfinish, interrupt_thread::interrupt_thread,
    lua_tothread::lua_tothread,
  },
  macros::{lua_lib_fn::lua_cont_fn, lua_upvalueindex::lua_upvalueindex},
  records::lua_state::LuaState,
};

/// 首参 `l` 的存活/独占前提已由 `&mut` 接收者类型承载（本票收形为引用形，`@ref` 臂于 FFI 一处
/// 重建）；保留 `unsafe fn` 的真前提有两处：其一在协程帧指针 `co` 一侧——`co` 由 `lua_tothread`
/// 自 upvalue 1 号位取回（由建立续体的 `auxwrapy` 保证非空存活），本函数须解引用读 `(*co).status`；
/// 其二在跨帧转手——`interrupt_thread`/`auxresumecont`/`auxwrapfinish` 仍以裸形接收（其并发消费方
/// `auxwrapy`/`auxwrapfinish` 本批只许读），须把 `l` 的自身地址交其重建借用，可抛错/可 GC，故须在
/// 受保护帧。cpp/VM/src/lcorolib.cpp:312 auxwrapcont。
pub(crate) unsafe fn auxwrapcont(l: &mut LuaState, _status: i32) -> i32 {
  let co = lua_tothread(&*l, lua_upvalueindex(1))
    .expect("auxwrapy 建立续体时 upvalue1 为 thread，契约保证非空");

  // SAFETY: `co` 为上方 `expect` 已证的存活协程帧，此处只读其 status 字段
  if unsafe { (*co).status == LuaStatus::Break as u8 } {
    // SAFETY: `l.as_mut_ptr()` 为接收者存活独占借用的自身地址、`co` 同上，借用窗止于本次调用
    return unsafe { interrupt_thread(l.as_mut_ptr(), co) };
  }

  // SAFETY: 两指针前提同上；`auxresumecont`/`auxwrapfinish` 的可 GC/可抛错前提即其自身 `# Safety`
  let r = unsafe { auxresumecont(l.as_mut_ptr(), co) };
  unsafe { auxwrapfinish(l.as_mut_ptr(), r) }
}

lua_cont_fn!(pub(crate) fn auxwrapcont @ref, auxwrapcont_arm);
