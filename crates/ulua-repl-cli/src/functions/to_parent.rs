//! cpp `ReplRequirer.cpp` 的 `static to_parent`。
//! 回调体为 `functions/mod.rs` 模板宏 `requirer_ctx_cb!` 的一次调用，
//! C-ABI 签名与安全契约见宏模板。

use ulua_cli_lib::functions::convert_navigation_status::convert_navigation_status;
use ulua_require::enums::luarequire_navigate_result::LuarequireNavigateResult;

requirer_ctx_cb!(to_parent | req = requirer_mut -> LuarequireNavigateResult =>
  convert_navigation_status(req.vfs.to_parent())
);
