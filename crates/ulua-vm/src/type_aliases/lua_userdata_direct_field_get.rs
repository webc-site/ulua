use core::{
  ffi::c_void,
  mem::{MaybeUninit, size_of},
  ptr::{copy_nonoverlapping, from_ref},
};

/// 宿主运行期登记的 userdata 直接字段取数回调（cpp `lua_UserdataDirectFieldGet`）。
///
/// 裸指针保留：宿主运行期注册回调，类型集合非静态可穷举（review.md §4 保留条款）——
/// 回调由 `lua_registeruserdatadirectfieldget` 在运行期按 tag/field 写入 dispatch gval
/// 槽，编译期无法枚举注册者类型，故函数指针面（`*mut c_void` 的 `ud`/`result` 参数与
/// `extern "C-unwind"` ABI）冻结不改：ABI 必须是 C 约定（跨 FFI 边界由宿主实现），
/// `-unwind` 保证回调内 panic 可穿透宿主栈而不 abort。
pub type LuaUserdataDirectFieldGet =
  Option<unsafe extern "C-unwind" fn(ud: *mut c_void, result: *mut c_void)>;

const _: () = assert!(size_of::<LuaUserdataDirectFieldGet>() == size_of::<*mut c_void>());

/// cpp `(lua_UserdataDirectFieldGet)ptr` 的等价转换。
///
/// 目标类型是 `Option<fn>`：其 niche 就是空指针，空槽安全地读出 `None`；
/// 若 `transmute` 成非 Option 函数指针，空槽即造出非法值并在调用前就是 UB。
/// 源、目标均为本帧定宽槽位，按 `*mut c_void` 宽度复制任意位模式恒安全，故本函数为 safe。
pub fn from_ptr(p: *mut c_void) -> LuaUserdataDirectFieldGet {
  let mut out = MaybeUninit::<LuaUserdataDirectFieldGet>::uninit();

  // SAFETY: 源为局部变量 p 的引用、目标为本帧 out，复制宽度不超出两侧槽位且对齐一致
  unsafe {
    copy_nonoverlapping(
      from_ref(&p).cast::<u8>(),
      out.as_mut_ptr().cast::<u8>(),
      size_of::<*mut c_void>(),
    );
    // Option<fn> 的 niche 为空指针，任意位模式（含全零）皆为合法判别值
    out.assume_init()
  }
}
