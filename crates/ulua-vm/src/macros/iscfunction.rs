//! `iscfunction`（cpp `iscCfunction` 判据）单源转发到安全方法
//! [`crate::records::lua_t_value::TValue::is_c_closure`]：谓词逻辑单点维护于
//! `TValue`，宏体仅保留既有调用点的 `(*slot)` 形状（解引用 unsafe 仍由调用
//! 上下文提供，与收口前一致）。
#[macro_export]
macro_rules! iscfunction {
  ($o:expr) => {
    (*$o).is_c_closure()
  };
}

pub use iscfunction;
