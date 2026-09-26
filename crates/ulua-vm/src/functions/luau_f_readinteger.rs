use core::{mem::size_of, ptr::read_unaligned};

use ulua_common::macros::luau_big_endian::LUAU_BIG_ENDIAN;

use crate::{
  macros::{
    checkoutofbounds::checkoutofbounds, luau_f_arm::luau_f_arm, setlvalue::setlvalue,
    setnvalue::setnvalue,
  },
  type_aliases::t_value::TValue,
};

/// 从 buffer `arg0` 按偏移 `args` 读取 `T` 类型标量；入参校验失败或越界返回 `None`。
///
/// # Safety
/// `arg0` 与 `args` 须有效指向当前快速调用帧传入的 `TValue` 槽位。
#[inline(always)]
unsafe fn read_buffer_scalar<T: Copy>(
  arg0: *mut TValue,
  args: *mut TValue,
  nparams: i32,
  nresults: i32,
) -> Option<T> {
  // Safety: 契约保证实参槽可读；界内偏移指针运算由 checkoutofbounds 收口
  unsafe {
    if !LUAU_BIG_ENDIAN
      && nparams >= 2
      && nresults <= 1
      && (*arg0).is_buffer()
      && (*args).is_number()
    {
      let offset = (*args).as_number() as i32;

      let b = (*arg0).as_buffer();
      let len = b.len as usize;
      if checkoutofbounds(offset, len, size_of::<T>()) {
        return None;
      }

      let src = b.data.as_ptr().add(offset as usize) as *const T;
      Some(read_unaligned(src))
    } else {
      None
    }
  }
}

luau_f_arm! {
  /// C++ `luauF_readinteger<T>`（`lbuiltins.cpp:1366`）：`buffer.read*i*` 的快速调用实现。
  ///
  /// # Safety
  /// `l` 为当前快速调用的存活 `lua_State`；`arg0` 与 `args` 指向的实参、`res` 结果槽均有效可读/可写，实参数与 `nparams`、结果数与 `nresults` 相符；`args` 为 `None` 表示 FASTCALL1 的单实参派发（无第二参数槽）。
  pub fn luau_f_readinteger [<T: Copy + Into<f64> >] (res, arg0, nresults, args, nparams) => args {
    // Safety: 契约保证快速调用帧存活：实参 buffer/偏移界内可读，`res` 为可写结果槽
    unsafe {
      if let Some(val) = read_buffer_scalar::<T>(arg0, args, nparams, nresults) {
        setnvalue!(res, val.into());
        1
      } else {
        -1
      }
    }
  }
}

luau_f_arm! {
  /// C++ `luauF_bufferreadlong`（`lbuiltins.cpp:2461`）：`buffer.readinteger` 的快速调用实现。
  ///
  /// # Safety
  /// `l` 为当前快速调用的存活 `lua_State`；`arg0` 与 `args` 指向的实参、`res` 结果槽均有效可读/可写，实参数与 `nparams`、结果数与 `nresults` 相符；`args` 为 `None` 表示 FASTCALL1 的单实参派发（无第二参数槽）。
  pub fn luau_f_bufferreadlong [] (res, arg0, nresults, args, nparams) => args {
    // Safety: 契约保证快速调用帧存活：实参 buffer/偏移界内可读，`res` 为可写结果槽
    unsafe {
      if let Some(val) = read_buffer_scalar::<i64>(arg0, args, nparams, nresults) {
        setlvalue!(res, val);
        1
      } else {
        -1
      }
    }
  }
}
