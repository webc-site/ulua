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
///
/// # Safety
/// `p` 必须来自 `lua_registeruserdatadirectfieldget` 存入的函数指针位模式。
pub unsafe fn from_ptr(p: *mut c_void) -> LuaUserdataDirectFieldGet {
  let mut out = MaybeUninit::<LuaUserdataDirectFieldGet>::uninit();

  unsafe {
    copy_nonoverlapping(
      from_ref(&p).cast::<u8>(),
      out.as_mut_ptr().cast::<u8>(),
      size_of::<*mut c_void>(),
    );
    out.assume_init()
  }
}
