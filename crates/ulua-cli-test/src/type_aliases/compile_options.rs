use ulua_compiler::records::compile_options;
pub type CompileOptions = unsafe extern "C-unwind" fn() -> *mut compile_options::CompileOptions;
