use crate::enums::compile_format::CompileFormat;

pub fn get_compile_format(name: &str) -> Option<CompileFormat> {
  match name {
    "text" => Some(CompileFormat::Text),
    "binary" => Some(CompileFormat::Binary),
    "remarks" => Some(CompileFormat::Remarks),
    "codegen" => Some(CompileFormat::Codegen),
    "codegenasm" => Some(CompileFormat::CodegenAsm),
    "codegenir" => Some(CompileFormat::CodegenIr),
    "codegenverbose" => Some(CompileFormat::CodegenVerbose),
    "codegennull" => Some(CompileFormat::CodegenNull),
    "null" => Some(CompileFormat::Null),
    _ => None,
  }
}
