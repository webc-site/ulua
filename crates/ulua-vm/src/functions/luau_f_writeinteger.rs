use core::mem::size_of;

use ulua_common::macros::luau_big_endian::LUAU_BIG_ENDIAN;

use crate::{
  functions::{
    buffer_swapbe::BufferInt,
    buffer_window::{buffer_slot_window_ref, store_scalar_ref},
  },
  macros::{
    checkoutofbounds::checkoutofbounds, luai_num_2_unsigned::luai_num2unsigned,
    luau_f_arm::luau_f_arm,
  },
};

luau_f_arm! {
  /// C++ `luauF_writeinteger<T>`（`lbuiltins.cpp:1387`）：`buffer.write*u*` 的快速调用实现。
  ///
  /// r12 T10 切片核形：取窗经 [`buffer_slot_window_ref`]（`ValueView::Buffer` 既有判据：
  /// 数据区可被写；与库函数共享 `lua_tobuffer.rs` 同一派生链，旧
  /// `as_buffer_ptr().data.as_mut_ptr().add(..)` 手工裸窗已消灭），界检保持
  /// [`checkoutofbounds`] 谓词，回写交 [`store_scalar_ref`]。可观察性逐位一致：本臂
  /// 只在 `!LUAU_BIG_ENDIAN` 下到达回写，`store_scalar_ref` 在该配置即原生序
  /// `write_unaligned`，与旧形 `copy_nonoverlapping` 同字节序；判据失败序
  /// （buffer→offset→value→界检）与旧形逐点相同，越界一律 -1 回退慢路径抛错。
  ///
  /// # Safety
  /// `l` 为当前快速调用的存活 `lua_State`；`arg0` 与 `args` 指向的实参、`res` 结果槽均有效可读/可写，实参数与 `nparams`、结果数与 `nresults` 相符；`args` 为 `None` 表示 FASTCALL1 的单实参派发（无第二参数槽）。
  pub fn luau_f_writeinteger [<T: BufferInt>] (_res, arg0, nresults, args, nparams) => args {
    // SAFETY: 契约保证快速调用帧存活：实参 buffer 可写且值经截断，`res` 为可写结果槽；
    // 取窗借出寿命由 `arg0` 槽钉住（契约见 `buffer_slot_window_ref`）
    unsafe {
      if !LUAU_BIG_ENDIAN && nparams >= 3 && nresults <= 0 {
        // 取窗（≙ 旧 `is_buffer()` 判据：非 buffer 槽返回 None，同样 -1 回退、不抛错）
        let Some(buf) = buffer_slot_window_ref(arg0) else {
          return -1;
        };
        if !(*args).is_number() || !(*args.add(1)).is_number() {
          return -1;
        }

        let offset = (*args).as_number() as i32;
        let access_size = size_of::<T>();
        if checkoutofbounds(offset, buf.len(), access_size) {
          return -1;
        }

        let value = luai_num2unsigned((*args.add(1)).as_number());

        // cpp `T val = T(value)`：数值截断，端序无关
        let val: T = T::from_u32_trunc(value);

        // 校验通过：`[offset, offset+access_size)` 必落数据界内（定位形与
        // `buffer_at_ref` 同源），按规范小端布局回写
        store_scalar_ref(&mut buf[offset as u32 as usize..][..access_size], val);
        return 0;
      }

      -1
    }
  }
}
