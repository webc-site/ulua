use ulua_compiler::records::compile_options::CompileOptions as LuauCompileOptions;

pub type CompileOptions = Option<fn() -> LuauCompileOptions>;
