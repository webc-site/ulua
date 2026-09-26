//! cpp `ReplRequirer.cpp` 的 `static reset`：按 requirer chunkname 复位导航目录。
//! 回调体为 `functions/mod.rs` 模板宏 `requirer_ctx_cb!` 的一次调用，
//! C-ABI 签名与安全契约见宏模板；chunkname 可为任意字节，与 cpp 一致地按
//! 字符串比较（非法序列 lossy 替换，见宏内 cstr_cow 步）。

use ulua_cli_lib::functions::convert_navigation_status::convert_navigation_status;
use ulua_require::enums::luarequire_navigate_result::LuarequireNavigateResult;

requirer_ctx_cb!(reset | req = requirer_mut, requirer_chunkname -> LuarequireNavigateResult =>
  if requirer_chunkname == "=stdin" {
    convert_navigation_status(req.vfs.reset_to_std_in())
  } else if let Some(path) = requirer_chunkname.strip_prefix('@') {
    convert_navigation_status(req.vfs.reset_to_path(path))
  } else {
    LuarequireNavigateResult::NAVIGATE_NOT_FOUND
  }
);
