use core::ptr::copy;

use crate::{
  functions::lua_rawcheckstack::lua_rawcheckstack,
  macros::{lua_lib_fn::lua_cont_fn, setbvalue::setbvalue, setobj_2_s::setobj_2_s},
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的接收者必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件（`l` 的存活与独占
/// 前提已由 `&mut` 接收者类型承载，其余为可观察时序契约）。
pub(crate) unsafe fn lua_b_pcallcont(l: &mut LuaState, status: i32) -> i32 {
  unsafe {
    lua_rawcheckstack(l, 1);
    if status == 0 {
      // r12-w7a2 收编（票面特别裁决位）：rawcheckstack 为唯一重分配点，其后窗读
      // `base`/`top` 已预绑定；栈顶抬升改经 advance_top 原语——窗口内仅做已界内槽的
      // copy/setbvalue，无场域写，现读场与绑定窗值恒等，时序逐位不变
      let base = l.base;
      let top = l.top;
      let count = top.offset_from(base) as usize;
      if count > 0 {
        copy(base, base.add(1), count);
      }
      setbvalue!(base, 1);
      l.advance_top(1);
      (count + 1) as i32
    } else {
      // 错误分支：rawcheckstack 后窗读 `top` 单次预绑定，写槽后抬顶同形收编
      let top = l.top;
      setobj_2_s!(l, top, top.sub(1));
      setbvalue!(top.sub(1), 0);
      l.advance_top(1); // 收编：同形抬顶经原语
      2
    }
  }
}

lua_cont_fn!(pub(crate) fn lua_b_pcallcont @ref, lua_b_pcallcont_arm);
