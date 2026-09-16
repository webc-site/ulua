// 转换实现已上移至 ulua-cli-lib。两侧原本各有一份逐分支相同的映射（repl 用
// `_ =>`、test 穷举 match，且 `LuarequireNavigateResult` 与
// `luarequire_NavigateResult` 本就是同一枚举的类型别名），现统一指向同一实现，
// 仅以别名保留 repl 侧既有引用路径。
pub use ulua_cli_lib::functions::convert_navigation_status::convert_navigation_status as convert;
pub use ulua_require::enums::luarequire_navigate_result::LuarequireNavigateResult as luarequire_NavigateResult;
