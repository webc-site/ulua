use crate::{
  functions::{lua_s_hash::lua_s_hash, newlstr::newlstr},
  records::{global_state::global_State, lua_state::LuaState, t_string::tstring},
};

/// cpp lstring.cpp:147 `luaS_newlstr` 对应：查串表命中同内容旧串则复用（复活死串），
/// 未命中走 `newlstr` 新分配驻留。长度由 `str_` 切片携带，NUL 结尾不再是入参约定
/// （TString 对象内部布局仍保 NUL，属存储细节而非接口语义）。桶链扫描收拢在
/// `Stringtable::interned`，本函数只做哈希→查表→缺省分配的编排。
///
/// `l` 以引用传入（存活由类型保证）；未命中时 `newlstr` 分配/换表，OOM 以 panic
/// 载荷抛出（受保护帧捕获，穿安全帧即普通 unwind）。返回串表中的存活 TString，
/// 其有效性止于后续 GC 回收或串表重置。
pub fn lua_s_newlstr(l: &mut LuaState, str_: &[u8]) -> *mut tstring {
  let h = lua_s_hash(str_);
  let g: *mut global_State = l.global;
  // SAFETY: `g` 为存活 LuaState 挂接的 global_State（结构不变量）；`interned` 只做
  // 串表桶链查找，`newlstr` 契约随 `l` 存活成立。
  unsafe {
    if let Some(el) = (*g).strt.interned(g, h, str_) {
      return el;
    }

    newlstr(l.as_mut_ptr(), str_, h)
  }
}
