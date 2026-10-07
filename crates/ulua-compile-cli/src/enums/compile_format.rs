#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompileFormat {
  Text,
  Binary,
  Remarks,
  /// Prints annotated native code including IR and assembly
  Codegen,
  /// Prints annotated native code assembly
  CodegenAsm,
  /// Prints annotated native code IR
  CodegenIr,
  /// Prints annotated native code including IR, assembly and outlined code
  CodegenVerbose,
  CodegenNull,
  Null,
}
