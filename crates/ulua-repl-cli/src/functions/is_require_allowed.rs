// 实现已上移至 ulua-cli-lib（`_l` 统一用 `*mut c_void`，与 config 字段一致）。
// 两侧判定条件语义相同（`== "=stdin"` 或首字节 `@`）。
pub use ulua_cli_lib::functions::is_require_allowed::is_require_allowed;
