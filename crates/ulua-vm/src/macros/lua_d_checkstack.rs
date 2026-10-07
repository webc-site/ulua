//! `luaD_checkstack!`（cpp `lvm.h` 同名宏）：栈余量检查 + -grow/-realloc 二选一。
//!
//! B1c 搭车台账（票面 6，裁定＝记录不改）：宏体把 `$l` 展开 4 次且混用
//! `&*$l` / `$l` / `$l.stacksize` 三种形态，正解是折成一处 `&mut LuaState` 入参的小
//! 函数（`stacklimitreached` 已是 `&LuaState` 安全签名，另两枚仍收 `*mut LuaState`）。
//! 本票不动的理由：① 消费面 9 处里 `luau_execute.rs` 的 2 处落在 `vm_protect!` 臂内、
//! `macros/incr_top.rs` 是全 VM 热宏（展开点数十），签名波及远超本票
//! userdata/directfield 半径，与并行票在同一文件冲突；② 折成 `&mut *l` 会把解引用
//! unsafe 从宏体内一处推到 9 个调用点（§2 反向）。留给后续「栈余量族」专票与
//! `lua_d_growstack`/`lua_d_reallocstack` 一并收口（同 [`LuaState`] 引用化后顺手折 fn）。
//!
//! [`LuaState`]: crate::records::lua_state::LuaState
macro_rules! luaD_checkstack {
  ($l:expr, $n:expr) => {
    if $crate::macros::stacklimitreached::stacklimitreached(&*$l, $n) {
      $crate::functions::lua_d_growstack::lua_d_growstack($l, $n);
    } else {
      $crate::macros::condhardstacktests::condhardstacktests!(
        $crate::functions::lua_d_reallocstack::lua_d_reallocstack(
          $l,
          $l.stacksize - $crate::macros::extra_stack::EXTRA_STACK,
          0
        )
      );
    }
  };
}

pub(crate) use luaD_checkstack;
