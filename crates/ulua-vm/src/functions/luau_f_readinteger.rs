use core::mem::size_of;

use ulua_common::macros::luau_big_endian::LUAU_BIG_ENDIAN;

use crate::{
  functions::{
    buffer_swapbe::SwapBe,
    buffer_window::{buffer_slot_window_ref, load_scalar_ref},
  },
  macros::{
    checkoutofbounds::checkoutofbounds, luau_f_arm::luau_f_arm, setlvalue::setlvalue,
    setnvalue::setnvalue,
  },
  type_aliases::t_value::TValue,
};

/// 从 buffer `arg0` 按偏移 `args` 读取 `T` 类型标量；入参校验失败或越界返回 `None`
/// （快速调用判据失败一律回退 -1 转慢路径，抛错由慢路径库函数承担）。
///
/// r12 T10 切片核形：取窗经 [`buffer_slot_window_ref`]（与库函数共享
/// `lua_tobuffer.rs` 同一派生链，旧 `as_buffer().data.as_ptr().add(..)` 手工裸窗已
/// 消灭），界检保持 [`checkoutofbounds`] 谓词，校验通过后切片定位必然界内，字节装载
/// 交 [`load_scalar_ref`]。可观察性逐位一致：本臂只在 `!LUAU_BIG_ENDIAN` 下到达装载，
/// `load_scalar_ref` 在该配置即 `read_unaligned`（无 `SwapBe` 翻转），与旧形同字节序。
///
/// # Safety
/// `arg0` 与 `args` 须有效指向当前快速调用帧传入的 `TValue` 槽位；取窗借出寿命由
/// `arg0` 槽钉住（契约见 [`buffer_slot_window_ref`]）。
#[inline(always)]
unsafe fn read_buffer_scalar<T: SwapBe>(
  arg0: *mut TValue,
  args: *mut TValue,
  nparams: i32,
  nresults: i32,
) -> Option<T> {
  // SAFETY: 契约保证实参槽可读；取窗走切片核 ref 门面，界检后切片必然界内
  unsafe {
    if !LUAU_BIG_ENDIAN && nparams >= 2 && nresults <= 1 {
      // 取窗（≙ 旧 `is_buffer()` 判据：非 buffer 槽返回 None，同样回退、不抛错）
      let buf = buffer_slot_window_ref(arg0)?;
      if !(*args).is_number() {
        return None;
      }

      let offset = (*args).as_number() as i32;
      if checkoutofbounds(offset, buf.len(), size_of::<T>()) {
        return None;
      }

      // 校验通过：`[offset, offset+size_of::<T>())` 必落数据界内（定位形与
      // `buffer_at_ref` 同源，契约违约暴露为 panic 级索引而非 UB 级 `add`）
      Some(load_scalar_ref::<T>(
        &buf[offset as u32 as usize..][..size_of::<T>()],
      ))
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
  pub fn luau_f_readinteger [<T: SwapBe + Into<f64> >] (res, arg0, nresults, args, nparams) => args {
    // SAFETY: 契约保证快速调用帧存活：实参 buffer/偏移界内可读，`res` 为可写结果槽
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
    // SAFETY: 契约保证快速调用帧存活：实参 buffer/偏移界内可读，`res` 为可写结果槽
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
