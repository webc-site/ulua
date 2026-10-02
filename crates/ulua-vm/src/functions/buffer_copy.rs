use crate::{
  functions::{
    buffer_errors::buffer_oob_error, buffer_window::buffer_data_ref,
    lua_l_optinteger::lua_l_optinteger,
  },
  macros::{isoutofbounds::isoutofbounds, lua_lib_fn::lua_lib_fn},
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须指向本次 buffer 库调用的存活 `LuaState`，索引/长度实参按约定可读，栈顶有压入结果的余量。
pub(crate) unsafe fn buffer_copy(l: *mut LuaState) -> i32 {
  // SAFETY: 契约保证 `l` 为存活调用帧，两段窗均经 `buffer_data_ref` 派生（下方
  // 借用序注记），目标/源区间经 isoutofbounds 三段校验落在各自 buffer 数据界内；
  // 同对象分支走 `copy_within`（底层即 memmove，重叠搬移语义等价），异对象分支
  // 两段数据块由对象同一性判据保证互不相交（见同对象分支实证注记），
  // `copy_from_slice` 仅触达已校验窗口。契约违约（绕过校验）暴露为切片级 panic
  // 而非旧 `core::ptr::copy` 裸形的越界 UB——如实注记的严格改善面。
  unsafe {
    // 抛错序逐位对齐 cpp（lbuflib.cpp:247-270）：#1 typeerror → #2 checkinteger →
    // #3 typeerror → #4/#5 optinteger → size<0 → oob(源) → oob(目标)。
    // 借用安排：同对象时两条 &mut 切片不得并存，故目标窗派生后、源窗派生前即折
    // 为 (ptr, len) 裸量、其 slice 借用到最后一次读为止（NLL）；全程不存在重叠
    // &mut 的读写时刻，相对 cpp 无条件单条 memmove 无可观察差。
    let tbuf = buffer_data_ref(l, 1);
    let toffset = (*l).check_integer(2);
    let tptr = tbuf.as_mut_ptr();
    let tlen = tbuf.len();

    let sbuf = buffer_data_ref(l, 3);
    let soffset = lua_l_optinteger(&mut *l, 4, 0);
    let sptr = sbuf.as_mut_ptr();
    let slen = sbuf.len();

    // C++ evaluates `int(slen) - soffset` as the default eagerly (signed overflow
    // is UB upstream for soffset = INT_MIN); wrapping_sub reproduces the two's-
    // complement value C++ relies on, which the `size < 0` / isoutofbounds checks
    // below then reject. (Upstream UBSan: lbuflib.cpp:257.)
    let size = lua_l_optinteger(&mut *l, 5, (slen as i32).wrapping_sub(soffset));

    if size < 0 {
      buffer_oob_error(l);
    }
    let size = size as usize;

    if isoutofbounds(soffset, slen, size) {
      buffer_oob_error(l);
    }

    if isoutofbounds(toffset, tlen, size) {
      buffer_oob_error(l);
    }

    // 校验通过后截断回绕必然落界内（`as u32 as usize` 与 `buffer_at_ref` 的定位
    // 同款规约：负偏移已在此处折算为大 u32，命中上方 isoutofbounds 抛错）
    let toff = toffset as u32 as usize;
    let soff = soffset as u32 as usize;

    if tptr == sptr {
      // 同一 userdata 对象。判据实证：cpp `luaL_checkbuffer`（laux.cpp:150）转发
      // `lua_tobuffer`（lapi.cpp:650-662）返回 `b->data`，即 `LuauBuffer` 的内联
      // 柔性块（lobject.h:324-330 `alignas(8) char data[1]`）——每对象自有、互不
      // 相交，故数据块首址相等 ⟺ 底层 buffer 对象同一；cpp 本身不做判定、无条件
      // memmove（lbuflib.cpp:268），区间重叠仅在两槽同对象时出现。
      //
      // SAFETY: 上方双 isoutofbounds 保证 [soff, soff+size) 与 [toff, toff+size)
      // 均落在同一块 [0, tlen) 内；此处重新物化唯一一条覆盖整块的 &mut 借用
      // （此前 tbuf/sbuf 两借用在折裸量后均已结束），copy_within 即 memmove 等价
      let buf = core::slice::from_raw_parts_mut(tptr, tlen);
      buf.copy_within(soff..soff + size, toff);
    } else {
      // SAFETY: 两窗各自经 isoutofbounds 校验落界内；异对象的数据块为各自内联自有
      // 块、互不相交（判据实证见同对象分支注记），dst/src 不重叠前提下
      // copy_from_slice（memcpy 形）与 cpp memmove 逐字节同义
      let dst = core::slice::from_raw_parts_mut(tptr.add(toff), size);
      let src = core::slice::from_raw_parts_mut(sptr.add(soff), size);
      dst.copy_from_slice(src);
    }

    0
  }
}

lua_lib_fn!(pub(crate) fn buffer_copy, buffer_copy_arm);
