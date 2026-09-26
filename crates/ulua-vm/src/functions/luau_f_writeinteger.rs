use core::{mem::size_of, ptr::copy_nonoverlapping};

use ulua_common::macros::luau_big_endian::LUAU_BIG_ENDIAN;

use crate::{
  functions::buffer_swapbe::BufferInt,
  macros::{
    checkoutofbounds::checkoutofbounds, luai_num_2_unsigned::luai_num2unsigned,
    luau_f_arm::luau_f_arm,
  },
};

luau_f_arm! {
  /// C++ `luauF_writeinteger<T>`（`lbuiltins.cpp:1387`）：`buffer.write*u*` 的快速调用实现。
  ///
  /// # Safety
  /// `l` 为当前快速调用的存活 `lua_State`；`arg0` 与 `args` 指向的实参、`res` 结果槽均有效可读/可写，实参数与 `nparams`、结果数与 `nresults` 相符；`args` 为 `None` 表示 FASTCALL1 的单实参派发（无第二参数槽）。
  pub fn luau_f_writeinteger [<T: BufferInt>] (_res, arg0, nresults, args, nparams) => args {
    // Safety: 契约保证快速调用帧存活：实参 buffer 可写且值经截断，`res` 为可写结果槽
    unsafe {
      if !LUAU_BIG_ENDIAN
        && nparams >= 3
        && nresults <= 0
        && (*arg0).is_buffer()
        && (*args).is_number()
        && (*args.add(1)).is_number()
      {
        let offset = (*args).as_number() as i32;

        let b = (*arg0).as_buffer();
        let len = b.len as usize;
        let access_size = size_of::<T>() as usize;
        if checkoutofbounds(offset, len, access_size) {
          return -1;
        }

        let value = luai_num2unsigned((*args.add(1)).as_number());

        // cpp `T val = T(value)`：数值截断，端序无关
        let val: T = T::from_u32_trunc(value);

        let dst = b.data.as_ptr().cast_mut().add(offset as usize) as *mut u8;
        copy_nonoverlapping(&val as *const T as *const u8, dst, size_of::<T>());
        return 0;
      }

      -1
    }
  }
}
