// 实现已上移至 ulua-cli-lib（两侧仅 lua_State 导入路径不同，逻辑逐行相同），
// 此处仅重导出以复用现有引用路径。
pub use ulua_cli_lib::functions::lua_collectgarbage::lua_collectgarbage;
