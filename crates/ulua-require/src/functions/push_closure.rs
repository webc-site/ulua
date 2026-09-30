use alloc::boxed::Box;
use core::{
  ffi::c_void,
  mem::size_of,
  ptr::{drop_in_place, write},
};

use ulua_common::functions::c_str::cstr;
use ulua_vm::{
  functions::{lua_newuserdatadtor::lua_newuserdatadtor, lua_pushcclosurek::lua_pushcclosurek},
  macros::lua_l_error::luaL_error,
  records::lua_state::LuaState,
  type_aliases::lua_c_function::LuaCFunction,
};

use crate::{
  functions::lua_requirecont::lua_requirecont,
  records::navigation_context::{HostSlot, RequireHost},
};

/// 宿主 Box 的 userdata 析构器（VM GC 终结回调，真边界）：`p` 为
/// [`push_closure`] 以 `size_of::<HostSlot>()` 分配并 placement
/// 构造过的块基址，GC 在该块释放前恰好一次运行。
///
/// # Safety
/// `p` 必须指向 `push_closure` 构造的存活 `HostSlot`。
unsafe extern "C-unwind" fn drop_require_host(ptr: *mut c_void) {
  // Safety: 契约保证 ptr 指向已构造的 Box 本体，drop_in_place 恰好一次。
  unsafe {
    drop_in_place(ptr.cast::<HostSlot>());
  }
}

/// 建立 require 闭包：把宿主机 `host` 装箱为 `HostSlot`（唯一擦除点，理由见其
/// 定义）放进带 GC 析构器的 userdata（与闭包同寿命），再以该 userdata 为唯一
/// upvalue 挂上 `requirelikefunc`（cpp `pushRequireClosureInternal` 的 Rust
/// 形态：配置函数指针表 + 不透明 ctx 两枚 upvalue 合并为一枚类型化宿主句柄）。
///
/// # Safety
/// `l` 必须指向存活的 `LuaState`；`requirelikefunc` 须为静态存活的
/// `LuaCFunction` 闭包体（其运行期按 upvalue(1) 取回本上下文）；`debugname`
/// 须为 NUL 结尾静态字节串（调用点 `b"..\0"`，经 `cstr` 交向
/// `lua_pushcclosurek` 的 C 形参收口）。
pub(crate) unsafe fn push_closure<C: RequireHost + 'static>(
  l: *mut LuaState,
  host: C,
  requirelikefunc: LuaCFunction,
  // NUL 结尾静态字节串（调用点 `b"..\0"`）。
  debugname: &'static [u8],
) -> i32 {
  // Safety: l 为宿主开启库时存活的 LuaState；lua_newuserdatadtor 按 Lua/C API
  // 语义把 userdata 压栈并返回其基址（null 即内存耗尽，判空后 luaL_error 发散），
  // 块大小恰为 size_of::<HostSlot>()、对齐由 VM 保证。
  let ud = unsafe {
    let ud = lua_newuserdatadtor(l, size_of::<HostSlot>(), Some(drop_require_host));
    if ud.is_null() {
      luaL_error!(
        l,
        "failed to allocate memory for require host configuration"
      );
    }
    ud
  };

  // 把装箱的宿主机移交（placement 写入）给 userdata 块，此后其寿命随闭包
  // upvalue（GC 经 drop_require_host 终结）。
  // Safety: ud 为上一句判过非空、同一 size_of 表达式分配的未初始化块，独占且
  // 对齐；写入后该块唯一持有者是栈上的 userdata。
  unsafe { write(ud.cast::<HostSlot>(), Box::new(host)) };

  // Safety: ud 处 userdata 此刻在栈顶，pushcclosurek(n=1) 将其收作唯一 upvalue；
  // debugname 为 NUL 结尾静态串，经 cstr 门面向 lua_pushcclosurek 交出 C 指针；
  // lua_requirecont 为静态存活函数指针。
  unsafe {
    lua_pushcclosurek(
      l,
      requirelikefunc,
      cstr(debugname),
      1,
      Some(lua_requirecont),
    );
  }

  1
}
