//! cpp `ReplRequirer.cpp` 的 `static jump_to_alias`：仅接受绝对路径。
//! 回调体为 `functions/mod.rs` 模板宏 `requirer_ctx_cb!` 的一次调用，
//! C-ABI 签名与安全契约见宏模板。

use ulua_cli_lib::functions::{
  convert_navigation_status::convert_navigation_status, is_absolute_path::is_absolute_path,
};
use ulua_require::enums::luarequire_navigate_result::LuarequireNavigateResult;

requirer_ctx_cb!(jump_to_alias | req = requirer_mut, path -> LuarequireNavigateResult =>
  if is_absolute_path(&path) {
    convert_navigation_status(req.vfs.reset_to_path(&path))
  } else {
    LuarequireNavigateResult::NAVIGATE_NOT_FOUND
  }
);
