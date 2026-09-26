//! cpp `ReplRequirer.cpp` 的 `static get_config_status`。
//! 回调体为 `functions/mod.rs` 模板宏 `requirer_ctx_cb!` 的一次调用，
//! C-ABI 签名与安全契约见宏模板。

use ulua_cli_lib::functions::convert_config_status::convert_config_status;
use ulua_require::enums::luarequire_config_status::LuarequireConfigStatus;

requirer_ctx_cb!(get_config_status | req = requirer -> LuarequireConfigStatus =>
  convert_config_status(req.vfs.get_config_status())
);
