use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::lua_l_addlstring::lua_l_addlstring,
  macros::{maxintsize::MAXINTSIZE, mc::MC, nb::NB, szint::SZINT},
  records::lua_l_strbuf::LuaLStrbuf,
};

/// # Safety
///
/// `b` 须为本次 pack 调用经 `lua_l_buffinit` 初始化、尚未 `pushresult` 的缓冲（写满
/// 扩容义务转单源 [`lua_l_addlstring`]），`size` ≤ MAXINTSIZE；位组装仅触及本地
/// `buff[..size]` 界内字节。
pub(crate) unsafe fn packint(b: &mut LuaLStrbuf, mut n: u64, islittle: i32, size: i32, neg: i32) {
  // SAFETY: 契约保证 `b` 为已接线可写缓冲；字节序位组装落本地 `buff`，追加窗界由
  // 切片 `&buff[..size]` 自带，不触及缓冲外字节
  unsafe {
    LUAU_ASSERT!(size <= MAXINTSIZE);
    // 字节序打包本质是 u8 序列：本地缓冲用 u8，直接交字节切片口追加
    let mut buff = [0u8; MAXINTSIZE as usize];
    buff[if islittle != 0 { 0 } else { size - 1 } as usize] = (n & MC as u64) as u8;

    for i in 1..size {
      n >>= NB as u32;
      buff[if islittle != 0 { i } else { size - 1 - i } as usize] = (n & MC as u64) as u8;
    }

    if neg != 0 && size > SZINT {
      for i in SZINT..size {
        buff[if islittle != 0 { i } else { size - 1 - i } as usize] = MC as u8;
      }
    }

    lua_l_addlstring(b, &buff[..size as usize]);
  }
}
