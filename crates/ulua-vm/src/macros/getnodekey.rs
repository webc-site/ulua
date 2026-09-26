//! Source: `VM/src/lobject.h:521-529` (hand-checked)
//!
//! §11 pass C3：tt 写经 C1 收口方法 `TValue::set_tt`（cpp 裸赋值 `i_o->tt =` 的 Rust
//! 对应，语义即 `self.tt = tt`，零行为变更）；value/extra 拷贝与尾部 checkliveness
//! 次序不动。
//!
//! C++ copies `n_->key.tt` — the 4-bit `tt` BITFIELD only, not the packed
//! `tt|next` word (the original translation wrote `key.tt_next` into `tt`,
//! smearing the next-pointer bits into the type tag). The Rust `TKey` packs
//! the bitfields as `tt_next` with a `tt()` accessor.

#[macro_export]
macro_rules! getnodekey {
  ($l:expr, $obj:expr, $node:expr) => {{
    let i_o: *mut $crate::type_aliases::t_value::TValue = $obj;
    let n_: *const $crate::records::lua_node::LuaNode = $node;
    (*i_o).value = (*n_).key.value;
    core::ptr::copy_nonoverlapping(
      (*n_).key.extra.as_ptr(),
      (*i_o).extra.as_mut_ptr(),
      (*i_o).extra.len(),
    );
    (*i_o).set_tt((*n_).key.tt());
    $crate::macros::checkliveness::checkliveness!((*$l).global, i_o);
  }};
}

pub use getnodekey;
