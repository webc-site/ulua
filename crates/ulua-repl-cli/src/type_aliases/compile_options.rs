use ulua_compiler::records::compile_options::CompileOptions as LuauCompileOptions;

/// cpp `ReplRequirer.h:16` 的 `using CompileOptions = Luau::CompileOptions (*)()`。
pub type CompileOptions = fn() -> LuauCompileOptions;
