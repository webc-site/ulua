use core::{
  ffi::c_void,
  mem::{MaybeUninit, size_of},
  ptr::{copy_nonoverlapping, from_ref},
};

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

  // Safety: 源为局部变量 p 的引用、目标为本帧 out，复制宽度不超出两侧槽位且对齐一致
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
