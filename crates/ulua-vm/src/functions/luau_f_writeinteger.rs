use core::{ffi::c_int, mem::size_of, ptr::copy_nonoverlapping};

use ulua_common::macros::luau_big_endian::LUAU_BIG_ENDIAN;

use crate::{
  functions::buffer_swapbe::BufferInt,
  macros::{
    bufvalue::bufvalue, checkoutofbounds::checkoutofbounds, luai_num_2_unsigned::luai_num2unsigned,
    nvalue::nvalue, ttisbuffer::ttisbuffer, ttisnumber::ttisnumber,
  },
  type_aliases::{lua_state::LuaState, stk_id::StkId, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn luau_f_writeinteger<T: BufferInt>(
  _l: *mut LuaState,
  _res: StkId,
  arg0: *mut TValue,
  nresults: c_int,
  args: StkId,
  nparams: c_int,
) -> c_int {
  unsafe {
    if !LUAU_BIG_ENDIAN
      && nparams >= 3
      && nresults <= 0
      && ttisbuffer!(arg0)
      && ttisnumber!(args)
      && ttisnumber!(args.add(1))
    {
      let offset = nvalue!(args) as c_int;

      let len = bufvalue!(arg0).len as usize;
      let access_size = size_of::<T>() as usize;
      if checkoutofbounds(offset, len, access_size) {
        return -1;
      }

      let value = luai_num2unsigned(nvalue!(args.add(1)));

      // cpp `T val = T(value)`：数值截断，端序无关
      let val: T = T::from_u32_trunc(value);

      let dst = bufvalue!(arg0).data.as_ptr().add(offset as usize) as *mut u8;
      copy_nonoverlapping(&val as *const T as *const u8, dst, size_of::<T>());
      return 0;
    }

    -1
  }
}
