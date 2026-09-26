//! cpp `ReplRequirer.cpp` 的 `static is_module_present`。
//! 回调体为 `functions/mod.rs` 模板宏 `requirer_ctx_cb!` 的一次调用，
//! C-ABI 签名与安全契约见宏模板。

use ulua_cli_lib::functions::is_file::is_file;

requirer_ctx_cb!(is_module_present | req = requirer -> bool => is_file(&req.vfs.real_path));
